/*!
 * Vector data wrappers
 */

#[cfg(feature = "check-size")]
use crate::{
    Error,
};

use crate::{
    Result,
    Status,

    ffi,
};

use std::{
    ptr::null_mut,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/**
 * Immutable floating point vector
 */
#[repr(transparent)]
pub struct FVec<'a> {
    fvec: ffi::fvec_t,
    _pd: PhantomData<&'a ()>,
}

impl<'a> FVec<'a> {
    pub(crate) fn as_ptr(&'a self) -> *const ffi::fvec_t {
        &self.fvec
    }

    pub(crate) fn size(&self) -> usize {
        self.fvec.length as usize
    }

    #[cfg(not(feature = "check-size"))]
    #[inline]
    pub(crate) fn check_size(&self, _min_size: usize) -> Status { Ok(()) }

    #[cfg(feature = "check-size")]
    #[inline]
    pub(crate) fn check_size(&self, min_size: usize) -> Status {
        if self.fvec.length < min_size {
            Err(Error::MismatchSize)
        } else {
            Ok(())
        }
    }
}

impl<'a> From<&'a [f32]> for FVec<'a> {
    fn from(data: &'a [f32]) -> Self {
        Self {
            fvec: ffi::fvec_t {
                length: data.len() as ffi::uint_t,
                data: data.as_ptr() as *mut _,
            },
            _pd: PhantomData,
        }
    }
}

impl<'a> From<&'a f32> for FVec<'a> {
    fn from(data: &'a f32) -> Self {
        Self {
            fvec: ffi::fvec_t {
                length: 1,
                data: data as *const _ as *mut _,
            },
            _pd: PhantomData,
        }
    }
}

/**
 * Mutable floating point vector
 */
#[repr(transparent)]
pub struct FVecMut<'a> {
    fvec: ffi::fvec_t,
    _pd: PhantomData<&'a mut ()>,
}

impl<'a> FVecMut<'a> {
    pub(crate) fn as_mut_ptr(&'a mut self) -> *mut ffi::fvec_t {
        &mut self.fvec
    }

    pub(crate) fn size(&self) -> usize {
        self.fvec.length as usize
    }

    #[cfg(not(feature = "check-size"))]
    #[inline]
    pub(crate) fn check_size(&self, _min_size: usize) -> Status { Ok(()) }

    #[cfg(feature = "check-size")]
    #[inline]
    pub(crate) fn check_size(&self, min_size: usize) -> Status {
        if self.fvec.length < min_size {
            Err(Error::MismatchSize)
        } else {
            Ok(())
        }
    }
}

impl<'a> From<&'a mut [f32]> for FVecMut<'a> {
    fn from(data: &'a mut [f32]) -> Self {
        Self {
            fvec: ffi::fvec_t {
                length: data.len() as ffi::uint_t,
                data: data.as_mut_ptr(),
            },
            _pd: PhantomData,
        }
    }
}

impl<'a> From<&'a mut f32> for FVecMut<'a> {
    fn from(data: &'a mut f32) -> Self {
        Self {
            fvec: ffi::fvec_t {
                length: 1,
                data: data as *mut _,
            },
            _pd: PhantomData,
        }
    }
}

/**
 * Immutable complex floating point vector
 */
#[repr(transparent)]
pub struct CVec<'a> {
    cvec: ffi::cvec_t,
    _pd: PhantomData<&'a ()>,
}

impl<'a> CVec<'a> {
    pub fn from_parts(norm: &[f32], phas: &[f32]) -> Result<Self> {
        #[cfg(feature = "check-size")]
        {
            if norm.len() != phas.len() {
                return Err(Error::MismatchSize);
            }
        }
        Ok(Self {
            cvec: ffi::cvec_t {
                length: norm.len() as ffi::uint_t,
                norm: norm.as_ptr() as *mut _,
                phas: phas.as_ptr() as *mut _,
            },
            _pd: PhantomData,
        })
    }

    pub(crate) fn as_ptr(&'a self) -> *const ffi::cvec_t {
        &self.cvec
    }

    pub(crate) fn size(&self) -> usize {
        self.cvec.length as usize
    }

