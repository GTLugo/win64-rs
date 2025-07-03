# `win64`

## Hand-crafted idiomatic Rust wrappers for Win32

The idea for this library is to offer safer wrappers for the `windows` and `windows-sys` crates that offer low-level control while also helping to prevent invalid states.

```rs
use win64::{Error, dpi::PhysicalSize, user::*};

struct State;

impl WindowProcedure for State {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    match message {
      Message::Create(wm_create) => wm_create.handle(|create_struct| CreateMessageResult::Create),
      _ => {
        println!("[{window:?}] {message:?}");
        None
      }
    }
  }
}

fn main() -> Result<(), Error> {
  let args = Args::get();

  let class = WindowClass::builder().name("Window Class").register();

  class
    .window_builder()
    .procedure(State)
    .name("Window")
    .style(WindowStyle::OverlappedWindow | WindowStyle::Visible)
    .size(PhysicalSize::new(800, 500))
    .instance(Some(args.instance))
    .create()?;

  for msg in Msg::get(MsgQueue::CurrentThread, None).flatten() {
    msg.translate();
    msg.dispatch();
  }

  Ok(())
}
```

## Features

* `safe`: This is a set of new API wrappers that aim to provide stronger checks against improper usage

> [!NOTE]
> No AI-generated code.
