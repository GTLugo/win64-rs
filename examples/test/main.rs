use std::time::{Duration, Instant};

use win64::prelude::*;
use windows::core::HSTRING;

use self::win32::SetWindowTextW;

fn main() {
  let window = Window::new(
    WindowClass::new(&WindowClassDescriptor {
      style: WindowClassStyle::HorizontalRedraw | WindowClassStyle::VerticalRedraw,
      ..Default::default()
    }),
    WindowDescriptor {
      title: "Test".to_owned(),
      size: Some((800, 500).into()),
      style: WindowStyle::OverlappedWindow | WindowStyle::Visible,
      ..Default::default()
    },
    UserData::new(),
  )
  .unwrap();

  let mut then = Instant::now();
  let mut delta;
  let mut timer = Duration::ZERO;
  let period = Duration::from_secs_f64(0.2);

  loop {
    // match Message::get(None, None) {
    //   GetMessageResult::Message(msg) => {
    //     msg.translate();
    //     msg.dispatch();
    //   }
    //   GetMessageResult::Quit => break,
    //   GetMessageResult::Error(e) => eprintln!("ERROR: {e}"),
    // }
    match Message::peek(None, None, PeekMessageFlags::Remove) {
      PeekMessageResult::Message(msg) => {
        msg.translate();
        msg.dispatch();
      }
      PeekMessageResult::Quit => break,
      _ => (),
    }

    let now = Instant::now();
    delta = now - then;
    then = now;

    timer += delta;
    if timer > period {
      let _ = unsafe {
        SetWindowTextW(
          window.as_handle(),
          &HSTRING::from(format!("{:?} fps", 1. / (delta.as_secs_f64() * 1000.))),
        )
      };
      timer = Duration::ZERO;
    }
  }
}

struct UserData {
  
}

impl UserData {
  fn new() -> Self {
    Self {
      
    }
  }
}

impl WindowProcedure for UserData {
  fn on_message(&mut self, window: Window, message: Message) -> ProcedureResult {
    if let win32::WM_DESTROY = message.id() {
      window.quit()
    }

    self.default_window_procedure(window, message)
  }
}
