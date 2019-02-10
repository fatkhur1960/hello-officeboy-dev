use apf::api::payment::models::*;
use apf::api::payment::{ActivateAccount, Credit, RegisterAccount, TxQuery};
use apf::api::SuccessReturn;
use apf::auth;
use apf::crypto::*;
use apf::models;
use apf::prelude::*;
use apf::schema_op::*;
use apf::util;
use diesel::{connection::Connection, pg::PgConnection};

use crate::{ApiKind, TestKit, TestKitApi};

use std::env;

pub struct AccountWithKey {
    pub account: Account,
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl AccountWithKey {
    pub fn new(account: Account, public_key: PublicKey, secret_key: SecretKey) -> Self {
        Self {
            account,
            public_key,
            secret_key,
        }
    }
}

pub struct TestHelper {
    testkit: TestKit,
}

impl TestHelper {
    pub fn new(testkit: &TestKit) -> Self {
        Self {
            testkit: testkit.clone(),
        }
    }

    fn get_db() -> PgConnection {
        PgConnection::establish(&env::var("DATABASE_URL").unwrap()).expect("Cannot connect to db")
    }

    pub fn get_account_by_id(&self, id: ID) -> Result<models::Account> {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        schema.get_account(id)
    }

    pub fn gen_access_token_for(&self, id: ID) -> Result<models::AccessToken> {
        let db = Self::get_db();
        let schema = auth::Schema::new(&db);
        schema.generate_access_token(id).map_err(From::from)
    }

    pub fn cleanup_registered_account(&self, token: &str) {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        let _ = schema.cleanup_registered_account(token);
    }

    pub fn generate_full_name(&self) -> String {
        // @TODO(robin): mungkin nantinya gunakan tool seperti ini: https://github.com/fnichol/names ?
        util::random_string(10)
    }

    pub fn generate_amount(&self) -> f64 {
        util::random_number_f64()
    }

    pub fn generate_email(&self) -> String {
        format!("{}@{}.com", util::random_string(10), util::random_string(5)).to_lowercase()
    }

    pub fn generate_phone_num(&self) -> String {
        let nums: String = (0..10).map(|_| util::random_number().to_string()).collect();
        format!("+628{}", nums)
    }

    /// Menggenerasikan beberapa akun sekaligus,
    /// ini tidak via rest API, tapi langsung ke database.
    pub fn generate_accounts(&self, count: usize, random_balance: bool) -> Vec<AccountWithKey> {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        let mut rv = vec![];
        let balance = if random_balance {
            self.generate_amount()
        } else {
            0.0
        };
        for _ in 0..count {
            let new_account = NewAccount {
                full_name: &self.generate_full_name(),
                balance: balance,
                email: &self.generate_email(),
                phone_num: &self.generate_phone_num(),
                active: true,
                register_time: util::now(),
            };
            let (account, (public_key, secret_key)) = schema
                .create_account(&new_account)
                .expect("cannot create account");
            rv.push(AccountWithKey::new(account.into(), public_key, secret_key));
        }
        rv
    }

    /// Menghapus akun berdasarkan ID.
    pub fn cleanup_account_by_id(&self, account_id: ID) {
        let db = Self::get_db();
        let schema = TestSchema::new(&db);
        let _ = schema.delete_account_by_id(account_id);
    }

    /// Menghapus akun
    pub fn cleanup_account(&self, account: Account) {
        self.cleanup_account_by_id(account.id);
    }

    pub fn cleanup_accounts(&self, accounts: Vec<AccountWithKey>) {
        let db = Self::get_db();
        let schema = TestSchema::new(&db);
        schema.cleanup_accounts(accounts.iter().map(|a| a.account.id).collect());
    }
}

pub struct ApiHelper<'a> {
    testkit: &'a TestKit,
}

impl<'a> ApiHelper<'a> {
    pub fn new(testkit: &'a TestKit) -> Self {
        Self { testkit }
    }

    /// Register account
    /// Mengembalikan token untuk aktivasi.
    pub fn register_account(&self, account_name: &str, email: &str, phone_number: &str) -> String {
        let api = self.testkit.api();

        let data = RegisterAccount {
            full_name: account_name.to_owned(),
            email: email.to_owned(),
            phone_num: phone_number.to_owned(),
        };

        api.public(ApiKind::Payment)
            .query(&data)
            .post::<SuccessReturn<String>>("v1/account/register")
            .expect("create account")
            .result
    }

    /// Aktivasi akun menggunakan token yang telah didapat dari hasil register.
    pub fn activate_account(&self, token: String, password: &str) -> Account {
        let api = self.testkit.api();

        let data = ActivateAccount {
            token,
            password: password.to_owned(),
        };

        api.public(ApiKind::Payment)
            .query(&data)
            .post::<Account>("v1/account/activate")
            .expect("activate account")
    }

    pub fn credit_account_balance(&self, account_id: ID, amount: f64, secret_key: &SecretKey) {
        let mut api = self.testkit.api();

        // login-kan
        api.authorize(account_id);

        let mut credit = Credit::new();
        credit.set_account(account_id);
        credit.set_amount(amount);
        credit.set_timestamp(util::current_time_millis());
        credit.set_seed(util::random_number() as u64);

        let data = TxQuery::new(credit).sign(secret_key);

        let result = api
            .private(ApiKind::Payment)
            .query(&data)
            .post::<api::ApiResult>("v1/credit")
            .expect("credit account");

        assert_eq!(result.code, 0);
    }
}
