use win64::{sys::SW_SHOW, user::*};

fn main() -> anyhow::Result<()> {
  let args = Args::get();

  let class = WindowClass::register(WindowClassStyle::empty(), args.instance, "Window");

  let hwnd = Window::new(CreateStruct {
    class,
    wnd_proc: Some(Box::new(State)),
    name: "Window".into(),
    style: WindowStyle::OverlappedWindow,
    ex_style: ExtendedWindowStyle::default(),
    position: WindowPos::Auto,
    size: WindowSize::Auto,
    parent: None,
    menu: None,
    instance: Some(args.instance),
  });

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
