use win64::{
  convert_error,
  user::{Args, CreateWindowError, CreateWindowParams, create_window, message::Message},
};

fn main() -> anyhow::Result<()> {
  let args = Args::get();
  eprintln!("{args:#?}");

  eprintln!("HINSTANCE: {:?}", args.hinstance);

  eprintln!("msg size: {}", size_of_val(&Message::Null));

  let hwnd = create_window(
    CreateWindowParams::default()
      .window_name("Window")
      .instance(Some(args.hinstance)),
  );

  eprintln!("HWND: {hwnd:?}");

  if let Ok(hwnd) = hwnd {
    eprintln!("IsWindow: {}", unsafe { hwnd.is_window() });
  }

  Ok(())
}
