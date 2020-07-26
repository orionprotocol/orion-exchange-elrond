#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

imports!();

use common::require;

#[elrond_wasm_derive::contract(TokenImpl)]
pub trait Token {
    /*----------  state  ----------*/

    #[view(totalSupply)]
    #[storage_get("total_supply")]
    fn get_total_supply(&self) -> BigUint;
    #[storage_set("total_supply")]
    fn set_total_supply(&self, total_supply: &BigUint);

    #[view(balanceOf)]
    #[storage_get_mut("balance")]
    fn get_mut_balance(&self, address: &Address) -> mut_storage!(BigUint);
    #[storage_set("balance")]
    fn set_balance(&self, address: &Address, balance: &BigUint);

    #[view(allowance)]
    #[storage_get_mut("allowance")]
    fn get_mut_allowance(&self, owner: &Address, spender: &Address) -> mut_storage!(BigUint);
    #[storage_set("allowance")]
    fn set_allowance(&self, owner: &Address, spender: &Address, allowance: &BigUint);

    #[storage_get("owner")]
    fn get_owner(&self) -> Address;
    #[storage_set("owner")]
    fn set_owner(&self, owner: &Address);

    /*----------  public  ----------*/

    #[endpoint]
    fn transfer(&self, to: &Address, amount: &BigUint) -> SCResult<()> {
        // the sender is the caller
        let sender = self.get_caller();
        self.perform_transfer(&sender, to, amount)
    }

    #[endpoint(transferFrom)]
    fn transfer_from(&self, sender: &Address, recipient: &Address, amount: &BigUint) -> SCResult<()> {
        let caller = self.get_caller();
        let mut allowance = self.get_mut_allowance(sender, &caller);
        // require!(*amount > 0, "Zero value transfers not allowed");
        require!(amount <= &*allowance, "allowance exceeded");
        *allowance -= amount; // saved automatically at the end of scope
        self.perform_transfer(sender, recipient, amount)
    }

    #[endpoint]
    fn approve(&self, spender: &Address, amount: &BigUint) -> SCResult<()> {
        let caller = self.get_caller();
        self.set_allowance(&caller, spender, amount);
        self.approve_event(&caller, spender, amount);
        Ok(())
    }

    #[endpoint]
    fn mint(&self, recipient: &Address, amount: &BigUint) -> SCResult<()> {
        sc_try!(self.abort_if_owner_not_caller());
        {
            let mut recipient_balance = self.get_mut_balance(&recipient);
            *recipient_balance += amount; // saved automatically at the end of scope
        }
        Ok(())
    }

    /*----------  internal  ----------*/

    fn perform_transfer(&self, sender: &Address, recipient: &Address, amount: &BigUint) -> SCResult<()> {        
        {
            let mut sender_balance = self.get_mut_balance(sender);
            require!(amount <= &*sender_balance, "insufficient funds");
            *sender_balance -= amount;
        }
        {
            let mut recipient_balance = self.get_mut_balance(&recipient);
            *recipient_balance += amount;
        }
        self.transfer_event(sender, recipient, amount);
        Ok(())
    }

    fn abort_if_owner_not_caller(&self) -> SCResult<()> {
        require!(self.get_caller() == self.get_owner(), "Must be called by owner");
        Ok(())
    }

    /*----------  events  ----------*/

    #[event("0x0000000000000000000000000000000000000000000000000000000000000001")]
    fn transfer_event(&self, sender: &Address, recipient: &Address, amount: &BigUint);

    #[event("0x0000000000000000000000000000000000000000000000000000000000000002")]
    fn approve_event(&self, sender: &Address, recipient: &Address, amount: &BigUint);

    /*------------------------------*/

    #[init]
    fn init(&self) {
        let creator = self.get_caller();
        self.set_owner(&creator);
    }
}
