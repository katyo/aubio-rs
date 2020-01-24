use crate::{
    ffi,
};

use std::{
    ffi::{c_void, CStr},
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
};

/**
 * Logging level
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LogLevel {
    /**
     * Critical errors
     */
    Error = ffi::aubio_log_level_AUBIO_LOG_ERR,

    /**
     * Infos
     */
    Info = ffi::aubio_log_level_AUBIO_LOG_INF,

    /**
     * General mesages
     */
    Message = ffi::aubio_log_level_AUBIO_LOG_MSG,

    /**
     * Debug messages
     */
    Debug = ffi::aubio_log_level_AUBIO_LOG_DBG,

    /**
     * Warnings
     */
    Warning = ffi::aubio_log_level_AUBIO_LOG_WRN,
}

impl LogLevel {
    fn from_ffi(level: ffi::sint_t) -> Option<Self> {
        use self::LogLevel::*;
        Some(match level as u32 {
            ffi::aubio_log_level_AUBIO_LOG_ERR => Error,
            ffi::aubio_log_level_AUBIO_LOG_INF => Info,
            ffi::aubio_log_level_AUBIO_LOG_MSG => Message,
            ffi::aubio_log_level_AUBIO_LOG_DBG => Debug,
            ffi::aubio_log_level_AUBIO_LOG_WRN => Warning,
            _ => return None,
        })
    }
}

impl AsRef<str> for LogLevel {
    fn as_ref(&self) -> &str {
        use self::LogLevel::*;
        match self {
            Error => "ERROR",
            Info => "INFO",
            Message => "MESSAGE",
            Debug => "DEBUG",
            Warning => "WARNING",
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.as_ref().fmt(f)
    }
}

/**
 * Log output handler
 */
pub trait Logger {
    fn log(&mut self, level: LogLevel, message: &str);
}

/**
 * Closure logger wrapper
 */
pub struct FnLogger<F>(F);

impl<F: FnMut(LogLevel, &str)> Logger for FnLogger<F> {
    fn log(&mut self, level: LogLevel, message: &str) {
        (self.0)(level, message);
    }
}

/**
 * Logging
 *
 * You can use own logger to handle library messages.
 * Only one logger supported at a time.
 * You should keep logger from dropping while it used.
 */
pub struct Log<T>(Box<T>);

impl<T> Drop for Log<T> {
    fn drop(&mut self) {
        unsafe { ffi::aubio_log_reset(); }
    }
}

impl<T> Deref for Log<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Log<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Logger> From<T> for Log<T> {
    fn from(logger: T) -> Self {
        let logger = Box::new(logger);

        unsafe {
            ffi::aubio_log_set_function(
                Some(handler::<T>),
                logger.as_ref() as *const _ as *mut _,
            );
        }

        Log(logger)
    }
}

impl<F: FnMut(LogLevel, &str)> Log<FnLogger<F>> {
    pub fn from_fn(function: F) -> Self {
        Log::from(FnLogger(function))
    }
}

unsafe extern "C" fn handler<T>(
        level: ffi::sint_t,
        message: *const ffi::char_t,
        data: *mut c_void,
)
where
    T: Logger,
{
    assert!(!data.is_null());

    let logger = &mut *(data as *mut T);
    let level = LogLevel::from_ffi(level).unwrap();
    let message = CStr::from_ptr(message).to_str().unwrap();

    logger.log(level, message);
}
