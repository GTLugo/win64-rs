/*
  The error strategy will be to create specific Error enums to match the output of individual functions.

  This will likely create many, many separate Errors (even after deduping), so this will be re-evaluated
  should issues arise with compile times.

  However, I firmly believe this will alleviate the pressure of porting tens of thousands of errors while
  also introducing self-documentation for errors that can actually be returned from specific functions.
*/

use std::fmt::Display;

use windows_result::HRESULT;

pub mod window;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HResult(HRESULT);

impl HResult {
  /// Returns [`true`] if `self` is a success code.
  #[inline]
  pub const fn is_ok(self) -> bool {
    self.0.is_ok()
  }

  /// Returns [`true`] if `self` is a failure code.
  #[inline]
  pub const fn is_err(self) -> bool {
    self.0.is_err()
  }

  /// Asserts that `self` is a success code.
  ///
  /// This will invoke the [`panic!`] macro if `self` is a failure code and display
  /// the [`HRESULT`] value for diagnostics.
  #[inline]
  #[track_caller]
  pub fn unwrap(self) {
    self.0.unwrap();
  }

  /// Converts the [`HRESULT`] to [`Result<()>`][Result<_>].
  #[inline]
  pub fn ok(self) -> Result<(), Error> {
    if self.0.is_ok() {
      Ok(())
    } else {
      Err(Error(windows_result::Error::from_hresult(self.0)))
    }
  }

  /// Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]
  /// converted to [`Result<T>`].
  #[inline]
  pub fn map<F, T>(self, op: F) -> Result<T, Error>
  where
    F: FnOnce() -> T,
  {
    self.ok()?;
    Ok(op())
  }

  /// Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]
  /// converted to [`Result<T>`].
  #[inline]
  pub fn and_then<F, T>(self, op: F) -> Result<T, Error>
  where
    F: FnOnce() -> Result<T, Error>,
  {
    self.ok()?;
    op()
  }

  /// The error message describing the error.
  pub fn message(self) -> String {
    self.0.message()
  }

  /// Maps a Win32 error code to an HRESULT value.
  pub const fn from_win32(error: u32) -> Self {
    Self(HRESULT::from_win32(error))
  }

  /// Maps an NT error code to an HRESULT value.
  pub const fn from_nt(error: i32) -> Self {
    Self(HRESULT::from_nt(error))
  }
}

impl Display for HResult {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[error(transparent)]
pub struct Error(windows_result::Error);
