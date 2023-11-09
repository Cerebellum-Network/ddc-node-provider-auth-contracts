# Node Provider Authorization Contracts Collection

These smart contracts provide different models of authorization layer that is involved when a node provider wants to join a cluster

## Dependencies

Note: The ink! version specidied in the [Cargo.toml](https://github.com/Cerebellum-Network/ddc-node-provider-auth-contracts/blob/main/white_list/Cargo.toml) must be compatible with the `pallet_contracts` version, which in its turn depends on the underlying substrate version. Currently, the Devnet, the QAnet and EDC environments support `ink! 3.4.0`

```bash
# Configure the compatible rust toolchain
rustup toolchain install nightly-2023-02-07
rustup component add rust-src --toolchain nightly-2023-02-07
rustup target add wasm32-unknown-unknown --toolchain nightly-2023-02-07

# Install cargo-contract with its dependencies
cargo install cargo-dylint
cargo install dylint-link
cargo install cargo-contract --version 1.5.0 --force --locked

# Install binaryen in a version >= 99
brew install binaryen 
# For Debian\Ubuntu:
# apt-get install binaryen
```

## Building

```bash
# Build DDC Bucket contract
cargo +nightly-2023-02-07 contract build --release --manifest-path white_list/Cargo.toml
```

Note: if you are encountering errors during build process, they may be related to your specific processor's architecture. If this is the case, try out the *Instalation using Docker image* option, [described in the official docs](https://github.com/paritytech/cargo-contract#installation-using-docker-image)