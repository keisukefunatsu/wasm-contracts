FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

USER vscode

RUN cargo install cargo-dylint dylint-link
RUN cargo install --force --locked cargo-contract