use windows::{
  core::{HSTRING, PCWSTR},
  Win32::{
    Foundation::HINSTANCE,
    UI::WindowsAndMessaging::{
      GetClassInfoExW, RegisterClassExW, UnregisterClassW, CW_USEDEFAULT, WNDCLASSEXW,
      WNDCLASS_STYLES,
    },
  },
};

use crate::{
  flag::WindowClassStyle,
  handle::Instance,
  procedure::{self},
};

pub struct Registered;
pub struct Unregistered;

pub struct WindowClass {
  instance: Instance,
  name: String,
}

impl WindowClass {
  pub fn new(desc: &WindowClassDescriptor) -> Self {
    let name = HSTRING::from(desc.name.clone());
    let wc = WNDCLASSEXW {
      cbSize: core::mem::size_of::<WNDCLASSEXW>() as _,
      hInstance: desc.instance.into(),
      lpszClassName: PCWSTR(name.as_ptr()),
      lpfnWndProc: Some(procedure::window_procedure),
      style: WNDCLASS_STYLES(desc.style.bits()),
      ..Default::default()
    };

    unsafe { RegisterClassExW(&wc) };

    Self {
      instance: desc.instance,
      name: desc.name.clone(),
    }
  }

  pub fn get(instance: &Instance, name: String) -> Result<Self, windows::core::Error> {
    let hstring = HSTRING::from(name.clone());
    let mut class = WNDCLASSEXW::default();
    let result =
      unsafe { GetClassInfoExW(HINSTANCE::from(*instance), &hstring, &mut class) };
    result.map(|_| Self {
      instance: *instance,
      name,
    })
  }

  pub fn unregister(self) -> Result<(), windows::core::Error> {
    let hstring = HSTRING::from(self.name);
    unsafe {
      UnregisterClassW(PCWSTR(hstring.as_ptr()), HINSTANCE::from(self.instance))
    }?;

    Ok(())
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn instance(&self) -> &Instance {
    &self.instance
  }
}

pub struct WindowClassDescriptor {
  pub instance: Instance,
  pub name: String,
  pub style: WindowClassStyle,
}

impl Default for WindowClassDescriptor {
  fn default() -> Self {
    Self {
      instance: Instance::default(),
      name: "Window Class".to_owned(),
      style: WindowClassStyle::empty(),
    }
  }
}

pub struct Size {
  pub width: i32,
  pub height: i32,
}

impl Size {
  pub const AUTO: Self = Self {
    width: CW_USEDEFAULT,
    height: CW_USEDEFAULT,
  };
}

impl From<[i32; 2]> for Size {
  fn from([width, height]: [i32; 2]) -> Self {
    Self { width, height }
  }
}

impl From<(i32, i32)> for Size {
  fn from((width, height): (i32, i32)) -> Self {
    Self { width, height }
  }
}

pub struct Position {
  pub x: i32,
  pub y: i32,
}

impl Position {
  pub const AUTO: Self = Self {
    x: CW_USEDEFAULT,
    y: CW_USEDEFAULT,
  };
}

impl From<[i32; 2]> for Position {
  fn from([x, y]: [i32; 2]) -> Self {
    Self { x, y }
  }
}

impl From<(i32, i32)> for Position {
  fn from((x, y): (i32, i32)) -> Self {
    Self { x, y }
  }
}
