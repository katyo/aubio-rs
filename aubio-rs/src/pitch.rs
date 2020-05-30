use crate::{
    check_init, ffi,
    vec::{FVec, FVecMut},
    AsNativeStr, Error, Result, Status,
};

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/**
 * Pitch detection method
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PitchMode {
    /**
     * Schmitt trigger
     *
     * This pitch extraction method implements a Schmitt trigger to estimate the period of a signal.
     *
     * This file was derived from the tuneit project, written by Mario Lang to detect the fundamental frequency of a sound.
     *
     * See [http://delysid.org/tuneit.html](http://delysid.org/tuneit.html)
     */
    Schmitt,

    /**
     * A fast harmonic comb filter
     *
     * This pitch extraction method implements a fast harmonic comb filter to determine the fundamental frequency of a harmonic sound.
     *
     * This file was derived from the tuneit project, written by Mario Lang to detect the fundamental frequency of a sound.
     *
     * See [http://delysid.org/tuneit.html](http://delysid.org/tuneit.html)
     */
    Fcomb,

    /**
     * Multiple-comb filter
     *
     * This fundamental frequency estimation algorithm implements spectral flattening, multi-comb filtering and peak histogramming.
     *
     * This method was designed by Juan P. Bello and described in:
     *
     * Juan-Pablo Bello. "Towards the Automated Analysis of Simple Polyphonic Music". PhD thesis, Centre for Digital Music, Queen Mary University of London, London, UK, 2003.
     */
    Mcomb,

    /**
     * YIN algorithm
     *
     * This algorithm was developed by A. de Cheveigne and H. Kawahara and published in:
     *
     * De Cheveigné, A., Kawahara, H. (2002) "YIN, a fundamental frequency estimator for speech and music", J. Acoust. Soc. Am. 111, 1917-1930.
     *
     * See [http://recherche.ircam.fr/equipes/pcm/pub/people/cheveign.html](http://recherche.ircam.fr/equipes/pcm/pub/people/cheveign.html)
     */
    Yin,

    /**
     * YIN fast algorithm
     *
     * This algorithm is equivalent to the YIN algorithm, but computed in the spectral domain for efficiency. See also python/demos/demo_yin_compare.py.
     */
    Yinfast,

    /**
     * YIN fft algorithm
     *
     * This algorithm was derived from the YIN algorithm. In this implementation, a Fourier transform is used to compute a tapered square difference function, which allows spectral weighting. Because the difference function is tapered, the selection of the period is simplified.
     *
     * Paul Brossier, Automatic annotation of musical audio for interactive systems, Chapter 3, Pitch Analysis, PhD thesis, Centre for Digital music, Queen Mary University of London, London, UK, 2006.
     */
    Yinfft,

    Specacf,
}

impl Default for PitchMode {
    fn default() -> Self {
        PitchMode::Yinfft
    }
}

impl AsNativeStr for PitchMode {
    fn as_native_str(&self) -> &'static str {
        use self::PitchMode::*;

        match self {
            Schmitt => "schmitt\0",
            Fcomb => "fcomb\0",
            Mcomb => "mcomb\0",
            Yin => "yin\0",
            Yinfast => "yinfast\0",
            Yinfft => "yinfft\0",
            Specacf => "specacf\0",
        }
    }
}

impl AsRef<str> for PitchMode {
    fn as_ref(&self) -> &'static str {
        self.as_rust_str()
    }
}

impl Display for PitchMode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.as_ref().fmt(f)
    }
}

impl FromStr for PitchMode {
    type Err = Error;

    fn from_str(src: &str) -> Result<Self> {
        use self::PitchMode::*;

        Ok(match src {
            "schmitt" => Schmitt,
            "fcomb" => Fcomb,
            "mcomb" => Mcomb,
            "yin" => Yin,
            "yinfast" => Yinfast,
            "yinfft" => Yinfft,
            "specacf" => Specacf,
            _ => return Err(Error::InvalidArg),
        })
    }
}

/**
 * Pitch output unit
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PitchUnit {
    /**
     * Hertz
     */
    Hz,

    /**
     * Midi
     */
    Midi,

    /**
     * Cent
     */
    Cent,

    /**
     * Bin
     */
    Bin,
}

impl Default for PitchUnit {
    fn default() -> Self {
        PitchUnit::Hz
    }
}

impl AsNativeStr for PitchUnit {
    fn as_native_str(&self) -> &'static str {
        use self::PitchUnit::*;

        match self {
            Hz => "hertz\0",
            Midi => "midi\0",
            Cent => "cent\0",
            Bin => "bin\0",
        }
    }
}

