#![cfg(target_os = "windows")]

/*
  The goal should be to keep the same general API as Win32.

  This allows users to keep using regular guides and docs with minimal resistance.
*/

pub mod processthreadsapi;
pub mod windef;
pub mod winuser;

use std::{ffi::OsString, os::windows::ffi::OsStringExt};

pub use dpi;
pub use keyboard_types;
use windows_sys::core::{PCWSTR, PWSTR};

pub trait Handle {
  /// # Safety
  /// Reading directly from raw pointers should be handled carefully and validation performed.
  ///
  unsafe fn from_raw(raw: usize) -> Self;

  fn to_raw(self) -> usize;
}

// Adapted from ash https://docs.rs/ash/latest/src/ash/vk/macros.rs.html#121-162
#[macro_export]
macro_rules! declare_handle {
  ($name: ident, &, &) => {
    $crate::declare_handle_struct!($name, $alias, $doc_link);
    $crate::declare_handle_body!($name);
  };
  ($name: ident, $alias: meta, &) => {
    $crate::declare_handle_struct!($name, $alias, doc = "");
    $crate::declare_handle_body!($name);
  };
  ($name: ident, &, $doc_link: meta) => {
    $crate::declare_handle_struct!($name, &, $doc_link);
    $crate::declare_handle_body!($name);
  };
  ($name: ident, $alias: meta, $doc_link: meta) => {
    $crate::declare_handle_struct!($name, $alias, $doc_link);
    $crate::declare_handle_body!($name);
  };
}

#[macro_export]
macro_rules! declare_handle_struct {
  ($name: ident, &, &) => {
    declare_handle_struct!($name, &, doc = "");
  };
  ($name: ident, $alias: meta, &) => {
    declare_handle_struct!($name, $alias, doc = "");
  };
  ($name: ident, &, $doc_link: meta) => {
    #[repr(transparent)]
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
    #[$doc_link]
    pub struct $name(*mut ());
  };
  ($name: ident, $alias: meta, $doc_link: meta) => {
    #[repr(transparent)]
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
    #[doc($alias)]
    #[$doc_link]
    pub struct $name(*mut ());
  };
}

#[macro_export]
macro_rules! declare_handle_body {
  ($name: ident) => {
    impl Default for $name {
      fn default() -> Self {
        Self::null()
      }
    }
    impl Handle for $name {
      unsafe fn from_raw(raw: usize) -> Self {
        Self(raw as _)
      }
      fn to_raw(self) -> usize {
        self.0 as _
      }
    }
    unsafe impl Send for $name {}
    unsafe impl Sync for $name {}
    impl $name {
      pub const fn null() -> Self {
        Self(::core::ptr::null_mut())
      }

      pub const fn is_null(&self) -> bool {
        self.0.is_null()
      }
    }
    impl std::fmt::Pointer for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.0, f)
      }
    }
    impl std::fmt::Debug for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&(self.0 as usize), f)
      }
    }
  };
}

pub unsafe fn wcslen(s: *mut u16) -> usize {
  unsafe extern "C" {
    fn wcslen(s: *const u16) -> usize;
  }
  unsafe { wcslen(s) }
}

pub trait WStrLen {
  unsafe fn strlen(&self) -> usize;
}

impl WStrLen for PWSTR {
  unsafe fn strlen(&self) -> usize {
    unsafe { wcslen(*self) }
  }
}

impl WStrLen for PCWSTR {
  unsafe fn strlen(&self) -> usize {
    unsafe { wcslen(self.cast_mut()) }
  }
}

pub unsafe fn pwstr_to_os(s: PWSTR) -> OsString {
  OsString::from_wide(unsafe { std::slice::from_raw_parts_mut(s, s.strlen()) })
}

pub unsafe fn pcwstr_to_os(s: PCWSTR) -> OsString {
  OsString::from_wide(unsafe { std::slice::from_raw_parts(s, s.strlen()) })
}
