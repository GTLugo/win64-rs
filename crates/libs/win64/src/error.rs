/*
  The error strategy will be to create specific Error enums to match the output of individual functions.

  This will likely create many, many separate Errors (even after deduping), so this will be re-evaluated
  should issues arise with compile times.

  However, I firmly believe this will alleviate the pressure of porting tens of thousands of errors while
  also introducing self-documentation for errors that can actually be returned from specific functions.
*/

pub mod window;

// Should the need arise for extensions, newtypes should be favored over ext traits.
pub use windows_result::Error;
pub use windows_result::HRESULT as HResult;

use windows_sys::Win32::Foundation::WIN32_ERROR;

#[inline]
pub fn convert_error(error: WIN32_ERROR) -> Error {
  crate::Error::from_hresult(crate::HResult::from_win32(error))
}

#[inline]
pub fn get_last_error() -> Error {
  Error::from_win32()
}
