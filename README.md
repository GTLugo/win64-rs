# `win64`

## An opinionated modernization of the Win32 windowing library for Rust

```rust
use win64::prelude::*;

fn main() {
  Window::new(
    &WindowClass::new(&WindowClassDescriptor::default()),
    &WindowDescriptor {
      title: "Test".to_owned(),
      size: Some((800, 500).into()),
      ..Default::default()
    },
    UserData::new(),
  )
  .unwrap();

  MessagePump::wait().run();
}

struct UserData;

impl WindowProcedure for UserData {
  fn on_message(&mut self, window: Window, message: Message) -> ProcedureResult {
    if let win32::WM_DESTROY = message.id() {
      window.quit()
    }

    self.default_window_procedure(window, message)
  }
}
```
