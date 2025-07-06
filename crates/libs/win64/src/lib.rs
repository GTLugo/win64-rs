#![cfg(target_os = "windows")]
//!
//! Tweaks that are made should be limited to things such as:
//!
//! * Naked consts/types -> Rust new-types
//! * C consts/enums -> Rust enums
//! * Long parameter lists -> Structs
//!
//! The goal is to keep the same general API as Win32. This allows users to keep using regular guides
//! and docs with minimal resistance while also benefitting from strong type-checking and clearer APIs.
//!

pub mod core;
pub use core::*;

#[cfg(feature = "safe")]
pub mod safe;

pub mod user;

pub use dpi;
pub use keyboard_types;

pub mod error;
pub use error::*;

pub mod sys {
  pub use windows_sys::Win32::Foundation::*;
  pub use windows_sys::Win32::UI::WindowsAndMessaging::*;
}

#[cfg(all(feature = "rwh_05", not(any(feature = "rwh_06"))))]
pub use rwh_05 as raw_window_handle;

#[cfg(all(feature = "rwh_06", not(any(feature = "rwh_05"))))]
pub use rwh_06 as raw_window_handle;

pub mod prelude {
  pub use crate::{dpi::*, user::*};
}
