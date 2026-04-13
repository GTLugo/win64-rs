use win64::prelude::*;

struct State {}

impl State {
  pub fn new() -> Self {
    Self {}
  }
}

const POINTS: &[PhysicalPosition<i32>] = &[
  PhysicalPosition::new(100, 250),
  PhysicalPosition::new(300, 250),
  PhysicalPosition::new(200, 100),
];

impl WindowProcedure for State {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    match message {
      Message::Create(_) => {
        // message.use_dark_mode(window, is_os_dark_mode());
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
          let brush = Brush::solid((255, 0, 0));
          hdc.polygon(POINTS, &brush);
          brush.delete();
        });
      },
      Message::KeyDown(m) => {
        log::info!("{:?}", m.event());
      },
      Message::KeyUp(m) => {
        log::info!("{:?}", m.event());
      },
      Message::LButtonDown(m) => {
        log::info!("{:?}", m.event());
      },
      Message::LButtonUp(m) => {
        log::info!("{:?}", m.event());
      },
      Message::LButtonDblClk(m) => {
        log::info!("{:?}", m.event());
      },
      Message::RButtonDown(m) => {
        log::info!("{:?}", m.event());
      },
      Message::RButtonUp(m) => {
        log::info!("{:?}", m.event());
      },
      Message::RButtonDblClk(m) => {
        log::info!("{:?}", m.event());
      },
      _ => (),
    }

    None
  }
}

fn main() -> win64::Result<()> {
  env_logger::builder()
    .filter(None, log::LevelFilter::Trace)
    .format_source_path(true)
    .init();

  let class = WindowClass::builder()
    .with_name("Window Class")
    .with_style(WindowClassStyle::DoubleClicks)
    .with_background_brush(Brush::color_window())
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
