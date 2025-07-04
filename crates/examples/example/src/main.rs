use win64::{Error, dpi::PhysicalSize, user::*};

struct State;

impl WindowProcedure for State {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    match message {
      Message::Create(wm_create) => wm_create.handle(|_create_struct| {
        println!("[{window:?}] Window created!");
        CreateMessageResult::Create
      }),
      Message::Paint => {
        window.set_window_text("Uhhh, why"); // So this works now...
        println!("[{window:?} | {}] {message:?}", window.get_window_text().unwrap()); // but this still doesn't...
        None
      }
      _ => None,
    }
  }
}

fn main() -> Result<(), Error> {
  let args = Args::get();

  let class = WindowClass::builder().name("Window Class").register();

  let hwnd = class
    .window_builder()
    .procedure(State)
    .name("Window")
    .style(WindowStyle::OverlappedWindow | WindowStyle::Visible)
    .size(PhysicalSize::new(800, 500))
    .instance(Some(args.instance))
    .create()?;

  for msg in Msg::get(MsgQueue::CurrentThread, None).flatten() {
    msg.translate();
    msg.dispatch();
  }

  if !unsafe { hwnd.is_window() } {
    println!("[{hwnd:?}] Window destroyed!");
  }

  Ok(())
}
