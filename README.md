# Aubio library bindings

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-brightgreen.svg)](https://opensource.org/licenses/GPL-3.0)
[![Crates.io Package](https://img.shields.io/crates/v/aubio-rs.svg?style=popout)](https://crates.io/crates/aubio-rs)
[![Docs.rs API Docs](https://docs.rs/aubio-rs/badge.svg)](https://docs.rs/aubio-rs)
[![Travis-CI Status](https://travis-ci.com/katyo/aubio-rs.svg?branch=master)](https://travis-ci.com/katyo/aubio-rs)

This projects aims provide safe Rust bindings for [_aubio_](//github.com/aubio/aubio) C library.

> _Aubio_ is a library to label music and sounds.
>
> It listens to audio signals and attempts to detect events.
> For instance, when a drum is hit, at which frequency is a note,
> or at what tempo is a rhythmic melody.
>
> Its features include segmenting a sound file before each of its attacks,
> performing pitch detection, tapping the beat and producing midi streams
> from live audio.
>
> aubio provide several algorithms and routines, including:
>
> * several onset detection methods
> * different pitch detection methods
> * tempo tracking and beat detection
> * MFCC (mel-frequency cepstrum coefficients)
> * FFT and phase vocoder
> * up/down-sampling
> * digital filters (low pass, high pass, and more)
> * spectral filtering
> * transient/steady-state separation
> * sound file read and write access
> * various mathematics utilities for music applications
>
> The name _aubio_ comes from audio with a typo: some errors are likely
> to be found in the results.

## Crate features

The following features can be used to customize crate configuration:

- _generate-bindings_ Runs __bindgen__ to generate bindings (_useful for unsupported archs_)
- _compile-library_ Clones source from __git__ and builds _aubio_ C library
- _fetch-prebuilt_ Forces to fetch prebuilt library (_currently not supported_)
- _dynamic-link_ Forces to use dynamic linking instead of static

By default the installed _aubio_ C library will be used.

You may provide `AUBIO_LIBDIR` environment variable to specify path
to precompiled _aubio_ C libraries.
Also `AUBIO_LIB` environment variable may be used to override the name
of library.
The `AUBIO_INCLUDEDIR` environment variable can help specify path
to C headers of library.

The following features can be used to customize library configuration:

- _with-fftw3f_ Enables floating-point __fftw3__ support
- _with-fftw3_ Enables __fftw3__ support
- _with-wav_ Enables _wavread_/_wavwrite_ support
- _with-jack_ Enables __jack__ support
- _with-sndfile_ Enables __libsndfile__ support
- _with-avcodec_ Enables __libavcodec__ support
- _with-samplerate_ Enables __libsamplerate__ support

Pre-generated bindings supported for the following architectures:

- __x86__
- __x86_64__
- __arm__
- __aarch64__
- __mips__
- __mips64__
- __powerpc__
- __powerpc64__
- __sparc__ _(currently blacklisted)_
- __sparc64__
- __wasm32__
