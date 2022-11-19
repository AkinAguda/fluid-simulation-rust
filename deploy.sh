#!/bin/bash

echo -e "\e[1;33m BUILDING... Installing and setting up cargo \e[0m"
yes | curl https://sh.rustup.rs -sSf | sh  &&
echo -e "\e[1;32m BUILDING... Cargo Installed sucessfully \e[0m"

echo -e "\e[1;33m BUILDING... Installing wasm-bindgen \e[0m"
cargo install -f wasm-bindgen-cli &&
echo -e "\e[1;32m BUILDING... wasm-bindgen Installed sucessfully \e[0m"

rustup target add wasm32-unknown-unknown

./build.sh