use std::ffi::OsString;

use windows_sys::Win32::UI::WindowsAndMessaging::{RegisterClassExW, WNDCLASSEXW};

use crate::Handle;

use super::{HInstance, WindowClassStyle, procedure::window_procedure};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowClass {
  Local {
    style: WindowClassStyle,
    instance: HInstance,
    name: OsString,
  },
  System(OsString),
}

impl Default for WindowClass {
  fn default() -> Self {
    Self::Local {
      style: WindowClassStyle::empty(),
      instance: Default::default(),
      name: Default::default(),
    }
  }
}

impl WindowClass {
  pub fn atom(&self) -> *const u16 {
    match self {
      WindowClass::Local { name, .. } => name,
      WindowClass::System(name) => name,
    }
    .as_encoded_bytes()
    .as_ptr()
    .cast()
  }
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct WindowClass<'name> {
//   pub style: WindowClassStyle,
//   pub instance: HInstance,
//   pub name: &'name OsStr,
// }

impl WindowClass {
  pub fn register(&self) {
    if let Self::Local { style, instance, name } = self {
      // let lpsz_class_name = self.name.encode_wide().collect::<Vec<u16>>();
      let name = name.as_encoded_bytes();

      let wc = WNDCLASSEXW {
        cbSize: core::mem::size_of::<WNDCLASSEXW>() as _,
        hInstance: instance.to_ptr(),
        lpszClassName: name.as_ptr().cast(),
        lpfnWndProc: Some(window_procedure),
        style: style.to_raw(),
        hCursor: std::ptr::null_mut(),
        ..Default::default()
      };

      unsafe { RegisterClassExW(&wc) };
    }
  }
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Atom(*const u16);

// impl Atom {
//   pub fn as_ptr(&self) -> *const u16 {
//     self.0.as_ptr()
//   }
// }

// impl From<&str> for Atom {
//     fn from(value: &str) -> Self {
//       Self(value.as_ptr().cast())
//     }
// }
