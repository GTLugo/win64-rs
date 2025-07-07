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

pub(crate) enum WindowState {
  Creating,
  Running,
  Destroying,
}

pub(crate) struct UserData {
  pub proc: Box<dyn WindowProcedure>,
  pub state: WindowState,
}

impl UserData {
  pub fn new(proc: Box<dyn WindowProcedure>) -> Self {
    Self {
      proc,
      state: WindowState::Creating,
    }
  }
}

/// # Safety
/// Window procedure is unsafe due to ffi with Win32
pub(crate) unsafe extern "system" fn window_procedure(
  hwnd: HWND,
  msg: u32,
  w_param: WPARAM,
  l_param: LPARAM,
) -> LRESULT {
  let window = unsafe { Window::from_ptr(hwnd) };
  let message = Message::new(msg.into(), WParam(w_param), LParam(l_param));
  let result = on_message(&window, &message).unwrap_or_else(|| window.def_window_proc_raw(msg, w_param, l_param));
  result.0
}

fn on_message(window: &Window, message: &Message) -> Option<LResult> {
  match (window.user_data(), message) {
    (None, Message::NcCreate(nc_create_message)) => {
      nc_create_message.handle(|wnd_proc| {
        let data_ptr = Box::into_raw(Box::new(UserData::new(wnd_proc)));
        let _ = window.set_window_ptr(WindowPtrIndex::UserData, data_ptr as isize);

        unsafe { data_ptr.as_mut() }
          .expect("window user data ptr went invalid during creation")
          .state = WindowState::Running;

        true
      });

      let data = (unsafe { (window.get_window_ptr(WindowPtrIndex::UserData) as *mut UserData).as_mut() })?;
      data.proc.on_message(window, message)
    }
    (Some(data), Message::NcDestroy) => {
      let mut data = unsafe { Box::from_raw(data) };
      data.proc.on_message(window, message)
    }
    (Some(data), message) => data.proc.on_message(window, message),
    (_, _) => None,
  }
}
