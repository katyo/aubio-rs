use crate::{ffi, vec::FVec};

/**
 * Convert frequency bin to frequency (Hz)
 * 
 * - `bin`: Frequency bin to convert
 * - `sample_rate` Sampling rate of the original signal the bin was from
 * - `fft_size` Size of the FFT window used to obtain the frequency bin
 */
pub fn bin_to_freq(bin: f32, sample_rate: f32, fft_size: f32) -> f32 {
    unsafe { ffi::aubio_bintofreq(bin, sample_rate, fft_size) }
}

/**
 * Compute sound pressure level (SPL) in dB.
 * This quantity is often wrongly called "loudness".
 * This gives ten times the log10 of the average of the square amplitudes.
 * 
 * - `input`: Vector to compute dB SPL from.
 */
pub fn db_spl<'i, I>(input: I) -> f32
where I: Into<FVec<'i>>
{
    let input = input.into();
    unsafe { ffi::aubio_db_spl(input.as_ptr()) }
}

/**
 * Check if buffer level in dB SPL is under a given threshold.
 * Returns `true` if the level is under the threshold, `false` otherwise.
 * Note: this is currently the opposite of the official doc, which seems to
 * have a typo in it.
 *
 * - `input` Vector to get level from
 * - `threshold` Threshold in dB SPL
 */
pub fn silence_detection<'i, I>(input: I, threshold: f32) -> bool
where I: Into<FVec<'i>>
{
    let input = input.into();
    0 != unsafe { ffi::aubio_silence_detection(input.as_ptr(), threshold) }
}
