
use apf::api::payment::{ActivateAccount, RegisterAccount};
use apf::api::SuccessReturn;
use apf::prelude::*;
use apf::schema_op::*;
use apf::models::*;
use apf::crypto::*;
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

    pub fn register_account(&self, account_name: &str, email: &str, phone_number: &str) -> ID {
        let api = self.testkit.api();

        let data = RegisterAccount {
            full_name: account_name.to_owned(),
            email: email.to_owned(),
            phone_num: phone_number.to_owned(),
        };

        api.public(ApiKind::Payment)
            .query(&data)
            .post::<SuccessReturn<ID>>("v1/account/register")
            .expect("create account")
            .result
    }

    pub fn activate_account(&self, reg_id: ID, initial_balance: f64, password: &String) -> ID {
        let api = self.testkit.api();

        let data = ActivateAccount {
            reg_id,
            initial_balance,
            password: password.to_owned(),
        };

        api.public(ApiKind::Payment)
            .query(&data)
            .post::<SuccessReturn<ID>>("v1/account/activate")
            .expect("create account")
            .result
    }

    fn get_db() -> PgConnection {
        PgConnection::establish(&env::var("DATABASE_URL").unwrap()).expect("Cannot connect to db")
    }

    pub fn cleanup_registered_account(&self, reg_id: ID) {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        let _ = schema.cleanup_registered_account(reg_id);
    }

    pub fn generate_full_name() -> String {
        // @TODO(robin): mungkin nantinya gunakan tool seperti ini: https://github.com/fnichol/names ?
        util::random_string(10)
    }

    pub fn generate_amount() -> f64 {
        util::random_number_f64()
    }

    pub fn generate_email() -> String {
        format!("{}@{}.com", util::random_string(10), util::random_string(5))
            .to_lowercase()
    }

    pub fn generate_phone_num() -> String {
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
                full_name: &Self::generate_full_name(),
                balance: Self::generate_amount(),
                email: &Self::generate_email(),
                phone_num: &Self::generate_phone_num(),
                active: true,
                register_time: util::now(),
            };
            let (account, (public_key, secret_key)) = schema
                .create_account(&new_account)
                .expect("cannot create account");
            rv.push(AccountWithKey::new(account, public_key, secret_key));
        }
        rv
    }

    pub fn cleanup_accounts(&self, accounts: Vec<AccountWithKey>) {
        let db = Self::get_db();
        let schema = TestSchema::new(&db);

        let accounts: Vec<Account> = accounts.iter().map(|a| a.account.clone()).collect();
        schema.cleanup_accounts(accounts);
    }
}

