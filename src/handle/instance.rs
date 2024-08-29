use windows::{
  core::{HSTRING, PCWSTR},
  Win32::{Foundation::HINSTANCE, System::LibraryLoader::GetModuleHandleW},
};

use super::{Handle, Win32Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instance;

impl Default for Handle<Instance> {
  fn default() -> Self {
    Instance::get_exe()
  }
}

impl Win32Type for Handle<Instance> {
  type Type = HINSTANCE;

  fn to_win32(&self) -> Self::Type {
    (*self).into()
  }
}

impl Instance {
  #[inline]
  pub fn get_exe() -> Handle<Self> {
    Self::get(None::<&str>).unwrap() // shouldn't fail, right?
  }

  pub fn get(
    module: Option<impl Into<String>>,
  ) -> Result<Handle<Self>, windows::core::Error> {
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
}

impl From<Handle<Instance>> for HINSTANCE {
  fn from(value: Handle<Instance>) -> Self {
    Self(value.as_ptr())
  }
}

impl From<HINSTANCE> for Handle<Instance> {
  fn from(value: HINSTANCE) -> Self {
    unsafe { Self::from_raw(value.0) }
  }
}
