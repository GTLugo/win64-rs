use win64::prelude::*;

struct State;

impl WindowProcedure for State {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    match message {
      Message::Create(wm_create) => wm_create.handle(|create_struct| {
        println!("[{window:?}] Window created! {} {:?}", create_struct.name, create_struct.class);
        CreateMessageResult::Create
      }),
      Message::SettingChange(_) => {
        window.use_immersive_dark_mode(is_os_dark_mode());
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
    .procedure(State)
    .name("Window")
    .style(WindowStyle::OverlappedWindow)
    .size(PhysicalSize::new(800, 500))
    .create()?;

  hwnd.use_immersive_dark_mode(is_os_dark_mode());
  hwnd.show_window(CmdShow::ShowDefault);

  for msg in Msg::get(MsgQueue::CurrentThread, None).flatten() {
    msg.translate();
    msg.dispatch();
  }

  if !hwnd.is_window() {
    println!("[{hwnd:?}] Window destroyed!");
  }

  Ok(())
}
