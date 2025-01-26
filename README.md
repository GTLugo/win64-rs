# `win64`

## An opinionated modernization of the Win32 windowing API for Rust

```rust
// WindowsAndMessaging from windows-rs is re-exported as `win32` for grabbing any unimplemented flags
use win64::prelude::*;

fn main() {
  let class = WindowClass::default();

  class.spawn(
    WindowDescriptor::default()
      .with_title("Test")
      .with_size((800, 500)),
    WindowState,
  )
  .unwrap();

  MessagePump::default().run();
}

struct WindowState;

impl WindowProcedure for WindowState {
  fn on_message(&mut self, window: WindowId, message: Message) -> ProcedureResult {
    if let Message::CloseRequested = message {
      window.quit();
    }

    window.default_procedure(message)
  }
}
```

## Version Warning

`win64` is still in its infancy and as such is currently targeting whatever is the latest version of Rust. This means you may run into many breaking changes and/or require updating your local toolchain in order to use it. Only once the API is close to major version 1.0 will a minimum version of Rust be established.

## Credit

`win64` stands upon the shoulders of giants. It takes heavy inspiration from works such as `piston` and `winit`, in some cases incorporating code directly from them. In such cases, I have tried to take care to document what was taken, but please file an issue if I have missed any citations!
