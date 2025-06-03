use std::ffi::OsString;

use windows_sys::Win32::System::Threading::{GetStartupInfoW, STARTUPINFOW};

use crate::pwstr_to_os;

pub struct StartupInfo {
  lpDesktop: OsString,
  lpTitle: OsString,
  dwX: u32,
  dwY: u32,
  dwXSize: u32,
  dwYSize: u32,
  dwXCountChars: u32,
  dwYCountChars: u32,
  dwFillAttribute: u32,
  dwFlags: u32,
  wShowWindow: u16,
  hStdInput: *mut (),
  hStdOutput: *mut (),
  hStdError: *mut (),
}

impl StartupInfo {
  pub fn get() -> Self {
    let mut info = STARTUPINFOW {
      cb: std::mem::size_of::<STARTUPINFOW>() as u32,
      ..unsafe { std::mem::zeroed() }
    };

    unsafe { GetStartupInfoW(&mut info) };
    Self {
      lpDesktop: unsafe { pwstr_to_os(info.lpDesktop) },
      lpTitle: unsafe { pwstr_to_os(info.lpTitle) },
      dwX: info.dwX,
      dwY: info.dwY,
      dwXSize: info.dwXSize,
      dwYSize: info.dwYSize,
      dwXCountChars: info.dwXCountChars,
      dwYCountChars: info.dwYCountChars,
      dwFillAttribute: info.dwFillAttribute,
      dwFlags: info.dwFlags,
      wShowWindow: info.wShowWindow,
      hStdInput: info.hStdInput.cast(),
      hStdOutput: info.hStdOutput.cast(),
      hStdError: info.hStdError.cast(),
    }
  }

  pub fn is_show_window(&self) -> bool {
    self.wShowWindow == 0
  }
}
