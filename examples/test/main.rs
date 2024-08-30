use win64::prelude::*;

use self::fps::FPSCounter;
mod fps;

fn main() {
  let window = Window::new(
    &WindowClass::new(&WindowClassDescriptor::default()),
    WindowDescriptor::default()
      .with_title("Test")
      .with_size((800, 500)),
    WindowState::new(),
  )
  .unwrap();

  let mut fps = FPSCounter::new();

  MessagePump::wait().for_each(|_| {
    fps.update(|fps| {
      if fps.timer_up() {
        let _ =
          window.set_window_text(format!("{:?} fps", 1. / (fps.delta().as_secs_f64())));
        fps.reset_timer();
      }
    });
  });
}

struct WindowState {}

impl WindowState {
  fn new() -> Self {
    Self {}
  }
}

impl WindowProcedure for WindowState {
  fn on_message(&mut self, window: Handle<Window>, message: Message) -> ProcedureResult {
    if message.quit_requested() {
      window.quit()
    }

    window.default_procedure(message)
  }
}
