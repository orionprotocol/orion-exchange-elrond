#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
imports!();

mod order_status;

use order_status::OrderStatus;

type Bytes32 = [u8; 32];

#[elrond_wasm_derive::contract(OrionExchangeImpl)]
pub trait OrionExchange {

    /*----------  ownable  ----------*/
    
    #[view(owner)]
    #[storage_get("owner")]
    fn get_owner(&self) -> Address;
    
    #[storage_set("owner")]
    fn set_owner(&self, address: &Address);
    
    fn owner_is_caller(&self) -> bool {
        self.get_caller() == self.get_owner()
    }

    #[event("0x0000000000000000000000000000000000000000000000000000000000000001")]
    fn ownership_transferred(&self, previousOwner: &Address, newOwner: &Address);

    #[endpoint(transferOwnership)]
    fn transfer_ownership(&self, newOwner: &Address) {
        if self.owner_is_caller() {
            self.ownership_transferred(&self.get_owner(), newOwner); // event
            self.set_owner(newOwner)
        }
    }

    /*-------------------------------*/

    // mapping: (order_hash: Bytes32) => (orderStatus)
    #[storage_set("order_status")]
    fn set_trade_status(&self, order_hash: &Bytes32, status: &OrderStatus);

    #[storage_get("order_status")]
    fn get_trade_status(&self, order_hash: &Bytes32) -> OrderStatus;


    #[init]
    fn init(&self) {
        let initializer_address: Address = self.get_caller();
        self.set_owner(&initializer_address);
    }
}
