#![cfg(target_os = "windows")]

/*
  The goal should be to keep the same general API as Win32.

  This allows users to keep using regular guides and docs with minimal resistance.
*/

pub mod hresult;
pub mod processthreadsapi;
pub mod windef;
pub mod winuser;
pub mod handle;

pub use handle::*;
pub use hresult::*;

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

pub trait WStrLen {
  /// # Safety
  /// Must be null-terminated, valid UTF16
  unsafe fn strlen(&self) -> usize;
}

impl WStrLen for PWSTR {
  unsafe fn strlen(&self) -> usize {
    unsafe { wcslen(*self) }
  }
}

impl WStrLen for PCWSTR {
  unsafe fn strlen(&self) -> usize {
    unsafe { wcslen(self.cast_mut()) }
  }
}

/// # Safety
/// Must be null-terminated, valid UTF16
pub unsafe fn pwstr_to_os(s: PWSTR) -> OsString {
  OsString::from_wide(unsafe { std::slice::from_raw_parts_mut(s, s.strlen()) })
}

/// # Safety
/// Must be null-terminated, valid UTF16
pub unsafe fn pcwstr_to_os(s: PCWSTR) -> OsString {
  OsString::from_wide(unsafe { std::slice::from_raw_parts(s, s.strlen()) })
}
