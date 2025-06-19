use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use dpi::{Position, Size};
use thiserror::Error as ThisError;
use windows_sys::Win32::{
  Foundation::{ERROR_CANNOT_FIND_WND_CLASS, ERROR_INVALID_PARAMETER, ERROR_MOD_NOT_FOUND, ERROR_OUTOFMEMORY},
  UI::WindowsAndMessaging::{CW_USEDEFAULT, CreateWindowExW, IsWindow, ShowWindow},
};

use crate::{Handle, convert_error, declare_handle};

use super::HInstance;

declare_handle!(
  HWindow,
  alias = "HWND",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd"
);
// #[deprecated]
// pub type HWND = HWindow;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateWindowParams {
  pub ex_style: u32,
  pub class_name: OsString,
  pub window_name: OsString,
  pub style: u32,
  pub position: (Option<i32>, Option<i32>),
  pub size: (Option<i32>, Option<i32>),
  pub parent: Option<HWindow>,
  pub menu: Option<()>,
  pub instance: Option<HInstance>,
  pub void: Option<()>,
}

impl CreateWindowParams {
  pub fn ex_style(mut self, ex_style: u32) -> Self {
    self.ex_style = ex_style;
    self
  }

  pub fn class_name(mut self, name: impl Into<OsString>) -> Self {
    self.class_name = name.into();
    self
  }

  pub fn window_name(mut self, name: impl Into<OsString>) -> Self {
    self.window_name = name.into();
    self
  }

  pub fn position(mut self, pos: Option<Position>) -> Self {
    let (x, y) = match pos {
      Some(pos) => {
        let pos = pos.to_physical(1.0);
        (Some(pos.x), Some(pos.y))
      }
      None => (None, None),
    };
    self.position = (x, y);
    self
  }

  pub fn size(mut self, size: Option<Size>) -> Self {
    let (w, h) = match size {
      Some(size) => {
        let size = size.to_physical(1.0);
        (Some(size.width), Some(size.height))
      }
      None => (None, None),
    };
    self.size = (w, h);
    self
  }

  pub fn style(mut self, style: u32) -> Self {
    self.style = style;
    self
  }

  pub fn parent(mut self, parent: Option<HWindow>) -> Self {
    self.parent = parent;
    self
  }

  pub fn menu(mut self, menu: Option<()>) -> Self {
    self.menu = menu;
    self
  }

  pub fn instance(mut self, instance: Option<HInstance>) -> Self {
    self.instance = instance;
    self
  }

  pub fn void(mut self, void: Option<()>) -> Self {
    self.void = void;
    self
  }
}

#[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw"]
pub fn create_window(params: CreateWindowParams) -> Result<HWindow, CreateWindowError> {
  let class_name: Vec<u16> = params.class_name.encode_wide().collect();
  let window_name: Vec<u16> = params.window_name.encode_wide().collect();
  let hwnd = unsafe {
    CreateWindowExW(
      params.ex_style,
      class_name.as_ptr(),
      window_name.as_ptr(),
      params.style,
      params.position.0.unwrap_or(CW_USEDEFAULT),
      params.position.1.unwrap_or(CW_USEDEFAULT),
      params.size.0.unwrap_or(CW_USEDEFAULT),
      params.size.1.unwrap_or(CW_USEDEFAULT),
      match params.parent {
        Some(p) => p.to_raw() as _,
        None => HWindow::null().to_raw() as _,
      },
      match params.menu {
        Some(_m) => todo!(),
        None => std::ptr::null_mut() as _,
      },
      match params.instance {
        Some(i) => i.to_raw() as _,
        None => HInstance::null().to_raw() as _,
      },
      match params.void {
        Some(_v) => todo!(),
        None => std::ptr::null(),
      },
    )
  };

  match hwnd.is_null() {
    true => {
      let error = crate::Error::from_win32();
      match error {
        e if e == convert_error(ERROR_INVALID_PARAMETER) => Err(CreateWindowError::InvalidParameter(e)),
        e if e == convert_error(ERROR_MOD_NOT_FOUND) => Err(CreateWindowError::ModuleNotFound(e)),
        e if e == convert_error(ERROR_CANNOT_FIND_WND_CLASS) => Err(CreateWindowError::CannotFindWindowClass(e)),
        e if e == convert_error(ERROR_OUTOFMEMORY) => Err(CreateWindowError::OutOfMemory(e)),
        _ => Err(CreateWindowError::Other(error)),
      }
    }
    false => Ok(unsafe { HWindow::from_raw(hwnd as usize) }),
  }
}

#[derive(ThisError, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CreateWindowError {
  #[error(transparent)]
  InvalidParameter(crate::Error),
  #[error(transparent)]
  ModuleNotFound(crate::Error),
  #[error(transparent)]
  CannotFindWindowClass(crate::Error),
  #[error(transparent)]
  OutOfMemory(crate::Error),
  /*
   ...etc
  */
  #[error(transparent)]
  Other(crate::Error),
}

impl HWindow {
  /// Thin wrapper around [`create_window`] function
  #[inline]
  pub fn new(params: CreateWindowParams) -> Result<Self, CreateWindowError> {
    create_window(params)
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
