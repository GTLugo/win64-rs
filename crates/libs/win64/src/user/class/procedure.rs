use windows_sys::Win32::Foundation::{FALSE, HWND, LPARAM, LRESULT, WPARAM};

use crate::Handle;
use crate::user::{CreateStruct, LParam, Message, NcCreateMessage, WParam, Window, WindowPtrIndex};

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
  fn on_message(&mut self, window: Window, message: &Message) -> Option<LResult> {
    None
  }
}

// pub(crate) struct CreateInfo {
//   pub state: Option<Box<dyn WindowProcedure>>,
//   pub desc: WindowDescriptor,
// }

// impl CreateInfo {
//   pub fn new(window_state: impl 'static + WindowProcedure, desc: WindowDescriptor) -> Self {
//     Self {
//       state: Some(Box::new(window_state)),
//       desc,
//     }
//   }
// }

pub(crate) enum WindowState {
  Creating { create_struct: CreateStruct },
  Ready { app: Option<Box<dyn WindowProcedure>> },
  Destroying,
}

impl WindowState {
  pub fn new(create_struct: CreateStruct) -> Self {
    Self::Creating { create_struct }
  }

  pub fn set_ready(&mut self) {
    if let WindowState::Creating { create_struct, .. } = self {
      *self = Self::Ready {
        app: create_struct.wnd_proc.take(),
      }
    }
  }

  pub fn set_destroying(&mut self) {
    *self = Self::Destroying
  }

  pub fn app(&mut self) -> Option<&mut Box<dyn WindowProcedure>> {
    match self {
      WindowState::Creating { create_struct, .. } => create_struct.wnd_proc.as_mut(),
      WindowState::Ready { app } => app.as_mut(),
      WindowState::Destroying => None,
    }
  }
}

/// # Safety
/// Window procedure is unsafe due to ffi with Win32
pub unsafe extern "system" fn window_procedure(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  let window = unsafe { Window::from_ptr(hwnd) };
  let message = Message::new(msg, WParam(w_param), LParam(l_param));
  on_message(window, &message)
}

fn on_message(window: Window, message: &Message) -> LRESULT {
  if !unsafe { window.is_window() } {
    return 0;
  }

  let result = match (window.state(), message) {
    (None, Message::NcCreate(nc_create_message)) => {
      on_nc_create(window, nc_create_message);

      match window.state() {
        Some(state) => match state.app() {
          Some(app) => app.on_message(window, message),
          _ => None,
        },
        _ => None,
      }
    }
    (Some(state), Message::NcDestroy) => {
      window.quit();
      let state = unsafe { Box::from_raw(state) };
      drop(state);
      None
    }
    (Some(WindowState::Ready { app: Some(app) }), _) => app.on_message(window, message),
    _ => None,
  };

  result.unwrap_or_else(|| window.default_procedure(message)).0
}

fn on_nc_create(window: Window, message: &NcCreateMessage) -> LResult {
  let create_info = message.create_info();

  let state = WindowState::new(create_info);
  let state_ptr = Box::into_raw(Box::new(state));

  if window
    .set_window_ptr(WindowPtrIndex::UserData, state_ptr as isize)
    .is_err()
  {
    return LResult(FALSE as _);
  }

  if let Some(state) = unsafe { state_ptr.as_mut() } {
    state.set_ready();
  }

  LResult(1)
}
