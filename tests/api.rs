#![allow(unused_mut, unused_variables)]

extern crate apf_testkit;
extern crate env_logger;
extern crate log;
#[macro_use]
extern crate serde_json;

use serde_json::Value as JsonValue;

use apf_testkit::{ApiHelper, ApiKind, TestHelper, TestKitApi};

use apf::{
    api::payment::{Transfer, TxQuery},
    api::ErrorCode,
    crypto,
    crypto::SecretKey,
    models, util,
};

mod common;

use crate::common::create_testkit;

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
    assert!((a - b).abs() < 0.01);
}

#[test]
fn test_register_account() {
    let testkit = create_testkit();
    let h = testkit.helper();
    let ah = testkit.api_helper();

    let rv = ah.register_account("Akmal", "akmal@gmail.com", "+62857898122");
    assert!(rv.code == ErrorCode::NoError as i32);
    let token = rv.result.unwrap();
    h.cleanup_registered_account(&token);
    assert!(token.len() > 0);
}

fn test_credit<F>(func: F)
where
    F: FnOnce(&ApiHelper, &TestHelper, &models::Account, &SecretKey),
{
    let testkit = create_testkit();
    let h = testkit.helper();
    let ah = testkit.api_helper();
    let accs = h.generate_accounts(1, false);
    let acc = accs.iter().next().unwrap();
    let sk = &acc.secret_key;

    let acc = h.get_account_by_id(acc.account.id).unwrap();
    assert_eq!(acc.balance, 0.0);

    func(&ah, &h, &acc, &sk);

    h.cleanup_account_by_id(acc.id);
}

#[test]
// #[should_panic(expected = "credit account: BadRequest(\"Invalid parameter: Invalid amount\")")]
fn test_credit_account_balance_minus_amount() {
    test_credit(|ah, _h, acc, sk| {
        let rv = ah.credit_account_balance(acc.id, -0.001, sk);
        assert_eq!(rv.code, ErrorCode::InvalidParameter as i32);
        assert_eq!(rv.description, "Invalid parameter: Invalid amount".to_string());
    });
}

#[test]
// #[should_panic(expected = "credit account: BadRequest(\"Invalid parameter: Invalid amount\")")]
fn test_credit_account_balance_over_amount() {
    test_credit(|ah, _h, acc, sk| {
        let rv = ah.credit_account_balance(acc.id, 3_000_001f64, sk);
        assert_eq!(rv.code, ErrorCode::InvalidParameter as i32);
        assert_eq!(rv.description, "Invalid parameter: Invalid amount".to_string());
    });
}

#[test]
// #[should_panic(expected = "credit account: BadRequest(\"Unauthorized\")")]
fn test_credit_account_balance_invalid_key() {
    test_credit(|ah, _h, acc, _sk| {
        let (_pk, sk) = crypto::gen_keypair();
        let rv = ah.credit_account_balance(acc.id, 3_000_001f64, &sk);
        assert_eq!(rv.code, ErrorCode::Unauthorized as i32);

        // let acc = h.get_account_by_id(acc.id).unwrap();
        // assert_eq!(acc.balance, 10.0);
    });
}

#[test]
fn test_credit_account_balance_valid() {
    test_credit(|ah, h, acc, sk| {
        ah.credit_account_balance(acc.id, 10.0, sk);

        let acc = h.get_account_by_id(acc.id).unwrap();
        assert_eq!(acc.balance, 10.0);
    });
}

fn test_transfer<F, FAsserter>(func: F, asserter: FAsserter)
where
    F: FnOnce(
        &mut TestKitApi,
        &models::Account,
        &SecretKey,
        &models::Account,
        &SecretKey,
        Transfer,
    ) -> TxQuery<Transfer>,
    FAsserter: FnOnce(&JsonValue, &models::Account, &models::Account, &TestHelper) -> (),
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

    asserter(
        &api.public(ApiKind::Payment)
            .query(&data)
            .post::<JsonValue>("v1/transfer")
            .expect("transfer"),
        &ac1,
        &ac2,
        &h,
    );

    h.cleanup_accounts(vec![ac1.id, ac2.id]);
}

macro_rules! test_transfer_func {
    ($name:ident, $error_code:path, $error_msg:tt, |$api:ident, $ac1:ident, $ac1key:tt, $ac2:ident, $ac2key:tt, $transfer:tt| $($code:tt)+) => {

        #[test]
        // #[should_panic(expected = "transfer: BadRequest(\"Bad request: message has no signature.\")")]
        fn $name() {
            test_transfer(
                |$api, $ac1, $ac1key, $ac2, $ac2key, mut $transfer| {
                    $api.authorize($ac1.id);

                    $($code)+

                },
                |jv, _ac1, _ac2, _h| {
                    assert_eq!(
                        &json!({"code": $error_code as i32,
                            "status": "error",
                            "description": $error_msg,
                            "result": null}),
                        jv
                    )
                },
            );
        }


    };
}

test_transfer_func!(
    test_transfer_without_signature,
    ErrorCode::MessageHasNoSign,
    "Bad request: message has no signature.",
    |api, ac1, _, ac2, _, transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac2.id);
        transfer.set_amount(15.0);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        TxQuery::new(transfer)
    }
);

test_transfer_func!(
    test_transfer_invalid_signature,
    ErrorCode::Unauthorized,
    "Unauthorized",
    |api, ac1, ac1key, ac2, _, transfer| {
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
    }
);

test_transfer_func!(
    test_transfer_minus_amount,
    ErrorCode::InvalidParameter,
    "Invalid parameter: Invalid amount",
    |api, ac1, ac1key, ac2, _, transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac2.id);
        transfer.set_amount(-0.1);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        TxQuery::new(transfer).sign(ac1key)
    }
);

