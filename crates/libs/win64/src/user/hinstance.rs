use windows_sys::Win32::System::SystemServices::IMAGE_DOS_HEADER;

use crate::declare_handle;

declare_handle!(
  HInstance,
  alias = "HINSTANCE",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance"
);

impl HInstance {
  pub fn get() -> Self {
    // https://devblogs.microsoft.com/oldnewthing/20041025-00/?p=37483
    unsafe extern "C" {
      static mut __ImageBase: IMAGE_DOS_HEADER;
    }
    Self(std::ptr::addr_of_mut!(__ImageBase).cast())
  }
}
