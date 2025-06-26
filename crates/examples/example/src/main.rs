use win64::user::{Args, CreateWindowParams, Message, create_window};

fn main() -> anyhow::Result<()> {
  let args = Args::get();
  eprintln!("{args:#?}");

  eprintln!("HINSTANCE: {:?}", args.hinstance);

  eprintln!("msg size: {}", size_of_val(&Message::default()));

  let hwnd = create_window(
    CreateWindowParams::default()
      .window_name("Window")
      .instance(Some(args.hinstance)),
  );

  eprintln!("HWND: {hwnd:?}");

  if let Ok(hwnd) = hwnd {
    eprintln!("IsWindow: {}", unsafe { hwnd.is_window() });

    let wm_quit = Message::get(Some(hwnd), None).last();
    eprintln!("{wm_quit:?}")
  }

  Ok(())
}

/*
  TODO:
    WndClass
    Message Pump
    WndProc
*/
