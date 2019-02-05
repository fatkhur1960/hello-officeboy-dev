#![allow(dead_code, unused_variables)]

use sodiumoxide;
use apf_testkit::TestKit;

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
    env::set_var("DATABASE_URL", "postgresql://localhost/apf_test?sslmode=disable");

    TestKit::new()
}
