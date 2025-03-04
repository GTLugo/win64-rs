use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  UI::WindowsAndMessaging::CREATESTRUCTW,
};

use crate::{
  handle::window::WindowId,
  message::{Message, data::MessageData, id::MessageId},
};

// #[derive(Debug, Default, Clone, Copy, PartialEq)]
// pub struct ProcedureResult(pub isize);

// impl From<ProcedureResult> for LRESULT {
//   fn from(value: ProcedureResult) -> Self {
//     Self(value.0)
//   }
// }

// impl From<LRESULT> for ProcedureResult {
//   fn from(value: LRESULT) -> Self {
//     Self(value.0)
//   }
// }

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Response(pub isize);

impl From<Response> for LRESULT {
  fn from(value: Response) -> Self {
    Self(value.0)
  }
}
impl From<LRESULT> for Response {
  fn from(value: LRESULT) -> Self {
    Self(value.0)
  }
}

pub trait WindowProcedure {
  #[allow(unused_variables)]
  fn on_message(&mut self, window: WindowId, message: &Message) -> Option<Response> {
    None
  }

  // fn on_create(&mut self, window: WindowId, message: Message) {}

  // fn on_keyboard(&mut self, window: WindowId, message: KeyboardMessage) {}

  // fn on_mouse(&mut self, window: WindowId, message: MouseMessage) {}
}

pub(crate) struct CreateInfo {
  pub state: Option<Box<dyn WindowProcedure>>,
}

impl CreateInfo {
  pub fn new(window_state: impl 'static + WindowProcedure) -> Self {
    Self {
      state: Some(Box::new(window_state)),
    }
  }

  pub fn parse(message: Message) -> Self {
    let create_struct = unsafe { (message.raw().l as *mut CREATESTRUCTW).as_mut() }.unwrap();
    let create_info = unsafe { Box::from_raw(create_struct.lpCreateParams as *mut CreateInfo) };
    *create_info
  }
}

pub(crate) struct WindowData {
  proc: Box<dyn WindowProcedure>,
}

impl WindowData {
  pub fn new(mut create_info: CreateInfo) -> Self {
    Self {
      proc: create_info.state.take().unwrap(),
    }
  }
}

/// # Safety
/// Window procedure is inherently unsafe because Win32
pub unsafe extern "system" fn window_procedure(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  let window: WindowId = hwnd.into();
  let data = MessageData {
    w: w_param.0,
    l: l_param.0,
  };
  let message = Message::from_raw(msg.into(), data);
  on_message(window, &message, window.data())
}

fn on_nccreate(window: WindowId, message: &Message) -> LRESULT {
  let create_info = CreateInfo::parse(message.clone());

  window.initialize_data(create_info);

  if let Some(data) = window.data() {
    data.proc.on_message(window, message);
  }

  window.default_procedure(message).into()
}

fn on_message(window: WindowId, message: &Message, data: Option<&mut WindowData>) -> LRESULT {
  match (data, message.id()) {
    (None, MessageId::NcCreate) => on_nccreate(window, message),
    (Some(_), MessageId::NcDestroy) => {
      let mut data = window.take_data(); // take ownership so it drops at end of block
      data
        .proc
        .on_message(window, message)
        .unwrap_or_else(|| window.default_procedure(message))
        .into()
    }
    (Some(data), _) => {
      // ...
      data
        .proc
        .on_message(window, message)
        .unwrap_or_else(|| window.default_procedure(message))
        .into()
    }
    _ => window.default_procedure(message).into(),
  }
}
