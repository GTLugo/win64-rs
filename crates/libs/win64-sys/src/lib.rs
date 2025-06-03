#![cfg(target_os = "windows")]

/*
  The goal should be to keep the same general API as Win32.

  This allows users to keep using regular guides and docs with minimal resistance.
*/

pub mod window;

pub use dpi;
pub use keyboard_types;

pub trait Handle {
  fn from_raw(raw: usize) -> Self;

  fn to_raw(self) -> usize;
}
