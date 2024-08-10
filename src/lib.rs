use std::{io, ops::RangeInclusive};

use thiserror::Error;
use windows::Win32::{
  Foundation::{HWND, LRESULT},
  UI::WindowsAndMessaging::{self, GetMessageW, PeekMessageW, MSG},
};

use self::{
  flag::PeekMessageFlags,
  message::{Message, Metadata},
  window::Window,
};

pub mod flag;
pub mod handle;
pub mod message;
pub mod prelude;
pub mod procedure;
pub mod types;
pub mod window;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ProcedureResult(pub isize);

impl From<ProcedureResult> for LRESULT {
  fn from(value: ProcedureResult) -> Self {
    Self(value.0)
  }
}

impl From<LRESULT> for ProcedureResult {
  fn from(value: LRESULT) -> Self {
    Self(value.0)
  }
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("{0}")]
  IOError(#[from] io::Error),
  #[error("{0}")]
  Win32Error(#[from] windows::core::Error),
}

// #[derive(Error, Debug)]
// pub enum GetMessageError {
//   #[error("WM_QUIT recieved")]
//   Quit,
//   #[error("{0}")]
//   Win32Error(#[from] windows::core::Error),
// }

#[derive(Debug)]
pub enum GetMessageResult {
  Message(Message<Metadata>),
  Quit,
  Error(windows::core::Error),
}

#[derive(Debug)]
pub enum PeekMessageResult {
  Message(Message<Metadata>),
  Quit,
  None,
}

fn get_message(
  hwnd: Option<Window>,
  filter: &Option<RangeInclusive<u32>>,
) -> GetMessageResult {
  let (min, max) = match filter {
    Some(filter) => (*filter.start(), *filter.end()),
    None => (0, 0),
  };
  let mut msg = MSG::default();
  let result = match hwnd {
    Some(hwnd) => unsafe { GetMessageW(&mut msg, HWND::from(hwnd), min, max) },
    None => unsafe { GetMessageW(&mut msg, None, min, max) },
  };
  // WM_QUIT sends return value of zero, causing BOOL to be false. This is still valid though.
  // Only -1 is actually an error.
  match result.0 {
    0 => GetMessageResult::Quit,
    -1 => GetMessageResult::Error(result.ok().unwrap_err()),
    _ => GetMessageResult::Message(Message::from(msg)),
  }
}

fn peek_message(
  hwnd: Option<Window>,
  filter: &Option<RangeInclusive<u32>>,
  flags: PeekMessageFlags,
) -> PeekMessageResult {
  let (min, max) = match filter {
    Some(filter) => (*filter.start(), *filter.end()),
    None => (0, 0),
  };
  let mut msg = MSG::default();
  let result = match hwnd {
    Some(hwnd) => unsafe {
      PeekMessageW(&mut msg, HWND::from(hwnd), min, max, flags.into())
    },
    None => unsafe { PeekMessageW(&mut msg, None, min, max, flags.into()) },
  };
  // If a message is available, the return value is nonzero.
  // If no messages are available, the return value is zero.
  match (result.as_bool(), msg.message) {
    (true, WindowsAndMessaging::WM_QUIT) => PeekMessageResult::Quit,
    (true, _) => PeekMessageResult::Message(Message::from(msg)),
    (false, _) => PeekMessageResult::None,
  }
}

// pub trait Win32Thread {
//   fn get_message(
//     hwnd: Option<Window>,
//     filter: Option<RangeInclusive<u32>>,
//   ) -> Result<ThreadMessage, windows::core::Error>;

//   fn peek_message(
//     hwnd: Option<Window>,
//     filter: Option<RangeInclusive<u32>>,
//     flags: PeekMessageFlags,
//   ) -> Option<ThreadMessage>;

//   fn post_quit_message(exit_code: i32) {
//     unsafe { PostQuitMessage(exit_code) }
//   }

//   fn post_message(
//     hwnd: Option<Window>,
//     msg: Message,
//   ) -> Result<(), windows::core::Error> {
//     match hwnd {
//       Some(hwnd) => unsafe {
//         PostMessageW(HWND::from(hwnd), msg.id, WPARAM(msg.w), LPARAM(msg.l))
//       },
//       None => unsafe { PostMessageW(None, msg.id, WPARAM(msg.w), LPARAM(msg.l)) },
//     }
//   }
// }

// impl Win32Thread for std::thread::Thread {
//   fn get_message(
//     hwnd: Option<Window>,
//     filter: Option<RangeInclusive<u32>>,
//   ) -> Result<ThreadMessage, windows::core::Error> {
//     get_message(hwnd, filter)
//   }

//   fn peek_message(
//     hwnd: Option<Window>,
//     filter: Option<RangeInclusive<u32>>,
//     flags: PeekMessageFlags,
//   ) -> Option<ThreadMessage> {
//     peek_message(hwnd, filter, flags)
//   }
// }
