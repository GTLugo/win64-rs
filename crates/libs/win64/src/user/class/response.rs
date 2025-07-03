use windows_sys::Win32::Foundation::{self, LRESULT};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct LResult(pub isize);

impl LResult {
  pub const TRUE: Self = LResult(Foundation::TRUE as _);
  pub const FALSE: Self = LResult(Foundation::FALSE as _);

  pub fn handled() -> Option<Self> {
    Some(LResult::default())
  }
}

impl From<LResult> for LRESULT {
  fn from(value: LResult) -> Self {
    value.0
  }
}

impl From<LRESULT> for LResult {
  fn from(value: LRESULT) -> Self {
    Self(value)
  }
}
