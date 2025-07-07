use std::ops::RangeInclusive;

use windows_result::Error;
use windows_sys::Win32::UI::WindowsAndMessaging::{MSG, PeekMessageW};

use crate::{
  Handle, get_last_error, reset_last_error,
  user::{Message, PeekMessageFlags},
};

use super::{Msg, MsgQueue};

pub fn peek_message(queue: MsgQueue, filter: Option<RangeInclusive<u32>>, flags: PeekMessageFlags) -> PeekResult {
  let (min, max) = filter.map(RangeInclusive::into_inner).unwrap_or_default();
  let mut msg = MSG::default();
  reset_last_error();
  let result = unsafe { PeekMessageW(&mut msg, queue.unwrap_or_default().to_ptr(), min, max, flags.to_raw()) };
  // If a message is available, the return value is nonzero.
  // If no messages are available, the return value is zero.
  match (result, get_last_error()) {
    (0, None) => PeekResult::None,
    (0, Some(error)) => PeekResult::Err(error),
    _ => PeekResult::Msg(Msg::from(msg)),
  }
}

pub enum PeekMessageIterator {
  Iterating {
    queue: MsgQueue,
    filter: Option<RangeInclusive<u32>>,
    flags: PeekMessageFlags,
  },
  Quitting,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PeekResult {
  Msg(Msg),
  #[default]
  None,
  Err(Error),
}

impl PeekResult {
  pub fn ok(self) -> Option<Msg> {
    match self {
      Self::Msg(msg) => Some(msg),
      _ => None,
    }
  }

  pub fn take(&mut self) -> PeekResult {
    std::mem::replace(self, PeekResult::None)
  }
}

pub struct PeekMessageResultIntoIterator {
  pm: PeekResult,
}

impl Iterator for PeekMessageResultIntoIterator {
  type Item = Msg;

  fn next(&mut self) -> Option<Self::Item> {
    self.pm.take().ok()
  }
}

impl IntoIterator for PeekResult {
  type Item = Msg;
  type IntoIter = PeekMessageResultIntoIterator;

  fn into_iter(self) -> Self::IntoIter {
    PeekMessageResultIntoIterator { pm: self }
  }
}

impl Iterator for PeekMessageIterator {
  type Item = PeekResult;

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      PeekMessageIterator::Iterating { queue, filter, flags } => {
        let message = peek_message(*queue, filter.clone(), *flags);

        if let PeekResult::Msg(Msg {
          message: Message::Quit(_),
          ..
        }) = message
        {
          *self = PeekMessageIterator::Quitting;
        }

        Some(message)
      }
      PeekMessageIterator::Quitting => None,
    }
  }
}
