#!/usr/bin/ sh

BINARY_FILE="https://github.com/morelucks/bash_scripting/raw/refs/heads/main/target/release/cpc_rust"

echo "downloading binary file"

wget  "$BINARY_FILE"

chmod +x cpc_rust

sudo mv cpc_rust "/usr/local/bin/luckify"



