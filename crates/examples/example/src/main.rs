use win64::{prelude::*, sys::Dwm::{DWMSBT_MAINWINDOW, DWMSBT_TRANSIENTWINDOW}};

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
        window.dwm_set_window_attribute(DwmWindowAttribute::SystemBackdropType(DWMSBT_TRANSIENTWINDOW));
        // window.set_acrylic_background(if is_os_dark_mode() { 0x99202020 } else { 0x99D0D0D0 });
        window.extend_into_client_all();
      }
      Message::Destroy => {
        window.quit();
      }
      Message::Paint => {
        // window.begin_paint(|hdc, ps| {
        //   // hdc.fill_rect(ps.paint, Brush::color_window());
        // });
      }
      _ => (),
    }

    None
  }
}

fn main() -> win64::Result<()> {
  let class = WindowClass::builder()
    .name("Window Class")
    .background_brush(Brush::color_background())
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
