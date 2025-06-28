use dpi::{Pixel, PixelUnit, Position, Size};
use windows_sys::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;

const INIT_SCALE_FACTOR: f64 = 1.0;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum WindowPos {
  #[default]
  Auto,
  AutoX {
    y: PixelUnit,
  },
  AutoY {
    x: PixelUnit,
  },
  Position(Position),
}

impl WindowPos {
  pub fn x(&self) -> i32 {
    match self {
      Self::Auto => CW_USEDEFAULT,
      Self::Position(Position::Logical(pos)) => pos.to_physical(INIT_SCALE_FACTOR).x,
      Self::Position(Position::Physical(pos)) => pos.x,
      Self::AutoX { .. } => CW_USEDEFAULT,
      Self::AutoY {
        x: PixelUnit::Logical(x),
      } => x.to_physical(INIT_SCALE_FACTOR).0,
      Self::AutoY {
        x: PixelUnit::Physical(x),
      } => x.0,
    }
  }

  pub fn y(&self) -> i32 {
    match self {
      Self::Auto => CW_USEDEFAULT,
      Self::Position(Position::Logical(pos)) => pos.to_physical(INIT_SCALE_FACTOR).x,
      Self::Position(Position::Physical(pos)) => pos.x,
      Self::AutoX {
        y: PixelUnit::Logical(y),
      } => y.to_physical(INIT_SCALE_FACTOR).0,
      Self::AutoX {
        y: PixelUnit::Physical(y),
      } => y.0,
      Self::AutoY { .. } => CW_USEDEFAULT,
    }
  }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum WindowSize {
  #[default]
  Auto,
  AutoWidth {
    height: PixelUnit,
  },
  AutoHeight {
    width: PixelUnit,
  },
  Size(Size),
}

impl WindowSize {
  pub fn width(&self) -> i32 {
    match self {
      Self::Auto => CW_USEDEFAULT,
      Self::Size(Size::Logical(size)) => size.to_physical(INIT_SCALE_FACTOR).width,
      Self::Size(Size::Physical(size)) => size.width.cast(),
      Self::AutoWidth { .. } => CW_USEDEFAULT,
      Self::AutoHeight {
        width: PixelUnit::Logical(p),
      } => p.to_physical(INIT_SCALE_FACTOR).0,
      Self::AutoHeight {
        width: PixelUnit::Physical(p),
      } => p.0,
    }
  }

  pub fn height(&self) -> i32 {
    match self {
      Self::Auto => CW_USEDEFAULT,
      Self::Size(Size::Logical(size)) => size.to_physical(INIT_SCALE_FACTOR).height,
      Self::Size(Size::Physical(size)) => size.height.cast(),
      Self::AutoWidth {
        height: PixelUnit::Logical(p),
      } => p.to_physical(INIT_SCALE_FACTOR).0,
      Self::AutoWidth {
        height: PixelUnit::Physical(p),
      } => p.0,
      Self::AutoHeight { .. } => CW_USEDEFAULT,
    }
  }
}

// pub trait Auto {
//   fn auto() -> Self where Self: Sized;
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Size {
//   pub width: i32,
//   pub height: i32,
// }

// impl Size {
//   pub const AUTO: Self = Self {
//     width: CW_USEDEFAULT,
//     height: CW_USEDEFAULT,
//   };
// }

// impl From<[i32; 2]> for Size {
//   fn from([width, height]: [i32; 2]) -> Self {
//     Self { width, height }
//   }
// }

// impl From<(i32, i32)> for Size {
//   fn from((width, height): (i32, i32)) -> Self {
//     Self { width, height }
//   }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Position {
//   pub x: i32,
//   pub y: i32,
// }

// impl Position {
//   pub const AUTO: Self = Self {
//     x: CW_USEDEFAULT,
//     y: CW_USEDEFAULT,
//   };
// }

// impl From<[i32; 2]> for Position {
//   fn from([x, y]: [i32; 2]) -> Self {
//     Self { x, y }
//   }
// }

// impl From<(i32, i32)> for Position {
//   fn from((x, y): (i32, i32)) -> Self {
//     Self { x, y }
//   }
// }
