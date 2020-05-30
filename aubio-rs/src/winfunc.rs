use crate::{ffi, vec::FVecMut, AsNativeStr, Error, Result};

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/**
 * The window function type
 *
 * See:
 * - [Window function](http://en.wikipedia.org/wiki/Window_function) on Wikipedia
 * - Amalia de Götzen, Nicolas Bernardini, and Daniel Arfib. Traditional (?)
 *   implementations of a phase vocoder: the tricks of the trade. In Proceedings of
 *   the International Conference on Digital Audio Effects (DAFx-00), pages 37–44,
 *   Uni- versity of Verona, Italy, 2000.
 *   [pdf](http://www.cs.princeton.edu/courses/archive/spr09/cos325/Bernardini.pdf)
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowType {
    Ones,
    Rectangle,
    Hamming,
    Hanning,
    Hanningz,
    Blackman,
    BlackmanHarris,
    Gaussian,
    Welch,
    Parzen,
}

impl Default for WindowType {
    /**
     * Hanningz window by default
     */
    fn default() -> Self {
        WindowType::Hanningz
    }
}

impl AsNativeStr for WindowType {
    fn as_native_str(&self) -> &'static str {
        use self::WindowType::*;

        match self {
            Ones => "ones\0",
            Rectangle => "rectangle\0",
            Hamming => "hamming\0",
            Hanning => "hanning\0",
            Hanningz => "hanningz\0",
            Blackman => "blackman\0",
            BlackmanHarris => "blackman_harris\0",
            Gaussian => "gaussian\0",
            Welch => "welch\0",
            Parzen => "parzen\0",
        }
    }
}

impl AsRef<str> for WindowType {
    fn as_ref(&self) -> &'static str {
        self.as_rust_str()
    }
}

impl Display for WindowType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.as_ref().fmt(f)
    }
}

impl FromStr for WindowType {
    type Err = Error;

    fn from_str(src: &str) -> Result<Self> {
        use self::WindowType::*;

        Ok(match src {
            "ones" => Ones,
            "rectangle" => Rectangle,
            "hamming" => Hamming,
            "hanning" => Hanning,
            "hanningz" => Hanningz,
            "blackman" => Blackman,
            "blackman_harris" => BlackmanHarris,
            "gaussian" => Gaussian,
            "welch" => Welch,
            "parzen" => Parzen,
            _ => return Err(Error::InvalidArg),
        })
    }
}

impl WindowType {
    /**
     * Set elements of a vector to window coefficients
     */
    pub fn set<'a, W>(&self, window: W)
    where
        W: Into<FVecMut<'a>>,
    {
        let mut window = window.into();
        unsafe { ffi::fvec_set_window(window.as_mut_ptr(), self.as_native_cstr() as *mut _) };
    }
}
