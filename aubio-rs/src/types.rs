use crate::ffi;
use std::{
    error::{Error as StdError},
    fmt::{Display, Formatter, Result as FmtResult},
    result::{Result as StdResult},
};

/**
 * The error type
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /**
     * Failed to allocate object
     */
    Allocation,

    /**
     * Data size mismatched
     */
    MismatchSize,

    /**
     * Invalid argument
     */
    InvalidArg,
}

impl StdError for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Error::*;
        match self {
            Allocation => "allocation error".fmt(f),
            MismatchSize => "data size mismatch".fmt(f),
            InvalidArg => "invalid argument".fmt(f),
        }
    }
}

/**
 * The alias for result type with payload
 */
pub type Result<T> = StdResult<T, Error>;

/**
 * The alias for rusult type without payload
 */
pub type Status = Result<()>;

pub(crate) fn check_alloc<T>(ptr: *mut T) -> Status {
    if ptr.is_null() {
        Err(Error::Allocation)
    } else {
        Ok(())
    }
}
