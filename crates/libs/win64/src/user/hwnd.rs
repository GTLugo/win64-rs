use windows_sys::Win32::{
  Foundation::{ERROR_INVALID_PARAMETER, ERROR_MOD_NOT_FOUND},
  UI::WindowsAndMessaging::{CreateWindowExW, IsWindow, ShowWindow},
};

use crate::{Handle, declare_handle};

declare_handle!(
  HWindow,
  alias = "HWND",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd"
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateWindowInfo {}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CreateWindowError {
  #[error(transparent)]
  InvalidParameter(crate::Error),
  #[error(transparent)]
  ModuleNotFound(crate::Error),
  #[error(transparent)]
  WHCBTHookFailure(crate::Error),
  #[error(transparent)]
  WindowProcFailure(crate::Error),
  #[error(transparent)]
  ControlsNotRegistered(crate::Error),
  #[error(transparent)]
  Uncommon(crate::Error),
}

/*
  For legacy reasons, this should probably take the same params as CreateWindowExW rather than a custom struct.
*/
#[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw"]
pub fn create_window(info: &CreateWindowInfo) -> Result<HWindow, CreateWindowError> {
  let hwnd = unsafe {
    CreateWindowExW(
      todo!(),
      todo!(),
      todo!(),
      todo!(),
      todo!(),
      todo!(),
      todo!(),
      todo!(),
      todo!(),
      todo!(),
      todo!(),
      todo!(),
    )
  };
  match hwnd.is_null() {
    true => {
      let error = crate::Error::from_win32();
      match error {
        e if e == crate::Error::from_hresult(crate::HResult::from_win32(ERROR_INVALID_PARAMETER)) => {
          Err(CreateWindowError::InvalidParameter(error))
        }
        e if e == crate::Error::from_hresult(crate::HResult::from_win32(ERROR_MOD_NOT_FOUND)) => {
          Err(CreateWindowError::ModuleNotFound(error))
        }
        error => Err(CreateWindowError::Uncommon(error)),
        _ => todo!(),
      }
    }
    false => Ok(unsafe { HWindow::from_raw(hwnd as usize) }),
  }
}

impl HWindow {
  /// Thin wrapper around [`create_window`] function
  #[inline]
  pub fn new(info: &CreateWindowInfo) -> Result<Self, CreateWindowError> {
    create_window(info)
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
