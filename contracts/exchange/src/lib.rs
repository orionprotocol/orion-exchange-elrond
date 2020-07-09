#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(unused_variables)] // TODO: Remove this once stubs are written
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
    /*------  Contract state  -------*/

    // Mapping: (order_hash: Bytes32) => (orderStatus)
    #[view(getOrderStatus)]
    #[storage_get("order_status")]
    fn get_order_status(&self, order_hash: &Bytes32) -> OrderStatus;
    #[storage_set("order_status")]
    fn set_order_status(&self, order_hash: &Bytes32, status: &OrderStatus);

    // Mapping: (order_hash: Bytes32) => (Vec<Trade>)
    #[view(getOrderTrades)]
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
        panic!("not implemented")
    }

    #[view(getFilledAmounts)]
    fn get_filled_amounts(&self, order_hash: &Bytes32) -> Vec<(BigUint, BigUint)> {
        panic!("not implemented")
    }

    #[view(isOrderCancelled)]
    fn is_order_cancelled(&self, order_hash: &Bytes32) -> bool {
        panic!("not implemented")
    }

    #[view(validateOrder)]
    fn validate_order(&self, order_hash: &Bytes32) -> bool {
        panic!("not implemented")
    }

    /*----------  public  ----------*/

    #[endpoint(depositAsset)]
    fn deposit_asset(&self, asset_address: &Address, amount: &BigUint) -> Result<(), SCError> {
        panic!("not implemented")
    }

    #[payable]
    #[endpoint(depositERD)]
    fn deposit_erd(&self, #[payment] payment: &BigUint) -> Result<(), SCError> {
        let caller = self.get_caller();
        let mut balance = self.get_asset_balance(&caller, &ERD_ASSET_ADDRESS.into());
        *balance += payment; // this will be safely updated after the function ends according to Elrond docs
        self.new_asset_deposit(&caller, &ERD_ASSET_ADDRESS.into(), payment); // event
        Ok(())
    }

    #[endpoint]
    fn withdraw(&self, asset_address: &Address, amount: &BigUint) -> Result<(), SCError> {
        panic!("not implemented")
    }

    #[endpoint(fillOrders)]
    fn fill_orders(
        &self,
        buy_order: &Bytes32,
        sell_order: &Bytes32,
        filled_price: &BigUint,
        filled_amount: &BigUint,
    ) -> Result<(), SCError> {
        panic!("not implemented")
    }

    #[endpoint(cancelOrder)]
    fn cancel_order(&self, order_hash: &Bytes32) -> Result<(), SCError> {
        panic!("not implemented")
    }

    /*----------  internal  ----------*/

    fn update_order_balance(
        &self,
        order_hash: &Bytes32,
        filled_amount: &BigUint,
        amount_quote: &BigUint,
        is_buyer: bool,
    ) -> Result<(), SCError> {
        panic!("not implemented")
    }

    fn update_trade(
        &self,
        order_hash: &Bytes32,
        filled_amount: &BigUint,
        filled_price: &BigUint,
    ) -> Result<(), SCError> {
        panic!("not implemented")
    }

    /*----------  events  ----------*/

    #[event("0x0000000000000000000000000000000000000000000000000000000000000001")]
    fn new_asset_deposit(&self, user_address: &Address, asset_address: &Address, amount: &BigUint);

    #[event("0x0000000000000000000000000000000000000000000000000000000000000002")]
    fn new_asset_withdrawl(
        &self,
        user_address: &Address,
        asset_address: &Address,
        amount: &BigUint,
    );

    #[event("0x0000000000000000000000000000000000000000000000000000000000000003")]
    fn new_trade(
        &self,
        buyer: &Address,
        seller: &Address,
        base_asset: &Address,
        quote_asset: &Address,
        filled_price: &BigUint,
        filled_amount: &BigUint,
        amount_quote: &BigUint,
    );

    #[event("0x0000000000000000000000000000000000000000000000000000000000000004")]
    fn order_update(&self, order_hash: &Address, user: &Address, status: &OrderStatus);
    // cannot use Bytes32 in event. Bug?
    /*--------------------------------------*/

    #[init]
    fn init(&self) {}
}
