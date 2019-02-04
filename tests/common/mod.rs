use sodiumoxide;

use apf::api::payment::{ActivateAccount, RegisterAccount};
use apf::api::SuccessReturn;
use apf::models::*;
use apf::prelude::*;

use apf_testkit::{ApiKind, TestKit};
// use env_logger;
use diesel::{connection::Connection, pg::PgConnection};

use std::env;

pub mod prelude {
    pub use actix_web::test::*;
}

pub fn setup() {
    // env_logger::init();
    sodiumoxide::init().expect("Cannot initialize sodiumoxide");
}

pub fn create_testkit() -> TestKit {
    setup();
    env::set_var(
        "DATABASE_URL",
        "postgresql://localhost/payment_test?sslmode=disable",
    );

    TestKit::new()
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

    // pub fn generate_users(&self) -> (alice, bob) {
    //     let api = self.testkit.api()
    //         .public(ApiKind::Payment)
    //         .post()

    // }
}
