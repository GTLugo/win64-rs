#![cfg(target_os = "windows")]

/*
  The goal should be to keep the same general API as Win32.

  This allows users to keep using regular guides and docs with minimal resistance.
*/

pub mod handle;
pub mod processthreadsapi;
pub mod error;
pub mod windef;
pub mod user;

pub use handle::*;
pub use error::*;

use std::{ffi::OsString, os::windows::ffi::OsStringExt};

pub use dpi;
pub use keyboard_types;
use windows_sys::core::{PCWSTR, PWSTR};

/// # Safety
/// Must be null-terminated, valid UTF16
pub unsafe fn wcslen(s: *mut u16) -> usize {
  unsafe extern "C" {
    fn wcslen(s: *const u16) -> usize;
  }
  unsafe { wcslen(s) }
}

/// # Safety
/// Must be null-terminated, valid UTF16
#[inline(always)]
pub unsafe fn pwstr_to_os(s: PWSTR) -> OsString {
  OsString::from_wide(unsafe { std::slice::from_raw_parts_mut(s, wcslen(s.cast())) })
}

/// # Safety
/// Must be null-terminated, valid UTF16
#[inline(always)]
pub unsafe fn pcwstr_to_os(s: PCWSTR) -> OsString {
  OsString::from_wide(unsafe { std::slice::from_raw_parts(s, wcslen(s.cast_mut())) })
}
