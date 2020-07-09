

use elrond_wasm_debug::{
	ArwenMockRef,
	TxResult,
	TxData,

};
use elrond_wasm::{Address, CallableContract};

pub struct Contract {
	address: Address, // gets set when it is deployed
}

impl Contract {
	pub fn new_deployed(mock_ref: &ArwenMockRef, contract: Box<dyn CallableContract>, deployer: &Address, destination: &Address) -> Self {
		let deploy_tx = TxData::new_create(contract, deployer.clone(), destination.clone());
		mock_ref.execute_tx(deploy_tx);
		Contract {
			address: destination.clone(),
		}
	}

	pub fn call(&self, fname: &'static str) -> CallBuilder {
		CallBuilder {
			fname,
			caller_address: None,
			contract_address: Some(self.address.clone()),
			args: Vec::new()
		}
	}
}

pub struct CallBuilder {
	contract_address: Option<Address>,
	caller_address: Option<Address>,
	fname: &'static str,
	args: Vec<Vec<u8>>
}

impl CallBuilder {
	pub fn call(fname: &'static str) -> Self {
		CallBuilder {
			contract_address: None,
			caller_address: None,
			fname,
			args: Vec::new()
		}
	}

	pub fn on(mut self, contract_address: &Address) -> Self {
		self.contract_address = Some(contract_address.clone());
		self
	}

	pub fn as_caller(mut self, caller_address: &Address) -> Self {
		self.caller_address = Some(caller_address.clone());
		self
	}

	pub fn with_arg(mut self, arg: Vec<u8>) -> Self {
		self.args.push(arg);
		self
	}

	pub fn exec(self, mock_ref: &ArwenMockRef) -> TxResult {
		let mut tx = TxData::new_call(
	        self.fname, 
	        self.caller_address.expect("Must provide a calling address"), 
	        self.contract_address.expect("Must provide a contract address")
	    );
	    for arg in self.args {
	    	tx.add_arg(arg);
	    }
	    mock_ref.execute_tx(tx)
	}

}
