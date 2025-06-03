use win64_sys::{Handle, windef::WindowId};

fn main() -> anyhow::Result<()> {
  let hwnd = unsafe { WindowId::from_raw(11) };
  println!("Is valid? {}", unsafe { hwnd.is_window() });
  Ok(())
}
