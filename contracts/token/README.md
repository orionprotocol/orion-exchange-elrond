# Elrond test token contract

This code has been taken from the [Elrond Rust examples repo](https://github.com/ElrondNetwork/sc-examples-rs)

**IMPORTANT! - The code for the token contract has not been audited. Not for production use.**

## Contract Interface

This contract exposes a very similar interface to that set by the ERC20 standard but with a few differences. Differences from the ERC20 standard will be highlighted in the specification

### Methods

```rust
totalSupply() -> BigUint
```

```rust
balanceOf(address: Address) -> BigUint
```

```rust
transfer(to: Address, ammount: BigUint) -> Result<(), error> 
```

```rust
transferFrom(sender: Address, recipient: Address, ammount: BigUint) -> Result<(), error> 
```

```rust
approve(spender: Address, amount: BigUint) -> Result<(), error>
```

```rust
allowance(owner: Address, spender: Address) -> BigUint
```

### Events

These differs from the ERC20 specification in that Elrond requires events to identify themselves by an explicit address. The listening code will have to listen for events at the addresses listed below.

```rust
// 0x0000000000000000000000000000000000000000000000000000000000000001
transfer_event(
    sender: Address,
    recipient: Address,
    amount: BigUint
)
```


```rust
// 0x0000000000000000000000000000000000000000000000000000000000000002
approve_event(
    sender: Address,
    recipient: Address,
    amount: BigUint
)
```