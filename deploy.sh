#!/bin/bash

echo -e "\e[1;33m BUILDING... Installing and setting up cargo \e[0m"
curl https://sh.rustup.rs -sSf -y | sh &&
rustup install stable &&
rustup default stable &&
echo -e "\e[1;32m BUILDING... Cargo Installed sucessfully \e[0m"

echo -e "\e[1;33m BUILDING... Installing wasm-pack \e[0m"
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh &&
echo -e "\e[1;32m BUILDING... wasm-pack Installed sucessfully \e[0m"

echo -e "\e[1;33m BUILDING... Installing wasm-bindgen \e[0m"
cargo install -f wasm-bindgen-cli &&
echo -e "\e[1;32m BUILDING... wasm-bindgen Installed sucessfully \e[0m"

./build.sh