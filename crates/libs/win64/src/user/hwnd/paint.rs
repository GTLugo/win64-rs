use {
  super::Window,
  crate::{
    Handle,
    Rect,
    declare_handle,
    user::{
      DeviceContext,
      is_os_dark_mode,
    },
  },
  rgb::RGB8,
  windows_sys::Win32::Graphics::Gdi::{
    BeginPaint,
    COLOR_BACKGROUND,
    COLOR_WINDOW,
    CreateSolidBrush,
    DeleteObject,
    EndPaint,
    PAINTSTRUCT,
  },
};

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
  pub fn begin_paint(&self, mut f: impl FnMut(DeviceContext, Paint)) {
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
    f(hdc, paint);
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

// impl Drop for Brush {
//     fn drop(&mut self) {
//         // TODO!
//     }
// }

impl Brush {
  pub fn delete(self) {
    unsafe { DeleteObject(self.to_ptr()) };
  }

  pub fn color_window_auto_dark() -> Self {
    match is_os_dark_mode() {
      true => Self::solid((32, 32, 32)),
      false => unsafe { Self::from_raw((COLOR_WINDOW + 1) as _) },
    }
  }

  pub fn color_window() -> Self {
    unsafe { Self::from_raw((COLOR_WINDOW + 1) as _) }
  }

  pub fn color_background() -> Self {
    unsafe { Self::from_raw((COLOR_BACKGROUND + 1) as _) }
  }

  pub fn solid(color: impl Into<RGB8>) -> Self {
    let RGB8 { r, g, b } = color.into();
    let color: u32 = ((b as u32) << 16) | ((g as u32) << 8) | r as u32;
    unsafe { Self::from_ptr(CreateSolidBrush(color)) }
  }
}
