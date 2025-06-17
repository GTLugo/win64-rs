use win64::user::{create_window, message::Message, Args, HWindow};

fn main() -> anyhow::Result<()> {
  let args = Args::get();
  eprintln!("{args:#?}");

  eprintln!("HINSTANCE: {:?}", args.hinstance);

  let hwnd = HWindow::null();
  eprintln!("HWND: {hwnd:?}");
  eprintln!("IsWindow: {}", unsafe { hwnd.is_window() });

  let window = create_window(0, "", "Window", 0, (0, 0), (800, 500), None, None, Some(args.hinstance), None)?;

  Ok(())
}
