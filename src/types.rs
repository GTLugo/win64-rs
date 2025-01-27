use cursor_icon::CursorIcon;
use windows::Win32::UI::WindowsAndMessaging::{self, CW_USEDEFAULT, HCURSOR};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Size {
  pub width: i32,
  pub height: i32,
}

impl Size {
  pub const AUTO: Self = Self {
    width: CW_USEDEFAULT,
    height: CW_USEDEFAULT,
  };
}

impl From<[i32; 2]> for Size {
  fn from([width, height]: [i32; 2]) -> Self {
    Self { width, height }
  }
}

impl From<(i32, i32)> for Size {
  fn from((width, height): (i32, i32)) -> Self {
    Self { width, height }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

impl Position {
  pub const AUTO: Self = Self {
    x: CW_USEDEFAULT,
    y: CW_USEDEFAULT,
  };
}

impl From<[i32; 2]> for Position {
  fn from([x, y]: [i32; 2]) -> Self {
    Self { x, y }
  }
}

impl From<(i32, i32)> for Position {
  fn from((x, y): (i32, i32)) -> Self {
    Self { x, y }
  }
}

pub(crate) trait ToHCURSOR {
  fn to_cursor(&self) -> HCURSOR;
}

impl ToHCURSOR for CursorIcon {
  fn to_cursor(&self) -> HCURSOR {
    match self {
      CursorIcon::Default => HCURSOR(WindowsAndMessaging::IDC_ARROW.as_ptr().cast_mut().cast()),
      CursorIcon::ContextMenu => todo!(),
      CursorIcon::Help => todo!(),
      CursorIcon::Pointer => todo!(),
      CursorIcon::Progress => todo!(),
      CursorIcon::Wait => todo!(),
      CursorIcon::Cell => todo!(),
      CursorIcon::Crosshair => todo!(),
      CursorIcon::Text => todo!(),
      CursorIcon::VerticalText => todo!(),
      CursorIcon::Alias => todo!(),
      CursorIcon::Copy => todo!(),
      CursorIcon::Move => todo!(),
      CursorIcon::NoDrop => todo!(),
      CursorIcon::NotAllowed => todo!(),
      CursorIcon::Grab => todo!(),
      CursorIcon::Grabbing => todo!(),
      CursorIcon::EResize => todo!(),
      CursorIcon::NResize => todo!(),
      CursorIcon::NeResize => todo!(),
      CursorIcon::NwResize => todo!(),
      CursorIcon::SResize => todo!(),
      CursorIcon::SeResize => todo!(),
      CursorIcon::SwResize => todo!(),
      CursorIcon::WResize => todo!(),
      CursorIcon::EwResize => todo!(),
      CursorIcon::NsResize => todo!(),
      CursorIcon::NeswResize => todo!(),
      CursorIcon::NwseResize => todo!(),
      CursorIcon::ColResize => todo!(),
      CursorIcon::RowResize => todo!(),
      CursorIcon::AllScroll => todo!(),
      CursorIcon::ZoomIn => todo!(),
      CursorIcon::ZoomOut => todo!(),
      _ => todo!(),
    }
  }
}
