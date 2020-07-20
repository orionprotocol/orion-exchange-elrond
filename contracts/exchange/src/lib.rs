#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(unused_variables)] // TODO: Remove this once stubs are written
imports!();

use common::{require, Bytes32};

mod events;
mod order;
mod order_status;
mod trade;

use events::*;
use order::Order;
use order_status::OrderStatus;
use trade::Trade;

// ERD by convention is stored at the asset address of all zero in the asset_balance map
static ERD_ASSET_ADDRESS: [u8; 32] = [0; 32];

#[elrond_wasm_derive::callable(TransferFromProxy)]
pub trait TransferFrom {
    #[callback(transferFromCallback)]
    fn transferFrom(
        &self,
        #[callback_arg] cb_asset_address: &Address,
        #[callback_arg] cb_recipient_address: &Address,
        #[callback_arg] cb_amount: BigUint,
        sender: &Address,
        recipient: &Address,
        token_amount: BigUint // really weird bug, using amount as arg name breaks it!!
    );
}

#[elrond_wasm_derive::contract(OrionExchangeImpl)]
pub trait OrionExchange {
    /*------  Contract state  -------*/

    // Mapping: (order_hash: Bytes32) => (orderStatus)
    #[view(getOrderStatus)]
    #[storage_get("order_status")]
    fn get_order_status(&self, order_hash: &Bytes32) -> OrderStatus;
    #[storage_set("order_status")]
    fn set_order_status(&self, order_hash: &Bytes32, status: &OrderStatus);

    // Mapping: (order_hash: Bytes32) => (Vec<Trade>)
    #[storage_get("order_trades")]
    fn get_order_trades(&self, order_hash: &Bytes32) -> Vec<Trade>;
    #[storage_set("order_trades")]
    fn set_order_trades(&self, order_hash: &Bytes32, trades: &Vec<Trade>);

    // Mapping: (user_address: Address, asset_address: Address) => BigUInt
    #[view(getBalance)]
    #[storage_get_mut("asset_balance")]
    fn get_asset_balance(
        &self,
        asset_address: &Address,
        user_address: &Address,
    ) -> mut_storage!(BigUint);
    #[storage_set("asset_balance")]
    fn set_asset_balance(&self, asset_address: &Address, user_address: &Address, balance: BigUint);

    /*----------  views  ----------*/

    #[view(getBalances)]
    fn get_balances(&self, asset_addresses: &Vec<Address>, user: &Address) -> Vec<BigUint> {
        unimplemented!()
    }

    #[view(getOrderTrades)]
    fn get_order_trades_public(&self, order: &Order) -> Vec<Trade> {
        unimplemented!()
    }

    #[view(getFilledAmounts)]
    fn get_filled_amounts(&self, order: &Order) -> (BigUint, BigUint) {
        unimplemented!()
    }

    #[view(isOrderCancelled)]
    fn is_order_cancelled(&self, order_hash: &Bytes32) -> bool {
        unimplemented!()
    }

    #[view(validateOrder)]
    fn validate_order(&self, order: &Order) -> bool {
        match order.validate() {
            Ok(_) => true,
            _ => false,
        }
    }

    /*----------  public  ----------*/

    #[inline]
    fn update_balance(&self, asset_address: &Address, recipient_address: &Address, amount: &BigUint) -> SCResult<()> {
        let mut balance = self.get_asset_balance(asset_address, &recipient_address);
        *balance += amount; // this will be safely updated after the function ends according to Elrond docs
        self.events()
            .new_asset_deposit(&recipient_address, asset_address, amount); // event
        Ok(())
    }

    #[endpoint(depositAsset)]
    fn deposit_asset(&self, asset_address: &Address, amount: BigUint) -> SCResult<()> {
        let token_contract = contract_proxy!(self, asset_address, TransferFrom);
        token_contract.transferFrom(
            asset_address,
            &self.get_caller(),
            amount.clone(),
            &self.get_caller(),
            &self.get_sc_address(),
            amount
        );
        Ok(())
    }

    #[payable]
    #[endpoint(depositERD)]
    fn deposit_erd(&self, #[payment] payment: &BigUint) -> SCResult<()> {
        self.update_balance(&ERD_ASSET_ADDRESS.into(), &self.get_caller(), payment)
    }

    #[endpoint]
    fn withdraw(&self, asset_address: &Address, amount: &BigUint) -> SCResult<()> {
        let caller = self.get_caller();
        if asset_address == &(ERD_ASSET_ADDRESS.into()) {
            self.send_tx(&caller, amount, "")
        } else {
            unimplemented!()
        }
        let mut balance = self.get_asset_balance(asset_address, &caller);
        *balance -= amount;
        self.events()
            .new_asset_withdrawl(&caller, asset_address, amount);
        Ok(())
    }

    #[endpoint(fillOrders)]
    fn fill_orders(
        &self,
        buy_order: &Order,
        sell_order: &Order,
        filled_price: &BigUint,
        filled_amount: &BigUint,
    ) -> SCResult<()> {
        unimplemented!()
    }

    #[endpoint(cancelOrder)]
    fn cancel_order(&self, order: &Order) -> SCResult<()> {
        let caller = self.get_caller();
        sc_try!(order.validate());
        require!(order.sender_address == caller, "Not owner");

        let order_hash = order.get_type_value_hash();

        require!(
            !self.is_order_cancelled(&order_hash),
            "Order already cancelled"
        );

        let (total_filled, _) = self.get_filled_amounts(order);

        if total_filled > 0 {
            self.set_order_status(&order_hash, &OrderStatus::PartiallyCancelled)
        } else {
            self.set_order_status(&order_hash, &OrderStatus::Cancelled)
        }

        self.events().order_update(
            &order_hash.into(),
            &caller,
            &self.get_order_status(&order_hash),
        );
        Ok(())
    }

    /*----------  callbacks  ----------*/

    #[callback]
    fn transferFromCallback(
        &self,
        call_result: AsyncCallResult<()>,
        #[callback_arg] cb_asset_address: &Address,
        #[callback_arg] cb_recipient_address: &Address,
        #[callback_arg] cb_amount: BigUint,
    ) {
        if let AsyncCallResult::Ok(()) = call_result {
            self.update_balance(cb_asset_address, cb_recipient_address, &cb_amount);
        }
    }

    /*----------  internal  ----------*/

    fn update_order_balance(
        &self,
        order: &Order,
        filled_amount: &BigUint,
        amount_quote: &BigUint,
        is_buyer: bool,
    ) -> SCResult<()> {
        unimplemented!()
    }

    fn update_trade(
        &self,
        order_hash: &Bytes32,
        order: &Order,
        filled_amount: &BigUint,
        filled_price: &BigUint,
    ) -> SCResult<()> {
        unimplemented!()
    }

    #[module(EventsModuleImpl)]
    fn events(&self) -> EventsModuleImpl<T, BigInt, BigUint>;

    #[init]
    fn init(&self) {}
}
