#!/bin/bas
#RUST_LOG=trace cargo run
# export CC="/Users/johannesader/ibi/x-tools/arm-unknown-linux-gnueabihf/bin/arm-unknown-linux-gnueabihf-gcc"
# sudo cargo build --target=arm-unknown-linux-gnueabihf --release
# xattr -d -r com.apple.quarantine /Users/johannesader/ibi/x-tools/arm-linux-musleabihf-cross/arm-linux-musleabihf
# export PATH="/Users/johannesader/ibi/x-tools/arm-linux-musleabihf-cross/bin":$PATH

# export PKG_CONFIG_ALLOW_CROSS=1
# export CC_armv7-unknown-linux-musleabihf=arm-linux-musleabihf-gcc


export CC=arm-linux-musleabihf-gcc
export CXX=arm-linux-musleabihf-g++
export AR=arm-linux-musleabihf-ar
export RANLIB=arm-linux-musleabihf-ranlib

# Set the sysroot path to your musl-cross installation directory
export SYSROOT=/opt/homebrew/Cellar/musl-cross/0.9.9_1/libexec/arm-linux-musleabihf

# Configure pkg-config for cross-compilation
export PKG_CONFIG_DIR=
export PKG_CONFIG_LIBDIR=${SYSROOT}/lib/pkgconfig:${SYSROOT}/share/pkgconfig
export PKG_CONFIG_SYSROOT_DIR=${SYSROOT}
export PKG_CONFIG_ALLOW_CROSS=1


cargo build --target=armv7-unknown-linux-musleabihf --release