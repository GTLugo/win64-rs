use win64::{
  flag::{WindowClassStyle, WindowStyle},
  message::Message,
  procedure::WindowProcedure,
  types::{WindowClass, WindowClassDescriptor},
  window::{Window, WindowDescriptor},
  GetMessageResult,
};
use windows::Win32::UI::WindowsAndMessaging;

fn main() {
  Window::new(
    WindowClass::new(&WindowClassDescriptor {
      style: WindowClassStyle::HorizontalRedraw | WindowClassStyle::VerticalRedraw,
      ..Default::default()
    }),
    WindowDescriptor {
      title: "Test".to_owned(),
      size: Some((800, 600).into()),
      style: WindowStyle::OverlappedWindow | WindowStyle::Visible,
      ..WindowDescriptor::new(UserData)
    },
  )
  .unwrap();

  loop {
    match Message::get(None, None) {
      GetMessageResult::Message(msg) => {
        msg.translate();
        msg.dispatch();
      }
      GetMessageResult::Quit => break,
      GetMessageResult::Error(e) => eprintln!("ERROR: {e}"),
    }
  }
}

struct UserData;

impl WindowProcedure for UserData {
  fn on_message(&mut self, window: Window, message: Message) -> win64::ProcedureResult {
    match message.id() {
      WindowsAndMessaging::WM_DESTROY => window.quit(),
      _ => {
        println!("{message:?}");
      }
    }

    self.default_window_procedure(window, message)
  }
}
