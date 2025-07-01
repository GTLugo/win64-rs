use win64::{sys::SW_SHOW, user::*};

fn main() -> anyhow::Result<()> {
  let args = Args::get();

  let class = WindowClass::builder().name("Window Class").register();

  let hwnd = class
    .window()
    .wndproc(State)
    .name("Window")
    .style(WindowStyle::OverlappedWindow)
    .instance(Some(args.instance))
    .create();

  if let Ok(hwnd) = hwnd {
    hwnd.show_window(SW_SHOW);

    for msg in Msg::get(MsgQueue::CurrentThread, None).flatten() {
      msg.translate();
      msg.dispatch();
    }
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
