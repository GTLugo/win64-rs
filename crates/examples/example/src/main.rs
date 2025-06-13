use win64::user::{Args, CreateWindowInfo, HWindow};

fn main() -> anyhow::Result<()> {
  let args = Args::get();
  eprintln!("{args:#?}");

  eprintln!("HINSTANCE: {:?}", args.hinstance);

  let hwnd = HWindow::null();
  eprintln!("HWND: {hwnd:?}");
  eprintln!("IsWindow: {}", unsafe { hwnd.is_window() });

  let window = HWindow::new(&CreateWindowInfo {})?;

  Ok(())
}
