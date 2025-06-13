#![cfg(target_os = "windows")]

/*
  The goal should be to keep the same general API as Win32.

  This allows users to keep using regular guides and docs with minimal resistance.
*/

pub mod core;
pub use core::*;

#[cfg(feature = "safe")]
pub mod safe;

pub mod user;

pub use dpi;
pub use keyboard_types;

pub mod error;
pub use error::*;
