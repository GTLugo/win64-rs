use win64::{
  sys::SW_SHOW,
  user::{
    Args, CreateWindowParams, HWindow, LResult, Message, WindowClass, WindowClassStyle, WindowProcedure, WindowStyle,
    create_window,
  },
};

fn main() -> anyhow::Result<()> {
  let args = Args::get();
  eprintln!("{args:#?}");

  eprintln!("HINSTANCE: {:?}", args.hinstance);

  eprintln!("msg size: {}", size_of_val(&Message::default()));

  let class = WindowClass::Local {
    instance: args.hinstance,
    name: "Window".into(),
    style: WindowClassStyle::empty(),
  };
  class.register();

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

  eprintln!("HWND: {hwnd:?}");

  if let Ok(hwnd) = hwnd {
    eprintln!("IsWindow: {}", unsafe { hwnd.is_window() });

    hwnd.show_window(SW_SHOW);

    for msg in Message::get(None, None).flatten() {
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

/*
  TODO:
    WndClass
    Message Pump
    WndProc
*/
