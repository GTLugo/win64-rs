use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use windows_sys::Win32::UI::WindowsAndMessaging::{RegisterClassExW, WNDCLASSEXW};

use crate::Handle;

use super::{HInstance, WindowClassStyle, procedure::window_procedure};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowClass {
  pub style: WindowClassStyle,
  pub instance: HInstance,
  pub name: OsString,
}

impl WindowClass {
  pub fn register(self) -> ClassId {
    let lpsz_class_name = self.name.encode_wide().collect::<Vec<u16>>();

    let wc = WNDCLASSEXW {
      cbSize: core::mem::size_of::<WNDCLASSEXW>() as _,
      hInstance: self.instance.to_ptr(),
      lpszClassName: lpsz_class_name.as_ptr(),
      lpfnWndProc: Some(window_procedure),
      style: self.style.to_raw(),
      hCursor: std::ptr::null_mut(),
      ..Default::default()
    };

    unsafe { RegisterClassExW(&wc) };

    ClassId(lpsz_class_name)
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassId(Vec<u16>);

impl ClassId {
  pub fn as_ptr(&self) -> *const u16 {
    self.0.as_ptr()
  }
}

impl From<&str> for ClassId {
  fn from(value: &str) -> Self {
    let string: Vec<u16> = OsString::from(value).encode_wide().collect();
    Self(string)
  }
}
