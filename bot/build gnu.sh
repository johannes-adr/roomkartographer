#!/bin/bas
#RUST_LOG=trace cargo run
# export CC="/Users/johannesader/ibi/x-tools/arm-unknown-linux-gnueabihf/bin/arm-unknown-linux-gnueabihf-gcc"
# sudo cargo build --target=arm-unknown-linux-gnueabihf --release

export PKG_CONFIG_ALLOW_CROSS=1
export PATH="/Users/johannesader/ibi/x-tools/arm-unknown-linux-gnueabihf/bin":$PATH
cargo build --target=armv7-unknown-linux-gnueabihf --release

# echo $PATH
# /Users/johannesader/ibi/x-tools/arm-unknown-linux-gnueabihf/bin/arm-unknown-linux-gnueabihf-gcc --version
