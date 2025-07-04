use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use dpi::{PhysicalPosition, PhysicalSize, PixelUnit, Position, Size};
use widestring::WideCString;
use windows_result::{HRESULT, Result};
use windows_sys::Win32::UI::WindowsAndMessaging::{
  self, CREATESTRUCTW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow, IsWindow, PostQuitMessage,
  SHOW_WINDOW_CMD, SetWindowTextW, ShowWindow,
};

use crate::{Handle, declare_handle, get_last_error, reset_last_error};

use super::{
  Instance, LParam, LResult, Message, WParam, WindowClass, WindowPtrIndex, WindowStyle,
  procedure::{WindowProcedure, WindowState},
  styles::ExtendedWindowStyle,
};

declare_handle!(
  Window,
  alias = "HWND",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd"
);

pub struct LpParam {
  pub create_struct: CreateStruct,
  pub wnd_proc: Option<Box<dyn WindowProcedure>>,
}

impl LpParam {
  pub fn from_raw(l: LParam) -> *mut Self {
    unsafe { (l.0 as *mut CREATESTRUCTW).as_mut() }
      .map(|cs| cs.lpCreateParams.cast())
      .unwrap()
  }
}

pub struct CreateStruct {
  pub class: WindowClass,
  pub name: String,
  pub style: WindowStyle,
  pub ex_style: ExtendedWindowStyle,
  pub position: (Option<PixelUnit>, Option<PixelUnit>),
  pub size: (Option<PixelUnit>, Option<PixelUnit>),
  pub parent: Option<Window>,
  pub menu: Option<*mut ()>,
  pub instance: Option<Instance>,
}

// impl Drop for CreateStruct {
//   fn drop(&mut self) {
//     println!("It dropped!")
//   }
// }

#[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw"]
pub fn create_window(create_struct: CreateStruct, wnd_proc: Box<dyn WindowProcedure>) -> Result<Window> {
  // remove visible style and reapply it later in the window procedure
  // let mut new_style = desc.style;
  // new_style.remove(WindowStyle::Visible);

  let position = {
    (
      create_struct
        .position
        .0
        .map(|p| p.to_physical(1.0).0)
        .unwrap_or(CW_USEDEFAULT),
      create_struct
        .position
        .1
        .map(|p| p.to_physical(1.0).0)
        .unwrap_or(CW_USEDEFAULT),
    )
  };
  let size = {
    (
      create_struct
        .size
        .0
        .map(|p| p.to_physical(1.0).0)
        .unwrap_or(CW_USEDEFAULT),
      create_struct
        .size
        .1
        .map(|p| p.to_physical(1.0).0)
        .unwrap_or(CW_USEDEFAULT),
    )
  };

  let name = WideCString::from_str_truncate(create_struct.name.clone());
  let hwnd = unsafe {
    CreateWindowExW(
      create_struct.ex_style.to_raw(),
      create_struct.class.atom(),
      name.as_ptr(),
      create_struct.style.to_raw(),
      position.0,
      position.1,
      size.0,
      size.1,
      match create_struct.parent {
        Some(p) => p.to_raw() as _,
        None => Window::null().to_raw() as _,
      },
      match create_struct.menu {
        Some(m) => m as _,
        None => std::ptr::null_mut() as _,
      },
      match create_struct.instance {
        Some(i) => i.to_raw() as _,
        None => Instance::null().to_raw() as _,
      },
      Box::into_raw(Box::new(LpParam {
        create_struct,
        wnd_proc: Some(wnd_proc),
      }))
      .cast(),
    )
  };

  match hwnd.is_null() {
    true => Err(get_last_error()),
    false => Ok(unsafe { Window::from_raw(hwnd as usize) }),
  }
}

impl Window {
  /// Thin wrapper around [`create_window`] function
  #[inline]
  #[allow(clippy::too_many_arguments)]
  pub fn new(create_struct: CreateStruct, wnd_proc: Box<dyn WindowProcedure>) -> Result<Self> {
    create_window(create_struct, wnd_proc)
  }

  pub fn builder() -> WindowBuilder<NoClass, NoProc> {
    WindowBuilder::default()
  }
}

pub struct NoClass;
pub struct Class(WindowClass);

pub struct NoProc;
pub struct Proc(Box<dyn WindowProcedure>);

pub struct WindowBuilder<WndClass, WndProc> {
  class: WndClass,
  wnd_proc: WndProc,
  name: String,
  style: WindowStyle,
  ex_style: ExtendedWindowStyle,
  position: (Option<PixelUnit>, Option<PixelUnit>),
  size: (Option<PixelUnit>, Option<PixelUnit>),
  parent: Option<Window>,
  menu: Option<*mut ()>,
  instance: Option<Instance>,
}

