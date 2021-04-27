use crate::{
    check_init, ffi,
    vec::{CVec, FVecMut},
    Result, Smpl, Status,
};

/**
 * MFCC object
 *
 * Mel-Frequency Cepstrum Coefficients object.
 *
 * This object computes MFCC coefficients on an input CVec.
 *
 * The implementation follows the specifications established by Malcolm Slaney in its Auditory Toolbox, available online at the following address (see file mfcc.m):
 *
 * [https://engineering.purdue.edu/~malcolm/interval/1998-010/](https://engineering.purdue.edu/~malcolm/interval/1998-010/)
 */
pub struct MFCC {
    mfcc: *mut ffi::aubio_mfcc_t,
    buf_size: usize,
    n_coeffs: usize,
}

impl Drop for MFCC {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_mfcc(self.mfcc) }
    }
}

impl MFCC {
    /**
     * Create MFCC object
     *
     * - `buf_size` Size of analysis buffer (and length the FFT transform)
     * - `n_filters` Number of desired filters
     * - `n_coeffs` Number of desired coefficients
     * - `samplerate` Audio sampling rate
     */
    pub fn new(
        buf_size: usize,
        n_filters: usize,
        n_coeffs: usize,
        sample_rate: u32,
    ) -> Result<Self> {
        let mfcc = unsafe {
            ffi::new_aubio_mfcc(
                buf_size as ffi::uint_t,
                n_filters as ffi::uint_t,
                n_coeffs as ffi::uint_t,
                sample_rate as ffi::uint_t,
            )
        };

        check_init(mfcc)?;

        Ok(Self {
            mfcc,
            buf_size,
            n_coeffs,
        })
    }

    /**
     * Set power parameter
     */
    pub fn with_power(mut self, power: Smpl) -> Self {
        self.set_power(power);
        self
    }

    /**
     * Set scaling parameter
     */
    pub fn with_scale(mut self, scale: Smpl) -> Self {
        self.set_scale(scale);
        self
    }

    /**
     * Mel filterbank initialization
     *
     * - `fmin` Start frequency, in Hz
     * - `fmax` End frequency, in Hz
     *
     * The filterbank will be initialized with bands linearly spaced in the mel scale, from `fmin` to `fmax`.
     */
    pub fn with_mel_coeffs(mut self, fmin: Smpl, fmax: Smpl) -> Self {
        self.set_mel_coeffs(fmin, fmax);
        self
    }

    /**
     * Mel filterbank initialization
     *
     * - `fmin` Start frequency, in Hz
     * - `fmax` End frequency, in Hz
     *
     * The bank of filters will be initalized to to cover linearly spaced bands in the Htk mel scale, from `fmin` to `fmax`.
     */
    pub fn with_mel_coeffs_htk(mut self, fmin: Smpl, fmax: Smpl) -> Self {
        self.set_mel_coeffs_htk(fmin, fmax);
        self
    }

    /**
     * Mel filterbank initialization  (Auditory Toolbox's parameters)
     *
     * The filter coefficients are built to match exactly Malcolm Slaney's Auditory Toolbox implementation. The number of filters should be 40.
     *
     * This is the default filterbank when mf was created with `n_filters = 40`.
     */
    pub fn with_mel_coeffs_slaney(mut self) -> Self {
        self.set_mel_coeffs_slaney();
        self
    }

    /**
     * MFCC object processing
     *
     * - `in` Input spectrum (`buf_size` long)
     * - `out` Output mel coefficients buffer (`n_coeffs` long)
     */
    pub fn do_<'i, 'o, I, O>(&mut self, input: I, output: O) -> Status
    where
        I: Into<CVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let input = input.into();
        let mut output = output.into();

        input.check_size(self.buf_size)?;
        output.check_size(self.n_coeffs)?;

        unsafe { ffi::aubio_mfcc_do(self.mfcc, input.as_ptr(), output.as_mut_ptr()) }
        Ok(())
    }

    /**
     * Set power parameter
     */
    pub fn set_power(&mut self, power: Smpl) {
        unsafe {
            ffi::aubio_mfcc_set_power(self.mfcc, power);
        }
    }

    /**
     * Get power parameter
     */
    pub fn get_power(&self) -> Smpl {
        unsafe { ffi::aubio_mfcc_get_power(self.mfcc) }
    }

    /**
     * Set scaling parameter
     */
    pub fn set_scale(&mut self, scale: Smpl) {
        unsafe {
            ffi::aubio_mfcc_set_scale(self.mfcc, scale);
        }
    }

    /**
     * Get scaling parameter
     */
    pub fn get_scale(&self) -> Smpl {
        unsafe { ffi::aubio_mfcc_get_scale(self.mfcc) }
    }

    /**
     * Mel filterbank initialization
     *
     * - `fmin` Start frequency, in Hz
     * - `fmax` End frequency, in Hz
     *
     * The filterbank will be initialized with bands linearly spaced in the mel scale, from `fmin` to `fmax`.
     */
    pub fn set_mel_coeffs(&mut self, fmin: Smpl, fmax: Smpl) {
        unsafe {
            ffi::aubio_mfcc_set_mel_coeffs(self.mfcc, fmin, fmax);
        }
    }

    /**
     * Mel filterbank initialization
     *
     * - `fmin` Start frequency, in Hz
     * - `fmax` End frequency, in Hz
     *
     * The bank of filters will be initalized to to cover linearly spaced bands in the Htk mel scale, from `fmin` to `fmax`.
     */
    pub fn set_mel_coeffs_htk(&mut self, fmin: Smpl, fmax: Smpl) {
        unsafe {
            ffi::aubio_mfcc_set_mel_coeffs_htk(self.mfcc, fmin, fmax);
        }
    }

    /**
     * Mel filterbank initialization  (Auditory Toolbox's parameters)
     *
     * The filter coefficients are built to match exactly Malcolm Slaney's Auditory Toolbox implementation. The number of filters should be 40.
     *
     * This is the default filterbank when mf was created with `n_filters = 40`.
     */
    pub fn set_mel_coeffs_slaney(&mut self) {
        unsafe {
            ffi::aubio_mfcc_set_mel_coeffs_slaney(self.mfcc);
        }
    }
}
