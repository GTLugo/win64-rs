pub mod procedure;
use std::sync::OnceLock;

pub use procedure::*;

pub mod response;
pub use response::*;

use cursor_icon::CursorIcon;
use widestring::U16CString;
use win64_macro::ClassAtom;
use windows_result::Error;
use windows_sys::Win32::UI::WindowsAndMessaging::{RegisterClassExW, WNDCLASSEXW};

use crate::{Handle, get_last_error, reset_last_error};

use super::{Brush, Class, Instance, LoadCursor, NoProc, Window, WindowBuilder, styles::WindowClassStyle};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomClass {
  name: &'static U16CString, // Class names are stored as static string slices to ensure their pointers remain valid.
  style: WindowClassStyle,
  instance: Instance,
  background: Brush, // I will add more fields later :)
}

impl CustomClass {
  pub fn atom(&self) -> *const u16 {
    self.name.as_ptr()
  }
}

#[derive(ClassAtom, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowClass {
  Custom(CustomClass),
  Button,
  ComboBox,
  Edit,
  ListBox,
  MDIClient,
  ScrollBar,
  Static,
}

impl WindowClass {
  pub fn builder() -> WindowClassBuilder<NoName> {
    WindowClassBuilder::default()
  }

  pub fn window_builder(&self) -> WindowBuilder<Class, NoProc> {
    Window::builder().class(self.clone()).instance(Some(self.instance()))
  }

  pub fn instance(&self) -> Instance {
    match self {
      WindowClass::Custom(custom_class) => custom_class.instance,
      _ => Instance::get(),
    }
  }
}

pub struct NoName;
pub struct Name<'s>(&'s str);

pub struct WindowClassBuilder<N> {
  name: N,
  style: WindowClassStyle,
  instance: Instance,
  background: Brush,
}

impl Default for WindowClassBuilder<NoName> {
  fn default() -> Self {
    WindowClassBuilder {
      name: NoName,
      style: WindowClassStyle::default(),
      instance: Instance::get(),
      background: Brush::default(),
    }
  }
}

impl WindowClassBuilder<NoName> {
  pub fn name<'s>(self, name: &'s str) -> WindowClassBuilder<Name<'s>> {
    WindowClassBuilder {
      name: Name(name),
      style: self.style,
      instance: self.instance,
      background: self.background,
    }
  }
}

impl<N> WindowClassBuilder<N> {
  pub fn instance(mut self, instance: Instance) -> Self {
    self.instance = instance;
    self
  }

  pub fn style(mut self, style: WindowClassStyle) -> Self {
    self.style = style;
    self
  }

  pub fn background_brush(mut self, brush: Brush) -> Self {
    self.background = brush;
    self
  }
}

impl WindowClassBuilder<Name<'_>> {
  pub fn register(self) -> Result<WindowClass, Error> {
    static WIDE_STRING: OnceLock<U16CString> = OnceLock::new();
    let name = WIDE_STRING.get_or_init(|| U16CString::from_str_truncate(self.name.0));
    let wc = WNDCLASSEXW {
      cbSize: core::mem::size_of::<WNDCLASSEXW>() as _,
      hInstance: self.instance.to_ptr(),
      lpszClassName: name.as_ptr(),
      lpfnWndProc: Some(window_procedure),
      style: self.style.to_raw(),
      hCursor: CursorIcon::Default.load(),
      hbrBackground: self.background.to_ptr(),
      ..Default::default()
    };

    reset_last_error();
    let atom = unsafe { RegisterClassExW(&wc) };
    let error = get_last_error();

    match (atom, error) {
      (0, Some(error)) => Err(error),
      _ => Ok(WindowClass::Custom(CustomClass {
        name,
        style: self.style,
        instance: self.instance,
        background: self.background,
      })),
    }
  }
}
