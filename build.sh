#!/bin/bash

cd "$(dirname "$0")"

mkdir -p public

cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/web.wasm --no-typescript --target web --out-dir ./public --debug
cp ./crates/web/index.html public/