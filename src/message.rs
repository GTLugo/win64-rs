use std::{marker::PhantomData, ops::RangeInclusive};

use windows::Win32::{
  Foundation::{self, LPARAM, WPARAM},
  UI::WindowsAndMessaging::{self, DispatchMessageW, TranslateMessage, MSG},
};

use crate::{
  flag::PeekMessageFlags,
  get_message,
  handle::{window::Window, Handle, Win32Type},
  peek_message, GetMessageResult, PeekMessageResult, ProcedureResult,
};

// pub enum PumpType {
//   Wait,
//   Poll,
// }

pub struct Wait;
pub struct Poll;

pub struct MessagePump<PumpType> {
  ty: PhantomData<PumpType>,
  hwnd: Option<Handle<Window>>,
  filter: Option<RangeInclusive<u32>>,
  flags: PeekMessageFlags,
  translate: bool,
}

impl MessagePump<Wait> {
  pub fn wait() -> Self {
    Self {
      ty: PhantomData,
      hwnd: None,
      filter: None,
      flags: PeekMessageFlags::empty(),
      translate: true,
    }
  }

  pub fn with_window(self, window: Handle<Window>) -> Self {
    Self {
      hwnd: Some(window),
      ..self
    }
  }

  pub fn with_filter(self, filter: RangeInclusive<u32>) -> Self {
    Self {
      filter: Some(filter),
      ..self
    }
  }

  pub fn with_translation(self, enable: bool) -> Self {
    Self {
      translate: enable,
      ..self
    }
  }

  /// returns false when Quit message is sent
  pub fn run_once(&self) -> bool {
    match get_message(self.hwnd, &self.filter) {
      GetMessageResult::Message(msg) => {
        if self.translate {
          msg.translate();
        }
        msg.dispatch();
      }
      GetMessageResult::Quit => return false,
      GetMessageResult::Error(e) => tracing::error!("{e}"),
    }

    true
  }

  pub fn run(&self) {
    while self.run_once() {}
  }

  pub fn for_each(&self, mut f: impl FnMut(Message<Metadata>)) {
    loop {
      match get_message(self.hwnd, &self.filter) {
        GetMessageResult::Message(msg) => {
          if self.translate {
            msg.translate();
          }
          msg.dispatch();
          f(msg);
        }
        GetMessageResult::Quit => break,
        GetMessageResult::Error(e) => tracing::error!("{e}"),
      }
    }
  }
}

impl MessagePump<Poll> {
  pub fn poll() -> Self {
    Self {
      ty: PhantomData,
      hwnd: None,
      filter: None,
      flags: PeekMessageFlags::Remove,
      translate: true,
    }
  }

  pub fn with_window(self, window: Handle<Window>) -> Self {
    Self {
      hwnd: Some(window),
      ..self
    }
  }

  pub fn with_filter(self, filter: RangeInclusive<u32>) -> Self {
    Self {
      filter: Some(filter),
      ..self
    }
  }

  pub fn with_flags(self, flags: PeekMessageFlags) -> Self {
    Self { flags, ..self }
  }

  pub fn with_translation(self, enable: bool) -> Self {
    Self {
      translate: enable,
      ..self
    }
  }

  /// returns false when Quit message is sent
  pub fn run_once(&self) -> bool {
    match peek_message(self.hwnd, &self.filter, self.flags) {
      PeekMessageResult::Message(msg) => {
        if self.translate {
          msg.translate();
        }
        msg.dispatch();
      }
      PeekMessageResult::Quit => return false,
      PeekMessageResult::None => (),
    }

    true
  }

  pub fn run(&self) {
    while self.run_once() {}
  }

  pub fn for_each(&self, mut f: impl FnMut(Option<Message<Metadata>>)) {
    loop {
      match peek_message(self.hwnd, &self.filter, self.flags) {
        PeekMessageResult::Message(msg) => {
          if self.translate {
            msg.translate();
          }
          msg.dispatch();
          f(Some(msg));
        }
        PeekMessageResult::Quit => break,
        PeekMessageResult::None => f(None),
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
  hwnd: Handle<Window>,
  time: u32,
  pt: Foundation::POINT,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoMetadata;

#[derive(Debug, Clone, PartialEq)]
pub struct Message<M = NoMetadata> {
  id: u32,
  w: usize,
  l: isize,
  metadata: M,
}

// make new() method for both With and Without and getters for the fields

impl From<MSG> for Message<Metadata> {
  fn from(value: MSG) -> Self {
    Self {
      id: value.message,
      w: value.wParam.0,
      l: value.lParam.0,
      metadata: Metadata {
        hwnd: value.hwnd.into(),
        time: value.time,
        pt: value.pt,
      },
    }
  }
}

impl From<Message<Metadata>> for MSG {
  fn from(msg: Message<Metadata>) -> Self {
    Self {
      hwnd: msg.window().to_win32(),
      message: msg.id(),
      wParam: WPARAM(msg.w()),
      lParam: LPARAM(msg.l()),
      time: msg.time(),
      pt: *msg.point(),
    }
  }
}

impl<M> Message<M> {
  pub const QUIT: u32 = WindowsAndMessaging::WM_QUIT;
  pub const DESTROY: u32 = WindowsAndMessaging::WM_DESTROY;

  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn w(&self) -> usize {
    self.w
  }

  pub fn l(&self) -> isize {
    self.l
  }

  pub fn quit_requested(&self) -> bool {
    self.id == Self::DESTROY
  }
}

impl Message<NoMetadata> {
  pub fn new(msg: u32, w: WPARAM, l: LPARAM) -> Self {
    Self {
      id: msg,
      w: w.0,
      l: l.0,
      metadata: NoMetadata,
    }
  }
}

impl Message<Metadata> {
  pub fn get(
    hwnd: Option<Handle<Window>>,
    filter: Option<RangeInclusive<u32>>,
  ) -> GetMessageResult {
    get_message(hwnd, &filter)
  }

  pub fn peek(
    hwnd: Option<Handle<Window>>,
    filter: Option<RangeInclusive<u32>>,
    flags: PeekMessageFlags,
  ) -> PeekMessageResult {
    peek_message(hwnd, &filter, flags)
  }

  pub fn translate(&self) -> bool {
    let msg = MSG::from(self.clone());
    unsafe { TranslateMessage(&msg) }.as_bool()
  }

  pub fn dispatch(&self) -> ProcedureResult {
    let msg = MSG::from(self.clone());
    unsafe { DispatchMessageW(&msg) }.into()
  }

  pub fn window(&self) -> &Handle<Window> {
    &self.metadata.hwnd
  }

  pub fn time(&self) -> u32 {
    self.metadata.time
  }

  pub fn point(&self) -> &Foundation::POINT {
    &self.metadata.pt
  }
}
