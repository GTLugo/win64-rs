use windows_sys::Win32::Graphics::Gdi::{BeginPaint, COLOR_WINDOW, CreateSolidBrush, EndPaint, PAINTSTRUCT};

use crate::{Handle, Rect, declare_handle, user::DeviceContext};

use super::Window;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reserved {
  restore: windows_sys::core::BOOL,
  update: windows_sys::core::BOOL,
  rgb_reserved: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Paint {
  pub hdc: DeviceContext,
  pub erase: bool,
  pub paint: Rect,
  _reserved: Reserved,
}

// #[derive(Debug)]
// pub struct PaintGuard {
//   pub inner: Paint,
//   _hwnd: Window,
// }

// impl Deref for PaintGuard {
//   type Target = Paint;

//   fn deref(&self) -> &Self::Target {
//     &self.inner
//   }
// }

// impl Drop for PaintGuard {
//   fn drop(&mut self) {
//     self._hwnd.end_paint(self.inner);
//   }
// }

impl Paint {
  pub fn to_raw(&self) -> PAINTSTRUCT {
    PAINTSTRUCT {
      hdc: self.hdc.to_ptr(),
      fErase: self.erase as i32,
      rcPaint: self.paint.to_raw(),
      fRestore: self._reserved.restore,
      fIncUpdate: self._reserved.update,
      rgbReserved: self._reserved.rgb_reserved,
    }
  }
}

impl Window {
  // TODO: make this return a guard object to ensure end paint is always called.
  pub fn begin_paint(&self, mut f: impl FnMut(Paint)) {
    let mut paint = PAINTSTRUCT::default();
    let hdc = unsafe { BeginPaint(self.to_ptr(), &raw mut paint) };
    let hdc = unsafe { DeviceContext::from_ptr(hdc) };
    let paint = Paint {
      hdc,
      erase: paint.fErase != 0,
      paint: Rect::from(paint.rcPaint),
      _reserved: Reserved {
        restore: paint.fRestore,
        update: paint.fIncUpdate,
        rgb_reserved: paint.rgbReserved,
      },
    };
    f(paint);
    self.end_paint(paint);
  }

  fn end_paint(&self, paint: Paint) {
    let raw = paint.to_raw();
    unsafe { EndPaint(self.to_ptr(), &raw const raw) };
  }
}

declare_handle!(
  Brush,
  alias = "HBRUSH",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbrush"
);

impl Brush {
  pub fn color_window() -> Self {
    unsafe { Self::from_raw((COLOR_WINDOW + 1) as _) }
  }

  pub fn solid(color: u32) -> Self {
    unsafe { Self::from_ptr(CreateSolidBrush(color)) }
  }
}
