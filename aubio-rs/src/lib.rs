/*!
 * # Safe bindings for _aubio_ library
 *
 * > _Aubio_ is a library to label music and sounds.
 * >
 * > It listens to audio signals and attempts to detect events.
 * > For instance, when a drum is hit, at which frequency is a note,
 * > or at what tempo is a rhythmic melody.
 * >
 * > Its features include segmenting a sound file before each of its attacks,
 * > performing pitch detection, tapping the beat and producing midi streams
 * > from live audio.
 * >
 * > aubio provide several algorithms and routines, including:
 * >
 * > * several onset detection methods
 * > * different pitch detection methods
 * > * tempo tracking and beat detection
 * > * MFCC (mel-frequency cepstrum coefficients)
 * > * FFT and phase vocoder
 * > * up/down-sampling
 * > * digital filters (low pass, high pass, and more)
 * > * spectral filtering
 * > * transient/steady-state separation
 * > * sound file read and write access
 * > * various mathematics utilities for music applications
 * >
 * > The name _aubio_ comes from audio with a typo: some errors are likely
 * > to be found in the results.
 *
 * ## Crate features
 *
 * The following features can be used to customize configuration:
 *
 * - __bindgen__ Force generate bindings itself instead of use pre-generated (_useful for unsupported archs_)
 * - __builtin__ Force compile builtin _aubio_ C-library
 * - __pkg-config__ Use _pkg-config_ to find installed libraries
 * - __shared__ Build shared _aubio_ C-library
 * - __static__ Build static _aubio_ C-library
 * - __fftw3__ Enable using _fftw3_ library
 *
 * When __pkg-config__ feature is used the installed __aubio__ library will be used if found.
 * To force build and link builtin version you can use __builtin__ feature.
 */

pub(crate) use aubio_sys as ffi;

mod fft;
mod filterbank;
mod log;
mod mfcc;
mod notes;
mod onset;
mod pitch;
mod pvoc;
mod resampler;
mod specdesc;
mod tempo;
mod types;
mod utils;
mod winfunc;

pub mod vec;

pub use self::fft::*;
pub use self::filterbank::*;
pub use self::log::*;
pub use self::mfcc::*;
pub use self::notes::*;
pub use self::onset::*;
pub use self::pitch::*;
pub use self::pvoc::*;
pub use self::resampler::*;
pub use self::specdesc::*;
pub use self::tempo::*;
pub use self::types::*;
pub use self::utils::*;
pub use self::winfunc::*;

#[macro_export]
macro_rules! farr {
    ($len: expr) => {
        [0f32; $len]
    };
}

#[macro_export]
macro_rules! carr {
    ($len: expr) => {
        [0f32; $len + 2]
    };
}
