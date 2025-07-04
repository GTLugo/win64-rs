pub mod procedure;
pub use procedure::*;

pub mod response;
pub use response::*;

use cursor_icon::CursorIcon;
use win64_macro::ClassAtom;
use windows_sys::Win32::UI::WindowsAndMessaging::{RegisterClassExW, WNDCLASSEXW};

use crate::Handle;

use super::{Class, Instance, LoadCursor, NoProc, Window, WindowBuilder, styles::WindowClassStyle};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomClass {
  name: &'static str, // Class names are stored as static string slices to ensure their pointers remain valid.
  style: WindowClassStyle,
  instance: Instance,
  // I will add more fields later :)
}

impl CustomClass {
  pub const fn atom(&self) -> *const u16 {
    self.name.as_ptr().cast()
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
    Window::builder().class(self.clone())
  }
}

pub struct NoName;
pub struct Name(&'static str);

pub struct WindowClassBuilder<N> {
  name: N,
  style: WindowClassStyle,
  instance: Instance,
}

impl Default for WindowClassBuilder<NoName> {
  fn default() -> Self {
    WindowClassBuilder {
      name: NoName,
      style: WindowClassStyle::default(),
      instance: Instance::get(),
    }
  }
}

impl WindowClassBuilder<NoName> {
  pub fn name(self, name: &'static str) -> WindowClassBuilder<Name> {
    WindowClassBuilder {
      name: Name(name),
      style: self.style,
      instance: self.instance,
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
}

impl WindowClassBuilder<Name> {
  pub fn register(self) -> WindowClass {
    let wc = WNDCLASSEXW {
      cbSize: core::mem::size_of::<WNDCLASSEXW>() as _,
      hInstance: self.instance.to_ptr(),
      lpszClassName: self.name.0.as_ptr().cast(),
      lpfnWndProc: Some(window_procedure),
      style: self.style.to_raw(),
      hCursor: CursorIcon::Default.load(),
      ..Default::default()
    };

    unsafe { RegisterClassExW(&wc) };

    WindowClass::Custom(CustomClass {
      name: self.name.0,
      style: self.style,
      instance: self.instance,
    })
  }
}
