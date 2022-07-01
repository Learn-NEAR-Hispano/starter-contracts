#!/bin/sh
set -e

export RUSTFLAGS="-C link-arg=-s --remap-path-prefix $PWD=/pwd --remap-path-prefix $CARGO_HOME=/cargo_home"

cargo build --target wasm32-unknown-unknown --release
mkdir -p res
cp target/wasm32-unknown-unknown/release/*.wasm ./res
