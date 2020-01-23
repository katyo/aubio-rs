#!/bin/sh

targets="arm-unknown-linux-gnueabi aarch64-unknown-linux-gnu i686-unknown-linux-gnu x86_64-unknown-linux-gnu mips-unknown-linux-gnu mips64-unknown-linux-gnuabi64 powerpc-unknown-linux-gnu powerpc64-unknown-linux-gnu sparc64-unknown-linux-gnu wasm32-unknown-unknown"
# blacklisted: sparc-unknown-linux-gnu

cd aubio-sys
#cargo clean

for target in $targets; do
    cargo build --features compile-library,generate-bindings --target $target
done

cd ..

for target in $targets; do
    cp target/$target/debug/build/aubio-sys-*/out/bindings.rs aubio-sys/src/bindings_$(echo $target | sed -r 's/^([^-]+).*$/\1/' | sed 's/i686/x86/').rs
done
