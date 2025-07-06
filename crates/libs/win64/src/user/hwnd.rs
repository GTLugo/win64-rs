use std::{
  ffi::{OsStr, OsString},
  os::windows::ffi::{OsStrExt, OsStringExt},
};

use dpi::{PhysicalPosition, PhysicalSize, PixelUnit, Position, Size};
use windows_result::{Error, Result};
use windows_sys::Win32::{
  Foundation::{LPARAM, WPARAM},
  Graphics::Dwm::DwmSetWindowAttribute,
  System::Threading::GetCurrentThreadId,
  UI::WindowsAndMessaging::{
    self, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow, GetWindowLongPtrW, GetWindowTextLengthW,
    GetWindowTextW, GetWindowThreadProcessId, IsWindow, PostQuitMessage, SHOW_WINDOW_CMD, SetWindowLongPtrW,
    SetWindowTextW, ShowWindow,
  },
};

use crate::{Handle, declare_handle, get_last_error, reset_last_error};

use super::{
  Instance, LResult, Message, WindowClass, WindowPtrIndex, WindowStyle,
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
  /// Returns whether or not the handle identifies an existing window. Will also return false if the window is not owned by the calling thread.
  ///
  #[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow"]
  pub fn is_window(&self) -> bool {
    self.is_current_thread() && unsafe { IsWindow(self.to_raw() as _) != 0 }
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
  pub const fn from_raw(raw: SHOW_WINDOW_CMD) -> Self {
    match raw {
      WindowsAndMessaging::SW_HIDE => CmdShow::Hide,
      WindowsAndMessaging::SW_NORMAL => CmdShow::Normal,
      WindowsAndMessaging::SW_SHOWMINIMIZED => CmdShow::ShowMinimized,
      WindowsAndMessaging::SW_SHOWMAXIMIZED => CmdShow::Maximize,
      WindowsAndMessaging::SW_SHOWNOACTIVATE => CmdShow::ShowNoActivate,
      WindowsAndMessaging::SW_SHOW => CmdShow::Show,
      WindowsAndMessaging::SW_MINIMIZE => CmdShow::Minimize,
      WindowsAndMessaging::SW_SHOWMINNOACTIVE => CmdShow::ShowMinNoActive,
      WindowsAndMessaging::SW_SHOWNA => CmdShow::ShowNA,
      WindowsAndMessaging::SW_RESTORE => CmdShow::Restore,
      WindowsAndMessaging::SW_FORCEMINIMIZE => CmdShow::ForceMinimize,
      _ => CmdShow::ShowDefault,
    }
  }

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

  pub fn use_immersive_dark_mode(&self, enable: bool) {
    // https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/ui/apply-windows-themes
    const DWMWA_USE_IMMERSIVE_DARK_MODE: u32 = 20;

    let value = enable as windows_sys::core::BOOL;
    unsafe {
      DwmSetWindowAttribute(
        self.to_ptr(),
        DWMWA_USE_IMMERSIVE_DARK_MODE,
        (&raw const value).cast(),
        size_of_val(&value) as _,
      )
    };
  }

  pub fn destroy(&self) -> Result<()> {
    if self.is_window() {
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

  pub fn get_thread_id(&self) -> Option<u32> {
    let id = unsafe { GetWindowThreadProcessId(self.to_ptr(), std::ptr::null_mut()) };
    match id {
      0 => None,
      _ => Some(id),
    }
  }

  pub fn is_current_thread(&self) -> bool {
    let current_thread = unsafe { GetCurrentThreadId() };
    let window_thread = self.get_thread_id();
    window_thread.is_some_and(|id| id == current_thread)
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

    let text_len = unsafe { GetWindowTextLengthW(self.to_ptr()) } as usize;
    if text_len == 0 {
      if let Some(error) = get_last_error() {
        return Err(error);
      }
    };

    let mut buffer: Vec<u16> = vec![0; text_len + 1];

    let result = unsafe { GetWindowTextW(self.to_ptr(), buffer.as_mut_ptr(), buffer.len() as i32) };
    if result == 0 {
      if let Some(error) = get_last_error() {
        return Err(error);
      }
    };

    Ok(OsString::from_wide(&buffer).to_string_lossy().into_owned())
  }

  pub(crate) fn get_window_ptr(&self, index: WindowPtrIndex) -> isize {
    unsafe { GetWindowLongPtrW(self.to_ptr(), index.to_raw()) as _ }
  }

  pub fn instance(&self) -> Instance {
    unsafe { Instance::from_raw(self.get_window_ptr(WindowPtrIndex::Instance) as _) }
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
