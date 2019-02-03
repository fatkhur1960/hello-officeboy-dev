extern crate apf;
#[macro_use]
extern crate log;
extern crate apf_testkit;
extern crate env_logger;
extern crate sodiumoxide;

use actix_web::http::Method;

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
