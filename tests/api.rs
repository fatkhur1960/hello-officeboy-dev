extern crate apf;
//#[macro_use]
extern crate apf_testkit;
extern crate env_logger;
extern crate log;
extern crate sodiumoxide;

use apf::api::SuccessReturn;
use apf_testkit::ApiKind;

mod common;

#[test]
fn test_get_info() {
    let testkit = common::create_testkit();
    let api = testkit.api();

    assert_eq!(
        api.public(ApiKind::Payment)
            .get::<SuccessReturn<String>>("v1/info")
            .unwrap(),
        SuccessReturn::new("success".to_string())
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
