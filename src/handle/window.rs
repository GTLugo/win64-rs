use std::ptr::NonNull;

use windows::{
  Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::{DefWindowProcW, GetWindowLongPtrW, PostQuitMessage, SetWindowLongPtrW, SetWindowTextW},
  },
  core::HSTRING,
};

use crate::{ProcedureResult, flag::LongPointerIndex, message::Message};

use super::{Handle, Win32Type};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId
where
  Self: Send + Sync,
{
  hwnd: Option<NonNull<usize>>,
}

unsafe impl Send for WindowId {}
unsafe impl Sync for WindowId {}

type Void = core::ffi::c_void;

impl Handle for WindowId {
  fn as_ptr(&self) -> *mut Void {
    self.hwnd.map_or(core::ptr::null_mut(), |ptr| ptr.as_ptr().cast())
  }

  unsafe fn from_ptr(ptr: *mut Void) -> Self {
    let ptr: *mut usize = ptr.cast();
    let hwnd = match ptr.is_null() {
      true => None,
      false => Some(unsafe { NonNull::new_unchecked(ptr) }),
    };
    Self { hwnd }
  }

  fn is_valid(&self) -> bool {
    self.hwnd.is_some()
  }
}

impl Win32Type for WindowId {
  type Type = HWND;

  fn to_win32(&self) -> Self::Type {
    (*self).into()
  }
}

impl WindowId {
  pub fn default_procedure(&self, message: Message) -> ProcedureResult {
    unsafe { DefWindowProcW(self.to_win32(), message.id(), WPARAM(message.w()), LPARAM(message.l())) }.into()
  }

  pub fn quit(&self) {
    self.quit_with_code(0)
  }

  pub fn quit_with_code(&self, exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
  }

  pub fn set_window_text(&self, text: impl Into<String>) -> windows::core::Result<()> {
    let text = HSTRING::from(text.into());
    unsafe { SetWindowTextW(self.to_win32(), &text) }
  }

  pub fn get_window_long(hwnd: HWND, index: LongPointerIndex) -> isize {
    unsafe { GetWindowLongPtrW(hwnd, index.to_win32()) }
  }

  pub fn set_window_long(hwnd: HWND, index: LongPointerIndex, dwnewlong: isize) -> isize {
    unsafe { SetWindowLongPtrW(hwnd, index.to_win32(), dwnewlong) }
  }
}

impl From<WindowId> for HWND {
  fn from(value: WindowId) -> Self {
    Self(value.as_ptr())
  }
}

impl From<HWND> for WindowId {
  fn from(value: HWND) -> Self {
    unsafe { Self::from_ptr(value.0) }
  }
}
