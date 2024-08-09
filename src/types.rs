use std::marker::PhantomData;

use windows::{
  core::{HSTRING, PCWSTR},
  Win32::{
    Foundation::HINSTANCE,
    UI::WindowsAndMessaging::{
      CreateWindowExW, RegisterClassExW, UnregisterClassW, CW_USEDEFAULT, WNDCLASSEXW,
    },
  },
};

use crate::{
  flag::{ExtendedWindowStyle, WindowClassStyle, WindowStyle},
  handle::Instance,
  procedure::{self, CreateInfo, WindowProcedure},
  window::Window,
};

pub struct Registered;
pub struct Unregistered;

// pub struct NoProcedure;
// pub struct WithProcedure<W: WindowProcedure>(W);

pub enum WindowClass2 {
  Descriptor(WindowClassDescriptor),
  Handle { instance: Instance, name: String },
}

pub struct WindowClassDescriptor {
  pub instance: Instance,
  pub name: String,
  pub style: WindowClassStyle,
}

impl WindowClassDescriptor {
  pub fn try_default() -> Result<Self, windows::core::Error> {
    Ok(Self {
      instance: Instance::get_exe()?,
      name: "Window Class".to_owned(),
      style: WindowClassStyle::empty(),
    })
  }

  pub(crate) fn register(&self) -> Result<WindowClass<Registered>, windows::core::Error> {
    let name = HSTRING::from(self.name.clone());
    let wc = WNDCLASSEXW {
      cbSize: core::mem::size_of::<WNDCLASSEXW>() as _,
      hInstance: self.instance.into(),
      lpszClassName: PCWSTR(name.as_ptr()),
      lpfnWndProc: Some(procedure::window_procedure),
      ..Default::default()
    };

    unsafe { RegisterClassExW(&wc) };

    Ok(WindowClass {
      instance: self.instance.into(),
      name,
      style: self.style,
      _registration: PhantomData,
    })
  }
}

pub struct WindowClass<Registration> {
  instance: Option<Instance>,
  name: HSTRING,
  style: WindowClassStyle,
  _registration: PhantomData<Registration>,
}

// impl Default for WindowClass<Unregistered> {
//   fn default() -> Self {
//     Self {
//       instance: None,
//       name: "Window Class".into(),
//       style: WindowClassStyle::empty(),
//       _registration: PhantomData,
//     }
//   }
// }

impl WindowClass<Unregistered> {
  pub fn new(name: impl Into<HSTRING>) -> WindowClass<Unregistered> {
    WindowClass {
      instance: None,
      name: name.into(),
      style: WindowClassStyle::empty(),
      _registration: PhantomData,
    }
  }
}

impl WindowClass<Unregistered> {
  pub fn with_instance(self, instance: Instance) -> Self {
    Self {
      instance: Some(instance),
      ..self
    }
  }

  pub fn with_style(self, style: WindowClassStyle) -> Self {
    Self { style, ..self }
  }

  pub fn register(self) -> Result<WindowClass<Registered>, windows::core::Error> {
    let instance = HINSTANCE::from(match self.instance {
      None => Instance::get_exe()?, // this pesky result means we can't do unwrap_or_else
      Some(instance) => instance,
    });

    let wc = WNDCLASSEXW {
      cbSize: core::mem::size_of::<WNDCLASSEXW>() as _,
      hInstance: instance,
      lpszClassName: PCWSTR(self.name.as_ptr()),
      lpfnWndProc: Some(procedure::window_procedure),
      ..Default::default()
    };

    unsafe { RegisterClassExW(&wc) };

    Ok(WindowClass {
      instance: self.instance,
      name: self.name,
      style: self.style,
      _registration: PhantomData,
    })
  }
}

impl WindowClass<Registered> {
  pub fn create_window<Procedure: 'static + WindowProcedure>(
    &self,
    title: impl Into<HSTRING>,
    position: Option<Position>,
    size: Option<Size>,
    style: WindowStyle,
    ext_style: ExtendedWindowStyle,
    procedure: Procedure,
  ) -> Result<Window, windows::core::Error> {
    let title: HSTRING = title.into();
    let mut create_info = CreateInfo {
      user_state: Some(Box::new(procedure)),
    };
    let position = position.unwrap_or(Position::AUTO);
    let size = size.unwrap_or(Size::AUTO);
    unsafe {
      CreateWindowExW(
        ext_style.into(),
        &self.name,
        &title,
        style.into(),
        position.x,
        position.y,
        size.width,
        size.height,
        None,
        None,
        HINSTANCE::from(self.instance.unwrap()),
        Some(std::ptr::addr_of_mut!(create_info).cast()),
      )
    }
    .map(Into::into)
  }

  pub fn unregister(self) -> Result<WindowClass<Unregistered>, windows::core::Error> {
    unsafe {
      UnregisterClassW(
        PCWSTR(self.name.as_ptr()),
        HINSTANCE::from(self.instance.unwrap()),
      )
    }?;

    Ok(WindowClass {
      instance: self.instance,
      name: self.name,
      style: self.style,
      _registration: PhantomData,
    })
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

// impl<W: WindowProcedure, Registration> From<WindowClass<WithProcedure<W>, Registration>> for WNDCLASSEXW {
//   fn from(value: WindowClass<WithProcedure<W>, Registration>) -> Self {
//     Self {
//       cbSize: std::mem::size_of::<Self>() as u32,
//       cbWndExtra: std::mem::size_of::<Self>() as i32,
//       lpszClassName: PCWSTR::from(value.name),
//       lpfnWndProc: Some(procedure::window_procedure),
//       ..Default::default()
//     }
//   }
// }

// impl<W: WindowProcedure> From<WindowClassBuilder<HasWindowClassName>> for WindowClass<W> {
//   fn from(value: WindowClassBuilder<HasWindowClassName>) -> Self {
//     value.build()
//   }
// }

// pub struct HasWindowClassName(String);
// pub struct NoWindowClassName;

// pub struct WindowClassBuilder<Name> {
//   name: Name,
// }

// impl Default for WindowClassBuilder<NoWindowClassName> {
//   fn default() -> Self {
//     WindowClassBuilder {
//       name: NoWindowClassName,
//     }
//   }
// }

// impl WindowClassBuilder<NoWindowClassName> {
//   pub fn with_name(
//     self,
//     name: impl Into<String>,
//   ) -> WindowClassBuilder<HasWindowClassName> {
//     WindowClassBuilder {
//       name: HasWindowClassName(name.into()),
//     }
//   }
// }

// impl WindowClassBuilder<HasWindowClassName> {
//   pub fn build<W: WindowProcedure>(self) -> WindowClass<W> {
//     WindowClass { name: self.name.0 }
//   }
// }
