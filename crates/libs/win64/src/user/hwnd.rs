use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use windows_sys::Win32::{
  Foundation::{ERROR_INVALID_PARAMETER, ERROR_MOD_NOT_FOUND},
  UI::WindowsAndMessaging::{CreateWindowExW, IsWindow, ShowWindow},
};
use thiserror::Error as ThisError;

use crate::{convert_error, declare_handle, Handle};

use super::HInstance;

declare_handle!(
  HWindow,
  alias = "HWND",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd"
);
// #[deprecated]
// pub type HWND = HWindow;

#[derive(ThisError, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CreateWindowError {
  #[error(transparent)]
  InvalidParameter(crate::Error),
  #[error(transparent)]
  InvalidModule(crate::Error),
  #[error(transparent)]
  InvalidClass(crate::Error),
  #[error(transparent)]
  OutOfMemory(crate::Error),
  /*
    ...etc
   */
  #[error(transparent)]
  Other(crate::Error),
}

/*
  For legacy reasons, this should probably take the same params as CreateWindowExW rather than a custom struct.
*/
#[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw"]
pub fn create_window(
  ex_style: u32,
  class_name: impl Into<OsString>,
  window_name: impl Into<OsString>,
  style: u32,
  position: (i32, i32),
  size: (i32, i32),
  parent: Option<HWindow>,
  menu: Option<()>,
  instance: Option<HInstance>,
  void: Option<()>,
) -> Result<HWindow, CreateWindowError> {
  let class_name: Vec<u16> = class_name.into().encode_wide().collect();
  let window_name: Vec<u16> = window_name.into().encode_wide().collect();
  let hwnd = unsafe {
    CreateWindowExW(
      ex_style,
      class_name.as_ptr(),
      window_name.as_ptr(),
      style,
      position.0,
      position.1,
      size.0,
      size.1,
      match parent {
        Some(p) => p.to_raw() as _,
        None => HWindow::null().to_raw() as _,
      },
      match menu {
        Some(m) => std::ptr::null_mut() as _,
        None => std::ptr::null_mut() as _,
      },
      match instance {
        Some(i) => i.to_raw() as _,
        None => HInstance::null().to_raw() as _,
      },
      match void {
        Some(v) => std::ptr::null(),
        None => std::ptr::null(),
      },
    )
  };

  match hwnd.is_null() {
    true => {
      let error = crate::Error::from_win32();
      match error {
        e if e == convert_error(ERROR_INVALID_PARAMETER) => {
          Err(CreateWindowError::InvalidParameter(e))
        }
        e if e == convert_error(ERROR_MOD_NOT_FOUND) => {
          Err(CreateWindowError::InvalidModule(e))
        }
        _ => Err(CreateWindowError::Other(error)),
      }
    }
    false => Ok(unsafe { HWindow::from_raw(hwnd as usize) }),
  }
}

impl HWindow {
  /// Thin wrapper around [`create_window`] function
  #[inline]
  pub fn new(
    ex_style: u32,
    class_name: impl Into<OsString>,
    window_name: impl Into<OsString>,
    style: u32,
    position: (i32, i32),
    size: (i32, i32),
    parent: Option<HWindow>,
    menu: Option<()>,
    instance: Option<HInstance>,
    void: Option<()>,
  ) -> Result<Self, CreateWindowError> {
    create_window(ex_style, class_name, window_name, style, position, size, parent, menu, instance, void)
  }
}

impl HWindow {
  /// Returns whether or not the handle identifies an existing window.
  /// # Safety
  /// A thread should not use [`WindowId::is_window`] for a window that it did not create because the window could be destroyed after this function was called. Further, because window handles are recycled the handle could even point to a different window.
  ///
  #[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow"]
  pub unsafe fn is_window(&self) -> bool {
    // check for null here is probably redundant, but allows for a short-circuit which may or may not be faster.
    !self.is_null() && unsafe { IsWindow(self.to_raw() as _) != 0 }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShowWindowResult {
  WasVisible,
  WasHidden,
}

impl HWindow {
  #[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow"]
  pub fn show_window(&self, cmd_show: i32) -> ShowWindowResult {
    match unsafe { ShowWindow(self.to_raw() as _, cmd_show) } {
      0 => ShowWindowResult::WasHidden,
      _ => ShowWindowResult::WasVisible,
    }
  }
}
