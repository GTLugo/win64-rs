use windows_sys::Win32::UI::WindowsAndMessaging::IsWindow;

use crate::{Handle, declare_handle};

declare_handle!(
  Window,
  alias = "HWND",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd"
);

impl Window {
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

declare_handle!(
  Instance,
  alias = "HINSTANCE",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance"
);

#[doc(alias = "docs")]
pub struct WindowClass {}
