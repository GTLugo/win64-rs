use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

use crate::Handle;
use crate::user::{LParam, Message, MessageHandler, WParam, Window, WindowPtrIndex};

use super::LResult;

#[allow(unused_variables)]
pub trait WindowProcedure {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
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
  Creating,
  Running(Box<dyn WindowProcedure>),
  Destroying,
}

impl WindowState {
  pub fn new() -> Self {
    Self::Creating
  }

  pub fn set_running(&mut self, wnd_proc: Box<dyn WindowProcedure>) {
    *self = Self::Running(wnd_proc);
  }

  pub fn set_destroying(&mut self) {
    *self = Self::Destroying;
  }

  // pub fn inner(&mut self) -> Option<&mut Box<dyn WindowProcedure>> {
  //   match self {
  //     WindowState::Creating(create_struct) => create_struct.wnd_proc.as_mut(),
  //     WindowState::Ready(app) => app.as_mut(),
  //     WindowState::Destroying => None,
  //   }
  // }
}

/// # Safety
/// Window procedure is unsafe due to ffi with Win32
pub unsafe extern "system" fn window_procedure(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  let window = unsafe { Window::from_ptr(hwnd) };
  let message = Message::new(msg, WParam(w_param), LParam(l_param));
  on_message(window, message)
    .unwrap_or_else(|| window.def_window_proc_raw(msg, w_param, l_param))
    .0
}

fn on_message(window: Window, message: Message) -> Option<LResult> {
  match (window.state(), message) {
    (None, Message::NcCreate(nc_create_message)) => {
      nc_create_message.handle(|wnd_proc| {
        let state_ptr = Box::into_raw(Box::new(WindowState::new()));

        let result = window.set_window_ptr(WindowPtrIndex::UserData, state_ptr as isize);
        if result.is_err() {
          return false;
        }

        match unsafe { state_ptr.as_mut() } {
          Some(state) => {
            state.set_running(wnd_proc);
            true
          }
          _ => false,
        }
      });
      None
    }
    (Some(state), Message::NcDestroy) => {
      window.quit();
      drop(unsafe { Box::from_raw(state) });
      Some(LResult(0))
    }
    (Some(WindowState::Running(state)), message) => state.on_message(&window, &message),
    (_, _) => None,
  }
}
