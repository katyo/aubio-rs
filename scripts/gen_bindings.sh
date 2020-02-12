#!/bin/sh

name=aubio
crate=aubio-sys
#targets="arm-unknown-linux-gnueabi aarch64-unknown-linux-gnu i686-unknown-linux-gnu x86_64-unknown-linux-gnu mips-unknown-linux-gnu mips64-unknown-linux-gnuabi64 powerpc-unknown-linux-gnu powerpc64-unknown-linux-gnu sparc64-unknown-linux-gnu wasm32-unknown-unknown"
# blacklisted: sparc-unknown-linux-gnu

targets="i686-unknown-linux-gnu x86_64-unknown-linux-gnu"

cd $crate
#cargo clean

for target in $targets; do
    cargo build --features generate-bindings --target $target
done

cd -

rm -f $crate/src/bindings_*.rs

for target in $targets; do
    cp target/$target/debug/build/$crate-*/out/bindings.rs $crate/src/bindings_$(echo $target | sed -r 's/^([^-]+).*$/\1/' | sed 's/i686/x32/' | sed 's/x86_64/x64/').rs
done
