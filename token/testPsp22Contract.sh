#!/bin/bash

swanky contract compile psp22 -v
cp contracts/psp22/target/ink/psp22.json artifacts/psp22.json
cp contracts/psp22/target/ink/psp22.wasm artifacts/psp22.wasm
npx typechain-polkadot --in artifacts/ --out ./typedContract

yarn test
