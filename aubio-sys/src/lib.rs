#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "generate-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(not(feature = "generate-bindings"), target_arch = "x86"))]
include!("bindings_x86.rs");

#[cfg(all(not(feature = "generate-bindings"), target_arch = "x86_64"))]
include!("bindings_x86_64.rs");

#[cfg(all(not(feature = "generate-bindings"), target_arch = "arm"))]
include!("bindings_arm.rs");

#[cfg(all(not(feature = "generate-bindings"), target_arch = "aarch64"))]
include!("bindings_aarch64.rs");
