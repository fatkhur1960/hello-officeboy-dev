#![allow(dead_code, unused_variables)]

use apf_testkit::TestKit;
use sodiumoxide;

use std::env;

pub mod prelude {
    //pub use actix_web::test::*;
    pub use super::{create_testkit, setup};
    pub use apf_testkit::{TestHelper, TestKit, TestKitApi};
}

pub fn setup() {
    let _ = env_logger::try_init();
    sodiumoxide::init().expect("Cannot initialize sodiumoxide");
}

pub fn create_testkit() -> TestKit {
    setup();
    // env::set_var("DATABASE_URL", "postgresql://localhost/apf_test?sslmode=disable");

    TestKit::new()
}
