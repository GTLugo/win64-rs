use crate::{Handle, declare_handle};

declare_handle!(
  HWindow,
  alias = "HWND",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd"
);

declare_handle!(
  HInstance,
  alias = "HINSTANCE",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance"
);

#[doc(alias = "docs")]
pub struct WindowClass {}
