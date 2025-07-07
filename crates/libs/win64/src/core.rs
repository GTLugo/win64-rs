pub mod handle;
pub use handle::*;

pub mod point;
pub use point::*;

pub mod rect;
pub use rect::*;

use libloading::{Library, Symbol};
use std::{ffi::OsString, os::windows::ffi::OsStringExt, sync::LazyLock};
use windows_sys::{
  Win32::{
    Foundation::NTSTATUS,
    System::{
      SystemInformation::OSVERSIONINFOW,
      Threading::{GetStartupInfoW, STARTUPINFOW},
    },
  },
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
      ..Default::default()
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OsVersionInfo {
  pub major_version: u32,
  pub minor_version: u32,
  pub build_number: u32,
  pub platform_id: u32,
  pub csdversion: String,
}

pub fn rtl_get_version() -> Option<&'static OsVersionInfo> {
  static VERSION: LazyLock<Option<OsVersionInfo>> = LazyLock::new(|| {
    let Ok(ntdll) = (unsafe { Library::new("ntdll.dll\0") }) else {
      return None;
    };

    let Ok(rtl_get_version): Result<Symbol<unsafe extern "system" fn(*mut OSVERSIONINFOW) -> NTSTATUS>, _> =
      (unsafe { ntdll.get(b"RtlGetVersion") })
    else {
      return None;
    };

    unsafe {
      let mut info = OSVERSIONINFOW::default();

      let status = rtl_get_version(&mut info);

      if status >= 0 {
        Some(OsVersionInfo {
          major_version: info.dwMajorVersion,
          minor_version: info.dwMinorVersion,
          build_number: info.dwBuildNumber,
          platform_id: info.dwPlatformId,
          csdversion: String::from_utf16_lossy(&info.szCSDVersion)
            .trim_matches('\0')
            .to_string(),
        })
      } else {
        None
      }
    }
  });
  VERSION.as_ref()
}

pub fn win10_build_version() -> Option<u32> {
  let version = rtl_get_version()?;
  if version.major_version == 10 && version.minor_version == 0 {
    Some(version.build_number)
  } else {
    None
  }
}
