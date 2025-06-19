use win64::user::{Args, CreateWindowParams, HWindow, create_window, message::Message};

fn main() -> anyhow::Result<()> {
  let args = Args::get();
  eprintln!("{args:#?}");

  eprintln!("HINSTANCE: {:?}", args.hinstance);

  eprintln!("msg size: {}", size_of_val(&Message::Null));

  let hwnd = HWindow::null();
  eprintln!("HWND: {hwnd:?}");
  eprintln!("IsWindow: {}", unsafe { hwnd.is_window() });

  let window = create_window(
    CreateWindowParams::default()
      .window_name("Window")
      .instance(Some(args.hinstance)),
  );

  eprintln!("IsWindow: {:?}", window);

  Ok(())
}
