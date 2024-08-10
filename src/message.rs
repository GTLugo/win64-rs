use std::ops::RangeInclusive;

use windows::Win32::{
  Foundation::{self, LPARAM, WPARAM},
  UI::WindowsAndMessaging::{self, DispatchMessageW, TranslateMessage, MSG},
};

use crate::{
  flag::PeekMessageFlags, get_message, peek_message, window::Window, GetMessageResult, PeekMessageResult, ProcedureResult
};

// #[derive(Debug, Clone, PartialEq)]
// pub struct ThreadMessage {
//   msg: Message,
//   metadata: Metadata,
// }

// impl ThreadMessage {
//   pub fn get(
//     hwnd: Option<Window>,
//     filter: Option<RangeInclusive<u32>>,
//   ) -> Result<Self, windows::core::Error> {
//     get_message(hwnd, filter)
//   }

//   pub fn translate(&self) -> bool {
//     let msg = MSG::from(self.clone());
//     unsafe { TranslateMessage(&msg) }.as_bool()
//   }

//   pub fn dispatch(&self) -> ProcedureResult {
//     let msg = MSG::from(self.clone());
//     unsafe { DispatchMessageW(&msg) }.into()
//   }

//   pub fn hwnd(&self) -> Window {
//     self.metadata.hwnd
//   }

//   pub fn message(&self) -> &Message {
//     &self.msg
//   }

//   pub fn time(&self) -> u32 {
//     self.metadata.time
//   }

//   pub fn point(&self) -> Foundation::POINT {
//     self.metadata.pt
//   }
// }

// impl From<ThreadMessage> for MSG {
//   fn from(msg: ThreadMessage) -> Self {
//     Self {
//       hwnd: msg.hwnd().into(),
//       message: msg.message().id,
//       wParam: WPARAM(msg.message().w),
//       lParam: LPARAM(msg.message().l),
//       time: msg.time(),
//       pt: msg.point(),
//     }
//   }
// }

// impl From<MSG> for ThreadMessage {
//   fn from(msg: MSG) -> Self {
//     Self {
//       msg: msg.into(),
//       metadata: Metadata {
//         hwnd: msg.hwnd.into(),
//         time: msg.time,
//         pt: msg.pt,
//       },
//     }
//   }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
  hwnd: Window,
  time: u32,
  pt: Foundation::POINT,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoMetadata;

#[derive(Debug, Clone, PartialEq)]
pub struct Message<M = NoMetadata> {
  id: u32,
  w: usize,
  l: isize,
  metadata: M,
}

// make new() method for both With and Without and getters for the fields

impl From<MSG> for Message<Metadata> {
  fn from(value: MSG) -> Self {
    Self {
      id: value.message,
      w: value.wParam.0,
      l: value.lParam.0,
      metadata: Metadata {
        hwnd: value.hwnd.into(),
        time: value.time,
        pt: value.pt,
      },
    }
  }
}

impl From<Message<Metadata>> for MSG {
  fn from(msg: Message<Metadata>) -> Self {
    Self {
      hwnd: (*msg.window()).into(),
      message: msg.id(),
      wParam: WPARAM(msg.w()),
      lParam: LPARAM(msg.l()),
      time: msg.time(),
      pt: *msg.point(),
    }
  }
}

impl<M> Message<M> {
  pub const QUIT: u32 = WindowsAndMessaging::WM_QUIT;
  pub const DESTROY: u32 = WindowsAndMessaging::WM_DESTROY;

  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn w(&self) -> usize {
    self.w
  }

  pub fn l(&self) -> isize {
    self.l
  }
}

impl Message<NoMetadata> {
  pub fn new(msg: u32, w: WPARAM, l: LPARAM) -> Self {
    Self {
      id: msg,
      w: w.0,
      l: l.0,
      metadata: NoMetadata,
    }
  }
}

impl Message<Metadata> {
  pub fn get(
    hwnd: Option<Window>,
    filter: Option<RangeInclusive<u32>>,
  ) -> GetMessageResult {
    get_message(hwnd, filter)
  }

  pub fn peek(
    hwnd: Option<Window>,
    filter: Option<RangeInclusive<u32>>,
    flags: PeekMessageFlags,
  ) -> PeekMessageResult {
    peek_message(hwnd, filter, flags)
  }

  pub fn translate(&self) -> bool {
    let msg = MSG::from(self.clone());
    unsafe { TranslateMessage(&msg) }.as_bool()
  }

  pub fn dispatch(&self) -> ProcedureResult {
    let msg = MSG::from(self.clone());
    unsafe { DispatchMessageW(&msg) }.into()
  }

  pub fn window(&self) -> &Window {
    &self.metadata.hwnd
  }

  pub fn time(&self) -> u32 {
    self.metadata.time
  }

  pub fn point(&self) -> &Foundation::POINT {
    &self.metadata.pt
  }
}
