use crate::{
    check_init, ffi,
    vec::{CVec, CVecMut, CVecNormMut, CVecPhasMut, FVec, FVecMut},
    Result, Status,
};

/**
 * FFT (Fast Fourier Transformation) object
 *
 * This object computes forward and backward FFTs.
 *
 * Depending on how _aubio_ was compiled, FFT are computed using one of:
 *
 * - Ooura
 * - FFTW3
 * - vDSP
 */
pub struct FFT {
    fft: *mut ffi::aubio_fft_t,
    win_size: usize,
}

impl Drop for FFT {
    fn drop(&mut self) {
        unsafe {
            ffi::del_aubio_fft(self.fft);
        }
    }
}

impl FFT {
    /**
     * Create new FFT computation object
     */
    pub fn new(win_size: usize) -> Result<Self> {
        let fft = unsafe { ffi::new_aubio_fft(win_size as ffi::uint_t) };

        check_init(fft)?;

        Ok(Self { fft, win_size })
    }

    /**
     * Get window size
     */
    pub fn get_win(&self) -> usize {
        self.win_size
    }

    /**
     * Get fft size
     */
    pub fn get_fft(&self) -> usize {
        self.get_win() / 2 + 1
    }

    /**
     * Compute forward (direct) FFT
     */
    pub fn do_<'i, 'o, I, O>(&mut self, input: I, spectrum: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<CVecMut<'o>>,
    {
        let input = input.into();
        let mut spectrum = spectrum.into();

        input.check_size(self.get_win())?;

        unsafe {
            ffi::aubio_fft_do(self.fft, input.as_ptr(), spectrum.as_mut_ptr());
        }
        Ok(())
    }

    /**
     * Compute backward (inverse) FFT
     */
    pub fn rdo<'i, 'o, I, O>(&mut self, spectrum: I, output: O) -> Status
    where
        I: Into<CVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let spectrum = spectrum.into();
        let mut output = output.into();

        output.check_size(self.get_win())?;

        unsafe {
            ffi::aubio_fft_rdo(self.fft, spectrum.as_ptr(), output.as_mut_ptr());
        }
        Ok(())
    }

    /**
     * Compute forward (direct) FFT
     */
    pub fn do_complex<'i, 'o, I, O>(&mut self, input: I, compspec: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let input = input.into();
        let mut compspec = compspec.into();

        input.check_size(self.get_win())?;
        compspec.check_size(self.get_win())?;

        unsafe {
            ffi::aubio_fft_do_complex(self.fft, input.as_ptr(), compspec.as_mut_ptr());
        }
        Ok(())
    }

    /**
     * Compute backward (inverse) FFT
     */
    pub fn rdo_complex<'i, 'o, I, O>(&mut self, compspec: I, output: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let compspec = compspec.into();
        let mut output = output.into();

        compspec.check_size(self.get_win())?;
        output.check_size(self.get_win())?;

        unsafe {
            ffi::aubio_fft_rdo_complex(self.fft, compspec.as_ptr(), output.as_mut_ptr());
        }
        Ok(())
    }

    /**
     * Convert real/imag spectrum to norm/phas spectrum
     */
    pub fn get_spectrum<'i, 'o, I, O>(compspec: I, spectrum: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<CVecMut<'o>>,
    {
        let compspec = compspec.into();
        let mut spectrum = spectrum.into();

        spectrum.check_size(compspec.size())?;

        unsafe {
            ffi::aubio_fft_get_spectrum(compspec.as_ptr(), spectrum.as_mut_ptr());
        }
        Ok(())
    }

    /**
     * Convert norm/phas spectrum to real/imag spectrum
     */
    pub fn get_realimag<'i, 'o, I, O>(spectrum: I, compspec: O) -> Status
    where
        I: Into<CVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let spectrum = spectrum.into();
        let mut compspec = compspec.into();

        compspec.check_size(spectrum.size())?;

        unsafe {
            ffi::aubio_fft_get_realimag(spectrum.as_ptr(), compspec.as_mut_ptr());
        }
        Ok(())
    }

    /**
     * Compute phas spectrum from real/imag parts
     */
    pub fn get_phas<'i, 'o, I, O>(compspec: I, spectrum_phas: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<CVecPhasMut<'o>>,
    {
        let compspec = compspec.into();
        let mut spectrum_phas = spectrum_phas.into();

        spectrum_phas.check_size(compspec.size())?;

        unsafe {
            ffi::aubio_fft_get_phas(compspec.as_ptr(), spectrum_phas.as_mut_ptr());
        }
        Ok(())
    }

    /**
     * Compute norm component from real/imag parts
     */
    pub fn get_norm<'i, 'o, I, O>(compspec: I, spectrum_norm: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<CVecNormMut<'o>>,
    {
        let compspec = compspec.into();
        let mut spectrum_norm = spectrum_norm.into();

        spectrum_norm.check_size(compspec.size())?;

        unsafe {
            ffi::aubio_fft_get_norm(compspec.as_ptr(), spectrum_norm.as_mut_ptr());
        }
        Ok(())
    }

    /**
     * Compute imaginary part from the norm/phas cvec
     */
    pub fn get_imag<'i, 'o, I, O>(spectrum: I, compspec: O) -> Status
    where
        I: Into<CVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let spectrum = spectrum.into();
        let mut compspec = compspec.into();

        compspec.check_size(spectrum.size())?;

        unsafe {
            ffi::aubio_fft_get_imag(spectrum.as_ptr(), compspec.as_mut_ptr());
        }
        Ok(())
    }

    /**
     * Compute real part from the norm/phas cvec
     */
    pub fn get_real<'i, 'o, I, O>(spectrum: I, compspec: O) -> Status
    where
        I: Into<CVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let spectrum = spectrum.into();
        let mut compspec = compspec.into();

        compspec.check_size(spectrum.size())?;

        unsafe {
            ffi::aubio_fft_get_real(spectrum.as_ptr(), compspec.as_mut_ptr());
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test() {
        const ITERS: usize = 100; // number of iterations
        const WIN: usize = 512; // window size

        let mut in_ = [0f32; WIN]; // input buffer
                                   //let mut in_ = farr!(WIN); // input buffer
                                   //let mut fftgrain = [0f32; (WIN+1)*2]; // fft norm and phase
        let mut fftgrain = carr!(WIN); // fft norm and phase
        let mut out = [0.0; WIN]; // output buffer
                                  // create fft object
        let mut fft = FFT::new(WIN).unwrap();

        // fill input with some data
        in_[0] = 1.0;
        in_[1] = 2.0;
        in_[2] = 3.0;
        in_[3] = 4.0;
        in_[4] = 5.0;
        in_[5] = 6.0;
        in_[6] = 5.0;
        in_[7] = 6.0;
        println!("in: {:?}", in_.as_ref());

        for _i in 0..ITERS {
            // execute stft
            fft.do_(in_.as_ref(), fftgrain.as_mut()).unwrap();
            println!("fftgrain: {:?}", fftgrain.as_ref());
            // execute inverse fourier transform
            fft.rdo(fftgrain.as_ref(), out.as_mut()).unwrap();
        }

        println!("out: {:?}", out.as_ref());
    }
}
