pub mod hwnd;
pub use hwnd::*;

#[cfg(any(feature = "rwh_06", feature = "rwh_05"))]
pub mod rwh;

#[cfg(any(feature = "rwh_06", feature = "rwh_05"))]
#[allow(unused)]
pub use rwh::*;

pub mod hinstance;
pub use hinstance::*;

pub mod class;
pub use class::*;

pub mod winmain;
pub use winmain::*;

pub mod message;
pub use message::*;

pub mod point;
pub use point::*;

pub mod flags;
pub use flags::*;

pub mod cursor;
pub use cursor::*;

pub fn is_os_dark_mode() -> bool {
  // based on https://stackoverflow.com/a/70753913

  let key = windows_registry::CURRENT_USER
    .open(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize")
    .expect("failed to open registry key");

  let light_theme = key
    .get_u32("AppsUseLightTheme")
    .expect("failed to read value from registry key");

  light_theme == 0
}
