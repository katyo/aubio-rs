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

    SpecFunc,
};

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/**
 * Onset detection function
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OnsetMode {
    /**
     * Energy based onset detection function
     *
     * This function calculates the local energy of the input spectral frame.
     */
    Energy,

    /**
     * High Frequency Content onset detection function
     *
     * This method computes the High Frequency Content (HFC) of the input spectral frame. The resulting function is efficient at detecting percussive onsets.
     *
     * Paul Masri. Computer modeling of Sound for Transformation and Synthesis of Musical Signal. PhD dissertation, University of Bristol, UK, 1996.
     */
    Hfc,

    /**
     * Complex Domain Method onset detection function
     *
     * Christopher Duxbury, Mike E. Davies, and Mark B. Sandler. Complex domain onset detection for musical signals. In Proceedings of the Digital Audio Effects Conference, DAFx-03, pages 90-93, London, UK, 2003.
     */
    Complex,

    /**
     * Phase Based Method onset detection function
     *
     * Juan-Pablo Bello, Mike P. Davies, and Mark B. Sandler. Phase-based note onset detection for music signals. In Proceedings of the IEEE International Conference on Acoustics Speech and Signal Processing, pages 441­444, Hong-Kong, 2003.
     */
    Phase,

    /**
     * Weighted Phase Deviation onset detection function
     *
     * S. Dixon. Onset detection revisited. In Proceedings of the 9th International Conference on Digital Audio Ef- fects (DAFx) , pages 133–137, 2006.
     *
     * See [http://www.eecs.qmul.ac.uk/~simond/pub/2006/dafx.pdf](http://www.eecs.qmul.ac.uk/~simond/pub/2006/dafx.pdf)
     */
    WPhase,

    /**
     * Spectral difference method onset detection function
     *
     * Jonhatan Foote and Shingo Uchihashi. The beat spectrum: a new approach to rhythm analysis. In IEEE International Conference on Multimedia and Expo (ICME 2001), pages 881­884, Tokyo, Japan, August 2001.
     */
    SpecDiff,

    /**
     * Kullback-Liebler onset detection function
     *
     * Stephen Hainsworth and Malcom Macleod. Onset detection in music audio signals. In Proceedings of the International Computer Music Conference (ICMC), Singapore, 2003.
     */
    Kl,

    /**
     * Modified Kullback-Liebler onset detection function
     *
     * Paul Brossier, "Automatic annotation of musical audio for interactive systems", Chapter 2, Temporal segmentation, PhD thesis, Centre for Digital music, Queen Mary University of London, London, UK, 2006.
     */
    Mkl,

    /**
     * Spectral Flux
     *
     * Simon Dixon, Onset Detection Revisited, in "Proceedings of the 9th International Conference on Digital Audio Effects" (DAFx-06), Montreal, Canada, 2006.
     */
    SpecFlux,
}

impl SpecFunc for OnsetMode {
    fn func_name(&self) -> &str {
        self.as_ref()
    }
}

impl Default for OnsetMode {
    fn default() -> Self {
        OnsetMode::Hfc
    }
}

impl AsRef<str> for OnsetMode {
    fn as_ref(&self) -> &'static str {
        use self::OnsetMode::*;

        match self {
            Energy => "energy",
            Hfc => "hfc",
            Complex => "complex",
            Phase => "phase",
            WPhase => "wphase",
            Mkl => "mkl",
            Kl => "kl",
            SpecFlux => "specflux",
            SpecDiff => "specdiff",
        }
    }
}

impl Display for OnsetMode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.as_ref().fmt(f)
    }
}

impl FromStr for OnsetMode {
    type Err = Error;

    fn from_str(src: &str) -> Result<Self> {
        use self::OnsetMode::*;

        Ok(match src {
            "energy" => Energy,
            "hfc" => Hfc,
            "complex" => Complex,
            "phase" => Phase,
            "wphase" => WPhase,
            "mkl" => Mkl,
            "kl" => Kl,
            "specflux" => SpecFlux,
            "specdiff" => SpecDiff,
            _ => return Err(Error::InvalidArg),
        })
    }
}

/**
 * Onset detection object
 *
 * The following routines compute the onset detection function and detect peaks in these functions.
 * When onsets are found above a given silence threshold, and after a minimum inter-onset interval,
 * the output vector returned by `do_()` is filled with 1. Otherwise, the output vector remains 0.
 *
 * The peak-picking threshold, the silence threshold, and the minimum inter-onset interval can be
 * adjusted during the execution of the `do_()` routine using the corresponding functions.
 */
