# `win64`

## An opinionated modernization of the Win32 windowing library for Rust

```rust
use win64::prelude::*;

fn main() {
  Window::new(
    &WindowClass::new(&WindowClassDescriptor::default()),
    WindowDescriptor::default()
      .with_title("Test")
      .with_size((800, 500)),
    WindowState,
  )
  .unwrap();

  MessagePump::wait().run();
}

struct WindowState;

impl WindowProcedure for WindowState {
  fn on_message(&mut self, window: Handle<Window>, message: Message) -> ProcedureResult {
    if message.quit_requested() {
      window.quit()
    }

    window.default_procedure(message)
  }
}
```
