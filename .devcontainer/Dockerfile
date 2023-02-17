FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# RUN apt update && apt install -y protobuf-compiler

USER vscode

RUN cargo install cargo-dylint dylint-link
RUN cargo install cargo-contract

# RUN https://github.com/paritytech/substrate-contracts-node/releases/download/v0.24.0/substrate-contracts-node-linux.tar.gz