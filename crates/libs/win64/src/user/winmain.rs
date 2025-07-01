use crate::StartupInfo;

use super::Instance;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Args {
  pub instance: Instance,
  pub prev_instance: u32,
  pub cmd_line: Vec<String>,
  pub cmd_show: bool,
}

impl Args {
  pub fn get() -> Self {
    let instance = Instance::get();
    let cmd_line = std::env::args().collect();
    let info = StartupInfo::get();

    Self {
      instance,
      prev_instance: 0,
      cmd_line,
      cmd_show: info.show_window,
    }
  }
}

pub fn args() -> Args {
  Args::get()
}
