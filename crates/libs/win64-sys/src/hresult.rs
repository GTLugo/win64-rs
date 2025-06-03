#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HResult(u32);

#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
  #[error("The operation completed successfully.")]
  Success,
  
  
}

impl TryFrom<u32> for Error {
  type Error = u32;

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::Success),
      _ => Err(value)
    }
  }
}
