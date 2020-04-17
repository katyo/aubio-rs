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

    pub fn size(&self) -> usize {
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

impl<'a, T: AsRef<[f32]>> From<T> for FVec<'a> {
    fn from(data: T) -> Self {
        let data = data.as_ref();
        Self {
            fvec: ffi::fvec_t {
                length: data.len() as ffi::uint_t,
                data: data.as_ptr() as *mut _,
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

    pub fn size(&self) -> usize {
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

impl<'a, T: AsMut<[f32]>> From<T> for FVecMut<'a> {
    fn from(mut data: T) -> Self {
        let data = data.as_mut();
        Self {
            fvec: ffi::fvec_t {
                length: data.len() as ffi::uint_t,
                data: data.as_mut_ptr(),
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
    pub fn from_parts<T: AsRef<[f32]>>(norm: T, phas: T) -> Result<Self> {
        let norm = norm.as_ref();
        let phas = phas.as_ref();
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

    pub fn size(&self) -> usize {
        self.cvec.length as usize
    }

    pub fn norm(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(self.cvec.norm, self.size())
        }
    }

    pub fn phas(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(self.cvec.phas, self.size())
        }
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

impl<'a, T: AsRef<[f32]>> From<T> for CVec<'a> {
    fn from(data: T) -> Self {
        let data = data.as_ref();
        let (norm, phas) = data.split_at(data.len()/2);
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
    pub fn from_parts<T: AsMut<[f32]>>(mut norm: T, mut phas: T) -> Result<Self> {
        let norm = norm.as_mut();
        let phas = phas.as_mut();
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

    pub fn size(&self) -> usize {
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

impl<'a, T: AsMut<[f32]>> From<T> for CVecMut<'a> {
    fn from(mut data: T) -> Self {
        let data = data.as_mut();
        let (norm, phas) = data.split_at_mut(data.len()/2);
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

impl<'a, T: AsMut<[f32]>> From<T> for CVecNormMut<'a> {
    fn from(mut data: T) -> Self {
        let norm = data.as_mut();
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

impl<'a, T: AsMut<[f32]>> From<T> for CVecPhasMut<'a> {
    fn from(mut data: T) -> Self {
        let phas = data.as_mut();
        Self { cvec: CVecMut::from_phas(phas) }
    }
}
