use std::ptr::NonNull;

use windows::{
  core::{HSTRING, PCWSTR},
  Win32::{Foundation::HINSTANCE, System::LibraryLoader::GetModuleHandleW},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Handle(Option<NonNull<usize>>);

impl Handle {
  pub fn as_ptr(&self) -> *mut core::ffi::c_void {
    self
      .0
      .map_or(core::ptr::null_mut(), |ptr| ptr.cast::<core::ffi::c_void>().as_ptr())
  }

  pub fn from_raw(ptr: *mut core::ffi::c_void) -> Self {
    Self(NonNull::new(ptr.cast()))
  }

  pub fn is_null(&self) -> bool {
    self.0.is_none()
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instance(Handle);

impl Instance {
  #[inline]
  pub fn get_exe() -> Result<Self, windows::core::Error> {
    Self::get(None::<&str>)
  }

  pub fn get(module: Option<impl Into<String>>) -> Result<Self, windows::core::Error> {
    unsafe {
      GetModuleHandleW(
        module
          .map(Into::into)
          .map(HSTRING::from)
          .map_or_else(PCWSTR::null, |n| PCWSTR(n.as_ptr())),
      )
    }
    .map(|value| HINSTANCE(value.0).into())
  }

  pub fn is_null(&self) -> bool {
    self.0.is_null()
  }
}

impl From<Instance> for HINSTANCE {
  fn from(value: Instance) -> Self {
    Self(value.0.as_ptr())
  }
}

impl From<HINSTANCE> for Instance {
  fn from(value: HINSTANCE) -> Self {
    Self(Handle::from_raw(value.0))
  }
}
