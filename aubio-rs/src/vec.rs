/*!
 * Vector data wrappers
 */

use crate::Error;

use crate::{ffi, Result, Status};

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::null_mut,
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
    pub(crate) fn check_size(&self, _min_size: usize) -> Status {
        Ok(())
    }

    #[cfg(feature = "check-size")]
    #[inline]
    pub(crate) fn check_size(&self, min_size: usize) -> Status {
        if self.fvec.length < min_size as _ {
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
    pub(crate) fn check_size(&self, _min_size: usize) -> Status {
        Ok(())
    }

    #[cfg(feature = "check-size")]
    #[inline]
    pub(crate) fn check_size(&self, min_size: usize) -> Status {
        if self.fvec.length < min_size as _ {
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
        unsafe { std::slice::from_raw_parts(self.cvec.norm, self.size()) }
    }

    pub fn phas(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self.cvec.phas, self.size()) }
    }

    #[cfg(not(feature = "check-size"))]
    #[inline]
    pub(crate) fn check_size(&self, _min_size: usize) -> Status {
        Ok(())
    }

    #[cfg(feature = "check-size")]
    #[inline]
    pub(crate) fn check_size(&self, min_size: usize) -> Status {
        if (self.cvec.length - 1) * 2 < min_size as _ {
            Err(Error::MismatchSize)
        } else {
            Ok(())
        }
    }
}

impl<'a, T: AsRef<[f32]>> From<T> for CVec<'a> {
    fn from(data: T) -> Self {
        let data = data.as_ref();
        let (norm, phas) = data.split_at(data.len() / 2);
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
    pub(crate) fn check_size(&self, _min_size: usize) -> Status {
        Ok(())
    }

    #[cfg(feature = "check-size")]
    #[inline]
    pub(crate) fn check_size(&self, min_size: usize) -> Status {
        if (self.cvec.length - 1) * 2 < min_size as _ {
            Err(Error::MismatchSize)
        } else {
            Ok(())
        }
    }
}

impl<'a, T: AsMut<[f32]>> From<T> for CVecMut<'a> {
    fn from(mut data: T) -> Self {
        let data = data.as_mut();
        let (norm, phas) = data.split_at_mut(data.len() / 2);
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
        Self {
            cvec: CVecMut::from_norm(norm),
        }
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
        Self {
            cvec: CVecMut::from_phas(phas),
        }
    }
}

/**
 * Immutable matrix of real valued data.
 */
#[repr(C)]
pub struct FMat<'a, X> {
    fmat: ffi::fmat_t,
    _x: X,
    _pd: PhantomData<&'a ()>,
}

impl<'a, X> FMat<'a, X> {
    pub(crate) fn as_ptr(&'a self) -> *const ffi::fmat_t {
        &self.fmat
    }

    pub fn length(&self) -> usize {
        self.fmat.length as usize
    }

    pub fn height(&self) -> usize {
        self.fmat.height as usize
    }

    /// Read sample value in a buffer 
    pub fn get_sample(&self, channel: usize, position: usize) -> Result<f32> {
        if channel >= self.height() || position >= self.length() {
            return Err(Error::InvalidArg);
        }
        Ok(unsafe {
            ffi::fmat_get_sample(
                self.as_ptr(),
                channel as ffi::uint_t,
                position as ffi::uint_t,
            )
        })
    }

    pub fn get_vec(&self) -> Vec<&mut [f32]> {
        let mut vec = Vec::with_capacity(self.height());
        let mut ptr = self.fmat.data;
        let end = self.fmat.data.wrapping_add(self.height());

        while ptr != end {
            vec.push(unsafe { std::slice::from_raw_parts_mut(*ptr, self.length()) });
            ptr = ptr.wrapping_add(1);
        }
        vec
    }
}

impl<'a> FMat<'a, ()> {
    /**
     * Create a matrix from an already existing `fmat_t` pointer.
     *
     * The matrix is non-owned; useful to avoid double-frees for `FilterBank`,
     * for instance.
     */
    pub unsafe fn from_raw_ptr(ptr: *const ffi::fmat_t) -> Self {
        FMat {
            fmat: *ptr,
            _x: (),
            _pd: PhantomData,
        }
    }
}

pub type FMatVecs = Vec<*const f32>;


impl<'a, T: AsRef<[&'a [f32]]>> From<T> for FMat<'a, FMatVecs> {
    /**
     * Create a matrix from a `FMatVecs`
     *
     * Matrix's horizontal height is the `Vec`'s len, and
     * its vertical length the slice's len.
     */
    fn from(data: T) -> Self {
        let data = data.as_ref();

        #[cfg(feature = "check-size")]
        {
            let mut vecs = data.iter();
            if let Some(fst) = vecs.next() {
                let len = fst.len();
                if len == 0 {
                    panic!("No values in slice");
                }
                if vecs.any(|nxt| nxt.len() != len) {
                    panic!("Slices have different lengths");
                }
            } else {
                panic!("No slices in vec");
            }
        }

        let array = data.iter().map(|v| v.as_ptr()).collect::<Vec<_>>();

        Self {
            fmat: ffi::fmat_t {
                height: data.len() as _,
                length: data[0].len() as _,
                data: array.as_ptr() as _,
            },
            _x: array,
            _pd: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    #[cfg(feature = "check-size")]
    fn test_from_fmat_wrong_size() {
        let x: &[&[f32]] = &[&[1.0, 2.0], &[4.0, 5.0, 6.0], &[7.0, 8.0, 9.0]];
        let _fmat: FMat<_> = x.into();
    }

    #[test]
    fn test_from_fmat() {
        let x: &[&[f32]] = &[&[1.0, 2.0], &[4.0, 5.0], &[7.0, 8.0]];
        let fmat: FMat<_> = x.into();
        assert_eq!(2, fmat.length());
        assert_eq!(3, fmat.height());
        
        assert_eq!(1., fmat.get_sample(0, 0).unwrap());
        assert_eq!(2., fmat.get_sample(0, 1).unwrap());
        assert_eq!(4., fmat.get_sample(1, 0).unwrap());
        assert_eq!(5., fmat.get_sample(1, 1).unwrap());
        assert_eq!(7., fmat.get_sample(2, 0).unwrap());
        assert_eq!(8., fmat.get_sample(2, 1).unwrap());
    }

    #[test]
    fn test_to_vec() {
        let x: &[&[f32]] = &[&[1.0, 2.0], &[4.0, 5.0], &[7.0, 8.0]];
        let fmat: FMat<_> = x.into();

        let matrix = fmat.get_vec();

        assert_eq!(matrix, vec![&[1.0, 2.0], &[4.0, 5.0], &[7.0, 8.0]]);
    }


    #[test]
    fn test_get_sample_fmat_wrong_size() {
        let x: &[&[f32]] = &[&[1.0, 2.0], &[4.0, 5.0], &[7.0, 8.0]];
        let fmat: FMat<_> = x.into();
        
        assert_eq!(Err(Error::InvalidArg), fmat.get_sample(70, 80));
    }

    #[test]
    fn test_fmat_non_owned() {
        let x: &[&[f32]] = &[&[1.0, 2.0], &[4.0, 5.0], &[7.0, 8.0]];
        let fmat: FMat<_> = x.into();

        {
            let _non_owned_fmat: FMat<()> = unsafe { FMat::from_raw_ptr(fmat.as_ptr()) };
        }
    }
}
