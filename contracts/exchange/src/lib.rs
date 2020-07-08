#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
imports!();

mod order_status;
mod trade;

use order_status::OrderStatus;
use trade::Trade;

type Bytes32 = [u8; 32];

// ERD by convention is stored at the asset address of all zero in the asset_balance map
static ERD_ASSET_ADDRESS: [u8; 32] = [0; 32];

#[elrond_wasm_derive::contract(OrionExchangeImpl)]
pub trait OrionExchange {

    /*----------  Contract state  ----------*/
    
    // Mapping: (order_hash: Bytes32) => (orderStatus)
    #[storage_set("order_status")]
    fn set_order_status(&self, order_hash: &Bytes32, status: &OrderStatus);
    #[storage_get("order_status")]
    fn get_order_status(&self, order_hash: &Bytes32) -> OrderStatus;

    // Mapping: (order_hash: Bytes32) => (Vec<Trade>)
    #[storage_set("order_trades")]
    fn set_order_trades(&self, order_hash: &Bytes32, trades: &Vec<Trade>);
    #[storage_get("order_trades")]
    fn get_order_trades(&self, order_hash: &Bytes32) -> Vec<Trade>;

    // Mapping: (user_address: Address, asset_address: Address) => BigUInt
    #[storage_set("asset_balance")]
    fn set_asset_balance(&self, user_address: &Address, asset_address: Address, balance: BigUint);
    #[storage_get_mut("asset_balance")]
    fn get_asset_balance(&self, user_address: &Address, asset_address: Address) -> mut_storage!(BigUint);    

    /*--------------------------------------*/


    /*-------------  ownable  -------------*/
    
    #[view(owner)]
    #[storage_get("owner")]
    fn get_owner(&self) -> Address;
    
    #[storage_set("owner")]
    fn set_owner(&self, address: &Address);
    
    fn owner_is_caller(&self) -> bool {
        self.get_caller() == self.get_owner()
    }

    #[event("0x0000000000000000000000000000000000000000000000000000000000000010")]
    fn ownership_transferred(&self, previousOwner: &Address, newOwner: &Address);

    #[endpoint(transferOwnership)]
    fn transfer_ownership(&self, newOwner: &Address) {
        if self.owner_is_caller() {
            self.ownership_transferred(&self.get_owner(), newOwner); // event
            self.set_owner(newOwner)
        }
    }

    /*--------------------------------------*/

    #[payable]
    #[endpoint(depositERD)]
    fn deposit_erd(&self, #[payment] payment: &BigUint) {
        let caller = self.get_caller();
        let mut balance = self.get_asset_balance(&caller, ERD_ASSET_ADDRESS.into());
        *balance += payment; // this will be safely updated after the function ends according to Elrond docs
        self.new_asset_deposit(&caller, &ERD_ASSET_ADDRESS.into(), payment); // event
    }


    #[init]
    fn init(&self) {
        let initializer_address: Address = self.get_caller();
        self.set_owner(&initializer_address);
    }

    /*----------  events  ----------*/

    #[event("0x0000000000000000000000000000000000000000000000000000000000000001")]
    fn new_asset_deposit(&self, user_address: &Address, asset_address: &Address, amount: &BigUint);
    
}
