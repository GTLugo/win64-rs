use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  UI::WindowsAndMessaging::{
    self, DefWindowProcW, GetWindowLongPtrW, SetWindowLongPtrW, CREATESTRUCTW,
  },
};

use crate::{handle::{window::Window, Handle}, message::{Message, NoMetadata}, ProcedureResult};

pub trait WindowProcedure {
  fn default_window_procedure(
    &mut self,
    window: Handle<Window>,
    message: Message<NoMetadata>,
  ) -> ProcedureResult {
    unsafe {
      DefWindowProcW(HWND::from(window), message.id(), WPARAM(message.w()), LPARAM(message.l()))
    }
    .into()
  }

  fn on_message(&mut self, window: Handle<Window>, message: Message<NoMetadata>) -> ProcedureResult {
    self.default_window_procedure(window, message)
  }
}

pub(crate) struct CreateInfo {
  pub user_state: Option<Box<dyn WindowProcedure>>,
}

struct WindowState {
  user_state: Box<dyn WindowProcedure>,
}

/// # Safety
/// Window procedure is inherently unsafe because Win32
pub unsafe extern "system" fn window_procedure(
  hwnd: HWND,
  msg: u32,
  w_param: WPARAM,
  l_param: LPARAM,
) -> LRESULT {
  let state_ptr = unsafe { GetWindowLongPtrW(hwnd, WindowsAndMessaging::GWLP_USERDATA) };
  let state = unsafe { (state_ptr as *mut WindowState).as_mut() };

  match (state, msg) {
    (None, WindowsAndMessaging::WM_NCCREATE) => on_nccreate(hwnd, msg, w_param, l_param),
    (Some(state), _) => state
      .user_state
      .on_message(hwnd.into(), Message::new(msg, w_param, l_param))
      .into(),
    _ => unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) },
  }
}

fn on_nccreate(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  let create_struct = unsafe { (l_param.0 as *mut CREATESTRUCTW).as_mut().unwrap() };
  let create_info = unsafe {
    (create_struct.lpCreateParams as *mut CreateInfo)
      .as_mut()
      .unwrap()
  };

  let state = WindowState {
    user_state: create_info.user_state.take().unwrap(),
  };
  let state_ptr = Box::into_raw(Box::new(state));
  unsafe {
    SetWindowLongPtrW(hwnd, WindowsAndMessaging::GWLP_USERDATA, state_ptr as isize)
  };
  let state = unsafe { state_ptr.as_mut() };

  state
    .unwrap()
    .user_state
    .on_message(hwnd.into(), Message::new(msg, w_param, l_param));

  unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) }
}
