pub mod paint;
pub use paint::*;

use std::{
  ffi::{OsStr, OsString},
  os::windows::ffi::{OsStrExt, OsStringExt},
};

use dpi::{PhysicalPosition, PhysicalSize, PixelUnit, Position, Size};
use libloading::Symbol;
use windows_result::{Error, Result};
use windows_sys::Win32::{
  Foundation::{HWND, LPARAM, WPARAM},
  Graphics::{
    Dwm::{
      DWMSBT_AUTO, DWMSBT_MAINWINDOW, DWMSBT_NONE, DWMSBT_TABBEDWINDOW, DWMSBT_TRANSIENTWINDOW, DWMWA_BORDER_COLOR,
      DWMWA_CAPTION_COLOR, DWMWA_SYSTEMBACKDROP_TYPE, DWMWA_USE_IMMERSIVE_DARK_MODE, DWMWINDOWATTRIBUTE,
      DwmExtendFrameIntoClientArea, DwmSetWindowAttribute,
    },
    Gdi::UpdateWindow,
  },
  System::Threading::GetCurrentThreadId,
  UI::{
    Controls::MARGINS,
    WindowsAndMessaging::{
      self, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow, GetSystemMetrics, GetWindowLongPtrW,
      GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsWindow, PostMessageW, PostQuitMessage,
      SHOW_WINDOW_CMD, SM_CYCAPTION, SendMessageW, SetWindowLongPtrW, SetWindowTextW, ShowWindow,
    },
  },
};

use crate::{Handle, declare_handle, get_last_error, reset_last_error, win10_build_version};

