use windows_sys::Win32::Foundation::RECT;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect {
  pub left: i32,
  pub top: i32,
  pub right: i32,
  pub bottom: i32,
}

impl Rect {
  pub fn to_raw(&self) -> RECT {
    RECT {
      left: self.left,
      top: self.top,
      right: self.right,
      bottom: self.bottom,
    }
  }
}

impl From<RECT> for Rect {
  fn from(value: RECT) -> Self {
    Self {
      left: value.left,
      top: value.top,
      right: value.right,
      bottom: value.bottom,
    }
  }
}