impl Default for WindowBuilder<NoClass, NoProc> {
  fn default() -> Self {
    WindowBuilder {
      class: NoClass,
      wnd_proc: NoProc,
      name: "Window".to_string(),
      style: WindowStyle::OverlappedWindow,
      ex_style: ExtendedWindowStyle::default(),
      position: (None, None),
      size: (None, None),
      parent: None,
      menu: None,
      instance: Some(Instance::get()),
    }
  }
}

impl<WndProc> WindowBuilder<NoClass, WndProc> {
  pub fn class(self, class: WindowClass) -> WindowBuilder<Class, WndProc> {
    WindowBuilder {
      class: Class(class),
      wnd_proc: self.wnd_proc,
      name: self.name,
      style: self.style,
      ex_style: self.ex_style,
      position: self.position,
      size: self.size,
      parent: self.parent,
      menu: self.menu,
      instance: self.instance,
    }
  }
}

impl<WndClass> WindowBuilder<WndClass, NoProc> {
  pub fn procedure(self, wndproc: impl 'static + WindowProcedure) -> WindowBuilder<WndClass, Proc> {
    WindowBuilder {
      class: self.class,
      wnd_proc: Proc(Box::new(wndproc)),
      name: self.name,
      style: self.style,
      ex_style: self.ex_style,
      position: self.position,
      size: self.size,
      parent: self.parent,
      menu: self.menu,
      instance: self.instance,
    }
  }
}

impl<WndClass, WndProc> WindowBuilder<WndClass, WndProc> {
  pub fn name(mut self, name: impl Into<String>) -> WindowBuilder<WndClass, WndProc> {
    self.name = name.into();
    self
  }

  pub fn style(mut self, style: WindowStyle) -> WindowBuilder<WndClass, WndProc> {
    self.style = style;
    self
  }

  pub fn ex_style(mut self, ex_style: ExtendedWindowStyle) -> WindowBuilder<WndClass, WndProc> {
    self.ex_style = ex_style;
    self
  }

  pub fn x(mut self, x: Option<PixelUnit>) -> WindowBuilder<WndClass, WndProc> {
    self.position.0 = x;
    self
  }

  pub fn y(mut self, y: Option<PixelUnit>) -> WindowBuilder<WndClass, WndProc> {
    self.position.1 = y;
    self
  }

  pub fn position(mut self, position: impl Into<Position>) -> WindowBuilder<WndClass, WndProc> {
    let pos: PhysicalPosition<i32> = position.into().to_physical(1.0);
    self.position = (Some(PixelUnit::Physical(pos.x.into())), Some(PixelUnit::Physical(pos.y.into())));
    self
  }

  // TODO: Refactor these out. Split into separate x,y,width,height to simplify things. Make each Option<PixelUnit>.
  pub fn width(mut self, width: Option<PixelUnit>) -> WindowBuilder<WndClass, WndProc> {
    self.size.0 = width;
    self
  }

  pub fn height(mut self, height: Option<PixelUnit>) -> WindowBuilder<WndClass, WndProc> {
    self.size.1 = height;
    self
  }

  pub fn size(mut self, size: impl Into<Size>) -> WindowBuilder<WndClass, WndProc> {
    let size: PhysicalSize<i32> = size.into().to_physical(1.0);
    self.size = (Some(PixelUnit::Physical(size.width.into())), Some(PixelUnit::Physical(size.height.into())));
    self
  }

  pub fn parent(mut self, parent: Option<Window>) -> WindowBuilder<WndClass, WndProc> {
    self.parent = parent;
    self
  }

  pub fn menu(mut self, menu: Option<*mut ()>) -> WindowBuilder<WndClass, WndProc> {
    self.menu = menu;
    self
  }

  pub fn instance(mut self, instance: Option<Instance>) -> WindowBuilder<WndClass, WndProc> {
    self.instance = instance;
    self
  }
}

impl WindowBuilder<Class, Proc> {
  pub fn create(self) -> Result<Window> {
    Window::new(
      CreateStruct {
        class: self.class.0,
        name: self.name,
        style: self.style,
        ex_style: self.ex_style,
        position: self.position,
        size: self.size,
        parent: self.parent,
        menu: self.menu,
        instance: self.instance,
      },
      self.wnd_proc.0,
    )
  }
}

