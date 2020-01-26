#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "generate-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(not(feature = "generate-bindings"), target_pointer_width = "32"))]
include!("bindings_x32.rs");

#[cfg(all(not(feature = "generate-bindings"), target_pointer_width = "64"))]
include!("bindings_x64.rs");

#[cfg(all(
    not(feature = "generate-bindings"),
    not(target_arch = "x86"),
    not(target_arch = "x86_64"),
    not(target_arch = "arm"),
    not(target_arch = "aarch64"),
    not(target_arch = "mips"),
    not(target_arch = "mips64"),
    not(target_arch = "powerpc"),
    not(target_arch = "powerpc64"),
    not(target_arch = "sparc"),
    not(target_arch = "sparc64"),
))]
compile_error!("Missing pre-generated bindings for specific target arch. Try to use 'generate-bindings' feature.");
