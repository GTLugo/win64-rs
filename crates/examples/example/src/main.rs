use win64::{user::{HInstance, HWindow}, StartupInfo};

fn main() -> anyhow::Result<()> {
  let info = StartupInfo::get();
  eprintln!("{info:#?}");

  let hinstance = HInstance::get();
  eprintln!("HINSTANCE: {hinstance:?}");

  let hwnd = HWindow::null();
  eprintln!("HWND: {hwnd:?}");

  Ok(())
}
