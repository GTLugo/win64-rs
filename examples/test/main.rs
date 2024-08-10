use win64::prelude::*;

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
      ..Default::default()
    },
    UserData,
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
  fn on_message(&mut self, window: Window, message: Message) -> ProcedureResult {
    match message.id() {
      win32::WM_DESTROY => window.quit(),
      _ => {
        println!("{message:?}");
      }
    }

    self.default_window_procedure(window, message)
  }
}
