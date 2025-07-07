use windows_sys::Win32::Foundation::POINT;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl From<POINT> for Point {
  fn from(value: POINT) -> Self {
    Self { x: value.x, y: value.y }
  }
}
