use crate::{
    Result,
    Status,

    ffi,
    check_init,
    vec::{
        FVec,
        FVecMut,
    },
};

/**
 * Tempo detection object
 */
pub struct Tempo {
    tempo: *mut ffi::aubio_tempo_t,
    hop_size: usize,
}

impl Drop for Tempo {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_tempo(self.tempo) }
    }
}

impl Tempo {
    /**
     * Create tempo detection object
     *
     * - `buf_size` Length of FFT
     * - `hop_size` Number of frames between two consecutive runs
     * - `sample_rate` Sampling rate of the signal to analyze
     */
    pub fn new(buf_size: usize, hop_size: usize, sample_rate: u32) -> Result<Self> {
        let tempo = unsafe {
            ffi::new_aubio_tempo(
                "default".as_ptr() as *const _,
                buf_size as ffi::uint_t,
                hop_size as ffi::uint_t,
                sample_rate as ffi::uint_t,
            )
        };

        check_init(tempo)?;

        Ok(Self { tempo, hop_size })
    }

    /**
     * Set tempo detection silence threshold
     */
    pub fn with_silence(mut self, silence: f32) -> Self {
        self.set_silence(silence);
        self
    }

    /**
     * Set tempo detection peak picking threshold
     */
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.set_threshold(threshold);
        self
    }

    /**
     * Set current delay in samples
     */
    pub fn with_delay(mut self, delay: isize) -> Self {
        self.set_delay(delay);
        self
    }

    /**
     * Set current delay in seconds
     */
    pub fn with_delay_s(mut self, delay: f32) -> Self {
        self.set_delay_s(delay);
        self
    }

    /**
     * Set current delay in milliseconds
     */
    pub fn with_delay_ms(mut self, delay: f32) -> Self {
        self.set_delay_ms(delay);
        self
    }

    /**
     * Get hop size
     */
    pub fn get_hop(&self) -> usize {
        self.hop_size
    }

    /**
     * Execute tempo detection
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

        unsafe { ffi::aubio_tempo_do(self.tempo, input.as_ptr(), output.as_mut_ptr()); }
        Ok(())
    }

    /**
     * Execute tempo detection
     */
    pub fn do_result<'i, I>(&mut self, input: I) -> Result<f32>
    where
        I: Into<FVec<'i>>,
    {
        let mut output = 0f32;
        self.do_(input, &mut output)?;
        Ok(output)
    }

    /**
     * Get the time of the latest beat detected, in samples
     */
    pub fn get_last(&self) -> usize {
        (unsafe { ffi::aubio_tempo_get_last(self.tempo) }) as usize
    }

    /**
     * Get the time of the latest beat detected, in seconds
     */
    pub fn get_last_s(&self) -> f32 {
        unsafe { ffi::aubio_tempo_get_last_s(self.tempo) }
    }

    /**
     * Get the time of the latest beat detected, in milliseconds
     */
    pub fn get_last_ms(&self) -> f32 {
        unsafe { ffi::aubio_tempo_get_last_ms(self.tempo) }
    }

    /**
     * Set tempo detection silence threshold
     */
    pub fn set_silence(&mut self, silence: f32) {
        unsafe { ffi::aubio_tempo_set_silence(self.tempo, silence); }
    }

    /**
     * Get tempo detection silence threshold
     */
    pub fn get_silence(&self) -> f32 {
        unsafe { ffi::aubio_tempo_get_silence(self.tempo) }
    }

    /**
     * Set tempo detection peak picking threshold
     */
    pub fn set_threshold(&mut self, threshold: f32) {
        unsafe { ffi::aubio_tempo_set_threshold(self.tempo, threshold); }
    }

    /**
     * Get tempo peak picking threshold
     */
    pub fn get_threshold(&self) -> f32 {
        unsafe { ffi::aubio_tempo_get_threshold(self.tempo) }
    }

    /**
     * Get the current beat period in samples
     */
    pub fn get_period(&self) -> usize {
        (unsafe { ffi::aubio_tempo_get_period(self.tempo) }) as usize
    }

    /**
     * Get the current beat period in seconds
     */
    pub fn get_period_s(&self) -> f32 {
        unsafe { ffi::aubio_tempo_get_period_s(self.tempo) }
    }

    /**
     * Get the current tempo
     */
    pub fn get_bpm(&self) -> f32 {
        unsafe { ffi::aubio_tempo_get_bpm(self.tempo) }
    }

    /**
     * Get the current tempo confidence
     */
    pub fn get_confidence(&self) -> f32 {
        unsafe { ffi::aubio_tempo_get_confidence(self.tempo) }
    }

    /**
     * Set number of tatum per beat
     */
    pub fn set_tatum_signature(&mut self, signature: u32) {
        unsafe { ffi::aubio_tempo_set_tatum_signature(self.tempo, signature); }
    }

    /**
     * Check whether a tatum was detected in the current frame
     */
    pub fn was_tatum(&self) -> u32 {
        unsafe { ffi::aubio_tempo_was_tatum(self.tempo) }
    }

    /**
     * Get position of last tatum in samples
     */
    pub fn get_last_tatum(&self) -> f32 {
        unsafe { ffi::aubio_tempo_get_last_tatum(self.tempo) }
    }

    /**
     * Set current delay in samples
     */
    pub fn set_delay(&mut self, delay: isize) {
        unsafe { ffi::aubio_tempo_set_delay(self.tempo, delay as ffi::sint_t); }
    }

    /**
     * Get current delay in samples
     */
    pub fn get_delay(&self) -> usize {
        (unsafe { ffi::aubio_tempo_get_delay(self.tempo) }) as usize
    }

    /**
     * Set current delay in seconds
     */
    pub fn set_delay_s(&mut self, delay: f32) {
        unsafe { ffi::aubio_tempo_set_delay_s(self.tempo, delay); }
    }

    /**
     * Get current delay in seconds
     */
    pub fn get_delay_s(&self) -> f32 {
        unsafe { ffi::aubio_tempo_get_delay_s(self.tempo) }
    }

    /**
     * Set current delay in milliseconds
     */
    pub fn set_delay_ms(&mut self, delay: f32) {
        unsafe { ffi::aubio_tempo_set_delay_ms(self.tempo, delay); }
    }

    /**
     * Get current delay in milliseconds
     */
    pub fn get_delay_ms(&self) -> f32 {
        unsafe { ffi::aubio_tempo_get_delay_ms(self.tempo) }
    }
}
