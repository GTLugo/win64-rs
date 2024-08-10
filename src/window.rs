use windows::{
  core::HSTRING,
  Win32::{
    Foundation::{HINSTANCE, HWND},
    UI::WindowsAndMessaging::{CreateWindowExW, PostQuitMessage},
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
      style: WindowStyle::empty(),
      ext_style: ExtendedWindowStyle::empty(),
    }
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
    class: WindowClass,
    desc: WindowDescriptor,
    procedure: Procedure,
  ) -> Result<Self, windows::core::Error> {
    let title: HSTRING = desc.title.into();
    let mut create_info = CreateInfo {
      user_state: Some(Box::new(procedure)),
    };
    let position = desc.position.unwrap_or(Position::AUTO);
    let size = desc.size.unwrap_or(Size::AUTO);
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
