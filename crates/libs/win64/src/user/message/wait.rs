use {
  crate::{
    core::handle::Handle,
    declare_handle,
  },
  windows_sys::Win32::{
    Foundation::{
      CloseHandle,
      WAIT_OBJECT_0,
    },
    System::Threading::{
      self,
      CreateEventW,
      SetEvent,
    },
    UI::WindowsAndMessaging::{
      self,
      MsgWaitForMultipleObjectsEx,
    },
  },
};

declare_handle!(
  Event,
  alias = "HANDLE",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#handle"
);

impl Event {
  pub fn new() -> Self {
    let event = unsafe { CreateEventW(std::ptr::null(), 0, 0, std::ptr::null()) };
    unsafe { Self::from_ptr(event) }
  }

  pub fn set(&self) {
    unsafe { SetEvent(self.to_ptr()) };
  }

  // TODO: RAII
  pub fn close(&self) {
    unsafe { CloseHandle(self.to_ptr()) };
  }
}

// TODO: Make this arbitrary for HANDLES rather than events so it matches the original API. Add the other params
pub fn msg_wait_for_multiple_events(events: &[Event]) -> Option<()> {
  let result = unsafe {
    MsgWaitForMultipleObjectsEx(
      events.len() as u32,
      events.as_ptr().cast(),
      Threading::INFINITE,
      WindowsAndMessaging::QS_SENDMESSAGE,
      0,
    )
  };

  (result == WAIT_OBJECT_0 + events.len() as u32).then_some(())
}
