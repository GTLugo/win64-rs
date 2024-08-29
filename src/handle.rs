use std::marker::PhantomData;

pub mod instance;
pub mod window;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Handle<T> {
  ptr: Option<*mut usize>,
  _tag: PhantomData<T>,
}

impl<T> Handle<T> {
  pub fn as_ptr(&self) -> *mut core::ffi::c_void {
    self
      .ptr
      .map_or(core::ptr::null_mut(), |ptr| ptr.cast::<core::ffi::c_void>())
  }

  /// # Safety
  /// This function takes in a raw c void pointer and assumes it can be converted to an Option<*mut usize> that is compatible with whatever handle it is assumed to be. Null pointers are converted to None values.
  pub unsafe fn from_raw(ptr: *mut core::ffi::c_void) -> Self {
    let ptr: *mut usize = ptr.cast();
    Self {
      ptr: match ptr.is_null() {
        true => None,
        false => Some(ptr),
      },
      _tag: PhantomData,
    }
  }

  pub fn is_null(&self) -> bool {
    self.ptr.is_none()
  }

  pub fn as_isize(&self) -> Option<*mut isize> {
    self.ptr.map(|p| p.cast())
  }
}

pub trait Win32Type {
  type Type;

  fn to_win32(&self) -> Self::Type;
}
