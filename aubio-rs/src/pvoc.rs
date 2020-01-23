use crate::{
    Error,
    Result,
    Status,

    WindowType,

    ffi,
    check_init,
    vec::{
        FVec, FVecMut,
        CVec, CVecMut,
    },
};

/**
 * Phase vocoder object
 */
pub struct PVoc {
    pvoc: *mut ffi::aubio_pvoc_t,
}

impl Drop for PVoc {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_pvoc(self.pvoc); }
    }
}

impl PVoc {
    /**
     * Create phase vocoder object
     *
     * - `win_size` Size of analysis buffer (and length the FFT transform)
     * - `hop_size` Step size between two consecutive analysis
     */
    pub fn new(win_size: usize, hop_size: usize) -> Result<Self> {
        let pvoc = unsafe {
            ffi::new_aubio_pvoc(
                win_size as ffi::uint_t,
                hop_size as ffi::uint_t,
            )
        };

        check_init(pvoc)?;

        Ok(Self {
            pvoc
        })
    }

    /**
     * Select window type
     */
    pub fn with_window(mut self, window_type: WindowType) -> Result<Self> {
        self.set_window(window_type).map(|_| self)
    }

    /**
     * Get hop size
     */
    pub fn get_hop(&self) -> usize {
        unsafe { ffi::aubio_pvoc_get_hop(self.pvoc) as usize }
    }

    /**
     * Get window size
     */
    pub fn get_win(&self) -> usize {
        unsafe { ffi::aubio_pvoc_get_win(self.pvoc) as usize }
    }

    /**
     * Compute spectral frame
     *
     * This function accepts an input vector of size `hop_size`.
     * The analysis buffer is rotated and filled with the new data.
     * After windowing of this signal window, the Fourier transform
     * is computed and returned in fftgrain as two vectors, magnitude
     * and phase.
     *
     * - `input` New input signal (`hop_size` long)
     * - `fftgrain` Output spectral frame (`win_size` long)
     */
    pub fn do_<'i, 'o, I, O>(&mut self, input: I, fftgrain: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<CVecMut<'o>>,
    {
        let input = input.into();
        let mut fftgrain = fftgrain.into();

        input.check_size(self.get_hop())?;
        fftgrain.check_size(self.get_win())?;

        unsafe { ffi::aubio_pvoc_do(self.pvoc, input.as_ptr(), fftgrain.as_mut_ptr()); }
        Ok(())
    }

    /**
     * Compute signal from spectral frame
     *
     * This function takes an input spectral frame fftgrain of size `win_size`
     * and computes its inverse Fourier transform. Overlap-add synthesis is then
     * computed using the previously synthetised frames, and the output stored in out.
     * - `fftgrain` Input spectral frame (`win_size` long)
     * - `output` Output signal (`hop_size` long)
     */
    pub fn rdo<'i, 'o, I, O>(&mut self, fftgrain: I, output: O) -> Status
    where
        I: Into<CVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let fftgrain = fftgrain.into();
        let mut output = output.into();

        fftgrain.check_size(self.get_win())?;
        output.check_size(self.get_hop())?;

        // It seems the second arg have missing const qualifier so we need 'as *mut _' here
        unsafe { ffi::aubio_pvoc_rdo(self.pvoc, fftgrain.as_ptr() as *mut _, output.as_mut_ptr()); }
        Ok(())
    }

    /**
     * Set window type
     */
    pub fn set_window(&mut self, window_type: WindowType) -> Status {
        let window_type = window_type.as_ref().as_ptr();

        if 0 == unsafe { ffi::aubio_pvoc_set_window(self.pvoc, window_type as *const _) } {
            Ok(())
        } else {
            Err(Error::InvalidArg)
        }
    }
}
