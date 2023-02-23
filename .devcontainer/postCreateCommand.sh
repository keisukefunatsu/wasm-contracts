#!/bin/bash

echo github.com ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBEmKSENjQEezOmxkZMy7opKgwFB9nkt5YRrYMjNuG5N87uRgg6CLrbo5wAdT/y6v0mKV0U2w0WZ2YB/++Tpockg= >>/home/vscode/.ssh/known_hosts

git config --global "user.name" "john"
git config --global "user.email" "john@example.com"

cargo --config "net.git-fetch-with-cli = true" install cargo-dylint dylint-link
cargo --config "net.git-fetch-with-cli = true" install --force --locked cargo-contract

npm install -g npm@9.5.0
npm install -g @astar-network/swanky-cli@1.0.11
