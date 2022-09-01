#![doc = include_str!("../README.md")]

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

/**
 * Sample data type
 */
pub type Smpl = ffi::smpl_t;

#[macro_export]
macro_rules! farr {
    ($len: expr) => {
        [0. as $crate::Smpl; $len]
    };
}

#[macro_export]
macro_rules! carr {
    ($len: expr) => {
        [0. as $crate::Smpl; $len + 2]
    };
}
