pub mod styles;
pub use styles::*;

pub mod peek_message;
pub use peek_message::*;

use windows_sys::Win32::UI::WindowsAndMessaging;

// pub const GWLP_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
// pub const GWLP_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
// pub const GWLP_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
// pub const GWLP_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
// pub const GWLP_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum WindowPtrIndex {
  Instance,
  Parent,
  Id,
  UserData,
  WndProc,
}

impl WindowPtrIndex {
  #[inline]
  pub const fn to_raw(self) -> WindowsAndMessaging::WINDOW_LONG_PTR_INDEX {
    #[cfg(target_pointer_width = "32")]
    match self {
      WindowPtrIndex::Instance => WindowsAndMessaging::GWL_HINSTANCE,
      WindowPtrIndex::Parent => WindowsAndMessaging::GWL_HWNDPARENT,
      WindowPtrIndex::Id => WindowsAndMessaging::GWL_ID,
      WindowPtrIndex::UserData => WindowsAndMessaging::GWL_USERDATA,
      WindowPtrIndex::WndProc => WindowsAndMessaging::GWL_WNDPROC,
    }
    #[cfg(target_pointer_width = "64")]
    match self {
      WindowPtrIndex::Instance => WindowsAndMessaging::GWLP_HINSTANCE,
      WindowPtrIndex::Parent => WindowsAndMessaging::GWLP_HWNDPARENT,
      WindowPtrIndex::Id => WindowsAndMessaging::GWLP_ID,
      WindowPtrIndex::UserData => WindowsAndMessaging::GWLP_USERDATA,
      WindowPtrIndex::WndProc => WindowsAndMessaging::GWLP_WNDPROC,
    }
  }
}

// impl Default for LongPointerIndex {
//   fn default() -> Self {
//     Self::empty()
//   }
// }

// impl Default for PeekMessageFlags {
//   fn default() -> Self {
//     Self::empty()
//   }
// }
