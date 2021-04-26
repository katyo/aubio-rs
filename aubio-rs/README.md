# Safe _aubio_ library bindings

[![github](https://img.shields.io/badge/github-katyo/aubio--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/aubio-rs)
[![crate](https://img.shields.io/crates/v/aubio-rs.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/aubio-rs)
[![docs](https://img.shields.io/badge/docs.rs-aubio--rs-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/aubio-rs)
[![GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/GPL-3.0)
[![CI](https://img.shields.io/github/workflow/status/katyo/aubio-rs/Rust?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/aubio-rs/actions?query=workflow%3ARust)

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

* __bindgen__ Force generate bindings itself instead of use pre-generated (_useful for unsupported archs_)
* __builtin__ Force compile builtin _aubio_ C-library
* __pkg-config__ Use _pkg-config_ to find installed libraries
* __shared__ Build shared _aubio_ C-library
* __static__ Build static _aubio_ C-library
* __fftw3__ Enable using _fftw3_ library

When __pkg-config__ feature is used the installed __aubio__ library will be used if found.
To force build and link builtin version you can use __builtin__ feature.
