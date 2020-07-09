use elrond_wasm::Address;
use elrond_wasm::CallableContract;
use elrond_wasm_debug::AccountData;
use elrond_wasm_debug::ArwenMockRef;
use elrond_wasm_debug::HashMap;
use elrond_wasm_debug::TxData;
use elrond_wasm_debug::TxResult;

pub trait TestHelpers {
    fn deploy_contract(
        &self,
        caller: &Address,
        target: &Address,
        contract: Box<dyn CallableContract>,
    ) -> TxResult;
    fn new_test_account(&self, address: &Address);
}

impl TestHelpers for ArwenMockRef {
    fn deploy_contract(
        &self,
        caller: &Address,
        target: &Address,
        contract: Box<dyn CallableContract>,
    ) -> TxResult {
        let tx1 = TxData::new_create(contract, caller.clone(), target.clone());
        self.execute_tx(tx1)
    }

    fn new_test_account(&self, address: &Address) {
        self.add_account(AccountData {
            address: address.clone(),
            nonce: 0,
            balance: 0.into(),
            storage: HashMap::new(),
            contract: None,
        })
    }
}


