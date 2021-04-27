use crate::{
    ffi,
    vec::{FVec, FVecMut},
    Smpl,
};

/**
 * Compute the principal argument
 *
 * This function maps the input phase to its corresponding value wrapped in the range -π ..= π.
 *
 * - `phase` Unwrapped phase to map to the unit circle
 *
 * Returns equivalent phase wrapped to the unit circle
 */
pub fn unwrap_2pi(phase: Smpl) -> Smpl {
    unsafe { ffi::aubio_unwrap2pi(phase) }
}

/**
 * Convert frequency bin to midi value
 */
pub fn bin_to_midi(bin: Smpl, sample_rate: Smpl, fft_size: Smpl) -> Smpl {
    unsafe { ffi::aubio_bintomidi(bin, sample_rate, fft_size) }
}

/**
 * Convert midi value to frequency bin
 */
pub fn midi_to_bin(midi: Smpl, sample_rate: Smpl, fft_size: Smpl) -> Smpl {
    unsafe { ffi::aubio_miditobin(midi, sample_rate, fft_size) }
}

/**
 * Convert frequency bin to frequency (Hz)
 *
 * - `bin` Frequency bin to convert
 * - `sample_rate` Sampling rate of the original signal the bin was from
 * - `fft_size` Size of the FFT window used to obtain the frequency bin
 */
pub fn bin_to_freq(bin: Smpl, sample_rate: Smpl, fft_size: Smpl) -> Smpl {
    unsafe { ffi::aubio_bintofreq(bin, sample_rate, fft_size) }
}

pub use self::bin_to_freq as bin_to_hz;

/**
 * Convert frequency (Hz) to frequency bin
 *
 * - `freq` Frequency in Hz to convert
 * - `sample_rate` Sampling rate of the original signal the bin was from
 * - `fft_size` Size of the FFT window used to obtain the frequency bin
 */
#[inline]
pub fn freq_to_bin(freq: Smpl, sample_rate: Smpl, fft_size: Smpl) -> Smpl {
    unsafe { ffi::aubio_freqtobin(freq, sample_rate, fft_size) }
}

pub use self::freq_to_bin as hz_to_bin;

/**
 * Convert frequency (Hz) to mel
 *
 * - `freq` Input frequency in Hz to convert
 *
 * Converts a scalar from the frequency domain to the mel scale using Slaney
 * Auditory Toolbox's implementation:
 *
 * If f < 1000, m = 3 f / 200.
 * If f >= 1000, m = 1000 + 27 * (ln(f) - ln(1000)) / (ln(6400) - ln(1000))
 *
 * See also `mel_to_hz()`, `hz_to_mel_htk()`.
 */
#[inline]
pub fn hz_to_mel(freq: Smpl) -> Smpl {
    unsafe { ffi::aubio_hztomel(freq) }
}

/**
 * Convert mel to frequency (Hz)
 *
 * - `mel` Input mel to convert
 *
 * Converts a scalar from the mel scale to the frequency domain using Slaney
 * Auditory Toolbox's implementation:
 *
 * If f < 1000, f = 200 m/3.
 * If f >= 1000, f = 1000 + (6400 / 1000) ^ ((m - 1000) / 27)
 *
 * See also `hz_to_mel()`, `mel_to_hz_htk()`.
 *
 * See:
 * - Malcolm Slaney, *Auditory Toolbox Version 2, Technical Report #1998-010*
 *   https://engineering.purdue.edu/~malcolm/interval/1998-010/
 */
#[inline]
pub fn mel_to_hz(mel: Smpl) -> Smpl {
    unsafe { ffi::aubio_meltohz(mel) }
}

/**
 * Convert frequency (Hz) to mel
 *
 * - `freq` Input frequency to convert, in Hz
 *
 * Converts a scalar from the frequency domain to the mel scale, using the
 * equation defined by O'Shaughnessy, as implemented in the HTK speech
 * recognition toolkit:
 *
 * m = 1127 + ln(1 + f / 700)
 *
 * See also `mel_to_hz_htk()`, `hz_to_mel()`.
 *
 * See:
 * - Douglas O'Shaughnessy (1987). *Speech communication: human and machine*.
 *   Addison-Wesley. p. 150. ISBN 978-0-201-16520-3.
 * - HTK Speech Recognition Toolkit: http://htk.eng.cam.ac.uk/
 */
