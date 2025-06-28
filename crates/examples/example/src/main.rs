use win64::user::{
  Args, CreateWindowParams, Message, WindowClass, WindowClassStyle, create_window, procedure::WindowProcedure,
};

fn main() -> anyhow::Result<()> {
  let args = Args::get();
  eprintln!("{args:#?}");

  eprintln!("HINSTANCE: {:?}", args.hinstance);

  eprintln!("msg size: {}", size_of_val(&Message::default()));

  let class = WindowClass {
    instance: args.hinstance,
    style: WindowClassStyle::empty(),
    name: "Window".into(),
  }
  .register();

  let hwnd = create_window(
    CreateWindowParams::default()
      .class(class)
      .name("Window")
      .instance(Some(args.hinstance)),
    App,
  );

  eprintln!("HWND: {hwnd:?}");

  if let Ok(hwnd) = hwnd {
    eprintln!("IsWindow: {}", unsafe { hwnd.is_window() });

    let wm_quit = Message::get(Some(hwnd), None).last();
    eprintln!("{wm_quit:?}")
  }

  Ok(())
}

struct App;

impl WindowProcedure for App {
  fn on_message(&mut self, window: win64::user::HWindow, message: &Message) -> Option<win64::user::procedure::LResult> {
    None
  }
}

/*
  TODO:
    WndClass
    Message Pump
    WndProc
*/
