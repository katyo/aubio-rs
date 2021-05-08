use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    os::raw::c_char,
    result::Result as StdResult,
};

/**
 * The error type
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /**
     * Failed to initialize object
     */
    FailedInit,

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
            FailedInit => "creation error".fmt(f),
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

/**
 * The trait for null-terminated string constants
 */
pub trait AsNativeStr {
    /**
     * Implementations should return strings ended with '\0'
     * (for ex.: `"energy\0"`)
     */
    fn as_native_str(&self) -> &'static str;

    /**
     * Get constant as null-terminated C-string
     */
    fn as_native_cstr(&self) -> *const c_char {
        self.as_native_str().as_ptr() as *const _
    }

    /**
     * Get constant as rust string slice
     */
    fn as_rust_str(&self) -> &'static str {
        let nt_str = self.as_native_str();
        &nt_str[..nt_str.len() - 1]
    }
}

pub(crate) fn check_init<T>(ptr: *mut T) -> Status {
    if ptr.is_null() {
        Err(Error::FailedInit)
    } else {
        Ok(())
    }
}
