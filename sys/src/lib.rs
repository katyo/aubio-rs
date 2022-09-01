#![doc = include_str!("../README.md")]
#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    clippy::redundant_static_lifetimes // TODO: Remove later when bindgen resolve this issue
)]
#![cfg_attr(test, allow(deref_nullptr))]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
include!(concat!("bindings/", env!("AUBIO_BINDINGS")));