impl Window {
  /// Returns whether or not the handle identifies an existing window.
  /// # Safety
  /// A thread should not use [`WindowId::is_window`] for a window that it did not create because the window could be destroyed after this function was called. Further, because window handles are recycled the handle could even point to a different window.
  ///
  #[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow"]
  pub unsafe fn is_window(&self) -> bool {
    // check for null here is probably redundant, but allows for a short-circuit which may or may not be faster.
    !self.is_null() && unsafe { IsWindow(self.to_raw() as _) != 0 }
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CmdShow {
  Hide,
  Show,
  #[default]
  ShowDefault,
  Restore,
  Normal,
  Maximize,
  Minimize,
  ForceMinimize,
  ShowMinimized,
  ShowMinNoActive,
  ShowNoActivate,
  ShowNA,
}

impl CmdShow {
  pub const fn to_raw(&self) -> SHOW_WINDOW_CMD {
    match self {
      CmdShow::Hide => WindowsAndMessaging::SW_HIDE,
      CmdShow::Normal => WindowsAndMessaging::SW_NORMAL,
      CmdShow::ShowMinimized => WindowsAndMessaging::SW_SHOWMINIMIZED,
      CmdShow::Maximize => WindowsAndMessaging::SW_SHOWMAXIMIZED,
      CmdShow::ShowNoActivate => WindowsAndMessaging::SW_SHOWNOACTIVATE,
      CmdShow::Show => WindowsAndMessaging::SW_SHOW,
      CmdShow::Minimize => WindowsAndMessaging::SW_MINIMIZE,
      CmdShow::ShowMinNoActive => WindowsAndMessaging::SW_SHOWMINNOACTIVE,
      CmdShow::ShowNA => WindowsAndMessaging::SW_SHOWNA,
      CmdShow::Restore => WindowsAndMessaging::SW_RESTORE,
      CmdShow::ShowDefault => WindowsAndMessaging::SW_SHOWDEFAULT,
      CmdShow::ForceMinimize => WindowsAndMessaging::SW_FORCEMINIMIZE,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShowWindowResult {
  WasVisible,
  WasHidden,
}

impl Window {
  #[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow"]
  pub fn show_window(&self, cmd_show: CmdShow) -> ShowWindowResult {
    match unsafe { ShowWindow(self.to_raw() as _, cmd_show.to_raw()) } {
      0 => ShowWindowResult::WasHidden,
      _ => ShowWindowResult::WasVisible,
    }
  }
}

impl Window {
  pub fn send_message(&self) {
    // TODO: somehow ensure these are always sent to the correct thread, even when called from a different thread.
    // maybe do it by storing the thread id?
    // Reference winit for this!
    todo!()
  }

  pub fn default_procedure_raw(&self, message: u32, w: WParam, l: LParam) -> isize {
    unsafe { DefWindowProcW(self.to_ptr(), message, w.0, l.0) }
  }

  pub fn default_procedure(&self, message: &Message) -> LResult {
    LResult(self.default_procedure_raw(message.id().to_raw(), message.w(), message.l()))
  }

  pub fn destroy(&self) -> Result<()> {
    if unsafe { self.is_window() } {
      if let Some(state) = self.state() {
        state.set_destroying();
        reset_last_error();
        return match unsafe { DestroyWindow(self.to_ptr()) } {
          0 => Ok(()),
          _ => Err(get_last_error()),
        };
      }
    }
    Ok(())
  }

  pub fn quit(&self) {
    self.quit_with_code(0)
  }

  pub fn quit_with_code(&self, exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
  }

  pub fn set_window_text(&self, text: impl Into<String>) -> Result<()> {
    let text = OsString::from(text.into()).encode_wide().collect::<Vec<u16>>();
    reset_last_error();
    match unsafe { SetWindowTextW(self.to_ptr(), text.as_ptr()) } {
      0 => Ok(()),
      _ => Err(get_last_error()),
    }
  }

  pub(crate) fn get_window_ptr(&self, index: WindowPtrIndex) -> isize {
    unsafe {
      use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW;
      GetWindowLongPtrW(self.to_ptr(), index.to_raw()) as _
    }
  }

  pub(crate) fn set_window_ptr(&self, index: WindowPtrIndex, value: isize) -> Result<isize> {
    reset_last_error();

    let result = unsafe {
      use windows_sys::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW;
      SetWindowLongPtrW(self.to_ptr(), index.to_raw(), value) as _
    };

    let error = get_last_error();
    match result == 0 && error.code() != HRESULT(0) {
      true => Err(error),
      false => Ok(result),
    }
  }

  #[allow(clippy::mut_from_ref)] // This is fine because self is just a handle.
  #[inline]
  pub(crate) fn state(&self) -> Option<&mut WindowState> {
    unsafe { (self.get_window_ptr(WindowPtrIndex::UserData) as *mut WindowState).as_mut() }
  }
}