test_transfer_func!(
    test_transfer_max_amount,
    ErrorCode::TxMaxAmountReached,
    "Max limit reached",
    |api, ac1, ac1key, ac2, _, transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac2.id);
        transfer.set_amount(1_000_000_001f64);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        TxQuery::new(transfer).sign(ac1key)
    }
);

test_transfer_func!(
    test_transfer_from_and_to_is_same,
    ErrorCode::FromAndToTargetIsSame,
    "Bad request: Invalid target",
    |api, ac1, ac1key, _ac2, _, transfer| {
        api.authorize(ac1.id);

        transfer.set_from(ac1.id);
        transfer.set_to(ac1.id);
        transfer.set_amount(15.0);
        transfer.set_timestamp(util::current_time_millis());
        transfer.set_seed(util::current_time_millis());

        TxQuery::new(transfer).sign(ac1key)
    }
);

#[test]
fn test_transfer_valid() {
    test_transfer(
        |api, ac1, ac1key, ac2, _ac2key, mut transfer| {
            api.authorize(ac1.id);

            transfer.set_from(ac1.id);
            transfer.set_to(ac2.id);
            transfer.set_amount(15.0);
            transfer.set_timestamp(util::current_time_millis());
            transfer.set_seed(util::current_time_millis());

            TxQuery::new(transfer).sign(ac1key)
        },
        |_jv, ac1, ac2, h| {
            // check
            let ac1 = h.get_account_by_id(ac1.id).unwrap();
            let ac2 = h.get_account_by_id(ac2.id).unwrap();
            assert_eq!(ac1.balance, 5.0);
            assert_eq!(ac2.balance, 15.0);
        },
    );
}

// ----- INVOICE RELATED TESTS -------
//
use apf::api::payment::InvoiceItem;

const ID_REF: &'static str = "IV-001";

macro_rules! test_publish_invoice {
    ( $name:ident, |$ah:tt, $ac1:tt, $ac2:tt, $ac1key:tt, $ac2key:tt, $items:tt| $($op:tt)+ ) => {
        #[test]
        fn $name(){
            let testkit = create_testkit();
            let h = testkit.helper();
            let $ah = testkit.api_helper();

            let mut ac_wks = h.generate_accounts(2, true);

            let acwk1 = ac_wks.pop().unwrap();
            let acwk2 = ac_wks.pop().unwrap();

            let $ac1 = acwk1.account.clone();
            let $ac2 = acwk2.account.clone();

            let $ac1key = acwk1.secret_key;
            let $ac2key = acwk2.secret_key;

            let mut $items = vec![InvoiceItem {
                name: util::random_string(10),
                price: util::random_number_f64(),
                ..Default::default()
            }];

            $($op)+
        }
    };
}

test_publish_invoice!(
    test_publish_invoice_bad_key,
    |ah, ac1, ac2, _ac1key, ac2key, items| {
        let rv = ah.publish_invoice(ID_REF, ac1.id, ac2.id, 5.0, 10.0, "none", items, &ac2key);
        assert_eq!(rv.code, 3000);
        assert_eq!(rv.result, None);
    }
);

test_publish_invoice!(
    test_publish_invoice_minus_amount,
    |ah, ac1, ac2, ac1key, _ac2key, items| {
        let rv = ah.publish_invoice(ID_REF, ac1.id, ac2.id, 5.0, -10.0, "none", items, &ac1key);
        assert_eq!(rv.code, 7008);
    }
);

test_publish_invoice!(
    test_publish_invoice_max_amount,
    |ah, ac1, ac2, ac1key, _ac2key, items| {
        let rv = ah.publish_invoice(ID_REF, ac1.id, ac2.id, 5.0, 3_000_001.0, "none", items, &ac1key);
        assert_eq!(rv.code, 7008);
    }
);

test_publish_invoice!(
    test_publish_invoice_issuer_and_to_same,
    |ah, ac1, ac2, ac1key, _ac2key, items| {
        let rv = ah.publish_invoice(ID_REF, ac1.id, ac1.id, 5.0, 10.0, "none", items, &ac1key);
        assert_eq!(rv.code, 4005);
    }
);

test_publish_invoice!(
    test_publish_invoice_minus_discount,
    |ah, ac1, ac2, ac1key, _ac2key, items| {
        let rv = ah.publish_invoice(ID_REF, ac1.id, ac2.id, -5.0, 10.0, "none", items, &ac1key);
        assert_eq!(rv.code, 7009);
    }
);

test_publish_invoice!(
    test_publish_invoice_bad_invoice_items_data,
    |ah, ac1, ac2, ac1key, _ac2key, items| {
        items.push(InvoiceItem {
            name: util::random_string(10),
            price: -util::random_number_f64(),
            ..Default::default()
        });
        let rv = ah.publish_invoice(ID_REF, ac1.id, ac2.id, 5.0, 10.0, "none", items, &ac1key);
        assert_eq!(rv.code, 7010);
    }
);

test_publish_invoice!(
    test_publish_invoice_valid,
    |ah, ac1, ac2, ac1key, _ac2key, items| {
        let rv = ah.publish_invoice(ID_REF, ac1.id, ac2.id, 5.0, 10.0, "none", items, &ac1key);

        assert_eq!(rv.code, 0);
        assert!(rv.result.unwrap() > 0);

        // check for existence
        let rv = ah.get_invocie(rv.result.unwrap(), ac2.id);
        assert_eq!(rv.code, 0);
        let invoice = rv.result.unwrap();
        assert_eq!(invoice.id_ref, ID_REF);
        assert_eq!(invoice.discount, 5.0);
        assert_eq!(invoice.amount, 10.0);
    }
);
