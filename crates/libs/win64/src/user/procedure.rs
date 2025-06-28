use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{SW_SHOW, ShowWindow};

use crate::{Handle, get_last_error};

use super::descriptor::WindowDescriptor;
use super::{CreateMessage, HWindow, LParam, Message, NcCreateMessage, WParam, WindowStyle};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct LResult(pub isize);

impl LResult {
  pub fn handled() -> Option<Self> {
    Some(LResult::default())
  }
}

impl From<LResult> for LRESULT {
  fn from(value: LResult) -> Self {
    value.0
  }
}

impl From<LRESULT> for LResult {
  fn from(value: LRESULT) -> Self {
    Self(value)
  }
}

pub trait WindowProcedure {
  #[allow(unused_variables)]
  fn on_message(&mut self, window: HWindow, message: &Message) -> Option<LResult> {
    None
  }
}

pub(crate) struct CreateInfo {
  pub state: Option<Box<dyn WindowProcedure>>,
  pub desc: WindowDescriptor,
}

impl CreateInfo {
  pub fn new(window_state: impl 'static + WindowProcedure, desc: WindowDescriptor) -> Self {
    Self {
      state: Some(Box::new(window_state)),
      desc,
    }
  }
}

pub(crate) enum WindowState {
  Creating(WindowDescriptor),
  Ready,
  Destroying,
}

pub(crate) struct WindowData {
  pub state: WindowState,
  pub proc: Box<dyn WindowProcedure>,
}

impl WindowData {
  pub fn new(mut create_info: CreateInfo) -> Self {
    Self {
      state: WindowState::Creating(create_info.desc),
      proc: create_info.state.take().unwrap(),
    }
  }

  pub fn is_destroying(&self) -> bool {
    matches!(self.state, WindowState::Destroying)
  }
}

/// # Safety
/// Window procedure is unsafe due to ffi with Win32
pub unsafe extern "system" fn window_procedure(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  let window = unsafe { HWindow::from_ptr(hwnd) };
  let message = Message::new(msg, WParam(w_param), LParam(l_param));
  on_message(window, &message)
}

fn on_message(window: HWindow, message: &Message) -> LRESULT {
  if !unsafe { window.is_window() } {
    return 0;
  }

  let result = match (window.data(), message) {
    (None, Message::NcCreate(nc_create_message)) => on_nc_create(window, nc_create_message),
    (Some(data), Message::Create(create_message)) => on_create(window, create_message, data),
    (Some(_), Message::NcDestroy) => {
      window.quit();
      drop(window.take_data());
      None
    }
    (
      Some(WindowData {
        state: WindowState::Ready,
        proc,
      }),
      _,
    ) => proc.on_message(window, message),
    _ => None,
  };

  result.unwrap_or_else(|| window.default_procedure(message)).0
}

fn on_nc_create(window: HWindow, message: &NcCreateMessage) -> Option<LResult> {
  let create_info = message.create_info();

  window.initialize_data(create_info);

  // if let Some(data) = window.data() {
  //   data.proc.on_message(window, message);
  // }

  None
}

fn on_create(window: HWindow, _message: &CreateMessage, data: &mut WindowData) -> Option<LResult> {
  // let create_info = CreateInfo::from_message(message.clone());
  if let WindowState::Creating(desc) = &data.state {
    if desc.style.contains(WindowStyle::Visible) && unsafe { ShowWindow(window.to_ptr(), SW_SHOW) } != 1 {
      eprintln!("Error: {}", get_last_error());
      return Some(LResult(-1));
    }
  }
  data.state = WindowState::Ready;
  None
}
