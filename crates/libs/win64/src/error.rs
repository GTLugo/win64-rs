/*
  The error strategy will be to create specific Error enums to match the output of individual functions.

  This will likely create many, many separate Errors (even after deduping), so this will be re-evaluated
  should issues arise with compile times.

  However, I firmly believe this will alleviate the pressure of porting tens of thousands of errors while
  also introducing self-documentation for errors that can actually be returned from specific functions.
*/

pub use windows_result::HRESULT as HResult;

pub mod window;
