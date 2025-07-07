use windows_sys::Win32::Graphics::Gdi::{COLOR_WINDOW, FillRect, HBRUSH};

use crate::{Handle, Rect, declare_handle};

declare_handle!(
  DeviceContext,
  alias = "HDC",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc"
);

impl DeviceContext {
  pub fn fill_rect_color_window(&self, rect: Rect) {
    let r = rect.to_raw();
    unsafe { FillRect(self.to_ptr(), &raw const r, (COLOR_WINDOW + 1) as HBRUSH) };
  }
}
