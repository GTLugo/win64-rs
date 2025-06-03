use crate::{Handle, declare_handle};

declare_handle!(
  Window,
  alias = "HWND",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd"
);

declare_handle!(
  Instance,
  alias = "HINSTANCE",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance"
);

#[doc(alias = "docs")]
pub struct WindowClass {}
