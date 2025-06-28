use cursor_icon::CursorIcon;
use windows_sys::Win32::UI::WindowsAndMessaging::{self, HCURSOR, LoadCursorW};

const DEFAULT_CURSOR: *const u16 = WindowsAndMessaging::IDC_ARROW;

pub(crate) trait ToHCURSOR {
  fn to_raw(&self) -> *const u16;
}

impl ToHCURSOR for CursorIcon {
  fn to_raw(&self) -> *const u16 {
    match self {
      // CursorIcon::Default => DEFAULT_CURSOR,
      CursorIcon::Text => WindowsAndMessaging::IDC_IBEAM,
      CursorIcon::Wait => WindowsAndMessaging::IDC_WAIT,
      CursorIcon::Help => WindowsAndMessaging::IDC_HELP,
      CursorIcon::Crosshair => WindowsAndMessaging::IDC_CROSS,
      CursorIcon::NotAllowed => WindowsAndMessaging::IDC_NO,
      CursorIcon::Grabbing => WindowsAndMessaging::IDC_HAND,
      CursorIcon::Grab => WindowsAndMessaging::IDC_HAND,
      CursorIcon::Move => WindowsAndMessaging::IDC_SIZEALL,
      CursorIcon::Progress => WindowsAndMessaging::IDC_APPSTARTING,
      CursorIcon::VerticalText => WindowsAndMessaging::IDC_UPARROW,
      // CursorIcon::Alias => DEFAULT_CURSOR,
      // CursorIcon::Copy => DEFAULT_CURSOR,
      CursorIcon::Cell => WindowsAndMessaging::IDC_IBEAM,
      // CursorIcon::ContextMenu => DEFAULT_CURSOR,
      CursorIcon::NoDrop => WindowsAndMessaging::IDC_NO,
      CursorIcon::AllScroll => WindowsAndMessaging::IDC_SIZEALL,
      CursorIcon::Pointer => WindowsAndMessaging::IDC_HAND,
      CursorIcon::EResize => WindowsAndMessaging::IDC_SIZEWE,
      CursorIcon::NResize => WindowsAndMessaging::IDC_SIZENS,
      CursorIcon::NeResize => WindowsAndMessaging::IDC_SIZENESW,
      CursorIcon::NwResize => WindowsAndMessaging::IDC_SIZENWSE,
      CursorIcon::SResize => WindowsAndMessaging::IDC_SIZENS,
      CursorIcon::SeResize => WindowsAndMessaging::IDC_SIZENWSE,
      CursorIcon::SwResize => WindowsAndMessaging::IDC_SIZENESW,
      CursorIcon::WResize => WindowsAndMessaging::IDC_SIZEWE,
      CursorIcon::EwResize => WindowsAndMessaging::IDC_SIZEWE,
      CursorIcon::NsResize => WindowsAndMessaging::IDC_SIZENS,
      CursorIcon::NeswResize => WindowsAndMessaging::IDC_SIZENESW,
      CursorIcon::NwseResize => WindowsAndMessaging::IDC_SIZENWSE,
      CursorIcon::ColResize => WindowsAndMessaging::IDC_SIZEWE,
      CursorIcon::RowResize => WindowsAndMessaging::IDC_SIZENS,
      // CursorIcon::ZoomIn => DEFAULT_CURSOR,
      // CursorIcon::ZoomOut => DEFAULT_CURSOR,
      _ => DEFAULT_CURSOR,
    }
  }
}

pub(crate) trait LoadCursor {
  fn load(self) -> HCURSOR;
}

pub fn load_cursor(cursor: CursorIcon) -> HCURSOR {
  unsafe { LoadCursorW(std::ptr::null_mut(), cursor.to_raw()) }
}

impl LoadCursor for CursorIcon {
  fn load(self) -> HCURSOR {
    load_cursor(self)
  }
}
