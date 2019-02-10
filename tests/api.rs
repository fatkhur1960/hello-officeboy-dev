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
use apf_testkit::{ApiKind, TestKitApi};

use apf::{
    api::payment::{Transfer, TxQuery},
    crypto::SecretKey,
    models, util,
};

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
    let ah = testkit.api_helper();

    let token = ah.register_account("Akmal", "akmal@gmail.com", "+62857898122");
    h.cleanup_registered_account(&token);
    assert!(token.len() > 0);
}

#[test]
fn test_credit_account_balance() {
    let testkit = create_testkit();
    let h = testkit.helper();
    let ah = testkit.api_helper();
    let accs = h.generate_accounts(1, false);
    let acc = accs.iter().next().unwrap();
    let sk = &acc.secret_key;

    let acc = h.get_account_by_id(acc.account.id).unwrap();
    assert_eq!(acc.balance, 0.0);

    ah.credit_account_balance(acc.id, 10.0, sk);

    let acc = h.get_account_by_id(acc.id).unwrap();
    assert_eq!(acc.balance, 10.0);
}

fn test_transfer<F>(func: F)
where
    F: FnOnce(
        &mut TestKitApi,
        &models::Account,
        &SecretKey,
        &models::Account,
        &SecretKey,
        Transfer,
    ) -> TxQuery<Transfer>,
{
    let testkit = create_testkit();
    let mut api = testkit.api();
    let h = testkit.helper();
    let ah = testkit.api_helper();
    let accounts = h.generate_accounts(2, false);
    let mut iter = accounts.iter();
    let ac1 = iter.next().unwrap();
    let ac2 = iter.next().unwrap();
    let ac1key = &ac1.secret_key;
    let ac2key = &ac2.secret_key;
    let ac1 = h.get_account_by_id(ac1.account.id).unwrap();
    let ac2 = h.get_account_by_id(ac2.account.id).unwrap();
    assert_eq!(ac1.balance, 0.0);
    assert_eq!(ac2.balance, 0.0);

    // lakukan topup dulu ke akun 1
    // agar bisa transfer ke akun 2
    ah.credit_account_balance(ac1.id, 20.0, &ac1key);

    let transfer = Transfer::new();

    let data = func(&mut api, &ac1, &ac1key, &ac2, &ac2key, transfer);

    api.assert_success(
        &api.public(ApiKind::Payment)
            .query(&data)
            .post::<JsonValue>("v1/transfer")
            .expect("transfer"),
    );

    // check
    let ac1 = h.get_account_by_id(ac1.id).unwrap();
    let ac2 = h.get_account_by_id(ac2.id).unwrap();
    assert_eq!(ac1.balance, 5.0);
    assert_eq!(ac2.balance, 15.0);
}

#[test]
#[should_panic(expected = "transfer: BadRequest(\"Bad request: message has no signature.\")")]
fn test_transfer_without_signature() {
    test_transfer(|api, ac1, _, ac2, _, mut transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac2.id);
        transfer.set_amount(15.0);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        TxQuery::new(transfer)
    });
}

#[test]
#[should_panic(expected = "transfer: BadRequest(\"Unauthorized\")")]
fn test_transfer_invalid_signature() {
    test_transfer(|api, ac1, ac1key, ac2, _ac2key, mut transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac2.id);
        transfer.set_amount(15.0);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        let mut data = TxQuery::new(transfer).sign(ac1key);

        // di sini data ditamper setelah signing
        // sehingga seharusnya membuat signature tidak lagi valid.
        data.body.set_amount(100.0);

        data
    });
}

#[test]
#[should_panic(expected = "transfer: BadRequest(\"Unauthorized\")")]
fn test_transfer_invalid_signer() {
    test_transfer(|api, ac1, _ac1key, ac2, ac2key, mut transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac2.id);
        transfer.set_amount(15.0);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        // pake key-nya ac2
        let mut data = TxQuery::new(transfer).sign(ac2key);

        // di sini data ditamper setelah signing
        // sehingga seharusnya membuat signature tidak lagi valid.
        data.body.set_amount(100.0);

        data
    });
}

#[test]
#[should_panic(expected = "transfer: BadRequest(\"Invalid parameter: Invalid amount\")")]
fn test_transfer_minus_amount() {
    test_transfer(|api, ac1, ac1key, ac2, _ac2key, mut transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac2.id);
        transfer.set_amount(-0.1);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        TxQuery::new(transfer).sign(ac1key)
    });
}

#[test]
#[should_panic(expected = "transfer: BadRequest(\"Max limit reached\")")]
fn test_transfer_max_amount() {
    test_transfer(|api, ac1, ac1key, ac2, _ac2key, mut transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac2.id);
        transfer.set_amount(1_000_000_001f64);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        TxQuery::new(transfer).sign(ac1key)
    });
}

#[test]
#[should_panic(expected = "transfer: BadRequest(\"Bad request: Invalid target\")")]
fn test_transfer_from_and_to_is_same() {
    test_transfer(|api, ac1, ac1key, _ac2, _ac2key, mut transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac1.id);
        transfer.set_amount(15.0);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        TxQuery::new(transfer).sign(ac1key)
    });
}

#[test]
fn test_transfer_valid() {
    test_transfer(|api, ac1, ac1key, ac2, _ac2key, mut transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac2.id);
        transfer.set_amount(15.0);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        TxQuery::new(transfer).sign(ac1key)
    });
}
