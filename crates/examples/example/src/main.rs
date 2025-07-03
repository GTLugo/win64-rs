use win64::{Error, dpi::PhysicalSize, user::*};

fn main() -> Result<(), Error> {
  let args = Args::get();

  let class = WindowClass::builder().name("Window Class").register();

  class
    .window_builder()
    .wndproc(State)
    .name("Window")
    .style(WindowStyle::OverlappedWindow | WindowStyle::Visible)
    .size(PhysicalSize::new(800, 500))
    .instance(Some(args.instance))
    .create()?;

  for msg in Msg::get(MsgQueue::CurrentThread, None).flatten() {
    msg.translate();
    msg.dispatch();
  }

  Ok(())
}

struct State;

impl WindowProcedure for State {
  fn on_message(&mut self, window: Window, message: &Message) -> Option<LResult> {
    println!("[{window:?}] {message:?}");
    None
  }
}
