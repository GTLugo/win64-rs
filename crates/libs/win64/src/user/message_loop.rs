use {
  super::{
    MessageLoopQueue,
    Msg,
    PeekMessageFlags,
  },
  crate::user::PeekResult,
  std::ops::RangeInclusive,
  windows_result::Error,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MessageLoop {
  queue: MessageLoopQueue,
  filter: Option<RangeInclusive<u32>>,
  peek: Option<PeekMessageFlags>,
}

impl Default for MessageLoop {
  fn default() -> Self {
    Self::new()
  }
}

impl MessageLoop {
  pub fn new() -> MessageLoop {
    Self { queue: MessageLoopQueue::default(), filter: None, peek: None }
  }

  pub fn with_queue(mut self, queue: MessageLoopQueue) -> Self {
    self.queue = queue;
    self
  }

  pub fn with_filter(mut self, filter: Option<RangeInclusive<u32>>) -> Self {
    self.filter = filter;
    self
  }

  pub fn with_peek(mut self, flags: Option<PeekMessageFlags>) -> Self {
    self.peek = flags;
    self
  }

  pub fn run(self) {
    if self.peek.is_some() {
      self.peek(|msg| {
        if let MessageLoopResult::Peek(Some(msg)) = msg {
          msg.translate();
          msg.dispatch();
        }
      });
    } else {
      self.get(|msg| {
        if let MessageLoopResult::Get(msg) = msg {
          msg.translate();
          msg.dispatch();
        }
      });
    }
  }

  pub fn run_with(self, f: impl Fn(MessageLoopResult)) {
    if self.peek.is_some() {
      self.peek(f);
    } else {
      self.get(f);
    }
  }

  fn peek(&self, f: impl Fn(MessageLoopResult)) {
    for msg in Msg::peek(MessageLoopQueue::Thread, None, PeekMessageFlags::Remove) {
      f(match msg {
        PeekResult::Msg(msg) => MessageLoopResult::Peek(Some(msg)),
        PeekResult::None => MessageLoopResult::Peek(None),
        PeekResult::Err(error) => MessageLoopResult::Err(error),
      });
    }
  }

  fn get(&self, f: impl Fn(MessageLoopResult)) {
    for msg in Msg::get(MessageLoopQueue::Thread, None) {
      f(match msg {
        Ok(msg) => MessageLoopResult::Get(msg),
        Err(error) => MessageLoopResult::Err(error),
      });
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageLoopResult {
  Peek(Option<Msg>),
  Get(Msg),
  Err(Error),
}
