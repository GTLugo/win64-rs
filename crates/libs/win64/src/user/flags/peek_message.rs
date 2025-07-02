use bitflags::bitflags;
use windows_sys::Win32::UI::WindowsAndMessaging::{self, PEEK_MESSAGE_REMOVE_TYPE};

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct PeekMessageFlags: u32 {
    const NoRemove = WindowsAndMessaging::PM_NOREMOVE;
    const Remove = WindowsAndMessaging::PM_REMOVE;
    const NoYield = WindowsAndMessaging::PM_NOYIELD;
    const Input = WindowsAndMessaging::PM_QS_INPUT;
    const Paint = WindowsAndMessaging::PM_QS_PAINT;
    const PostMessage = WindowsAndMessaging::PM_QS_POSTMESSAGE;
    const SendMessage = WindowsAndMessaging::PM_QS_SENDMESSAGE;
  }
}

impl PeekMessageFlags {
  #[inline]
  pub const fn to_raw(self) -> PEEK_MESSAGE_REMOVE_TYPE {
    self.bits()
  }
}
