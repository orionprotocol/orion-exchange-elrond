use orion_token_elrond::TokenImpl;

use elrond_wasm_debug::*;
use orion_exchange_elrond_tests::TestHelpers;

type Address = [u8; 32]; // a bit weird to override this def but makes the tests more readable

static TOKEN: Address = [0x11u8; 32];
static OWNER: Address = [0x22u8; 32];
static NOTOWNER: Address = [0x33u8; 32];

macro_rules! true_result {
	() => {
		[0x1u8; 1].to_vec()
	};
}

macro_rules! false_result {
	() => {
		[0x0u8; 0].to_vec()
	};
}

fn deploy_token_contract(mock_ref: &ArwenMockRef) -> TxResult {
	mock_ref.new_test_account(&OWNER.into());
    mock_ref.deploy_contract(
        &OWNER.into(),
        &TOKEN.into(),
        Box::new(TokenImpl::new(mock_ref.clone())),
    )
}

fn init() -> ArwenMockRef {
	let mock_ref = ArwenMockState::new();
	mock_ref.new_test_account(&OWNER.into());
	mock_ref.new_test_account(&NOTOWNER.into());
	deploy_token_contract(&mock_ref);
	mock_ref
}

fn new_contract_call(function_name: &'static str, caller: &Address) -> TxData {
	TxData::new_call(
        function_name, 
        caller.into(), 
        TOKEN.into())
}

#[test]
fn check_minter_roles() {
	let mock_ref = init();

	// owner is minter
	let mut tx1 = new_contract_call("isMinter", &OWNER);
	tx1.add_arg(OWNER.to_vec()); // account

	let tx1_result = mock_ref.execute_tx(tx1);
	tx1_result.print();
	assert_eq!(tx1_result.result_status, 0);
	assert_eq!(tx1_result.result_values[0], true_result!(), "owner should be a minter after deployment");

	// non-owner is not minter
	let mut tx2 = new_contract_call("isMinter", &OWNER);
	tx2.add_arg(NOTOWNER.to_vec()); // account

	let tx2_result = mock_ref.execute_tx(tx2);
	tx2_result.print();
	assert_eq!(tx2_result.result_status, 0);
	assert_eq!(tx2_result.result_values[0], false_result!(), "non-owner should not be minter");
} 
