pub mod hwnd;
pub use hwnd::*;

use crate::declare_handle;

declare_handle!(
  HInstance,
  alias = "HINSTANCE",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance"
);

#[doc(alias = "docs")]
pub struct WindowClass {}
