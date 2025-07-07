use win64::prelude::*;

struct State {}

impl State {
  pub fn new() -> Self {
    Self {}
  }
}

impl WindowProcedure for State {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    println!("[{window:?}] {message:?}");

    match message {
      Message::Create(_) | Message::SettingChange(_) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        None
      }
      Message::Destroy => {
        window.quit();
        None
      }
      Message::Paint => {
        window.begin_paint(|hdc, ps| {
          hdc.fill_rect(ps.paint, Brush::solid(0x2C2020));
        });
        None
      }
      _ => None,
    }
  }
}

fn main() -> win64::Result<()> {
  let class = WindowClass::builder().name("Window Class").register()?;

  let hwnd = class
    .window_builder()
    .procedure(State::new())
    .name("Window")
    .style(WindowStyle::OverlappedWindow)
    .size(PhysicalSize::new(800, 500))
    .create()?;

  hwnd.show_window(CmdShow::ShowDefault);

  // let mut counter = fps_counter::FPSCounter::new();

  for msg in Msg::peek(MsgQueue::CurrentThread, None, PeekMessageFlags::Remove) {
    if let Some(msg) = msg.ok() {
      msg.translate();
      msg.dispatch();
    }

    // let fps = counter.tick();
    // hwnd.set_window_text(format!("WINDOW | {fps}"))?;
  }

  Ok(())
}
