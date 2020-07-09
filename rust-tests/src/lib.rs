use elrond_wasm::Address;
use elrond_wasm::CallableContract;
use elrond_wasm_debug::AccountData;
use elrond_wasm_debug::ArwenMockRef;
use elrond_wasm_debug::HashMap;


pub mod call_builder;
pub use call_builder::{CallBuilder, Contract};

pub trait TestHelpers {
    fn deploy_contract(
        &self,
        caller: &Address,
        target: &Address,
        contract: Box<dyn CallableContract>,
    ) -> Contract;
    fn new_test_account(&self, address: &Address);
}

impl TestHelpers for ArwenMockRef {
    fn deploy_contract(
        &self,
        caller: &Address,
        target: &Address,
        contract: Box<dyn CallableContract>,
    ) -> Contract {
        
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
