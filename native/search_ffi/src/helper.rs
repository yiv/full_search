use allo_isolate::{ffi, Isolate};
use anyhow::Error;
use ffi_helpers::{null_pointer_check, NullPointer, Nullable};
use tokio::runtime::Runtime;

use crate::*;

use crate::error::MyError;
use std::{
    ffi::{CStr, CString},
    io,
    os::raw::c_char,
};

macro_rules! runtime {
    () => {
        match &*RUNTIME {
            Ok(rt) => rt,
            Err(_) => {
                return 0;
            }
        }
    };
}

macro_rules! error {
    ($result:expr) => {
        error!($result, 0);
    };
    ($result:expr, $error:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                ffi_helpers::update_last_error(e);
                return $error;
            }
        }
    };
}

macro_rules! cstr {
    ($ptr:expr) => {
        cstr!($ptr, 0);
    };
    ($ptr:expr, $error:expr) => {{
        null_pointer_check!($ptr);
        error!(unsafe { CStr::from_ptr($ptr).to_str() }, $error)
    }};
}

pub fn last_err(err: Error) -> i32 {
    update_last_error(MyError::from(err));
    0
}

#[no_mangle]
pub unsafe extern "C" fn last_error_length() -> i32 {
    ffi_helpers::error_handling::last_error_length()
}

#[no_mangle]
pub unsafe extern "C" fn error_message_utf8(buf: *mut c_char, length: i32) -> i32 {
    ffi_helpers::error_handling::error_message_utf8(buf, length)
}

#[test]
fn test() {
    println!("hello");
}
