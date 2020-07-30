#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
imports!();

use common::{require, Bytes32};

mod events;
mod order;
mod order_status;
mod token_proxy;
mod trade;

use events::*;
use order::Order;
use order_status::OrderStatus;
use token_proxy::TransferFrom;
use trade::Trade;

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
    #[storage_get("order_trades")]
    fn get_order_trades(&self, order_hash: &Bytes32) -> Vec<Trade<BigUint>>;
    #[storage_set("order_trades")]
    fn set_order_trades(&self, order_hash: &Bytes32, trades: &Vec<Trade<BigUint>>);

    // Mapping: (user_address: Address, asset_address: Address) => BigUInt
    #[view(getBalance)]
    #[storage_get_mut("asset_balance")]
    fn get_asset_balance(&self, asset_address: &Address, user_address: &Address) -> mut_storage!(BigUint);


    /*----------  views  ----------*/

    #[view(getBalances)]
    fn get_balances(&self, asset_addresses: &Vec<Address>, user_address: &Address) -> Vec<BigUint> {
        asset_addresses
            .iter()
            .map(|asset_address| self.get_asset_balance(asset_address, user_address).clone())
            .collect()
    }

    #[view(getOrderTrades)]
    fn get_order_trades_public(&self, order: &Order<BigUint>) -> SCResult<Vec<Trade<BigUint>>> {
        let order_hash = sc_try!(self.hash_order(order));
        Ok(self.get_order_trades(&order_hash))
    }

    #[view(getFilledAmounts)]
    fn get_filled_amounts(&self, order: &Order<BigUint>) -> SCResult<(BigUint, BigUint)> {
        let order_hash = sc_try!(self.hash_order(order));
        Ok(self.get_order_trades(&order_hash).iter().fold(
            (BigUint::zero(), BigUint::zero()),
            |(total_filled, total_fees_paid), trade| {
                (
                    total_filled + trade.filled_amount.clone(),
                    total_fees_paid + trade.fee_paid.clone(),
                )
            },
        ))
    }

    #[view(isOrderCancelled)]
    fn is_order_cancelled(&self, order_hash: &Bytes32) -> bool {
        let order_status = self.get_order_status(order_hash);
        match order_status {
            OrderStatus::Cancelled | OrderStatus::PartiallyCancelled => false,
            _ => true,
        }
    }

    #[view(validateOrder)]
    fn validate_order(&self, order: &Order<BigUint>) -> bool {
        match order.validate() {
            Ok(_) => true,
            _ => false,
        }
    }

    /*----------  public  ----------*/

    #[endpoint(depositAsset)]
    fn deposit_asset(&self, asset_address: &Address, amount: BigUint) -> SCResult<()> {
        let token_contract = contract_proxy!(self, asset_address, TransferFrom);
        token_contract.transferFrom(
            asset_address,
            &self.get_caller(),
            amount.clone(),
            &self.get_caller(),
            &self.get_sc_address(),
            amount,
        );
        Ok(())
    }

    #[payable]
    #[endpoint(depositERD)]
    fn deposit_erd(&self, #[payment] payment: &BigUint) -> SCResult<()> {
        self.asset_deposit(&ERD_ASSET_ADDRESS.into(), &self.get_caller(), payment)
    }

    #[endpoint]
    fn withdraw(&self, asset_address: &Address, amount: &BigUint) -> SCResult<()> {
        let caller = self.get_caller();
        if asset_address == &(ERD_ASSET_ADDRESS.into()) {
            // TODO: can this handle transaction failures?
            self.send_tx(&caller, amount, "");
            self.asset_withdrawl(&ERD_ASSET_ADDRESS.into(), &caller, amount)
        } else {
            let token_contract = contract_proxy!(self, asset_address, TransferFrom);
            token_contract.transfer(
                asset_address,
                &self.get_caller(),
                amount.clone(),
                &self.get_caller(),
                amount.clone(),
            );
            Ok(())
        }
    }

    #[endpoint(fillOrders)]
    fn fill_orders(
        &self,
        buy_order: Order<BigUint>,
        sell_order: Order<BigUint>,
        filled_price: BigUint,
        filled_amount: BigUint,
    ) -> SCResult<()> {
        let amount_quote = filled_amount.clone() * filled_price.clone();

        let buy_order_hash = sc_try!(self.hash_order(&buy_order));
        let sell_order_hash = sc_try!(self.hash_order(&sell_order));

        sc_try!(Order::check_orders_info(
            &buy_order,
            &sell_order,
            &self.get_caller(),
            filled_amount.clone(),
            filled_price.clone(),
            self.get_block_timestamp()
        ));

        require!(!self.is_order_cancelled(&buy_order_hash), "E4");
        require!(!self.is_order_cancelled(&sell_order_hash), "E4");

        // state updates
        sc_try!(self.update_order_balance(
            buy_order.clone(),
            filled_amount.clone(),
            amount_quote.clone(),
            true,
        ));
        sc_try!(self.update_order_balance(
            sell_order.clone(),
            filled_amount.clone(),
            amount_quote.clone(),
            false,
        ));

        sc_try!(self.update_trade(
            &buy_order_hash,
            buy_order.clone(),
            filled_amount.clone(),
            filled_price.clone(),
        ));
        sc_try!(self.update_trade(
            &sell_order_hash,
            sell_order.clone(),
            filled_amount.clone(),
            filled_price.clone(),
        ));

        self.events().new_trade(
            &buy_order.sender_address,
            &sell_order.sender_address,
            &buy_order.base_asset,
            &buy_order.quote_asset,
            &filled_price,
            &filled_amount,
            &amount_quote,
        );

        Ok(())
    }

    #[endpoint(cancelOrder)]
    fn cancel_order(&self, order: &Order<BigUint>) -> SCResult<()> {
        let caller = self.get_caller();
        sc_try!(order.validate());
        require!(order.sender_address == caller, "Not owner");

        let order_hash = sc_try!(self.hash_order(order));

        require!(
            !self.is_order_cancelled(&order_hash),
            "Order already cancelled"
        );

        let (total_filled, _) = sc_try!(self.get_filled_amounts(order));

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

    /*----------  callbacks (used internally)  ----------*/

    #[callback]
    fn asset_deposit_callback(
        &self,
        call_result: AsyncCallResult<()>,
        #[callback_arg] cb_asset_address: &Address,
        #[callback_arg] cb_account_address: &Address,
        #[callback_arg] cb_amount: BigUint,
    ) -> elrond_wasm::SCResult<()> {
        if let AsyncCallResult::Ok(()) = call_result {
            self.asset_deposit(cb_asset_address, cb_account_address, &cb_amount)
        } else {
            sc_error!("Error completing asset deposit")
        }
    }

    #[callback]
    fn asset_withdrawl_callback(
        &self,
        call_result: AsyncCallResult<()>,
        #[callback_arg] cb_asset_address: &Address,
        #[callback_arg] cb_account_address: &Address,
        #[callback_arg] cb_amount: BigUint,
    ) -> elrond_wasm::SCResult<()>  {
        if let AsyncCallResult::Ok(()) = call_result {
            self.asset_withdrawl(cb_asset_address, cb_account_address, &cb_amount)
        } else {
            sc_error!("Error completing asset withdrawl")
        }
    }

    /*----------  internal  ----------*/

    #[inline]
    fn asset_deposit(
        &self,
        asset_address: &Address,
        account_address: &Address,
        amount: &BigUint,
    ) -> SCResult<()> {
        let mut balance = self.get_asset_balance(asset_address, &account_address);
        *balance += amount; // this will be safely updated after the function ends according to Elrond docs
        self.events()
            .new_asset_deposit(&account_address, asset_address, amount); // event
        Ok(())
    }

    #[inline]
    fn asset_withdrawl(
        &self,
        asset_address: &Address,
        account_address: &Address,
        amount: &BigUint,
    ) -> SCResult<()> {
        let mut balance = self.get_asset_balance(asset_address, account_address);
        *balance -= amount;
        self.events()
            .new_asset_withdrawl(account_address, asset_address, amount);
        Ok(())
    }

    #[inline]
    fn hash_order(&self, order: &Order<BigUint>) -> SCResult<Bytes32> {
        if let Result::Ok(order_bytes) = order.top_encode() {
            Ok(self.keccak256(order_bytes.as_slice()))
        } else {
            sc_error!("Error serializing order")
        }
    }

    #[inline]
    fn update_order_balance(
        &self,
        order: Order<BigUint>,
        filled_amount: BigUint,
        amount_quote: BigUint,
        is_buyer: bool,
    ) -> SCResult<()> {
        let user = order.sender_address;
        let matcher_fee =
            order.matcher_fee * filled_amount.clone() / order.amount; // TODO: Check how these operations are handled

        {
            let mut quote_asset_balance = self.get_asset_balance(&user, &order.quote_asset);
            let mut base_asset_balance = self.get_asset_balance(&user, &order.base_asset);

            if is_buyer {
                *quote_asset_balance -= amount_quote;
                *base_asset_balance += filled_amount;
            } else {
                *quote_asset_balance += amount_quote;
                *base_asset_balance -= filled_amount;
            }
        } // balances updated when scope ends

        // deduct the fees and transfer to matcher
        {
            let mut matcher_fee_asset_balance =
                self.get_asset_balance(&user, &order.matcher_fee_asset);
            *matcher_fee_asset_balance -= matcher_fee;
        }
        // TODO: Implement transfer of fees to matcher once there is a nicer tranfer function

        Ok(())
    }

    #[inline]
    fn update_trade(
        &self,
        order_hash: &Bytes32,
        order: Order<BigUint>,
        filled_amount: BigUint,
        filled_price: BigUint,
    ) -> SCResult<()> {
        let matcher_fee = order.matcher_fee.clone() * filled_amount.clone() / order.amount.clone(); // TODO: Check how these operations are handled
        let (total_filled, total_fees_paid) = sc_try!(self.get_filled_amounts(&order));

        require!(&total_filled + &filled_amount <= order.amount, "E3");
        require!(&total_fees_paid + &matcher_fee <= order.matcher_fee, "E3");

        let status = if total_filled.clone() + filled_amount.clone() < order.amount
            && self.get_order_trades(&order_hash).len() > 1
        {
            OrderStatus::PartiallyFilled
        } else if total_filled + filled_amount.clone() == order.amount {
            OrderStatus::Filled
        } else {
            OrderStatus::New
        };

        self.set_order_status(&order_hash, &status);

        let mut order_trades = self.get_order_trades(&order_hash);
        order_trades.push(Trade::new(
            filled_price,
            filled_amount,
            matcher_fee,
            self.get_block_timestamp(),
        ));
        self.set_order_trades(&order_hash, &order_trades);

        self.events()
            .order_update(&order_hash.into(), &order.sender_address, &status);

        Ok(())
    }

    /*---------------------------------*/

    #[module(EventsModuleImpl)]
    fn events(&self) -> EventsModuleImpl<T, BigInt, BigUint>;

    #[init]
    fn init(&self) {}
}
