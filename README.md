# `win64`

## Hand-crafted low-level Rust wrappers for Win32

The idea for this library is to offer low-level, safe(ish) wrappers for the `windows` and `windows-sys` crates.

```rs
use win64::{sys::SW_SHOW, user::*};

fn main() -> anyhow::Result<()> {
  let args = Args::get();

  let class = WindowClass::register(WindowClassStyle::empty(), args.instance, "Window Class");

  let hwnd = Window::new(CreateStruct {
    class,
    wnd_proc: Some(Box::new(State)),
    name: "Window".into(),
    style: WindowStyle::OverlappedWindow,
    ex_style: ExtendedWindowStyle::default(),
    position: WindowPos::Auto,
    size: WindowSize::Auto,
    parent: None,
    menu: None,
    instance: Some(args.instance),
  });

  if let Ok(hwnd) = hwnd {
    hwnd.show_window(SW_SHOW);

    for msg in Msg::get(MsgQueue::CurrentThread, None).flatten() {
      msg.translate();
      msg.dispatch();
    }
  }

  Ok(())
}

struct State;

impl WindowProcedure for State {
  fn on_message(&mut self, window: Window, message: &Message) -> Option<LResult> {
    println!("[{window:?}] {message:?}");
    None
  }
}

```

## Features

* `safe`: This is a set of new API wrappers that aim to provide stronger checks against improper usage

> [!NOTE]
> No AI was used to generate code.