pub struct Onset {
    onset: *mut ffi::aubio_onset_t,
}

impl Drop for Onset {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_onset(self.onset) }
    }
}

impl Onset {
    /**
     * Create onset detection object
     *
     * - `method` Onset detection type
     * - `buf_size` Buffer size for phase vocoder
     * - `hop_size` Hop size for phase vocoder
     * - `sample_rate` Sampling rate of the input signal
     */
    pub fn new(method: OnsetMode, buf_size: usize, hop_size: usize, sample_rate: u32) -> Result<Self> {
        let onset = unsafe {
            ffi::new_aubio_onset(
                method.as_ref().as_ptr() as *const _,
                buf_size as ffi::uint_t,
                hop_size as ffi::uint_t,
                sample_rate as ffi::uint_t,
            )
        };

        check_init(onset)?;

        Ok(Self { onset })
    }

    /**
     * Set onset detection adaptive whitening
     */
    pub fn with_awhitening(mut self, enable: bool) -> Self {
        self.set_awhitening(enable);
        self
    }

    /**
     * Set or disable log compression
     */
    pub fn with_compression(mut self, lambda: f32) -> Self {
        self.set_compression(lambda);
        self
    }

    /**
     * Set onset detection silence threshold
     */
    pub fn with_silence(mut self, silence: f32) -> Self {
        self.set_silence(silence);
        self
    }

