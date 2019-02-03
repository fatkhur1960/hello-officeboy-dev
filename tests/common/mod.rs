use sodiumoxide;

use apf::api::payment::*;
use apf_testkit::{ApiKind, TestKit};
use env_logger;

use std::env;

pub mod prelude {
    pub use actix_web::test::*;
}

pub fn setup() {
    env_logger::init();
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

    // pub fn generate_users(&self) -> (alice, bob) {
    //     let api = self.testkit.api()
    //         .public(ApiKind::Payment)
    //         .post()

    // }
}
