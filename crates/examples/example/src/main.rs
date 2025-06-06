use win64::user::{Args, HWindow};

fn main() -> anyhow::Result<()> {
  let args = Args::get();
  eprintln!("{args:#?}");

  eprintln!("HINSTANCE: {:?}", args.hinstance);

  let hwnd = HWindow::null();
  eprintln!("HWND: {hwnd:?}");

  Ok(())
}
