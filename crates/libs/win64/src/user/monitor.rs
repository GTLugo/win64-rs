use {
  super::{
    Window,
    dpi_to_scale_factor,
  },
  crate::{
    Handle,
    ORIGIN_POINT,
    declare_handle,
  },
  dpi::PhysicalPosition,
  std::collections::VecDeque,
  windows_sys::{
    Win32::{
      Foundation::{
        LPARAM,
        POINT,
        RECT,
      },
      Graphics::Gdi::{
        EnumDisplayMonitors,
        HDC,
        HMONITOR,
        MONITOR_DEFAULTTONEAREST,
        MONITOR_DEFAULTTONULL,
        MONITOR_DEFAULTTOPRIMARY,
        MONITOR_FROM_FLAGS,
        MonitorFromPoint,
        MonitorFromWindow,
      },
      UI::{
        HiDpi::{
          GetDpiForMonitor,
          MDT_EFFECTIVE_DPI,
        },
        WindowsAndMessaging,
      },
    },
    core::BOOL,
  },
};

// Adapted from Winit
// unsafe extern "system" fn(param0: HMONITOR, param1: HDC, param2: *mut super::super::Foundation::RECT, param3: super::super::Foundation::LPARAM) -> windows_sys::core::BOOL
unsafe extern "system" fn enum_display_monitors(
  hmonitor: HMONITOR,
  _hdc: HDC,
  _place: *mut RECT,
  data: LPARAM,
) -> BOOL {
  if let Some(monitors) = unsafe { (data as *mut VecDeque<Monitor>).as_mut() } {
    monitors.push_back(unsafe { Monitor::from_ptr(hmonitor) })
  };
  true.into() // continue enumeration
}

declare_handle!(
  Monitor,
  alias = "HMONITOR",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmonitor"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MonitorDefault {
  Null,
  Primary,
  Nearest,
}

impl MonitorDefault {
  const fn to_raw(self) -> MONITOR_FROM_FLAGS {
    match self {
      MonitorDefault::Null => MONITOR_DEFAULTTONULL,
      MonitorDefault::Primary => MONITOR_DEFAULTTOPRIMARY,
      MonitorDefault::Nearest => MONITOR_DEFAULTTONEAREST,
    }
  }
}

impl Monitor {
  pub const BASE_DPI: u32 = WindowsAndMessaging::USER_DEFAULT_SCREEN_DPI;

  /// # Returns
  /// If the point is contained by a display monitor, the return value is an `Monitor` handle to that display monitor.
  /// If the point is not contained by a display monitor, the return value depends on the value of `default`.
  /// If `default` is `MonitorDefault::Null`, then the return value would be `None`, otherwise it is always `Some(Monitor)`.
  ///
  pub fn from_point(pt: PhysicalPosition<i32>, default: MonitorDefault) -> Option<Self> {
    let hmonitor = unsafe { MonitorFromPoint(POINT { x: pt.x, y: pt.y }, default.to_raw()) };
    match hmonitor.is_null() {
      true => None,
      false => Some(unsafe { Self::from_ptr(hmonitor) }),
    }
  }

  pub fn available() -> VecDeque<Self> {
    let mut monitors: VecDeque<Monitor> = VecDeque::new();
    unsafe {
      EnumDisplayMonitors(
        std::ptr::null_mut(),
        std::ptr::null(),
        Some(enum_display_monitors),
        &mut monitors as *mut _ as LPARAM,
      );
    }
    monitors
  }

  pub fn primary() -> Self {
    Self::from_point(ORIGIN_POINT, MonitorDefault::Primary).unwrap()
  }

  pub fn from_window(window: Window, default: MonitorDefault) -> Option<Self> {
    let hmonitor = unsafe { MonitorFromWindow(window.to_ptr(), default.to_raw()) };
    match hmonitor.is_null() {
      true => None,
      false => Some(unsafe { Self::from_ptr(hmonitor) }),
    }
  }

  pub fn dpi(&self) -> u32 {
    let mut dpi = [0, 0];
    unsafe { GetDpiForMonitor(self.to_ptr(), MDT_EFFECTIVE_DPI, &mut dpi[0], &mut dpi[1]) };
    dpi[0]
  }

  pub fn scale_factor(&self) -> f64 {
    dpi_to_scale_factor(self.dpi())
  }
}
