/*
* This is how calling out to other contracts works in elrond wasm.
* A calling contract needs to define a proxy/interface to the callee contract
* so that type safety can be preserved
*
* This is a proxy to any token style contract that implements 'transferFrom'
*/

imports!();

