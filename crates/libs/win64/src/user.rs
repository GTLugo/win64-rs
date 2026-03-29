pub mod hwnd;
pub use hwnd::*;

pub mod hdc;
pub use hdc::*;

#[cfg(any(feature = "rwh_06", feature = "rwh_05"))]
pub mod rwh;

#[cfg(any(feature = "rwh_06", feature = "rwh_05"))]
#[allow(unused)]
pub use rwh::*;

pub mod hinstance;
pub use hinstance::*;

pub mod class;
pub use class::*;

pub mod winmain;
use windows_sys::Win32::UI::HiDpi::{
  DPI_AWARENESS_CONTEXT,
  DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE,
  DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
  DPI_AWARENESS_CONTEXT_SYSTEM_AWARE,
  DPI_AWARENESS_CONTEXT_UNAWARE,
  DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED,
  GetDpiForSystem,
  SetProcessDpiAwarenessContext,
};
pub use winmain::*;

pub mod message;
pub use message::*;

pub mod message_loop;
pub use message_loop::*;

pub mod monitor;
pub use monitor::*;

pub mod dark;
pub use dark::*;

pub mod flags;
pub use flags::*;

pub mod cursor;
pub use cursor::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DPIAwarenessContext {
  Unaware,
  SystemAware,
  PerMonitorAware,
  PerMonitorAwareV2,
  UnawareGdiscaled,
}

impl DPIAwarenessContext {
  const fn to_raw(self) -> DPI_AWARENESS_CONTEXT {
    match self {
      DPIAwarenessContext::Unaware => DPI_AWARENESS_CONTEXT_UNAWARE,
      DPIAwarenessContext::SystemAware => DPI_AWARENESS_CONTEXT_SYSTEM_AWARE,
      DPIAwarenessContext::PerMonitorAware => DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE,
      DPIAwarenessContext::PerMonitorAwareV2 => DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
      DPIAwarenessContext::UnawareGdiscaled => DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED,
    }
  }
}

pub fn set_process_dpi_awareness(context: DPIAwarenessContext) -> Option<()> {
  match unsafe { SetProcessDpiAwarenessContext(context.to_raw()) } {
    1 => Some(()),
    _ => None,
  }
}

pub fn system_dpi() -> u32 {
  unsafe { GetDpiForSystem() }
}

// Directly from Winit
pub fn dpi_to_scale_factor(dpi: u32) -> f64 {
  dpi as f64 / Monitor::BASE_DPI as f64
}
