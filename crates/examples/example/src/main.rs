use win64::{
  Point,
  prelude::*,
};

struct State {}

impl State {
  pub fn new() -> Self {
    Self {}
  }
}

const POINTS: &[Point] = &[Point::new(100, 250), Point::new(300, 250), Point::new(200, 100)];

impl WindowProcedure for State {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    match message {
      Message::Create(_) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        // window.dwm_set_window_attribute(DwmWindowAttribute::SystemBackdropType(SystemBackdropType::TransientWindow));
      },
      Message::SettingChange(_) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
      },
      Message::Destroy => {
        window.quit();
      },
      Message::Paint => {
        window.begin_paint(|hdc, _| {
          let brush = Brush::solid(0x000000FF);
          hdc.polygon(POINTS, &brush);
          brush.delete();
        });
      },
      Message::KeyDown(m) => {
        eprintln!("{:?}", m.event());
      },
      Message::KeyUp(m) => {
        eprintln!("{:?}", m.event());
      },
      _ => (),
    }

    None
  }
}

fn main() -> win64::Result<()> {
  let class = WindowClass::builder()
    .with_name("Window Class")
    .with_background_brush(Brush::color_window_auto_dark())
    .register()?;

  let hwnd = class
    .create_window()
    .with_procedure(State::new())
    .with_name("Window")
    .with_style(WindowStyle::OverlappedWindow)
    .with_size(Some(PhysicalSize::new(800, 500)))
    .create()?;

  hwnd.show_window(CmdShow::ShowDefault);

  // let mut counter = fps_counter::FPSCounter::new();\

  // let message_loop

  MessageLoop::new()
    .with_queue(MessageLoopQueue::Thread)
    .with_filter(None)
    .run();

  Ok(())
}