    #[cfg(not(feature = "check-size"))]
    #[inline]
    pub(crate) fn check_size(&self, _min_size: usize) -> Status { Ok(()) }

    #[cfg(feature = "check-size")]
    #[inline]
    pub(crate) fn check_size(&self, min_size: usize) -> Status {
        if self.cvec.length < min_size {
            Err(Error::MismatchSize)
        } else {
            Ok(())
        }
    }
}

impl<'a> From<(&'a [f32], &'a [f32])> for CVec<'a> {
    fn from((norm, phas): (&'a [f32], &'a [f32])) -> Self {
        Self::from_parts(norm, phas).unwrap()
    }
}

/**
 * Mutable complex floating point vector
 */
#[repr(transparent)]
pub struct CVecMut<'a> {
    cvec: ffi::cvec_t,
    _pd: PhantomData<&'a mut ()>,
}

impl<'a> CVecMut<'a> {
    pub fn from_parts(norm: &mut [f32], phas: &mut [f32]) -> Result<Self> {
        #[cfg(feature = "check-size")]
        {
            if norm.len() != phas.len() {
                return Err(Error::MismatchSize);
            }
        }
        Ok(Self {
            cvec: ffi::cvec_t {
                length: norm.len() as ffi::uint_t,
                norm: norm.as_mut_ptr(),
                phas: phas.as_mut_ptr(),
            },
            _pd: PhantomData,
        })
    }

    pub(crate) fn from_norm(norm: &mut [f32]) -> Self {
        Self {
            cvec: ffi::cvec_t {
                length: norm.len() as ffi::uint_t,
                norm: norm.as_mut_ptr(),
                phas: null_mut(),
            },
            _pd: PhantomData,
        }
    }

    pub(crate) fn from_phas(phas: &mut [f32]) -> Self {
        Self {
            cvec: ffi::cvec_t {
                length: phas.len() as ffi::uint_t,
                norm: null_mut(),
                phas: phas.as_mut_ptr(),
            },
            _pd: PhantomData,
        }
    }

    pub(crate) fn as_mut_ptr(&'a mut self) -> *mut ffi::cvec_t {
        &mut self.cvec
    }

    pub(crate) fn size(&self) -> usize {
        self.cvec.length as usize
    }

    #[cfg(not(feature = "check-size"))]
    #[inline]
    pub(crate) fn check_size(&self, _min_size: usize) -> Status { Ok(()) }

    #[cfg(feature = "check-size")]
    #[inline]
    pub(crate) fn check_size(&self, min_size: usize) -> Status {
        if self.cvec.length < min_size {
            Err(Error::MismatchSize)
        } else {
            Ok(())
        }
    }
}

impl<'a> From<(&'a mut [f32], &'a mut [f32])> for CVecMut<'a> {
    fn from((norm, phas): (&'a mut [f32], &'a mut [f32])) -> Self {
        Self::from_parts(norm, phas).unwrap()
    }
}

/**
 * Mutable complex floating point vector with norm part only
 */
#[repr(transparent)]
pub struct CVecNormMut<'a> {
    cvec: CVecMut<'a>,
}

impl<'a> Deref for CVecNormMut<'a> {
    type Target = CVecMut<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cvec
    }
}

impl<'a> DerefMut for CVecNormMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cvec
    }
}

impl<'a> From<&'a mut [f32]> for CVecNormMut<'a> {
    fn from(norm: &'a mut [f32]) -> Self {
        Self { cvec: CVecMut::from_norm(norm) }
    }
}

/**
 * Mutable complex floating point vector with phas part only
 */
#[repr(transparent)]
pub struct CVecPhasMut<'a> {
    cvec: CVecMut<'a>,
}

impl<'a> Deref for CVecPhasMut<'a> {
    type Target = CVecMut<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cvec
    }
}

impl<'a> DerefMut for CVecPhasMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cvec
    }
}

impl<'a> From<&'a mut [f32]> for CVecPhasMut<'a> {
    fn from(norm: &'a mut [f32]) -> Self {
        Self { cvec: CVecMut::from_phas(norm) }
    }
}
