use win64::{windef::HWindow, StartupInfo};

fn main() -> anyhow::Result<()> {
  let info = StartupInfo::get();
  eprintln!("{info:?}");

  let hwnd = HWindow::null();
  
  Ok(())
}
