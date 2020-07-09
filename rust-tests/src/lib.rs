use elrond_wasm::Address;
use elrond_wasm::CallableContract;
use elrond_wasm_debug::AccountData;
use elrond_wasm_debug::ArwenMockRef;
use elrond_wasm_debug::HashMap;
use elrond_wasm_debug::TxResult;

pub mod contract;
pub use contract::{CallBuilder, Contract};

pub trait MockRefExtensions {
    fn deploy_contract(
        &self,
        caller: &Address,
        target: &Address,
        contract: Box<dyn CallableContract>,
    ) -> (Contract, TxResult);
    fn new_test_account(&self, address: &Address);
}

impl MockRefExtensions for ArwenMockRef {
    fn deploy_contract(
        &self,
        caller: &Address,
        target: &Address,
        contract: Box<dyn CallableContract>,
    ) -> (Contract, TxResult) {
        Contract::new_deployed(self, contract, caller, target)
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

pub trait TxResultExtensions {
    fn ok(&self) -> bool;
}

impl TxResultExtensions for TxResult {
    fn ok(&self) -> bool {
        self.result_status == 0
    }
}

#[macro_export]
macro_rules! true_result {
    () => {
        [0x1u8; 1].to_vec()
    };
}

#[macro_export]
macro_rules! false_result {
    () => {
        [0x0u8; 0].to_vec()
    };
}
