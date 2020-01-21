use crate::{
    Error,
    Result,
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

impl AsRef<str> for WindowType {
    fn as_ref(&self) -> &'static str {
        use self::WindowType::*;

        match self {
            Ones => "ones",
            Rectangle => "rectangle",
            Hamming => "hamming",
            Hanning => "hanning",
            Hanningz => "hanningz",
            Blackman => "blackman",
            BlackmanHarris => "blackman_harris",
            Gaussian => "gaussian",
            Welch => "welch",
            Parzen => "parzen",
        }
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
