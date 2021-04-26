/*!
# Unsafe _aubio_ library bindings

This crate provides generated unsafe Rust bindings to [_aubio_](//github.com/aubio/aubio) C library.

Probably this isn't that you really need. See [safe bindings](https://crates.io/crates/aubio-rs).
 */

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![cfg_attr(test, allow(deref_nullptr))]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
include!(concat!("bindings/", env!("AUBIO_BINDINGS")));
