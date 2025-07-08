use windows_sys::Win32::Graphics::Gdi::{FillRect, Polygon, SelectObject};

use crate::{declare_handle, Handle, Point, Rect};

use super::Brush;

declare_handle!(
  DeviceContext,
  alias = "HDC",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc"
);

impl DeviceContext {
  pub fn fill_rect(&self, rect: &Rect, brush: &Brush) {
    let r = rect.to_raw();
    unsafe { FillRect(self.to_ptr(), &raw const r, brush.to_ptr()) };
  }

  pub fn polygon(&self, points: &[Point], brush: &Brush) {
    let old_brush = unsafe { SelectObject(self.to_ptr(), brush.to_ptr()) };
    unsafe { Polygon(self.to_ptr(), points.as_ptr().cast(), points.len() as _) };
    unsafe { SelectObject(self.to_ptr(), old_brush) };
  }
}
