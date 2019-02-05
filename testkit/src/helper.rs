use apf::api::payment::models::*;
use apf::api::payment::{ActivateAccount, RegisterAccount};
use apf::api::SuccessReturn;
use apf::crypto::*;
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

    /// Register account
    /// Mengembalikan token
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

    pub fn activate_account(&self, token: String, initial_balance: f64, password: &str) -> Account {
        let api = self.testkit.api();

        let data = ActivateAccount {
            token,
            initial_balance,
            password: password.to_owned(),
        };

        api.public(ApiKind::Payment)
            .query(&data)
            .post::<Account>("v1/account/activate")
            .expect("activate account")
    }

    fn get_db() -> PgConnection {
        PgConnection::establish(&env::var("DATABASE_URL").unwrap()).expect("Cannot connect to db")
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
    pub fn generate_accounts(&self, count: usize) -> Vec<AccountWithKey> {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        let mut rv = vec![];
        for _ in 0..count {
            let new_account = NewAccount {
                full_name: &self.generate_full_name(),
                balance: self.generate_amount(),
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