    /**
     * Set onset detection peak picking threshold
     */
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.set_threshold(threshold);
        self
    }

    /**
     * Set minimum inter onset interval in samples
     */
    pub fn with_minioi(mut self, minioi: usize) -> Self {
        self.set_minioi(minioi);
        self
    }

    /**
     * Set minimum inter onset interval in seconds
     */
    pub fn with_minioi_s(mut self, minioi: f32) -> Self {
        self.set_minioi_s(minioi);
        self
    }

    /**
     * Set minimum inter onset interval in milliseconds
     */
    pub fn with_minioi_ms(mut self, minioi: f32) -> Self {
        self.set_minioi_ms(minioi);
        self
    }

    /**
     * Set delay in samples
     */
    pub fn with_delay(mut self, delay: usize) -> Self {
        self.set_delay(delay);
        self
    }

    /**
     * Set delay in seconds
     */
    pub fn with_delay_s(mut self, delay: f32) -> Self {
        self.set_delay_s(delay);
        self
    }

    /**
     * Set delay in milliseconds
     */
    pub fn with_delay_ms(mut self, delay: f32) -> Self {
        self.set_delay_ms(delay);
        self
    }

    /**
     * Execute onset detection
     *
     * When no onset was detected, the first element of the output vector onset is set to 0.
     *
     * When an onset is found, the first element of the output vector onset is set to `offset = 1 + a` where `a` is a number in the range[0, 1].
     *
     * The final onset detection time, in samples, can be obtained with `Onset::get_last()`. It can also be derived from offset as follows:
     *
     * `t = total_frames + offset * hop_size - delay`
     *
     * where `total_frames` is the total number of frames processed so far, and delay is the current delay of the onset object, as returned by `Onset::get_delay()`.
     */
    pub fn do_<'i, 'o, I, O>(&mut self, input: I, onset: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let input = input.into();
        let mut onset = onset.into();

        input.check_size(self.get_hop())?;
        onset.check_size(1)?;

        unsafe { ffi::aubio_onset_do(self.onset, input.as_ptr(), onset.as_mut_ptr()) }
        Ok(())
    }

    /**
     * Execute onset detection
     */
    pub fn do_result<'i, I>(&mut self, input: I) -> Result<f32>
    where
        I: Into<FVec<'i>>,
    {
        let mut onset = 0f32;
        self.do_(input, &mut onset)?;
        Ok(onset)
    }

    /**
     * Get hop size
     */
    pub fn get_hop(&self) -> usize {
        (unsafe { ffi::aubio_pvoc_get_hop(self.onset.cast::<ffi::aubio_pvoc_t>()) }) as usize
    }

    /**
     * Get the time of the latest onset detected, in samples
     */
    pub fn get_last(&self) -> usize {
        (unsafe { ffi::aubio_onset_get_last(self.onset) }) as usize
    }

    /**
     * Get the time of the latest onset detected, in seconds
     */
    pub fn get_last_s(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_last_s(self.onset) }
    }

    /**
     * Get the time of the latest onset detected, in milliseconds
     */
    pub fn get_last_ms(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_last_ms(self.onset) }
    }

    /**
     * Set onset detection adaptive whitening
     */
    pub fn set_awhitening(&mut self, enable: bool) {
        unsafe { ffi::aubio_onset_set_awhitening(self.onset, if enable { 1 } else { 0 }); }
    }

    /**
     * Get onset detection adaptive whitening
     */
    pub fn get_awhitening(&self) -> bool {
        if 0.0 < (unsafe { ffi::aubio_onset_get_awhitening(self.onset) }) {
            true
        } else {
            false
        }
    }

    /**
     * Set or disable log compression
     */
    pub fn set_compression(&mut self, lambda: f32) {
        unsafe { ffi::aubio_onset_set_compression(self.onset, lambda); }
    }

    /**
     * Get onset detection log compression
     */
    pub fn get_compression(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_compression(self.onset) }
    }

    /**
     * Set onset detection silence threshold
     */
    pub fn set_silence(&mut self, silence: f32) {
        unsafe { ffi::aubio_onset_set_silence(self.onset, silence); }
    }

    /**
     * Get onset detection silence threshold
     */
    pub fn get_silence(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_silence(self.onset) }
    }

    /**
     * Get onset detection function
     */
    pub fn get_descriptor(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_descriptor(self.onset) }
    }

    /**
     * Get thresholded onset detection function
     */
    pub fn get_thresholded_descriptor(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_thresholded_descriptor(self.onset) }
    }

    /**
     * Set onset detection peak picking threshold
     */
    pub fn set_threshold(&mut self, threshold: f32) {
        unsafe { ffi::aubio_onset_set_threshold(self.onset, threshold); }
    }

    /**
     * Get onset peak picking threshold
     */
    pub fn get_threshold(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_threshold(self.onset) }
    }

    /**
     * Set minimum inter onset interval in samples
     */
    pub fn set_minioi(&mut self, minioi: usize) {
        unsafe { ffi::aubio_onset_set_minioi(self.onset, minioi as ffi::uint_t); }
    }

    /**
     * Get minimum inter onset interval in samples
     */
    pub fn get_minioi(&self) -> usize {
        (unsafe { ffi::aubio_onset_get_minioi(self.onset) }) as usize
    }

    /**
     * Set minimum inter onset interval in seconds
     */
    pub fn set_minioi_s(&mut self, minioi: f32) {
        unsafe { ffi::aubio_onset_set_minioi_s(self.onset, minioi); }
    }

    /**
     * Get minimum inter onset interval in seconds
     */
    pub fn get_minioi_s(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_minioi_s(self.onset) }
    }

    /**
     * Set minimum inter onset interval in milliseconds
     */
    pub fn set_minioi_ms(&mut self, minioi: f32) {
        unsafe { ffi::aubio_onset_set_minioi_ms(self.onset, minioi); }
    }

    /**
     * Get minimum inter onset interval in milliseconds
     */
    pub fn get_minioi_ms(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_minioi_ms(self.onset) }
    }

    /**
     * Set delay in samples
     */
    pub fn set_delay(&mut self, delay: usize) {
        unsafe { ffi::aubio_onset_set_delay(self.onset, delay as ffi::uint_t); }
    }

    /**
     * Get delay in samples
     */
    pub fn get_delay(&self) -> usize {
        (unsafe { ffi::aubio_onset_get_delay(self.onset) }) as usize
    }

    /**
     * Set delay in seconds
     */
    pub fn set_delay_s(&mut self, delay: f32) {
        unsafe { ffi::aubio_onset_set_delay_s(self.onset, delay); }
    }

    /**
     * Get delay in seconds
     */
    pub fn get_delay_s(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_delay_s(self.onset) }
    }

    /**
     * Set delay in milliseconds
     */
    pub fn set_delay_ms(&mut self, delay: f32) {
        unsafe { ffi::aubio_onset_set_delay_ms(self.onset, delay); }
    }

    /**
     * Get delay in milliseconds
     */
    pub fn get_delay_ms(&self) -> f32 {
        unsafe { ffi::aubio_onset_get_delay_ms(self.onset) }
    }

    /**
     * Set default parameters
     */
    pub fn set_default_parameters(&mut self, mode: OnsetMode) {
        unsafe {
            ffi::aubio_onset_set_default_parameters(
                self.onset,
                mode.as_ref().as_ptr() as *const _
            );
        }
    }

    /**
     * Reset onset detection
     */
    pub fn reset(&mut self) {
        unsafe { ffi::aubio_onset_reset(self.onset); }
    }
}
