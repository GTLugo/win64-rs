use win64::{
  sys::SW_SHOW,
  user::{
    Args, ExtendedWindowStyle, HWindow, LResult, Message, Msg, MsgQueue, WindowClass,
    WindowClassStyle, WindowPos, WindowProcedure, WindowSize, WindowStyle, create_window,
  },
};

fn main() -> anyhow::Result<()> {
  let args = Args::get();

  let class = WindowClass::register(WindowClassStyle::empty(), args.hinstance, "Window");

  let hwnd = create_window(
    ExtendedWindowStyle::default(),
    class,
    "Window".into(),
    WindowStyle::OverlappedWindow,
    WindowPos::Auto,
    WindowSize::Auto,
    None,
    None,
    Some(args.hinstance),
    State,
  );

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
  fn on_message(&mut self, window: HWindow, message: &Message) -> Option<LResult> {
    println!("[{window:?}] {message:?}");
    // window.destroy();
    None
  }
}
