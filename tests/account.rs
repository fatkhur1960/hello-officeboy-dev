mod common;

use common::create_testkit;

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
