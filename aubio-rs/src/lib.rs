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
 * - _generate-bindings_ which runs __bindgen__ to generate bindings (_useful for unsupported archs_)
 * - _compile-library_ which clones source from __git__ and builds _aubio_ C library
 * - _fetch-prebuilt_ which forces to fetch prebuilt library (_currently not supported_)
 * - _dynamic-link_ which forces to use dynamic linking instead of static
 *
 * By default the installed _aubio_ C library will be used.
 *
 * You may provide `AUBIO_LIBDIR` environment variable to specify path
 * to precompiled _aubio_ C libraries.
 * Also `AUBIO_LIB` environment variable may be used to override the name
 * of library.
 * The `AUBIO_INCLUDEDIR` environment variable can help specify path
 * to C headers of library.
 *
 * The following features can be used to customize library configuration:
 *
 * - _with-fftw3f_ Enables floating-point __fftw3__ support
 * - _with-fftw3_ Enables __fftw3__ support
 * - _with-wav_ Enables _wavread_/_wavwrite_ support
 * - _with-jack_ Enables __jack__ support
 * - _with-sndfile_ Enables __libsndfile__ support
 * - _with-avcodec_ Enables __libavcodec__ support
 * - _with-samplerate_ Enables __libsamplerate__ support
 *
 * Pre-generated bindings supported for the following architectures:
 *
 * - __x86__
 * - __x86_64__
 * - __arm__
 * - __aarch64__
 * - __mips__
 * - __mips64__
 * - __powerpc__
 * - __powerpc64__
 * - __sparc__
 * - __sparc64__
 *
 */

pub(crate) use aubio_sys as ffi;

mod types;
mod winfunc;
mod specdesc;
mod fft;
mod pvoc;
mod onset;
mod pitch;
mod tempo;
mod notes;
mod mfcc;
#[cfg(feature = "with-samplerate")]
mod resampler;
mod log;

pub mod vec;

pub use self::types::*;
pub use self::winfunc::*;
pub use self::specdesc::*;
pub use self::fft::*;
pub use self::pvoc::*;
pub use self::onset::*;
pub use self::pitch::*;
pub use self::tempo::*;
pub use self::notes::*;
pub use self::mfcc::*;
#[cfg(feature = "with-samplerate")]
pub use self::resampler::*;
pub use self::log::*;

#[macro_export]
macro_rules! farr {
    ($len: expr) => {
        [0f32; $len]
    };
}

#[macro_export]
macro_rules! carr {
    ($len: expr) => {
        [0f32; $len+2]
    };
}
