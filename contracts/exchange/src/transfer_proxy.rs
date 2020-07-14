/*
* This is how calling out to other contracts works in elrond wasm.
* A calling contract needs to define a proxy/interface to the callee contract
* so that type safety can be preserved
*
* This is a proxy to any token style contract that implements 'transferFrom'
*/

imports!();

#[elrond_wasm_derive::callable(TransferFromProxy)]
pub trait TransferFrom {
    #[callback(transferFromCallback)]
    fn transferFrom(&self, sender: &Address, recipient: &Address, amount: BigUint);
}
