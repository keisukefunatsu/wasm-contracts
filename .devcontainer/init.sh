#!/bin/bash
echo "[net]" >$CARGO_HOME/config.toml
echo "git-fetch-with-cli = true" >>$CARGO_HOME/config.toml

rustup default stable
rustup update
rustup update nightly
rustup component add rust-src
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup default nightly

cargo install cargo-dylint dylint-link
cargo install --force --locked cargo-contract

npm install -g npm@9.5.0
npm install -g @astar-network/swanky-cli@1.0.11
