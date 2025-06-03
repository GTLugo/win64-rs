use win64_sys::{Handle, windef::Window};

fn main() -> anyhow::Result<()> {
  let hwnd = unsafe { Window::from_raw(11) };
  println!("Is valid? {}", unsafe { hwnd.is_window() });
  Ok(())
}
