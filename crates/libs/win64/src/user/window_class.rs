use std::{ffi::OsString, marker::PhantomData};

use windows_sys::Win32::UI::WindowsAndMessaging::{RegisterClassExW, WNDCLASSEXW};

use crate::Handle;

use super::{HInstance, WindowClassStyle, procedure::window_procedure};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Registered;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotRegistered;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowClass<T> {
  New {
    style: WindowClassStyle,
    instance: HInstance,
    name: OsString,
    _0: PhantomData<T>,
  },
  System(OsString),
}

// impl Default for WindowClass<NotRegistered> {
//   fn default() -> Self {
//     Self::Local {
//       style: WindowClassStyle::empty(),
//       instance: Default::default(),
//       name: Default::default(),
//       _0: PhantomData,
//     }
//   }
// }

impl<T> WindowClass<T> {
  pub fn atom(&self) -> *const u16 {
    match self {
      WindowClass::New { name, .. } => name,
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

impl WindowClass<NotRegistered> {
  pub fn new(style: WindowClassStyle, instance: HInstance, name: impl Into<OsString>) -> Self {
    Self::New {
      style,
      instance,
      name: name.into(),
      _0: PhantomData,
    }
  }
  pub fn system(name: impl Into<OsString>) -> Self {
    Self::System(name.into())
  }

  pub fn register(self) -> WindowClass<Registered> {
    match self {
      Self::New {
        style, instance, name, ..
      } => {
        let new_class = WindowClass::New {
          style,
          instance,
          name,
          _0: PhantomData,
        };
        let WindowClass::New { name, .. } = &new_class else {
          unreachable!()
        };
        let wc = WNDCLASSEXW {
          cbSize: core::mem::size_of::<WNDCLASSEXW>() as _,
          hInstance: instance.to_ptr(),
          lpszClassName: name.as_encoded_bytes().as_ptr().cast(),
          lpfnWndProc: Some(window_procedure),
          style: style.to_raw(),
          hCursor: std::ptr::null_mut(),
          ..Default::default()
        };
        unsafe { RegisterClassExW(&wc) };
        new_class
      }
      Self::System(class) => WindowClass::System(class.clone()),
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
