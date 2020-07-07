#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(non_snake_case)]

imports!();

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

    #[init]
    fn init(&self) {
        let initializer_address: Address = self.get_caller();
        self.set_owner(&initializer_address);
    }
}
