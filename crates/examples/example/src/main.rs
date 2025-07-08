use win64::{Point, prelude::*};

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
      Message::Create(_) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        window.dwm_set_window_attribute(DwmWindowAttribute::SystemBackdropType(SystemBackdropType::TransientWindow));
      }
      Message::SettingChange(_) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
      }
      Message::Destroy => {
        window.quit();
      }
      Message::Paint => {
        window.begin_paint(|hdc, ps| {
          let brush = Brush::solid(0x000000FF);
          const POINTS: &[Point] = &[Point::new(100, 250), Point::new(300, 250), Point::new(200, 100)];
          hdc.polygon(POINTS, &brush);
          brush.delete();
        });
      }
      _ => (),
    }

    None
  }
}

fn main() -> win64::Result<()> {
  let class = WindowClass::builder()
    .name("Window Class")
    .background_brush(Brush::color_window_auto_dark())
    .register()?;

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
