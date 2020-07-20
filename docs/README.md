# orion-exchange-elrond

Exchange contracts for the Orion Protocol implented in Rust to target the Elrond blockchain

## Running tests

The simplest way to build the contract and run tests is with the [erdpy](https://docs.elrond.com/tools/erdpy/installing-erdpy) CLI tool. This will also take care of installing the required dependencies and Rust build tools. Ensure you have this installed before proceeding.

- To build and run tests use the following helper script:

```shell
sh ./scripts/build-and-test.sh
```

This will build the contracts and run the tests with the actual WASM code running in the VM used by Elrond.

- To run the Rust tests:

```shell
cd rust-tests && cargo test
```

## Contributors

### Contributors on GitHub
* [Contributors](https://github.com/orionprotocol/orion-exchange-elrond/graphs/contributors)

## License 
* see [LICENSE](https://github.com/orionprotocol/orion-exchange-elrond/blob/master/LICENSE) file