#[inline]
pub fn hz_to_mel_htk(freq: Smpl) -> Smpl {
    unsafe { ffi::aubio_hztomel_htk(freq) }
}

/**
 * Convert mel to frequency (Hz)
 *
 * - `mel` Input mel to convert
 *
 * Converts a scalar from the mel scale to the frequency domain, using the
 * equation defined by O'Shaughnessy, as implemented in the HTK speech
 * recognition toolkit:
 *
 * f = 700 * e ^ (f / 1127 - 1)
 *
 * See also `hz_to_mel_htk()`, `mel_to_hz()`.
 */
#[inline]
pub fn mel_to_hz_htk(mel: Smpl) -> Smpl {
    unsafe { ffi::aubio_meltohz_htk(mel) }
}

/**
 * Convert frequency (Hz) to midi value in range 0..128
 *
 * - `freq` Frequency in Hz to convert
 */
#[inline]
pub fn freq_to_midi(freq: Smpl) -> Smpl {
    unsafe { ffi::aubio_freqtomidi(freq) }
}

/**
 * Convert midi value in range 0..128 to frequency (Hz)
 *
 * - `midi` Midi note value to convert (0..128)
 */
#[inline]
pub fn midi_to_freq(midi: Smpl) -> Smpl {
    unsafe { ffi::aubio_miditofreq(midi) }
}

/**
 * Zero-crossing rate (ZCR)
 *
 * The zero-crossing rate is the number of times a signal changes sign,
 * divided by the length of this signal.
 *
 * - `input` Vector to compute ZCR from
 */
#[inline]
pub fn zero_crossing_rate<'i, I>(input: I) -> Smpl
where
    I: Into<FVec<'i>>,
{
    let input = input.into();
    unsafe { ffi::aubio_zero_crossing_rate(input.as_ptr() as *mut _) }
}

/**
 * Compute sound level on a linear scale.
 *
 * This gives the average of the square amplitudes.
 *
 * - `input` Vector to compute level from
 */
#[inline]
pub fn level_lin<'i, I>(input: I) -> Smpl
where
    I: Into<FVec<'i>>,
{
    let input = input.into();
    unsafe { ffi::aubio_level_lin(input.as_ptr()) }
}

/**
 * Compute sound pressure level (SPL) in dB.
 *
 * This quantity is often wrongly called "loudness".
 * This gives ten times the log10 of the average of the square amplitudes.
 *
 * - `input` Vector to compute dB SPL from.
 */
#[inline]
pub fn db_spl<'i, I>(input: I) -> Smpl
where
    I: Into<FVec<'i>>,
{
    let input = input.into();
    unsafe { ffi::aubio_db_spl(input.as_ptr()) }
}

/**
 * Check if buffer level in dB SPL is under a given threshold.
 *
 * Returns `true` if the level is under the threshold, `false` otherwise.
 *
 * Note: this is currently the opposite of the official doc, which seems to
 * have a typo in it.
 *
 * - `input` Vector to get level from
 * - `threshold` Threshold in dB SPL
 */
#[inline]
pub fn silence_detection<'i, I>(input: I, threshold: Smpl) -> bool
where
    I: Into<FVec<'i>>,
{
    let input = input.into();
    0 != unsafe { ffi::aubio_silence_detection(input.as_ptr(), threshold) }
}

/**
 * Get buffer level if level >= threshold, 1.0 otherwise
 *
 * - `input` Vector to get level from
 * - `threshold` Threshold in dB SPL
 */
#[inline]
pub fn level_detection<'i, I>(input: I, threshold: Smpl) -> Smpl
where
    I: Into<FVec<'i>>,
{
    let input = input.into();
    unsafe { ffi::aubio_level_detection(input.as_ptr(), threshold) }
}

impl<'a> FVec<'a> {
    /**
     * Clamp the values of a vector within the range -abs(max) ..= abs(max)
     *
     * - `input` Vector to clamp
     * - `absmax` Maximum value over which input vector elements should be clamped
     */
    #[inline]
    pub fn clamp<'i, I>(input: I, absmax: Smpl)
    where
        I: Into<FVecMut<'i>>,
    {
        let mut input = input.into();
        unsafe { ffi::fvec_clamp(input.as_mut_ptr(), absmax) };
    }
}
