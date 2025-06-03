use windows_sys::Win32::UI::WindowsAndMessaging::IsWindow;

use crate::Handle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HWindow(usize);

impl Handle for HWindow {
  fn from_raw(raw: usize) -> Self {
    Self(raw)
  }

  fn to_raw(self) -> usize {
    self.0
  }
}

impl HWindow {
  pub fn is_valid(&self) -> bool {
    unsafe { IsWindow(self.to_raw() as _) != 0 }
  }
}

impl std::fmt::Pointer for HWindow {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Pointer::fmt(&self, f)
  }
}

pub struct WindowClass {}
