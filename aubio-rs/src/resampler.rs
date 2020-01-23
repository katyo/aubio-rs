use crate::{
    Error,
    Result,
    Status,

    ffi,
    check_init,
    vec::{
        FVec,
        FVecMut,
    },
};

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/**
 * Resampling method
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ResampleMode {
    BestQuality = 0,
    MediumQuality = 1,
    Fastest = 2,
    OrderHold = 3,
    Linear = 4,
}

impl Default for ResampleMode {
    fn default() -> Self {
        ResampleMode::BestQuality
    }
}

impl AsRef<str> for ResampleMode {
    fn as_ref(&self) -> &'static str {
        use self::ResampleMode::*;

        match self {
            BestQuality => "best_quality",
            MediumQuality => "medium_quality",
            Fastest => "fastest",
            OrderHold => "order_hold",
            Linear => "linear",
        }
    }
}

impl Display for ResampleMode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.as_ref().fmt(f)
    }
}

impl FromStr for ResampleMode {
    type Err = Error;

    fn from_str(src: &str) -> Result<Self> {
        use self::ResampleMode::*;

        Ok(match src {
            "best_quality" => BestQuality,
            "medium_quality" => MediumQuality,
            "fastest" => Fastest,
            "order_hold" => OrderHold,
            "linear" => Linear,
            _ => return Err(Error::InvalidArg),
        })
    }
}

/**
 * Resampler object
 */
pub struct Resampler {
    resampler: *mut ffi::aubio_resampler_t,
    ratio: f32,
}

impl Drop for Resampler {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_resampler(self.resampler) }
    }
}

impl Resampler {
    /**
     * Create resampler object
     *
     * - `ratio` The `output_sample_rate` / `input_sample_rate`
     * - `type` Resampling method
     */
    pub fn new(ratio: f32, mode: ResampleMode) -> Result<Self> {
        let resampler = unsafe {
            ffi::new_aubio_resampler(
                ratio,
                mode as ffi::uint_t,
            )
        };

        check_init(resampler)?;

        Ok(Self { resampler, ratio })
    }

    /**
     * Get ratio
     */
    pub fn get_ratio(&self) -> f32 {
        self.ratio
    }

    /**
     * Resample input in output
     *
     * - `input` Input buffer of size N
     * - `output` Output buffer of size N*ratio
     */
    pub fn do_<'i, 'o, I, O>(&mut self, input: I, output: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let input = input.into();
        let mut output = output.into();

        output.check_size((input.size() as f32 * self.ratio).floor() as usize)?;

        unsafe { ffi::aubio_resampler_do(self.resampler, input.as_ptr(), output.as_mut_ptr()) }
        Ok(())
    }
}
