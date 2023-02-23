# Substrate contracts

This is a collection of contract samples.

## Feature

- Devcontainer
- Swanky CLI 1.0.11
- ink4
- OpenBrush

## Usage

- Set up

```
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup component add rust-src --toolchain nightly
rustup default nightly
rustup component add rustfmt

cargo --config install cargo-dylint dylint-link
cargo --config install --force --locked cargo-contract

npm install -g npm@9.5.0
npm install -g @astar-network/swanky-cli@1.0.11
```

At token/ directory

- Build Contract

```
swanky contract compile psp22 -v
```

- Build TS Type

```

npx typechain-polkadot --in artifacts/ --out ./typedContract
```

- Start Swanky Local Node

```
token/bin/swanky-node
```

- Deploy Contract

```
swanky contract deploy psp22 --account alice --gas 10000000 --args true
```
