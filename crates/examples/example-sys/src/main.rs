use win64_sys::{Handle, processthreadsapi::StartupInfo, windef::HWindow};

fn main() -> anyhow::Result<()> {
  let info = StartupInfo::get();
  println!("{info:?}");
  let hwnd = unsafe { HWindow::from_raw(11) };
  println!("Is valid? {}", unsafe { hwnd.is_window() });
  Ok(())
}
