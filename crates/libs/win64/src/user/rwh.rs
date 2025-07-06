#[cfg(any(feature = "rwh_06", feature = "rwh_05"))]
#[allow(unused)]
pub use inner::*;

#[cfg(all(feature = "rwh_05", not(any(feature = "rwh_06"))))]
pub mod inner {
  use rwh_05::{
    ActiveHandle, DisplayHandle, HandleError, HasDisplayHandle, HasRawDisplayHandle, HasRawWindowHandle,
    HasWindowHandle, RawDisplayHandle, RawWindowHandle, Win32WindowHandle, WindowHandle, WindowsDisplayHandle,
  };

  use crate::{Handle, user::Window};

  impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
      Ok(unsafe { WindowHandle::borrow_raw(self.raw_window_handle(), ActiveHandle::new()) })
    }
  }

  unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
      let mut handle = Win32WindowHandle::empty();
      handle.hwnd = self.to_ptr().cast();
      handle.hinstance = self.instance().to_ptr();
      RawWindowHandle::Win32(handle)
    }
  }

  impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
      Ok(unsafe { DisplayHandle::borrow_raw(self.raw_display_handle()) })
    }
  }

  unsafe impl HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> RawDisplayHandle {
      RawDisplayHandle::Windows(WindowsDisplayHandle::empty())
    }
  }
}

#[cfg(all(feature = "rwh_06", not(any(feature = "rwh_05"))))]
mod inner {
  use std::num::NonZero;

  use rwh_06::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    Win32WindowHandle, WindowHandle, WindowsDisplayHandle,
  };

  use crate::{Handle, user::Window};

  impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
      let mut handle = Win32WindowHandle::new(NonZero::new(self.to_raw().cast_signed()).unwrap());
      handle.hinstance = Some(NonZero::new(self.instance().to_raw().cast_signed()).unwrap());
      Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::from(handle)) })
    }
  }

  impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
      let handle = WindowsDisplayHandle::new();
      Ok(unsafe { DisplayHandle::borrow_raw(RawDisplayHandle::Windows(handle)) })
    }
  }
}
