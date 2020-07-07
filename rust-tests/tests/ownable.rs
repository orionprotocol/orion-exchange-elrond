use orion_exchange_elrond::*;

use elrond_wasm_debug::*;
use orion_exchange_elrond_tests::TestHelpers;

static ADDR1: [u8; 32] = [0x11u8; 32];
static ADDR2: [u8; 32] = [0x22u8; 32];

#[test]
fn can_deploy_contract() {
    let mock_ref = ArwenMockState::new();

    mock_ref.new_test_account(&ADDR1.into());

    let tx1_result = mock_ref.deploy_contract(
        &ADDR1.into(),
        &ADDR2.into(),
        Box::new(OrionExchangeImpl::new(mock_ref.clone())),
    );

    assert_eq!(0, tx1_result.result_status);
    tx1_result.print();
}