use super::{
  Instance, LResult, Message, UserData, WindowClass, WindowPtrIndex, WindowStyle,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DwmWindowAttribute {
  UseImmersiveDarkMode(bool),
  SystemBackdropType(SystemBackdropType),
  CaptionColor(u32),
  BorderColor(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemBackdropType {
  Auto,
  None,
  MainWindow,
  TransientWindow,
  TabbedWindow,
}

impl SystemBackdropType {
  pub const fn to_raw(&self) -> i32 {
    match self {
      Self::Auto => DWMSBT_AUTO,
      Self::None => DWMSBT_NONE,
      Self::MainWindow => DWMSBT_MAINWINDOW,
      Self::TransientWindow => DWMSBT_TRANSIENTWINDOW,
      Self::TabbedWindow => DWMSBT_TABBEDWINDOW,
    }
  }
}

impl Window {
  // WIP
  pub fn dwm_set_window_attribute(&self, attribute: DwmWindowAttribute) {
    match attribute {
      DwmWindowAttribute::UseImmersiveDarkMode(enable) => self.use_immersive_dark_mode(enable),
      DwmWindowAttribute::SystemBackdropType(backdrop_type) => self.system_backdrop_type(backdrop_type.to_raw()),
      DwmWindowAttribute::CaptionColor(color) => self.caption_color(color),
      DwmWindowAttribute::BorderColor(color) => self.border_color(color),
    }
  }

  fn border_color(&self, color: u32) {
    unsafe {
      DwmSetWindowAttribute(
        self.to_ptr(),
        DWMWA_CAPTION_COLOR as u32,
        &raw const color as *const std::ffi::c_void,
        std::mem::size_of::<DWMWINDOWATTRIBUTE>() as u32,
      )
    };
  }

  fn caption_color(&self, color: u32) {
    unsafe {
      DwmSetWindowAttribute(
        self.to_ptr(),
        DWMWA_BORDER_COLOR as u32,
        &raw const color as *const std::ffi::c_void,
        std::mem::size_of::<DWMWINDOWATTRIBUTE>() as u32,
      )
    };
  }

  fn system_backdrop_type(&self, backdrop_type: i32) {
    unsafe {
      DwmSetWindowAttribute(
        self.to_ptr(),
        DWMWA_SYSTEMBACKDROP_TYPE as u32,
        &raw const backdrop_type as *const std::ffi::c_void,
        std::mem::size_of::<DWMWINDOWATTRIBUTE>() as u32,
      )
    };
  }

  pub fn extend_into_client_all(&self) {
    let margins = MARGINS {
      cxLeftWidth: -1,
      cxRightWidth: -1,
      cyTopHeight: -1,
      cyBottomHeight: -1,
    };

    unsafe {
      DwmExtendFrameIntoClientArea(self.to_ptr(), &margins);
    }
  }

  // Work in progress. Need to fix titlebar colors.
  pub fn set_acrylic_background(&self, color: u32) {
    // Reference: https://gist.github.com/xv/43bd4c944202a593ac8ec6daa299b471
    // Keeping this internal for now while I figure out this API
    // #[allow(unused)]
    // #[repr(u32)]
    // enum WindowCompositionAttribute {
    //   Undefined = 0,
    //   NcRenderingEnabled = 1,
    //   NcRenderingPolicy = 2,
    //   TransitionsForceDisabled = 3,
    //   AllowNcPaint = 4,
    //   CaptionButtonBounds = 5,
    //   NonclientRtlLayout = 6,
    //   ForceIconicRepresentation = 7,
    //   ExtendedFrameBounds = 8,
    //   HasIconicBitmap = 9,
    //   ThemeAttributes = 10,
    //   NcRenderingExiled = 11,
    //   NcAdornmentInfo = 12,
    //   ExcludedFromLivePreview = 13,
    //   VideoOverlayActive = 14,
    //   ForceActiveWindowAppearance = 15,
    //   DisallowPeek = 16,
    //   Cloak = 17,
    //   Cloaked = 18,
    //   AccentPolicy = 19,
    //   FreezeRepresentation = 20,
    //   EverUncloaked = 21,
    //   VisualOwner = 22,
    //   Holographic = 23,
    //   ExcludedFromDda = 24,
    //   PassiveUpdateMode = 25,
    //   Last = 26,
    // }

    // #[allow(unused)]
    // #[repr(u32)]
    // enum AccentState {
    //   Disabled = 0,
    //   EnableGradient = 1,
    //   EnableTransparentGradient = 2,
    //   EnableBlurBehind = 3,
    //   EnableAcrylicBlurBehind = 4, // RS4 1803.17063
    //   EnableHostBackdrop = 5,      // RS5 1809
    //   InvalidState = 6,
    // }

    // #[repr(C)]
    // struct AccentPolicy {
    //   accent_state: AccentState,
    //   accent_flags: u32,
    //   gradient_color: u32,
    //   animation_id: u32,
    // }

    // #[repr(C)]
    // struct WindowCompositionAttributeData {
    //   attribute: WindowCompositionAttribute,
    //   data: *mut std::ffi::c_void,
    //   size: usize,
    // }

    // let user32 = unsafe { libloading::Library::new("user32.dll\0") }.unwrap();

    // let set_window_composition_attribute: Symbol<
    //   unsafe extern "system" fn(hwnd: HWND, data: *mut WindowCompositionAttributeData) -> i32,
    // > = unsafe { user32.get(b"SetWindowCompositionAttribute") }.unwrap();

    // let policy = AccentPolicy {
    //   accent_state: AccentState::EnableAcrylicBlurBehind,
    //   accent_flags: 2,
    //   gradient_color: color,
    //   animation_id: 0,
    // };

    // let mut data = WindowCompositionAttributeData {
    //   attribute: WindowCompositionAttribute::AccentPolicy,
    //   data: &raw const policy as *mut std::ffi::c_void,
    //   size: std::mem::size_of::<AccentPolicy>(),
    // };

    // unsafe {
    //   set_window_composition_attribute(self.to_ptr(), &mut data);
    // }

    // let titlebar_height = unsafe { GetSystemMetrics(SM_CYCAPTION) };
    // let margins = MARGINS {
    //   cxLeftWidth: 0,
    //   cxRightWidth: 0,
    //   cyTopHeight: titlebar_height,
    //   cyBottomHeight: 0,
    // };
  }

  fn use_immersive_dark_mode(&self, enable: bool) {
    // https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/ui/apply-windows-themes
    let value = enable as windows_sys::core::BOOL;
    let Some(version) = win10_build_version() else {
      return;
    };
    // May 2020 Update https://stackoverflow.com/a/70693198/17004103
    let dw_attribute: u32 = if version < 19041 {
      19
    } else {
      DWMWA_USE_IMMERSIVE_DARK_MODE as _
    };
    unsafe {
      DwmSetWindowAttribute(
        self.to_ptr(),
        dw_attribute,
        (&raw const value).cast(),
        std::mem::size_of::<DWMWINDOWATTRIBUTE>() as u32,
      )
    };
  }

  pub fn update(&self) -> Result<()> {
    reset_last_error();
    match unsafe { UpdateWindow(self.to_ptr()) } {
      0 => Err(get_last_error().unwrap_or(Error::empty())),
      _ => Ok(()),
    }
  }

  pub fn send_message(&self, message: Message) -> LResult {
    // TODO: somehow ensure these are always sent to the correct thread, even when called from a different thread.
    // maybe do it by storing the thread id?
    // Reference winit for this!
    let lresult = unsafe { SendMessageW(self.to_ptr(), message.id().to_raw(), message.w().0, message.l().0) };
    lresult.into()
  }

  pub fn post_message(&self, message: Message) -> Result<()> {
    // TODO: somehow ensure these are always sent to the correct thread, even when called from a different thread.
    // maybe do it by storing the thread id?
    // Reference winit for this!
    reset_last_error();
    let result = unsafe { PostMessageW(self.to_ptr(), message.id().to_raw(), message.w().0, message.l().0) };
    match result != 0 {
      true => Ok(()),
      false => Err(get_last_error().unwrap_or(Error::empty())),
    }
  }

  pub(crate) fn def_window_proc_raw(&self, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LResult {
    LResult(unsafe { DefWindowProcW(self.to_ptr(), msg, w_param, l_param) })
  }

  pub fn def_window_proc(&self, message: &Message) -> Option<LResult> {
    Some(self.def_window_proc_raw(message.id().to_raw(), message.w().0, message.l().0))
  }

  pub fn destroy(&self) -> Result<()> {
    if self.is_window()
      && let Some(data) = self.user_data()
      && let WindowState::Running = data.state
    {
      data.state = WindowState::Destroying;
      reset_last_error();
      return match unsafe { DestroyWindow(self.to_ptr()) } {
        0 => Ok(()),
        _ => Err(get_last_error().unwrap_or(Error::empty())),
      };
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
  pub(crate) fn user_data(&self) -> Option<&mut UserData> {
    unsafe { (self.get_window_ptr(WindowPtrIndex::UserData) as *mut UserData).as_mut() }
  }
}
