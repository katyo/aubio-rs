use crate::{
    Error,
    Result,
    Status,

    ffi,
    check_init,
    vec::{
        CVec,
        FVecMut,
    },
};

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/**
 * Spectral description function
 */
pub trait SpecFunc {
    fn func_name(&self) -> &str;
}

/**
 * Spectral shape descriptor
 *
 * The following descriptors are described in:
 *
 * Geoffroy Peeters, A large set of audio features for sound description (similarity and classification) in the CUIDADO project, CUIDADO I.S.T. Project Report 2004 ([pdf](http://www.ircam.fr/anasyn/peeters/ARTICLES/Peeters_2003_cuidadoaudiofeatures.pdf))
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecShape {
    /**
     * Spectral centroid
     *
     * The spectral centroid represents the barycenter of the spectrum.
     *
     * __Note__: This function returns the result in bin. To get the spectral centroid in Hz, `bintofreq()` should be used.
     */
    Centroid,

    /**
     * Spectral spread
     *
     * The spectral spread is the variance of the spectral distribution around its centroid.
     *
     * See also [Standard deviation](http://en.wikipedia.org/wiki/Standard_deviation) on Wikipedia.
     */
    Spread,

    /**
     *  Spectral skewness
     *
     * Similarly, the skewness is computed from the third order moment of the spectrum. A negative skewness indicates more energy on the lower part of the spectrum. A positive skewness indicates more energy on the high frequency of the spectrum.
     *
     * See also [Skewness](http://en.wikipedia.org/wiki/Skewness) on Wikipedia.
     */
    Skewness,

    /**
     * Spectral kurtosis
     *
     * The kurtosis is a measure of the flatness of the spectrum, computed from the fourth order moment.
     *
     * See also [Kurtosis](http://en.wikipedia.org/wiki/Kurtosis) on Wikipedia.
     */
    Kurtosis,

    /**
     * Spectral slope
     *
     * The spectral slope represents decreasing rate of the spectral amplitude, computed using a linear regression.
     */
    Slope,

    /**
     * Spectral decrease
     *
     * The spectral decrease is another representation of the decreasing rate, based on perceptual criteria.
     */
    Decrease,

    /**
     * Spectral roll-off
     *
     * This function returns the bin number below which 95% of the spectrum energy is found.
     */
    Rolloff,
}

impl SpecFunc for SpecShape {
    fn func_name(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for SpecShape {
    fn as_ref(&self) -> &'static str {
        use self::SpecShape::*;

        match self {
            Centroid => "centroid",
            Spread => "spread",
            Skewness => "skewness",
            Kurtosis => "kurtosis",
            Slope => "slope",
            Decrease => "decrease",
            Rolloff => "rolloff",
        }
    }
}

impl Display for SpecShape {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.as_ref().fmt(f)
    }
}

impl FromStr for SpecShape {
    type Err = Error;

    fn from_str(src: &str) -> Result<Self> {
        use self::SpecShape::*;

        Ok(match src {
            "centroid" => Centroid,
            "spread" => Spread,
            "skewness" => Skewness,
            "kurtosis" => Kurtosis,
            "slope" => Slope,
            "decrease" => Decrease,
            "rolloff" => Rolloff,
            _ => return Err(Error::InvalidArg),
        })
    }
}

/**
 * Spectral description object
 */
pub struct SpecDesc {
    specdesc: *mut ffi::aubio_specdesc_t,
}

impl Drop for SpecDesc {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_specdesc(self.specdesc) }
    }
}

impl SpecDesc {
    /**
     * Creation of a spectral description object
     *
     * - `method` Spectral description method
     * - `buf_size` Length of the input spectrum frame
     */
    pub fn new<M: SpecFunc>(method: M, buf_size: usize) -> Result<Self> {
        let specdesc = unsafe {
            ffi::new_aubio_specdesc(
                method.func_name().as_ptr() as *const _,
                buf_size as ffi::uint_t,
            )
        };

        check_init(specdesc)?;

        Ok(Self { specdesc })
    }

    /**
     * Execute spectral description function on a spectral frame
     *
     * Generic function to compute spectral description.
     */
    pub fn do_<'i, 'o, I, O>(&mut self, fftgrain: I, desc: O) -> Status
    where
        I: Into<CVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let fftgrain = fftgrain.into();
        let mut desc = desc.into();

        desc.check_size(1)?;

        unsafe {
            ffi::aubio_specdesc_do(
                self.specdesc,
                fftgrain.as_ptr(),
                desc.as_mut_ptr(),
            );
        }
        Ok(())
    }

    /**
     * Execute spectral description function on a spectral frame
     *
     * Generic function to compute spectral description.
     */
    pub fn do_result<'i, I>(&mut self, fftgrain: I) -> Result<f32>
    where
        I: Into<CVec<'i>>,
    {
        let mut desc = 0f32;
        self.do_(fftgrain, &mut desc)?;
        Ok(desc)
    }
}
