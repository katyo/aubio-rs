use crate::{
    check_init, ffi,
    vec::{CVec, FMat, FMatVecs, FVecMut},
    Result, Status,
};

/**
 * Filterbank object.
 *
 * General-purpose spectral filterbank object.
 */
pub struct FilterBank {
    filterbank: *mut ffi::aubio_filterbank_t,
    #[cfg(feature = "check-size")]
    n_filters: usize,
    #[cfg(feature = "check-size")]
    win_s: usize,
}

impl Drop for FilterBank {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_filterbank(self.filterbank) }
    }
}

impl FilterBank {
    /**
     * Create filterbank object.
     *
     * - `n_filters`: Number of filters to create
     * - `win_s`: size of analysis buffer (and length of the FFT transform)
     *
     * Allocates an empty matrix of length win_s / 2 + 1 and height n_filters
     */
    pub fn new(n_filters: usize, win_s: usize) -> Result<Self> {
        let filterbank =
            unsafe { ffi::new_aubio_filterbank(n_filters as ffi::uint_t, win_s as ffi::uint_t) };

        check_init(filterbank)?;

        #[cfg(feature = "check-size")]
        {
            Ok(Self {
                filterbank,
                n_filters,
                win_s,
            })
        }
        #[cfg(not(feature = "check-size"))]
        {
            Ok(Self { filterbank })
        }
    }

    pub fn set_coeffs(&mut self, filters: FMat<FMatVecs>) {
        #[cfg(feature = "check-size")]
        {
            if filters.height() != self.n_filters || filters.length() != self.win_s / 2 + 1 {
                panic!("Invalid FilterBank coeff size");
            }
        }
        unsafe {
            ffi::aubio_filterbank_set_coeffs(self.filterbank, filters.as_ptr());
        }
    }

    pub fn get_coeffs(&mut self) -> FMat<()> {
        unsafe { FMat::from_raw_ptr(ffi::aubio_filterbank_get_coeffs(self.filterbank)) }
    }

    pub fn do_<'i, 'o, I, O>(&mut self, input: I, output: O) -> Status
    where
        I: Into<CVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let input = input.into();
        let mut output = output.into();
        #[cfg(feature = "check-size")]
        {
            if output.size() < self.n_filters || input.size() != self.win_s / 2 + 1 {
                panic!("Invalid output or input size for FilterBank");
            }
        }
        unsafe { ffi::aubio_filterbank_do(self.filterbank, input.as_ptr(), output.as_mut_ptr()) };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Smpl;

    #[test]
    fn test_new_filterbank() {
        let _filter_bank = FilterBank::new(10, 512).unwrap();
        #[cfg(feature = "check-size")]
        {
            assert_eq!(_filter_bank.n_filters, 10);
            assert_eq!(_filter_bank.win_s, 512);
        }
    }

    #[test]
    fn test_set_coeffs() {
        let mut filter_bank = FilterBank::new(2, 4).unwrap();
        let filters: Vec<&[Smpl]> = vec![&[1.0, 1.0, 1.0], &[2.0, 2.0, 2.0]];

        filter_bank.set_coeffs(filters.into());
        let coeffs = filter_bank.get_coeffs();

        assert_eq!(coeffs.get_vec(), vec![&[1.0, 1.0, 1.0], &[2.0, 2.0, 2.0]]);
    }

    #[cfg(feature = "check-size")]
    #[should_panic]
    #[test]
    fn test_wrong_height_set_coeffs() {
        let mut filter_bank = FilterBank::new(2, 4).unwrap();
        let filters: Vec<&[Smpl]> = vec![&[1.0, 1.0, 1.0], &[2.0, 2.0, 2.0], &[0.0, 0.0, 0.0]];

        filter_bank.set_coeffs(filters.into());
    }

    #[cfg(feature = "check-size")]
    #[should_panic]
    #[test]
    fn test_wrong_length_set_coeffs() {
        let mut filter_bank = FilterBank::new(2, 4).unwrap();
        let filters: Vec<&[Smpl]> = vec![&[1.0], &[2.0], &[0.0], &[0.0]];

        filter_bank.set_coeffs(filters.into());
    }

    #[test]
    fn test_filterbank_do() {
        let mut filter_bank = FilterBank::new(2, 4).unwrap();
        let filters: Vec<&[Smpl]> = vec![&[1.0, 1.0, 1.0], &[2.0, 2.0, 2.0]];
        // norm is 2 2 2, phas is 100 100 100
        let input: Vec<Smpl> = vec![2., 2., 2., 100., 100., 100.];
        let mut output: Vec<Smpl> = vec![0.; 2];

        filter_bank.set_coeffs(filters.into());
        filter_bank
            .do_(input.as_slice().as_ref(), output.as_mut_slice().as_mut())
            .unwrap();

        assert_eq!(vec![6.0, 12.0], output);
    }

    #[cfg(feature = "check-size")]
    #[should_panic]
    #[test]
    fn test_filterbank_do_wrong_dimensions_input() {
        let mut filter_bank = FilterBank::new(2, 4).unwrap();
        let input: Vec<Smpl> = vec![2., 2., 2., 2., 2., 100., 100., 100., 100., 100.];
        let mut output: Vec<Smpl> = vec![0.; 2];

        filter_bank
            .do_(input.as_slice().as_ref(), output.as_mut_slice().as_mut())
            .unwrap();
    }

    #[cfg(feature = "check-size")]
    #[should_panic]
    #[test]
    fn test_filterbank_do_wrong_dimensions_output() {
        let mut filter_bank = FilterBank::new(2, 4).unwrap();
        let input: Vec<Smpl> = vec![2., 2., 2., 2.0, 100., 100., 100., 100.];
        let mut output: Vec<Smpl> = vec![0.; 1];

        filter_bank
            .do_(input.as_slice().as_ref(), output.as_mut_slice().as_mut())
            .unwrap();
    }
}
