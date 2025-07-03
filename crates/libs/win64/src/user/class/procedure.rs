use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

use crate::Handle;
use crate::user::{CreateStruct, LParam, Message, MessageHandler, NcCreateMessage, WParam, Window, WindowPtrIndex};

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
  Creating(CreateStruct),
  Ready(Option<Box<dyn WindowProcedure>>),
  Destroying,
}

impl WindowState {
  pub fn new(create_struct: CreateStruct) -> Self {
    Self::Creating(create_struct)
  }

  pub fn set_ready(&mut self) {
    if let WindowState::Creating(create_struct) = self {
      *self = Self::Ready(create_struct.wnd_proc.take())
    }
  }

  pub fn set_destroying(&mut self) {
    *self = Self::Destroying
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
  on_message(window, message).0
}

fn on_message(window: Window, message: Message) -> LResult {
  if !unsafe { window.is_window() } {
    return LResult(0);
  }

  match (window.state(), message) {
    (None, Message::NcCreate(nc_create_message)) => {
      // if on_nc_create(window, nc_create_message).0 == 0 {
      //   return LResult(0);
      // }

      // let Some(state) = window.state() else {
      //   return LResult(0);
      // };

      // match state.inner() {
      //   Some(inner) => inner.on_message(window, message),
      //   _ => None,
      // }
      on_nc_create(window, nc_create_message).expect("This should always return either Some(true) or Some(false)")
    }
    (Some(state), Message::NcDestroy) => {
      window.quit();
      let state = unsafe { Box::from_raw(state) };
      drop(state);
      LResult(0)
    }
    (Some(WindowState::Ready(Some(inner))), message) => inner
      .on_message(&window, &message)
      .unwrap_or_else(|| window.default_procedure(&message)),
    (_, message) => window.default_procedure(&message),
  }
}

fn on_nc_create(window: Window, message: NcCreateMessage) -> Option<LResult> {
  message.handle(|create_struct| {
    let state = WindowState::new(create_struct);
    let state_ptr = Box::into_raw(Box::new(state));

    if window
      .set_window_ptr(WindowPtrIndex::UserData, state_ptr as isize)
      .is_err()
    {
      return false;
    }

    if let Some(state) = unsafe { state_ptr.as_mut() } {
      state.set_ready();
    }

    true
  })
}
