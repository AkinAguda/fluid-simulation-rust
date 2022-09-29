#!/bin/bash

echo -e "\e[1;33m BUILDING... Installing and setting up cargo \e[0m"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh &&
echo -e "\e[1;32m BUILDING... Cargo Installed sucessfully \e[0m"

echo -e "\e[1;33m BUILDING... Installing wasm-bindgen \e[0m"
cargo install -f wasm-bindgen-cli &&
echo -e "\e[1;32m BUILDING... wasm-bindgen Installed sucessfully \e[0m"

./build.sh