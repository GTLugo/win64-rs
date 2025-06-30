use cursor_icon::CursorIcon;
use windows_sys::Win32::UI::WindowsAndMessaging::{RegisterClassExW, WNDCLASSEXW};

use crate::Handle;

use super::{HInstance, LoadCursor, WindowClassStyle, procedure::window_procedure};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppClass {
  style: WindowClassStyle,
  instance: HInstance,
  name: &'static str, // Class names are stored as static string slices to ensure their pointers remain valid.
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

struct ClassName(*const u16);

unsafe impl Sync for ClassName {}

impl WindowClass {
  pub const fn atom(&self) -> *const u16 {
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
}

impl WindowClass {
  pub fn register(style: WindowClassStyle, instance: HInstance, name: impl Into<&'static str>) -> Self {
    let class = AppClass {
      style,
      instance,
      name: name.into(),
    };

    let wc = WNDCLASSEXW {
      cbSize: core::mem::size_of::<WNDCLASSEXW>() as _,
      hInstance: instance.to_ptr(),
      lpszClassName: class.atom(),
      lpfnWndProc: Some(window_procedure),
      style: style.to_raw(),
      hCursor: CursorIcon::Default.load(),
      ..Default::default()
    };
    unsafe { RegisterClassExW(&wc) };

    Self::App(class)
  }
}
