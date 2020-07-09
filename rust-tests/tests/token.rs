use elrond_wasm::Address;
use elrond_wasm_debug::*;
use lazy_static::lazy_static;
use orion_exchange_elrond_tests::{false_result, true_result, Contract, MockRefExtensions, TxResultExtensions};
use orion_token_elrond::TokenImpl;

lazy_static! {
    static ref TOKEN: Address = Address::from_slice(&[0x11u8; 32]);
    static ref OWNER: Address = Address::from_slice(&[0x22u8; 32]);
    static ref NOTOWNER: Address = Address::from_slice(&[0x33u8; 32]);
}

fn deploy_token_contract(mock_ref: &ArwenMockRef) -> (Contract, TxResult) {
    mock_ref.new_test_account(&OWNER);
    mock_ref.deploy_contract(&OWNER, &TOKEN, Box::new(TokenImpl::new(mock_ref.clone())))
}

#[test]
fn check_minter_roles() {
    let mock_ref = ArwenMockState::new();

    mock_ref.new_test_account(&OWNER);
    mock_ref.new_test_account(&NOTOWNER);

    let (token, _) = deploy_token_contract(&mock_ref);

    let tx1 = token
        .call("isMinter")
        .as_caller(&OWNER)
        .with_arg(OWNER.to_vec()) // account
        .exec(&mock_ref);

    assert_eq!(tx1.ok(), true);
    assert_eq!(
        tx1.result_values[0],
        true_result!(),
        "owner should be a minter after deployment"
    );

    let tx2 = token
        .call("isMinter")
        .as_caller(&OWNER)
        .with_arg(NOTOWNER.to_vec()) // account
        .exec(&mock_ref);

    assert_eq!(tx2.ok(), true);
    assert_eq!(
        tx2.result_values[0],
        false_result!(),
        "non-owner should not be minter"
    );
}
