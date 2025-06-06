use windows_sys::Win32::UI::WindowsAndMessaging::{IsWindow, ShowWindow};

use crate::{Handle, declare_handle};

declare_handle!(
  HWindow,
  alias = "HWND",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd"
);

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
