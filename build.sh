#!/bin/bash

cd "$(dirname "$0")"

mkdir -p public

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/web.wasm --no-typescript --target web --out-dir ./public --debug
cp ./crates/web/index.html public/