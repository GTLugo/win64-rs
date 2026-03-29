use windows_sys::Win32::Foundation::POINT;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub const ORIGIN: Point = Point::new(0, 0);

  pub const fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  pub(crate) const fn to_raw(self) -> POINT {
    POINT { x: self.x, y: self.y }
  }
}

impl From<POINT> for Point {
  fn from(value: POINT) -> Self {
    Self { x: value.x, y: value.y }
  }
}
