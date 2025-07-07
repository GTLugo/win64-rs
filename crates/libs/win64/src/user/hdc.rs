use windows_sys::Win32::Graphics::Gdi::FillRect;

use crate::{Handle, Rect, declare_handle};

use super::Brush;

declare_handle!(
  DeviceContext,
  alias = "HDC",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc"
);

impl DeviceContext {
  pub fn fill_rect(&self, rect: Rect, brush: Brush) {
    let r = rect.to_raw();
    unsafe { FillRect(self.to_ptr(), &raw const r, brush.to_ptr()) };
  }
}
