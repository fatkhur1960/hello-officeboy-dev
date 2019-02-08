// extern crate apf;
//#[macro_use]
extern crate apf_testkit;
extern crate env_logger;
extern crate log;
// extern crate sodiumoxide;
#[macro_use]
extern crate serde_json;

use serde_json::Value as JsonValue;

// use apf::api::SuccessReturn;
use apf_testkit::ApiKind;

mod common;

use common::create_testkit;

#[test]
fn test_get_info() {
    let testkit = create_testkit();
    let api = testkit.api();

    assert_eq!(
        api.public(ApiKind::Payment).get::<JsonValue>("v1/info").unwrap(),
        json!({ "version": env!("CARGO_PKG_VERSION") })
    );
}

#[test]
fn test_float_compare() {
    let a = 1.234f64;
    let b = 1.235f64;

    dbg!((a - b).abs());
    dbg!((a - b).abs() < 0.1);

    assert!((a - b).abs() < 0.01);
}

#[test]
fn test_register_account() {
    let testkit = create_testkit();
    let h = testkit.helper();

    let token = h.register_account("Akmal", "akmal@gmail.com", "+62857898122");
    h.cleanup_registered_account(&token);
    assert!(token.len() > 0);
}
