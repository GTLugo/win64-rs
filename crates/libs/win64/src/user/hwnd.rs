use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use dpi::{Position, Size};
use windows_result::{HRESULT, Result};
use windows_sys::Win32::{
  Foundation::{SetLastError, WIN32_ERROR},
  UI::WindowsAndMessaging::{
    CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow, GetWindowLongPtrW, IsWindow, PostQuitMessage,
    SetWindowLongPtrW, SetWindowTextW, ShowWindow,
  },
};

use crate::{Handle, declare_handle, get_last_error};

use super::{
  ExtendedWindowStyle, HInstance, LongPointerIndex, Message, WindowClass, WindowStyle,
  descriptor::WindowDescriptor,
  procedure::{CreateInfo, LResult, WindowData, WindowProcedure, WindowState},
};

declare_handle!(
  HWindow,
  alias = "HWND",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd"
);
// #[deprecated]
// pub type HWND = HWindow;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateWindowParams {
  pub ex_style: ExtendedWindowStyle,
  pub class: WindowClass,
  pub name: OsString,
  pub style: WindowStyle,
  pub position: (Option<i32>, Option<i32>),
  pub size: (Option<i32>, Option<i32>),
  pub parent: Option<HWindow>,
  pub menu: Option<()>,
  pub instance: Option<HInstance>,
  // pub void: Option<()>,
}

impl CreateWindowParams {
  pub fn ex_style(mut self, ex_style: ExtendedWindowStyle) -> Self {
    self.ex_style = ex_style;
    self
  }

  pub fn system_class(mut self, name: impl Into<OsString>) -> Self {
    self.class = WindowClass::System(name.into());
    self
  }

  pub fn class(mut self, class: WindowClass) -> Self {
    self.class = class;
    self
  }

  pub fn name(mut self, name: impl Into<OsString>) -> Self {
    self.name = name.into();
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

  pub fn style(mut self, style: WindowStyle) -> Self {
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

  // pub fn void(mut self, void: Option<()>) -> Self {
  //   self.void = void;
  //   self
  // }
}

#[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw"]
pub fn create_window<P: 'static + WindowProcedure>(params: CreateWindowParams, procedure: P) -> Result<HWindow> {
  let window_name: Vec<u16> = params.name.encode_wide().collect();
  let desc = WindowDescriptor {
    ext_style: params.ex_style,
    style: params.style,
    ..Default::default()
  };

  // remove visible style and reapply it later in the window procedure
  // let mut new_style = desc.style;
  // new_style.remove(WindowStyle::Visible);

  let create_info = Box::into_raw(Box::new(CreateInfo::new(procedure, desc.clone())));
  let hwnd = unsafe {
    CreateWindowExW(
      params.ex_style.to_raw(),
      params.class.atom(),
      window_name.as_ptr(),
      params.style.to_raw(),
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
      create_info.cast(),
    )
  };

  // match error {
  //   e if e == convert_error(ERROR_INVALID_PARAMETER) => Err(CreateWindowError::InvalidParameter(e)),
  //   e if e == convert_error(ERROR_MOD_NOT_FOUND) => Err(CreateWindowError::ModuleNotFound(e)),
  //   e if e == convert_error(ERROR_CANNOT_FIND_WND_CLASS) => Err(CreateWindowError::CannotFindWindowClass(e)),
  //   e if e == convert_error(ERROR_OUTOFMEMORY) => Err(CreateWindowError::OutOfMemory(e)),
  //   _ => Err(CreateWindowError::Other(error)),
  // }

  match hwnd.is_null() {
    true => Err(get_last_error()),
    false => Ok(unsafe { HWindow::from_raw(hwnd as usize) }),
  }
}

// #[derive(ThisError, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum CreateWindowError {
//   #[error(transparent)]
//   InvalidParameter(crate::Error),
//   #[error(transparent)]
//   ModuleNotFound(crate::Error),
//   #[error(transparent)]
//   CannotFindWindowClass(crate::Error),
//   #[error(transparent)]
//   OutOfMemory(crate::Error),
//   /*
//    ...etc
//   */
//   #[error(transparent)]
//   Other(crate::Error),
// }

impl HWindow {
  /// Thin wrapper around [`create_window`] function
  #[inline]
  pub fn new<P: 'static + WindowProcedure>(params: CreateWindowParams, procedure: P) -> Result<Self> {
    create_window(params, procedure)
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

impl HWindow {
  pub fn send_message(&self) {
    // TODO: somehow ensure these are always sent to the correct thread, even when called from a different thread.
    // maybe do it by storing the thread id?
    // Reference winit for this!
    todo!()
  }

  pub fn default_procedure(&self, message: &Message) -> LResult {
    unsafe { DefWindowProcW(self.to_ptr(), message.id().to_raw(), message.w().0, message.l().0) }.into()
  }

  pub fn destroy(&self) -> Result<()> {
    if unsafe { self.is_window() } && !self.data().unwrap().is_destroying() {
      self.data().unwrap().state = WindowState::Destroying;
      unsafe { DestroyWindow(self.to_ptr()) };
    }
    Ok(())
  }

  pub fn quit(&self) {
    self.quit_with_code(0)
  }

  pub fn quit_with_code(&self, exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
  }

  pub fn set_window_text(&self, text: impl Into<String>) -> Result<()> {
    let text = OsString::from(text.into()).encode_wide().collect::<Vec<u16>>();
    unsafe { SetWindowTextW(self.to_ptr(), text.as_ptr()) };
    Ok(())
  }

  // TODO: Migrate these functions to use conditionals for diff ptr widths
  pub(crate) fn get_window_long_ptr(&self, index: LongPointerIndex) -> isize {
    unsafe { GetWindowLongPtrW(self.to_ptr(), index.to_raw()) }
  }

  pub(crate) fn set_window_long_ptr(&self, index: LongPointerIndex, value: isize) -> Option<isize> {
    unsafe { SetLastError(WIN32_ERROR::default()) }; // clear error
    let result = unsafe { SetWindowLongPtrW(self.to_ptr(), index.to_raw(), value) };
    let error = get_last_error();
    match result == 0 && error.code() != HRESULT(0) {
      true => {
        eprintln!("Error: {}", get_last_error());
        None
      }
      false => Some(result),
    }
  }

  pub(crate) fn initialize_data(&self, create_info: CreateInfo) {
    let data = WindowData::new(create_info);
    let data_ptr = Box::into_raw(Box::new(data));
    self.set_window_long_ptr(LongPointerIndex::UserData, data_ptr as isize);
  }

  pub(crate) fn data(&self) -> Option<&mut WindowData> {
    unsafe { (self.get_window_long_ptr(LongPointerIndex::UserData) as *mut WindowData).as_mut() }
  }

  pub(crate) fn take_data(&self) -> Box<WindowData> {
    unsafe { Box::from_raw(self.get_window_long_ptr(LongPointerIndex::UserData) as *mut WindowData) }
  }
}
