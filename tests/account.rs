mod common;

use common::prelude::*;

#[test]
fn test_new_account_has_keypair() {
    let testkit = create_testkit();
    let helper = testkit.helper();
    let accounts = helper.generate_accounts(2);
    assert_eq!(accounts.len(), 2);
    for acc in &accounts {
        assert!(acc.public_key.to_hex().len() > 0);
        assert!(acc.secret_key.to_hex().len() > 0);
    }
    helper.cleanup_accounts(accounts);
}

#[test]
fn test_register_and_activate_account() {
    let testkit = create_testkit();
    let helper = testkit.helper();
    let name = helper.generate_full_name();
    let reg_token = helper.register_account(&name, &helper.generate_email(), &helper.generate_phone_num());
    let account = helper.activate_account(reg_token, 0.0, "123");
    assert_eq!(account.full_name, name);
    helper.cleanup_account(account);
}

#[test]
#[should_panic(expected = "activate account: NotFound(\"Not found\")")]
fn test_activate_account_invalid_token() {
    let testkit = create_testkit();
    let helper = testkit.helper();

    let name = helper.generate_full_name();

    let reg_token = helper.register_account(&name, &helper.generate_email(), &helper.generate_phone_num());
    // this should panic
    let _ = helper.activate_account(reg_token + "invalid", 0.0, "123");
}

macro_rules! test_register_empty_param {
    ($name:ident, $error_msg:tt, (($helper:ident)| $($rega:tt)* )  ) => {
        #[test]
        #[should_panic(expected=$error_msg)]
        fn $name() {
            let testkit = create_testkit();
            let $helper = testkit.helper();

            $($rega)*;
        }
    };
}

test_register_empty_param!(
    test_register_account_empty_name_param,
    "create account: BadRequest(\"Invalid parameter: full name cannot be empty\")",
        ((helper)|
            let _ = helper.register_account(
                "",
                &helper.generate_email(),
                &helper.generate_phone_num(),
            )
        )
);

test_register_empty_param!(
    test_register_account_empty_email_param,
    "create account: BadRequest(\"Invalid parameter: email cannot be empty\")",
        ((helper)|
            let _ = helper.register_account(
                &helper.generate_full_name(),
                "",
                &helper.generate_phone_num(),
            )
        )
);

test_register_empty_param!(
    test_register_account_empty_phone_num_param,
    "create account: BadRequest(\"Invalid parameter: phone_num cannot be empty\")",
        ((helper)|
            let _ = helper.register_account(
                &helper.generate_full_name(),
                &helper.generate_email(),
                "",
            )
        )
);
