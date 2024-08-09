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

pub struct WindowDescriptor<Procedure: 'static + WindowProcedure> {
  pub title: String,
  pub position: Option<Position>,
  pub size: Option<Size>,
  pub style: WindowStyle,
  pub ext_style: ExtendedWindowStyle,
  pub procedure: Procedure,
}

impl<Procedure: 'static + WindowProcedure> WindowDescriptor<Procedure> {
  pub fn new(procedure: Procedure) -> Self {
    Self {
      title: "Window".to_owned(),
      position: Default::default(),
      size: Default::default(),
      style: WindowStyle::empty(),
      ext_style: ExtendedWindowStyle::empty(),
      procedure,
    }
  }
}

impl Window {
  pub fn is_invalid(&self) -> bool {
    self.0.is_null()
  }

  pub fn quit(&self) {
    self.quit_with_code(0)
  }

  pub fn quit_with_code(&self, exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
  }

  pub fn new(
    class: WindowClass,
    desc: WindowDescriptor<impl 'static + WindowProcedure>,
  ) -> Result<Self, windows::core::Error> {
    let title: HSTRING = desc.title.into();
    let mut create_info = CreateInfo {
      user_state: Some(Box::new(desc.procedure)),
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
