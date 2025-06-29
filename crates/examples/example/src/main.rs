use win64::{
  sys::SW_SHOW,
  user::{
    Args, CreateWindowParams, HWindow, LResult, Message, Msg, MsgQueue, WindowClass, WindowClassStyle, WindowProcedure,
    WindowStyle, create_window,
  },
};

fn main() -> anyhow::Result<()> {
  let args = Args::get();

  let class = WindowClass::new(WindowClassStyle::empty(), args.hinstance, "Window").register();

  let hwnd = create_window(
    CreateWindowParams {
      class,
      name: "Window".into(),
      style: WindowStyle::OverlappedWindow,
      instance: Some(args.hinstance),
      ..Default::default()
    },
    App,
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

struct App;

impl WindowProcedure for App {
  fn on_message(&mut self, window: HWindow, message: &Message) -> Option<LResult> {
    println!("[{window:?}] {message:?}");
    None
  }
}
