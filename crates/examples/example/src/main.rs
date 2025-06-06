use win64::{StartupInfo, user::HWindow};

fn main() -> anyhow::Result<()> {
  let info = StartupInfo::get();
  eprintln!("{info:#?}");

  let hwnd = HWindow::null();
  eprintln!("HWND: {hwnd:?}");

  Ok(())
}
