use crate::{
    check_init, ffi,
    vec::{FVec, FVecMut},
    Result, Status,
};

/**
 * Recognized note data
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub pitch: f32,
    pub velocity: f32,
}

impl Note {
    fn parse(values: &[f32; 3]) -> Vec<Self> {
        let mut notes = Vec::new();

        if values[2] != 0.0 {
            notes.push(Self {
                pitch: values[2],
                velocity: 0.0,
            });
        }

        if values[0] != 0.0 {
            notes.push(Self {
                pitch: values[0],
                velocity: values[1],
            });
        }

        notes
    }
}

/**
 * Notes detection object
 */
pub struct Notes {
    notes: *mut ffi::aubio_notes_t,
    hop_size: usize,
}

impl Drop for Notes {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_notes(self.notes) }
    }
}

impl Notes {
    /**
     * Create notes detection object
     *
     * - `buf_size` Buffer size for phase vocoder
     * - `hop_size` Hop size for phase vocoder
     * - `samplerate` Sampling rate of the input signal
     */
    pub fn new(buf_size: usize, hop_size: usize, sample_rate: u32) -> Result<Self> {
        let notes = unsafe {
            ffi::new_aubio_notes(
                "default\0".as_ptr() as *const _,
                buf_size as ffi::uint_t,
                hop_size as ffi::uint_t,
                sample_rate as ffi::uint_t,
            )
        };

        check_init(notes)?;

        Ok(Self { notes, hop_size })
    }

    /**
     * Set notes detection silence threshold
     */
    pub fn with_silence(mut self, silence: f32) -> Self {
        self.set_silence(silence);
        self
    }

    /**
     * Set notes detection minimum inter-onset interval, in millisecond
     */
    pub fn with_minioi_ms(mut self, minioi: f32) -> Self {
        self.set_minioi_ms(minioi);
        self
    }

    /**
     * Set note release drop level, in dB
     */
    pub fn with_release_drop(mut self, release_drop: f32) -> Self {
        self.set_release_drop(release_drop);
        self
    }

    /**
     * Get hop size
     */
    pub fn get_hop(&self) -> usize {
        self.hop_size
    }

    /**
     * Execute note detection on an input signal frame
     *
     * - `input` Input signal of size `hop_size`
     * - `output` Output notes, fvec of length 3
     *
     * The notes output is a vector of length 3 containing:
     *
     * 0. the midi note value, or 0 if no note was found
     * 1. the note velocity
     * 2. the midi note to turn off
     */
    pub fn do_<'i, 'o, I, O>(&mut self, input: I, output: O) -> Status
    where
        I: Into<FVec<'i>>,
        O: Into<FVecMut<'o>>,
    {
        let input = input.into();
        let mut output = output.into();

        input.check_size(self.get_hop())?;
        output.check_size(3)?;

        unsafe { ffi::aubio_notes_do(self.notes, input.as_ptr(), output.as_mut_ptr()) }
        Ok(())
    }

    /**
     * Execute note detection on an input signal frame
     */
    pub fn do_result<'i, I>(&mut self, input: I) -> Result<Vec<Note>>
    where
        I: Into<FVec<'i>>,
    {
        let mut output = [0f32; 3];
        self.do_(input, output.as_mut())?;
        Ok(Note::parse(&output))
    }

    /**
     * Set notes detection silence threshold
     */
    pub fn set_silence(&mut self, silence: f32) {
        unsafe {
            ffi::aubio_notes_set_silence(self.notes, silence);
        }
    }

    /**
     * Get notes detection silence threshold
     */
    pub fn get_silence(&self) -> f32 {
        unsafe { ffi::aubio_notes_get_silence(self.notes) }
    }

    /**
     * Set notes detection minimum inter-onset interval, in millisecond
     */
    pub fn set_minioi_ms(&mut self, minioi: f32) {
        unsafe {
            ffi::aubio_notes_set_minioi_ms(self.notes, minioi);
        }
    }

    /**
     * Get notes detection minimum inter-onset interval, in millisecond
     */
    pub fn get_minioi_ms(&self) -> f32 {
        unsafe { ffi::aubio_notes_get_minioi_ms(self.notes) }
    }

    /**
     * Set note release drop level, in dB
     */
    pub fn set_release_drop(&mut self, release_drop: f32) {
        unsafe {
            ffi::aubio_notes_set_release_drop(self.notes, release_drop);
        }
    }

    /**
     * Get notes release drop level, in dB
     */
    pub fn get_release_drop(&self) -> f32 {
        unsafe { ffi::aubio_notes_get_release_drop(self.notes) }
    }
}
