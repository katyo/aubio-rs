use crate::{
    Error,
    Result,

    AsNativeStr,
};

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/**
 * The window type
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
