use win64::prelude::*;

mod fps;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let class = WindowClass::default();

  class.spawn(WindowDescriptor::default().with_title("Test").with_size((800, 500)), App::new())?;

  MessagePump::default().with_mode(PollingMode::Poll).run();

  Ok(())
}

struct App {}

impl App {
  fn new() -> Self {
    Self {}
  }
}

impl WindowProcedure for App {
  fn on_message(&mut self, window: WindowId, message: Message) -> ProcedureResult {
    match message {
      Message::CloseRequested => window.quit(),
      _ => {
        println!("{message:?}");
      }
    }

    window.default_procedure(message)
  }
}
