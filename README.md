# `win64`

## Hand-crafted, idiomatic Rust wrappers for Win32

The idea for this library is to offer safer wrappers for the `windows` and `windows-sys` crates that offer low-level control while also helping to prevent invalid states.

```rs
use win64::prelude::*;

struct State;

impl WindowProcedure for State {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    println!("[{window:?}] {message:?}");
    None
  }
}

fn main() -> Result<(), Error> {
  let args = Args::get();

  let class = WindowClass::builder()
    .name("Window Class")
    .instance(Some(args.instance))
    .register()?;

  let hwnd = class
    .window_builder()
    .procedure(State)
    .name("Window")
    .style(WindowStyle::OverlappedWindow | WindowStyle::Visible)
    .size(PhysicalSize::new(800, 500))
    .create()?;

  hwnd.use_immersive_dark_mode(is_os_dark_mode());

  for msg in Msg::get(MsgQueue::CurrentThread, None).flatten() {
    msg.translate();
    msg.dispatch();
  }

  Ok(())
}
```

> [!WARNING]
> While efforts are put in place to maintain compatibility, this crate is largely untested on versions of Windows older than Windows 11. Here be dragons.

## Features

* `safe`: This is a set of new API wrappers that aim to provide stronger checks against improper usage
* `rwh_05 / rwh_06`: Implements the raw_window_handle traits on the window handle type.

> [!NOTE]
> No AI-generated code.
