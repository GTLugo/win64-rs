use windows::Win32::UI::WindowsAndMessaging;

use super::{FromMessage, RawMessage};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardMessage {}

impl KeyboardMessage {}

impl FromMessage for KeyboardMessage {
  type Err = ();
  
  const ID_LOWER_BOUND: u32 = WindowsAndMessaging::WM_KEYFIRST;
  const ID_UPPER_BOUND: u32 = WindowsAndMessaging::WM_KEYLAST;

  fn from_message(msg: &RawMessage) -> Result<Self, Self::Err> {
    match msg.id {
      Self::ID_LOWER_BOUND..=Self::ID_UPPER_BOUND => Ok(Self {}),
      _ => Err(()),
    }
  }
}
