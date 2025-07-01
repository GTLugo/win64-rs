use std::ops::RangeInclusive;

use windows_result::Error;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG};

use crate::{Handle, get_last_error, user::Message};

use super::{Msg, MsgQueue};

pub fn get_message(queue: MsgQueue, filter: Option<RangeInclusive<u32>>) -> Result<Msg, Error> {
  let (min, max) = filter.map(RangeInclusive::into_inner).unwrap_or_default();
  let mut msg = MSG::default();
  let result = unsafe { GetMessageW(&mut msg, queue.unwrap_or_default().to_ptr(), min, max) };
  // WM_QUIT sends return value of zero, causing BOOL to be false. This is still valid though.
  // Only -1 is actually an error.
  match result {
    -1 => Err(get_last_error()),
    _ => Ok(Msg::from(msg)),
  }
}

pub enum GetMessageIterator {
  Iterating {
    queue: MsgQueue,
    filter: Option<RangeInclusive<u32>>,
  },
  Quitting,
}

impl Iterator for GetMessageIterator {
  type Item = Result<Msg, Error>;

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      GetMessageIterator::Iterating { queue, filter } => {
        let message = get_message(*queue, filter.clone());
        match message {
          Ok(msg) => {
            if matches!(msg.message, Message::Quit(_)) {
              *self = GetMessageIterator::Quitting;
            }
            Some(Ok(msg))
          }
          Err(e) => Some(Err(e)),
        }
      }
      GetMessageIterator::Quitting => None,
    }
  }
}
