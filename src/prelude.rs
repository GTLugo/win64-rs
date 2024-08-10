pub use crate::{
  flag::{WindowClassStyle, WindowStyle},
  message::Message,
  procedure::WindowProcedure,
  types::{WindowClass, WindowClassDescriptor},
  window::{Window, WindowDescriptor},
  GetMessageResult, ProcedureResult,
};
pub use windows::Win32::UI::WindowsAndMessaging as win32;
