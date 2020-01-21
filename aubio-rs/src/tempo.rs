use crate::{
    Error,
    Result,
    Status,

    ffi,
    check_alloc,
    vec::{

    },
};

/**
 * Tempo detection object
 */
pub struct Tempo {
    tempo: *mut ffi::aubio_tempo_t,
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

        check_alloc(tempo)?;

        Ok(Self { tempo })
    }
}
