use win64::prelude::*;

struct State {}

impl State {
  pub fn new() -> Self {
    Self {}
  }
}

impl WindowProcedure for State {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    match message {
      Message::Create(_) | Message::SettingChange(_) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        None
      }
      Message::Paint => {
        window.begin_paint(|ps| {
          ps.hdc.fill_rect_color_window(ps.paint);
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

  for msg in Msg::get(MsgQueue::CurrentThread, None).flatten() {
    msg.translate();
    msg.dispatch();
  }

  Ok(())
}
