use std::{
  ffi::{OsStr, OsString},
  os::windows::ffi::{OsStrExt, OsStringExt},
};

use dpi::{PhysicalPosition, PhysicalSize, PixelUnit, Position, Size};
use windows_result::{Error, Result};
use windows_sys::Win32::{Foundation::{LPARAM, WPARAM}, UI::WindowsAndMessaging::{
  self, CreateWindowExW, DefWindowProcW, DestroyWindow, GetWindowLongPtrW, GetWindowTextLengthW, GetWindowTextW, IsWindow, PostQuitMessage, SetWindowLongPtrW, SetWindowTextW, ShowWindow, CW_USEDEFAULT, SHOW_WINDOW_CMD
}};

use crate::{Handle, declare_handle, get_last_error, reset_last_error};

use super::{
  procedure::{WindowProcedure, WindowState}, styles::ExtendedWindowStyle, Instance, LResult, Message, WindowClass, WindowPtrIndex, WindowStyle
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

impl CreateStruct {
  #[inline]
  pub fn x(&self) -> i32 {
    self.position.0.map(|p| p.to_physical(1.0).0).unwrap_or(CW_USEDEFAULT)
  }

  #[inline]
  pub fn y(&self) -> i32 {
    self.position.1.map(|p| p.to_physical(1.0).0).unwrap_or(CW_USEDEFAULT)
  }

  #[inline]
  pub fn width(&self) -> i32 {
    self.size.0.map(|p| p.to_physical(1.0).0).unwrap_or(CW_USEDEFAULT)
  }

  #[inline]
  pub fn height(&self) -> i32 {
    self.size.1.map(|p| p.to_physical(1.0).0).unwrap_or(CW_USEDEFAULT)
  }
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

  let lp_param_ptr = Box::into_raw(Box::new(LpParam {
    create_struct,
    wnd_proc: Some(wnd_proc),
  }));
  let lp_param = unsafe { lp_param_ptr.as_ref() }.unwrap();
  let name: Vec<u16> = OsStr::new(&lp_param.create_struct.name)
    .encode_wide()
    .chain(std::iter::once(0))
    .collect();

  let hwnd = unsafe {
    CreateWindowExW(
      lp_param.create_struct.ex_style.to_raw(),
      lp_param.create_struct.class.atom(),
      name.as_ptr(),
      lp_param.create_struct.style.to_raw(),
      lp_param.create_struct.x(),
      lp_param.create_struct.y(),
      lp_param.create_struct.width(),
      lp_param.create_struct.height(),
      lp_param.create_struct.parent.unwrap_or_default().to_raw() as _,
      lp_param.create_struct.menu.unwrap_or_else(std::ptr::null_mut) as _,
      lp_param.create_struct.instance.unwrap_or_default().to_raw() as _,
      lp_param_ptr.cast(),
    )
  };

  match hwnd.is_null() {
    true => Err(get_last_error().unwrap_or(Error::empty())),
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
  pub(crate) fn send_message(&self) {
    // TODO: somehow ensure these are always sent to the correct thread, even when called from a different thread.
    // maybe do it by storing the thread id?
    // Reference winit for this!
    todo!()
  }

  pub(crate) fn def_window_proc_raw(&self, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LResult {
    LResult(unsafe { DefWindowProcW(self.to_ptr(), msg, w_param, l_param) })
  }

  pub fn def_window_proc(&self, message: &Message) -> Option<LResult> {
    Some(self.def_window_proc_raw(message.id().to_raw(), message.w().0, message.l().0))
  }

  pub fn destroy(&self) -> Result<()> {
    if unsafe { self.is_window() } {
      if let Some(state) = self.state() {
        state.set_destroying();
        reset_last_error();
        return match unsafe { DestroyWindow(self.to_ptr()) } {
          0 => Ok(()),
          _ => Err(get_last_error().unwrap_or(Error::empty())),
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
    let text: Vec<u16> = OsStr::new(&text.into())
      .encode_wide()
      .chain(std::iter::once(0))
      .collect();
    reset_last_error();
    match unsafe { SetWindowTextW(self.to_ptr(), text.as_ptr()) } {
      0 => Err(get_last_error().unwrap_or(Error::empty())),
      _ => Ok(()),
    }
  }

  pub fn get_window_text(&self) -> Result<String> {
    reset_last_error();
    let text_len = unsafe { GetWindowTextLengthW(self.to_ptr()) };
    let mut buffer = Vec::with_capacity(text_len as usize + 1);
    if unsafe { GetWindowTextW(self.to_ptr(), buffer.as_mut_ptr(), text_len) } == 0 {
      if let Some(error) = get_last_error() {
        return Err(error);
      }
    };

    Ok(OsString::from_wide(&buffer).to_string_lossy().into())
  }

  pub(crate) fn get_window_ptr(&self, index: WindowPtrIndex) -> isize {
    unsafe { GetWindowLongPtrW(self.to_ptr(), index.to_raw()) as _ }
  }

  pub(crate) fn set_window_ptr(&self, index: WindowPtrIndex, value: isize) -> Result<isize> {
    reset_last_error();

    let result = unsafe { SetWindowLongPtrW(self.to_ptr(), index.to_raw(), value) as _ };

    match (result == 0, get_last_error()) {
      (true, Some(error)) => Err(error),
      _ => Ok(result),
    }
  }

  #[allow(clippy::mut_from_ref)] // This is fine because self is just a handle.
  #[inline]
  pub(crate) fn state(&self) -> Option<&mut WindowState> {
    unsafe { (self.get_window_ptr(WindowPtrIndex::UserData) as *mut WindowState).as_mut() }
  }
}
