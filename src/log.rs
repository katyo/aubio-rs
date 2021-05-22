use crate::ffi;
use std::{
    ffi::{c_void, CStr},
    fmt::{Display, Formatter, Result as FmtResult},
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
    Error = ffi::aubio_log_level_AUBIO_LOG_ERR as _,

    /**
     * Infos
     */
    Info = ffi::aubio_log_level_AUBIO_LOG_INF as _,

    /**
     * General mesages
     */
    Message = ffi::aubio_log_level_AUBIO_LOG_MSG as _,

    /**
     * Debug messages
     */
    Debug = ffi::aubio_log_level_AUBIO_LOG_DBG as _,

    /**
     * Warnings
     */
    Warning = ffi::aubio_log_level_AUBIO_LOG_WRN as _,
}

impl LogLevel {
    fn from_ffi(level: ffi::aubio_log_level) -> Option<Self> {
        if level < ffi::aubio_log_level_AUBIO_LOG_LAST_LEVEL {
            Some(unsafe { core::mem::transmute(level) })
        } else {
            None
        }
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
Closure logger wrapper

```
use aubio::{Log, FnLogger};

Log::set(FnLogger::from(|level, message: &str| {
    eprintln!("[{}]: {}", level, message);
}));
```
 */
pub struct FnLogger<F>(F);

impl<F> FnLogger<F>
where
    F: FnMut(LogLevel, &str),
{
    pub fn new(func: F) -> Self {
        Self(func)
    }
}

impl<F> From<F> for FnLogger<F>
where
    F: FnMut(LogLevel, &str),
{
    fn from(func: F) -> Self {
        Self(func)
    }
}

impl<F> Logger for FnLogger<F>
where
    F: FnMut(LogLevel, &str),
{
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
pub struct Log(Box<dyn Logger>);

impl Drop for Log {
    fn drop(&mut self) {
        unsafe {
            ffi::aubio_log_reset();
        }
    }
}

impl Log {
    fn new<T>(logger: T) -> Self
    where
        T: Logger + 'static,
    {
        let logger = Box::new(logger);

        unsafe {
            ffi::aubio_log_set_function(Some(handler::<T>), logger.as_ref() as *const _ as *mut _);
        }

        Log(logger)
    }
}

fn with_global_logger(func: impl FnOnce(&mut Option<Log>)) {
    use std::{
        ptr::null_mut,
        sync::{Arc, Mutex, Once},
    };

    static ONCE: Once = Once::new();
    static mut LOG: *mut Arc<Mutex<Option<Log>>> = null_mut();

    ONCE.call_once(|| unsafe {
        LOG = Box::into_raw(Box::new(Arc::new(Mutex::new(None))));
    });

    let log = (unsafe { &*LOG }).clone();
    let mut log = log.lock().unwrap();

    func(&mut log);
}

impl Log {
    /// Set logger
    pub fn set<T>(logger: T)
    where
        T: Logger + 'static,
    {
        with_global_logger(|global_logger| {
            if global_logger.is_some() {
                *global_logger = None;
            }
            *global_logger = Some(Log::new(logger));
        });
    }

    /// Reset logger
    pub fn reset() {
        with_global_logger(|global_logger| {
            *global_logger = None;
        });
    }
}

#[cfg(feature = "log")]
pub use log_impl::LogLogger;

#[cfg(feature = "log")]
mod log_impl {
    use super::{LogLevel, Logger};
    use log::{log, Level};

    /**
    Logger implementation backed by [log](https://crates.io/crates/log) crate.

    ```
    use aubio::{Log, LogLevel, LogLogger};

    Log::set(LogLogger::default());
    ```
     */
    pub struct LogLogger<S> {
        target: S,
    }

    impl Default for LogLogger<&'static str> {
        fn default() -> Self {
            Self::new("aubio")
        }
    }

    impl<S> LogLogger<S> {
        pub fn new(target: S) -> Self {
            Self { target }
        }
    }

    impl<S: AsRef<str>> Logger for LogLogger<S> {
        fn log(&mut self, level: LogLevel, message: &str) {
            log!(target: self.target.as_ref(), level.into(), "{}", message);
        }
    }

    impl From<LogLevel> for Level {
        fn from(level: LogLevel) -> Self {
            match level {
                LogLevel::Error => Level::Error,
                LogLevel::Warning => Level::Warn,
                LogLevel::Message => Level::Info,
                LogLevel::Info => Level::Info,
                LogLevel::Debug => Level::Debug,
            }
        }
    }
}

extern "C" fn handler<T>(level: ffi::sint_t, message: *const ffi::char_t, data: *mut c_void)
where
    T: Logger,
{
    assert!(!data.is_null());

    let logger = unsafe { &mut *(data as *mut T) };
    let level = LogLevel::from_ffi(level as _).unwrap();
    let message = unsafe { CStr::from_ptr(message).to_str().unwrap() };

    logger.log(level, message);
}
