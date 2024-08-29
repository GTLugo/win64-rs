use std::time::{Duration, Instant};

use win64::prelude::*;

fn main() {
  // let (sender, receiver) = std::sync::mpsc::channel::<Message>();

  let window = Window::new(
    &WindowClass::new(&WindowClassDescriptor::default()),
    &WindowDescriptor::default()
      .with_title("Test")
      .with_size((800, 500)),
    UserData::new(/*sender*/),
  )
  .unwrap();

  let mut then = Instant::now();
  let mut delta = Duration::ZERO;
  let mut timer = Duration::ZERO;
  let period = Duration::from_secs_f64(0.2);

  // let pump = MessagePump::wait();
  // while pump.run_once() {
  //   match receiver.recv() {
  //     Ok(msg) => {
  //       println!("{msg:?}");
  //     }
  //     Err(std::sync::mpsc::RecvError) => break,
  //   }
  // }

  MessagePump::wait().for_each(|_| {
    let now = Instant::now();
    delta = now - then;
    then = now;
    timer += delta;
    if timer > period {
      let _ = window.set_window_text(format!("{:?} fps", 1. / (delta.as_secs_f64())));
      timer = Duration::ZERO;
    }
  });
}

struct UserData {
  // sender: std::sync::mpsc::Sender<Message>,
}

impl UserData {
  fn new(/*sender: Sender<Message>*/) -> Self {
    Self { /*sender*/ }
  }
}

impl WindowProcedure for UserData {
  fn on_message(&mut self, window: Handle<Window>, message: Message) -> ProcedureResult {
    // if self.sender.send(message.clone()).is_err() {
    //   eprintln!("Failed to send message: {message:?}");
    // }

    if message.quit_requested() {
      window.quit()
    }

    self.default_window_procedure(window, message)
  }
}