impl AsRef<str> for PitchUnit {
    fn as_ref(&self) -> &'static str {
        self.as_rust_str()
    }
}

impl Display for PitchUnit {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.as_ref().fmt(f)
    }
}

impl FromStr for PitchUnit {
    type Err = Error;

    fn from_str(src: &str) -> Result<Self> {
        use self::PitchUnit::*;

        Ok(match src {
            "hertz" => Hz,
            "midi" => Midi,
            "cent" => Cent,
            "bin" => Bin,
            _ => return Err(Error::InvalidArg),
        })
    }
}

/**
 * Pitch detection object
 */
pub struct Pitch {
    pitch: *mut ffi::aubio_pitch_t,
    hop_size: usize,
}

impl Drop for Pitch {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_pitch(self.pitch) }
    }
}

impl Pitch {
    /**
     * Creation of the pitch detection object
     *
     * - `method` Pitch detection algorithm
     * - `buf_size` Size of the input buffer to analyse
     * - `hop_size` Step size between two consecutive analysis instant
     * - `sample_rate` Sampling rate of the signal
     */
    pub fn new(
        method: PitchMode,
        buf_size: usize,
        hop_size: usize,
        sample_rate: u32,
    ) -> Result<Self> {
        let pitch = unsafe {
            ffi::new_aubio_pitch(
                method.as_native_cstr(),
                buf_size as ffi::uint_t,
                hop_size as ffi::uint_t,
                sample_rate as ffi::uint_t,
            )
        };

        check_init(pitch)?;

        Ok(Self { pitch, hop_size })
    }

    /**
     * Change yin or yinfft tolerance threshold
     */
    pub fn with_tolerance(mut self, tolerance: f32) -> Self {
        self.set_tolerance(tolerance);
        self
    }

    /**
     * Set the silence threshold of the pitch detection object
     */
    pub fn with_silence(mut self, silence: f32) -> Self {
        self.set_silence(silence);
        self
    }

    /**
     * Set the output unit of the pitch detection object
     */
    pub fn with_unit(mut self, unit: PitchUnit) -> Self {
        self.set_unit(unit);
        self
    }

    /**
     * Get hop size
     */
    pub fn get_hop(&self) -> usize {
        self.hop_size
    }

    /**
     * Execute pitch detection on an input signal frame
     *
     * - `input` Input signal of size `hop_size`
     * - `output` Output pitch candidates of size 1
     */
    pub fn do_<'i, 'o, I, O>(&mut self, input: I, output: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let input = input.into();
        let mut output = output.into();

        input.check_size(self.get_hop())?;
        output.check_size(1)?;

        unsafe {
            ffi::aubio_pitch_do(self.pitch, input.as_ptr(), output.as_mut_ptr());
        }
        Ok(())
    }

    /**
     * Execute pitch detection on an input signal frame
     *
     * - `input` Input signal of size `hop_size`
     */
    pub fn do_result<'i, I>(&mut self, input: I) -> Result<f32>
    where
        I: Into<FVec<'i>>,
    {
        let mut output = [0f32; 1];
        self.do_(input, &mut output)?;
        Ok(output[0])
    }

    /**
     * Change yin or yinfft tolerance threshold
     */
    pub fn set_tolerance(&mut self, tolerance: f32) {
        unsafe {
            ffi::aubio_pitch_set_tolerance(self.pitch, tolerance);
        }
    }

    /**
     * Get yin or yinfft tolerance threshold
     */
    pub fn get_tolerance(&self) -> f32 {
        unsafe { ffi::aubio_pitch_get_tolerance(self.pitch) }
    }

    /**
     * Set the silence threshold of the pitch detection object
     */
    pub fn set_silence(&mut self, silence: f32) {
        unsafe {
            ffi::aubio_pitch_set_silence(self.pitch, silence);
        }
    }

    /**
     * Get the silence threshold of the pitch detection object
     */
    pub fn get_silence(&self) -> f32 {
        unsafe { ffi::aubio_pitch_get_silence(self.pitch) }
    }

    /**
     * Set the output unit of the pitch detection object
     */
    pub fn set_unit(&mut self, unit: PitchUnit) {
        unsafe {
            ffi::aubio_pitch_set_unit(self.pitch, unit.as_native_cstr());
        }
    }

    /**
     * Get the current confidence of the pitch algorithm
     */
    pub fn get_confidence(&self) -> f32 {
        unsafe { ffi::aubio_pitch_get_confidence(self.pitch) }
    }
}
