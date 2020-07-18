imports!();

use crate::order_status::OrderStatus;

#[elrond_wasm_derive::module(EventsModuleImpl)]
pub trait EventsModule {
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
}
