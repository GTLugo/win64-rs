use win64::user::{Args, HWindow, create_window, message::Message};

fn main() -> anyhow::Result<()> {
  let args = Args::get();
  eprintln!("{args:#?}");

  eprintln!("HINSTANCE: {:?}", args.hinstance);

  println!("msg size: {}", size_of_val(&Message::Null));

  let hwnd = HWindow::null();
  eprintln!("HWND: {hwnd:?}");
  eprintln!("IsWindow: {}", unsafe { hwnd.is_window() });

  let window = create_window(0, "", "Window", 0, (None, None), (None, None), None, None, Some(args.hinstance), None)?;

  Ok(())
}
