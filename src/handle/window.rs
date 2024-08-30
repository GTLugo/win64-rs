use windows::{
  core::HSTRING,
  Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, PostQuitMessage, SetWindowTextW},
  },
};

use crate::{
  message::{Message, NoMetadata}, prelude::{WindowDescriptor, WindowProcedure}, procedure::CreateInfo, types::{Position, Size, WindowClass}, ProcedureResult
};

use super::{Handle, Win32Type};

pub type WindowHandle = Handle<Window>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Window;

impl Window {
  pub fn new(
    class: &WindowClass,
    desc: &WindowDescriptor,
    window_state: impl 'static + WindowProcedure,
  ) -> Result<Handle<Window>, windows::core::Error> {
    let title = HSTRING::from(desc.title.clone());
    let mut create_info = CreateInfo {
      state: Some(Box::new(window_state)),
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
}

impl Win32Type for Handle<Window> {
  type Type = HWND;

  fn to_win32(&self) -> Self::Type {
    (*self).into()
  }
}

impl Handle<Window> {
  pub fn default_procedure(
    &self,
    message: Message<NoMetadata>,
  ) -> ProcedureResult {
    unsafe {
      DefWindowProcW(
        self.to_win32(),
        message.id(),
        WPARAM(message.w()),
        LPARAM(message.l()),
      )
    }
    .into()
  }

  pub fn quit(&self) {
    self.quit_with_code(0)
  }

  pub fn quit_with_code(&self, exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
  }

  pub fn set_window_text(&self, text: impl Into<String>) -> windows::core::Result<()> {
    let text = HSTRING::from(text.into());
    unsafe { SetWindowTextW(self.to_win32(), &text) }
  }
}

impl From<Handle<Window>> for HWND {
  fn from(value: Handle<Window>) -> Self {
    Self(value.as_ptr())
  }
}

impl From<HWND> for Handle<Window> {
  fn from(value: HWND) -> Self {
    unsafe { Self::from_raw(value.0) }
  }
}
