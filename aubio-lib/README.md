# Bundled aubio library

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-brightgreen.svg)](https://opensource.org/licenses/GPL-3.0)
[![Crates.io Package](https://img.shields.io/crates/v/aubio-lib.svg?style=popout)](https://crates.io/crates/aubio-lib)
[![Docs.rs API Docs](https://docs.rs/aubio-lib/badge.svg)](https://docs.rs/aubio-lib)
[![Travis-CI Status](https://travis-ci.com/katyo/aubio-rs.svg?branch=master)](https://travis-ci.com/katyo/aubio-rs)

This crate provides bundled [aubio](https://github.com/aubio/aubio) C library
for using with [__aubio-rs__](https://crates.io/crates/aubio-rs) crate in case
when system-installed library is not available.

## Usage

You can simply add this as dependency to your manifest:

```toml
[dependencies]
aubio = "^0.1"

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
- _with-fftw3f_ Enables floating-point __fftw3__ support
- _with-fftw3_ Enables __fftw3__ support
- _with-wav_ Enables _wavread_/_wavwrite_ support
- _with-jack_ Enables __jack__ support
- _with-sndfile_ Enables __libsndfile__ support
- _with-avcodec_ Enables __libavcodec__ support
- _with-samplerate_ Enables __libsamplerate__ support
