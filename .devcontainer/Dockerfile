FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

USER vscode

RUN rustup toolchain install nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
RUN rustup default nightly
RUN rustup component add rustfmt

RUN cargo install cargo-dylint dylint-link
RUN cargo install --force --locked cargo-contract