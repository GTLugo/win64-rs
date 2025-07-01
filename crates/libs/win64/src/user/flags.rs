pub mod styles;
pub use styles::*;

use bitflags::bitflags;
use windows_sys::Win32::UI::WindowsAndMessaging::{self, PEEK_MESSAGE_REMOVE_TYPE, WINDOW_LONG_PTR_INDEX};

// pub const GWLP_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
// pub const GWLP_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
// pub const GWLP_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
// pub const GWLP_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
// pub const GWLP_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum WindowPointerIndex {
  Instance,
  Parent,
  Id,
  UserData,
  WndProc,
}

impl WindowPointerIndex {
  #[inline]
  pub const fn to_raw(self) -> WINDOW_LONG_PTR_INDEX {
    #[cfg(target_pointer_width = "32")]
    match self {
      WindowPointerIndex::Instance => WindowsAndMessaging::GWL_HINSTANCE,
      WindowPointerIndex::Parent => WindowsAndMessaging::GWL_HWNDPARENT,
      WindowPointerIndex::Id => WindowsAndMessaging::GWL_ID,
      WindowPointerIndex::UserData => WindowsAndMessaging::GWL_USERDATA,
      WindowPointerIndex::WndProc => WindowsAndMessaging::GWL_WNDPROC,
    }
    #[cfg(target_pointer_width = "64")]
    match self {
      WindowPointerIndex::Instance => WindowsAndMessaging::GWLP_HINSTANCE,
      WindowPointerIndex::Parent => WindowsAndMessaging::GWLP_HWNDPARENT,
      WindowPointerIndex::Id => WindowsAndMessaging::GWLP_ID,
      WindowPointerIndex::UserData => WindowsAndMessaging::GWLP_USERDATA,
      WindowPointerIndex::WndProc => WindowsAndMessaging::GWLP_WNDPROC,
    }
  }
}

// impl Default for LongPointerIndex {
//   fn default() -> Self {
//     Self::empty()
//   }
// }

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

// impl Default for PeekMessageFlags {
//   fn default() -> Self {
//     Self::empty()
//   }
// }
