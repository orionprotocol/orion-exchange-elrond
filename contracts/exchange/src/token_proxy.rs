imports!();

#[elrond_wasm_derive::callable(TransferFromProxy)]
pub trait TransferFrom {
    #[callback(asset_deposit_callback)]
    fn transferFrom(
        &self,
        #[callback_arg] cb_asset_address: &Address,
        #[callback_arg] cb_account_address: &Address,
        #[callback_arg] cb_amount: BigUint,
        sender: &Address,
        recipient: &Address,
        token_amount: BigUint
    );

    #[callback(asset_withdrawl_callback)]
    fn transfer(&self,
        #[callback_arg] cb_asset_address: &Address,
        #[callback_arg] cb_account_address: &Address,
        #[callback_arg] cb_amount: BigUint,
        to: &Address,
        token_amount: BigUint
    );
}
