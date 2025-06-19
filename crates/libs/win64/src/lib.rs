#![cfg(target_os = "windows")]
//! 
//! Tweaks that are made should be limited to things such as:
//! 
//! * Naked consts/types -> Rust new-types
//! * C consts/enums -> Rust enums
//! * Long parameter lists -> Structs
//! 
//! The goal is to keep the same general API as Win32. This allows users to keep using regular guides
//!  and docs with minimal resistance while also benefitting from strong type-checking and clearer APIs.
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
