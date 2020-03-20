/*!
# Bundled aubio library

This crate provides bundled [aubio](https://github.com/aubio/aubio) C library
for using with [__aubio-rs__](https://crates.io/crates/aubio-rs) crate in case
when system-installed library is not available.

## Usage

You can simply add this as dependency to your manifest:

```toml
[dependencies]
aubio-rs = "^0.1"

# Use bundled library to avoid unresolved links
aubio-lib = "^0.1"
```

Next you should say compiler that you want to use that crate:

```rust
// Either in traditional manner
extern crate aubio_lib;

// Or in Rust2018 manner
use aubio_lib as _;
```

## Features

The following features can be used to customize library configuration:

- _shared_ Force bundle shared (or dynamic) library instead of static
- _with-fftw3_ Enables __fftw3__ support
- _nolink-fftw3_ Disable __fftw3__ link
- _shared-fftw3_ Force shared __fftw3__ link

 */
