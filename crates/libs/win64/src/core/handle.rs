pub mod experimental {
  use std::sync::Arc;

  pub type Handle<T> = Arc<T>;

  pub enum Handle1<T> {
    Valid(T),
    Invalid,
  }
}

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
    impl $crate::core::Handle for $name {
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
        Self(std::ptr::null_mut())
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
