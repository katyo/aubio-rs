use crate::{
    ffi,
};

use std::{
    ffi::{c_void, CStr},
    sync::{Arc, RwLock, Once},
    collections::HashMap,
    mem::MaybeUninit,
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

    /**
     * Set logging function for a given level
     */
    pub fn set_function<F>(&self, func: F)
    where
        F: FnMut(LogLevel, &str) + 'static,
    {
        let func = Box::new(func);

        unsafe {
            ffi::aubio_log_set_level_function(
                (*self) as ffi::sint_t,
                Some(handler::<F>),
                func.as_ref() as *const _ as *mut _,
            );
        }

        log_functions().store(Some(*self), func);
    }
}

unsafe extern "C" fn handler<F>(
        level: ffi::sint_t,
        message: *const ffi::char_t,
        data: *mut c_void,
)
where
    F: FnMut(LogLevel, &str)
{
    assert!(!data.is_null());

    let func = data as *mut F;
    let level = LogLevel::from_ffi(level).unwrap();
    let message = CStr::from_ptr(message).to_str().unwrap();

    (*func)(level, message);
}

/**
 * Logging
 */
pub struct Log(());

impl Log {
    /**
     * Reset all logging functions to the default one
     */
    pub fn reset() {
        unsafe { ffi::aubio_log_reset(); }

        log_functions().clear();
    }

    /**
     * Set logging function for all levels
     */
    pub fn set_function<F>(func: F)
    where
        F: FnMut(LogLevel, &str) + 'static,
    {
        let func = Box::new(func);

        unsafe {
            ffi::aubio_log_set_function(
                Some(handler::<F>),
                func.as_ref() as *const _ as *mut _,
            );
        }

        log_functions().store(None, func);
    }
}

#[derive(Clone)]
struct LogFunctions {
    map: Arc<RwLock<HashMap<Option<LogLevel>, Box<dyn FnMut(LogLevel, &str)>>>>,
}

impl LogFunctions {
    fn new() -> Self {
        Self { map: Arc::new(RwLock::new(HashMap::new())) }
    }

    fn store(&self, level: Option<LogLevel>, func: Box<dyn FnMut(LogLevel, &str)>) {
        let mut map = self.map.write().unwrap();
        map.insert(level, func);
    }

    fn clear(&self) {
        let mut map = self.map.write().unwrap();
        map.clear();
    }
}

fn log_functions() -> LogFunctions {
    static ONCE: Once = Once::new();
    static mut FUNCS: MaybeUninit<LogFunctions> = MaybeUninit::uninit();

    unsafe {
        ONCE.call_once(|| {
            FUNCS = MaybeUninit::new(LogFunctions::new());
        });
        (*FUNCS.as_ptr()).clone()
    }
}
