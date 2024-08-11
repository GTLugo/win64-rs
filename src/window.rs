use windows::{
  core::HSTRING,
  Win32::{
    Foundation::{HINSTANCE, HWND},
    UI::WindowsAndMessaging::{CreateWindowExW, PostQuitMessage, SetWindowTextW},
  },
};

use crate::{
  flag::{ExtendedWindowStyle, WindowStyle},
  handle::Handle,
  procedure::{CreateInfo, WindowProcedure},
  types::{Position, Size, WindowClass},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Window(Handle);

pub struct WindowDescriptor {
  pub title: String,
  pub position: Option<Position>,
  pub size: Option<Size>,
  pub style: WindowStyle,
  pub ext_style: ExtendedWindowStyle,
}

impl Default for WindowDescriptor {
  fn default() -> Self {
    Self {
      title: "Window".to_owned(),
      position: Default::default(),
      size: Default::default(),
      style: WindowStyle::OverlappedWindow | WindowStyle::Visible,
      ext_style: ExtendedWindowStyle::empty(),
    }
  }
}

impl WindowDescriptor {
  pub fn with_title(self, title: impl Into<String>) -> Self {
    Self {
      title: title.into(),
      ..self
    }
  }

  pub fn with_position(self, position: Option<impl Into<Position>>) -> Self {
    Self {
      position: position.map(Into::into),
      ..self
    }
  }

  pub fn with_size(self, size: Option<impl Into<Size>>) -> Self {
    Self {
      size: size.map(Into::into),
      ..self
    }
  }

  pub fn with_style(self, style: WindowStyle) -> Self {
    Self { style, ..self }
  }

  pub fn with_ext_style(self, ext_style: ExtendedWindowStyle) -> Self {
    Self { ext_style, ..self }
  }
}

impl Window {
  pub fn as_handle(&self) -> HWND {
    (*self).into()
  }

  pub fn is_invalid(&self) -> bool {
    self.0.is_null()
  }

  pub fn quit(&self) {
    self.quit_with_code(0)
  }

  pub fn quit_with_code(&self, exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
  }

  pub fn new<Procedure: 'static + WindowProcedure>(
    class: &WindowClass,
    desc: &WindowDescriptor,
    procedure: Procedure,
  ) -> Result<Self, windows::core::Error> {
    let title = HSTRING::from(desc.title.clone());
    let mut create_info = CreateInfo {
      user_state: Some(Box::new(procedure)),
    };
    let position = desc.position.clone().unwrap_or(Position::AUTO);
    let size = desc.size.clone().unwrap_or(Size::AUTO);
    let instance = HINSTANCE::from(*class.instance());
    let class_name = HSTRING::from(class.name());

    unsafe {
      CreateWindowExW(
        desc.ext_style.into(),
        &class_name,
        &title,
        desc.style.into(),
        position.x,
        position.y,
        size.width,
        size.height,
        None,
        None,
        instance,
        Some(std::ptr::addr_of_mut!(create_info).cast()),
      )
    }
    .map(Into::into)
  }

  pub fn set_text(&self, text: impl Into<String>) -> windows::core::Result<()> {
    let text = HSTRING::from(text.into());
    unsafe { SetWindowTextW(self.as_handle(), &text) }
  }
}

impl From<Window> for HWND {
  fn from(value: Window) -> Self {
    Self(value.0.as_ptr())
  }
}

impl From<HWND> for Window {
  fn from(value: HWND) -> Self {
    Self(Handle::from_raw(value.0))
  }
}

// #[cfg(all(feature = "rwh_06", not(feature = "rwh_05")))]
// pub mod rwh_06 {
//   use rwh_06::{
//     DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
//     RawWindowHandle, Win32WindowHandle, WindowHandle, WindowsDisplayHandle,
//   };

//   use crate::handle::Instance;

//   use super::Window;

//   impl Window {
//     pub fn raw_window_handle(&self) -> RawWindowHandle {
//       let mut handle = Win32WindowHandle::new(
//         std::num::NonZeroIsize::new(self.0.as_ptr() as _)
//           .expect("window handle should not be zero"),
//       );
//       let hinstance =
//         std::num::NonZeroIsize::new(unsafe { Instance::get_exe().as_ptr() } as _)
//           .expect("instance handle should not be zero");
//       handle.hinstance = Some(hinstance);
//       RawWindowHandle::from(handle)
//     }

//     pub fn raw_display_handle(&self) -> RawDisplayHandle {
//       let handle = WindowsDisplayHandle::new();
//       RawDisplayHandle::from(handle)
//     }
//   }

//   impl HasWindowHandle for Window {
//     fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
//       Ok(unsafe { WindowHandle::borrow_raw(self.raw_window_handle()) })
//     }
//   }

//   impl HasDisplayHandle for Window {
//     fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
//       Ok(unsafe { DisplayHandle::borrow_raw(self.raw_display_handle()) })
//     }
//   }
// }

// #[cfg(all(feature = "rwh_05", not(feature = "rwh_06")))]
// pub mod rwh_05 {
//   use rwh_05::{
//     HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
//     Win32WindowHandle, WindowsDisplayHandle,
//   };

//   use crate::handle::Instance;

//   use super::Window;

//   unsafe impl HasRawWindowHandle for Window {
//     fn raw_window_handle(&self) -> RawWindowHandle {
//       let mut handle = Win32WindowHandle::empty();
//       handle.hwnd = self.0.as_ptr();
//       handle.hinstance = unsafe { Instance::get_exe().as_ptr() };
//       RawWindowHandle::Win32(handle)
//     }
//   }

//   unsafe impl HasRawDisplayHandle for Window {
//     fn raw_display_handle(&self) -> RawDisplayHandle {
//       RawDisplayHandle::Windows(WindowsDisplayHandle::empty())
//     }
//   }
// }
