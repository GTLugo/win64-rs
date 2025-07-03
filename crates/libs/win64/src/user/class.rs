pub mod procedure;

pub use procedure::*;

use cursor_icon::CursorIcon;
use windows_sys::Win32::UI::WindowsAndMessaging::{RegisterClassExW, WNDCLASSEXW};

use crate::Handle;

use super::{Class, Instance, LoadCursor, NoProc, Window, WindowBuilder, styles::WindowClassStyle};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppClass {
  name: &'static str, // Class names are stored as static string slices to ensure their pointers remain valid.
  style: WindowClassStyle,
  instance: Instance,
  // I will add more fields later :)
}

impl AppClass {
  pub const fn atom(&self) -> *const u16 {
    self.name.as_ptr().cast()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowClass {
  App(AppClass),
  Button,
  ComboBox,
  Edit,
  ListBox,
  MDIClient,
  ScrollBar,
  Static,
}

impl WindowClass {
  pub const fn atom(&self) -> *const u16 {
    struct ClassName(*const u16);
    unsafe impl Sync for ClassName {}

    // Hmmm this can't be good for memory footprint...
    static BUTTON: ClassName = ClassName(windows_sys::w!("Button"));
    static COMBO_BOX: ClassName = ClassName(windows_sys::w!("ComboBox"));
    static EDIT: ClassName = ClassName(windows_sys::w!("Edit"));
    static LIST_BOX: ClassName = ClassName(windows_sys::w!("ListBox"));
    static MDICLIENT: ClassName = ClassName(windows_sys::w!("MDIClient"));
    static SCROLL_BAR: ClassName = ClassName(windows_sys::w!("ScrollBar"));
    static STATIC: ClassName = ClassName(windows_sys::w!("Static"));

    match self {
      WindowClass::App(class) => class.atom(),
      Self::Button => BUTTON.0,
      Self::ComboBox => COMBO_BOX.0,
      Self::Edit => EDIT.0,
      Self::ListBox => LIST_BOX.0,
      Self::MDIClient => MDICLIENT.0,
      Self::ScrollBar => SCROLL_BAR.0,
      Self::Static => STATIC.0,
    }
  }

  pub fn builder() -> WindowClassBuilder<NoName> {
    WindowClassBuilder {
      name: NoName,
      style: WindowClassStyle::default(),
      instance: Instance::get(),
    }
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

    WindowClass::App(AppClass {
      name: self.name.0,
      style: self.style,
      instance: self.instance,
    })
  }
}
