pub mod handle;
pub use handle::*;

use std::{ffi::OsString, os::windows::ffi::OsStringExt};
use windows_sys::{
  Win32::System::Threading::{GetStartupInfoW, STARTUPINFOW},
  core::{PCWSTR, PWSTR},
};

use crate::user::CmdShow;

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

#[derive(Debug, Clone)]
pub struct StartupInfo {
  pub desktop: OsString,
  pub title: OsString,
  pub pos: (u32, u32),
  pub size: (u32, u32),
  pub count_chars: (u32, u32),
  pub fill_attribute: u32,
  pub flags: u32,
  pub show_window: CmdShow,
  pub std_input: *mut (),
  pub std_output: *mut (),
  pub std_error: *mut (),
}

impl StartupInfo {
  pub fn get() -> Self {
    let mut info = STARTUPINFOW {
      cb: std::mem::size_of::<STARTUPINFOW>() as u32,
      ..unsafe { std::mem::zeroed() }
    };

    unsafe { GetStartupInfoW(&mut info) };
    Self {
      desktop: unsafe { pwstr_to_os(info.lpDesktop) },
      title: unsafe { pwstr_to_os(info.lpTitle) },
      pos: (info.dwX, info.dwY),
      size: (info.dwXSize, info.dwYSize),
      count_chars: (info.dwXCountChars, info.dwYCountChars),
      fill_attribute: info.dwFillAttribute,
      flags: info.dwFlags,
      show_window: CmdShow::from_raw(info.wShowWindow.into()),
      std_input: info.hStdInput.cast(),
      std_output: info.hStdOutput.cast(),
      std_error: info.hStdError.cast(),
    }
  }
}
