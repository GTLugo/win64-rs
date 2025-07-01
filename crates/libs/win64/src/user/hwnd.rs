use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use widestring::WideCString;
use windows_result::{HRESULT, Result};
use windows_sys::Win32::UI::WindowsAndMessaging::{
  CreateWindowExW, DefWindowProcW, DestroyWindow, IsWindow, PostQuitMessage, SetWindowTextW, ShowWindow,
};

use crate::{Handle, declare_handle, get_last_error, reset_last_error};

use super::{
  Instance, Message, WindowClass, WindowPointerIndex, WindowPos, WindowSize, WindowStyle,
  procedure::{LResult, WindowProcedure, WindowState},
  styles::ExtendedWindowStyle,
};

declare_handle!(
  Window,
  alias = "HWND",
  doc = "https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd"
);
// #[deprecated]
// pub type HWND = HWindow;

// #[derive(Debug, Clone)]
pub struct CreateStruct {
  pub class: WindowClass,
  pub wnd_proc: Option<Box<dyn WindowProcedure>>,
  pub name: String,
  pub style: WindowStyle,
  pub ex_style: ExtendedWindowStyle,
  pub position: WindowPos,
  pub size: WindowSize,
  pub parent: Option<Window>,
  pub menu: Option<*mut ()>,
  pub instance: Option<Instance>,
}

#[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw"]
#[allow(clippy::too_many_arguments)]
pub fn create_window(create_struct: CreateStruct) -> Result<Window> {
  // remove visible style and reapply it later in the window procedure
  // let mut new_style = desc.style;
  // new_style.remove(WindowStyle::Visible);

  let name = WideCString::from_str_truncate(create_struct.name.clone());
  let hwnd = unsafe {
    CreateWindowExW(
      create_struct.ex_style.to_raw(),
      create_struct.class.atom(),
      name.as_ptr(),
      create_struct.style.to_raw(),
      create_struct.position.x(),
      create_struct.position.y(),
      create_struct.size.width(),
      create_struct.size.height(),
      match create_struct.parent {
        Some(p) => p.to_raw() as _,
        None => Window::null().to_raw() as _,
      },
      match create_struct.menu {
        Some(m) => m as _,
        None => std::ptr::null_mut() as _,
      },
      match create_struct.instance {
        Some(i) => i.to_raw() as _,
        None => Instance::null().to_raw() as _,
      },
      Box::into_raw(Box::new(create_struct)).cast(),
    )
  };

  match hwnd.is_null() {
    true => Err(get_last_error()),
    false => Ok(unsafe { Window::from_raw(hwnd as usize) }),
  }
}

impl Window {
  /// Thin wrapper around [`create_window`] function
  #[inline]
  #[allow(clippy::too_many_arguments)]
  pub fn new(create_struct: CreateStruct) -> Result<Self> {
    create_window(create_struct)
  }
}

impl Window {
  /// Returns whether or not the handle identifies an existing window.
  /// # Safety
  /// A thread should not use [`WindowId::is_window`] for a window that it did not create because the window could be destroyed after this function was called. Further, because window handles are recycled the handle could even point to a different window.
  ///
  #[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow"]
  pub unsafe fn is_window(&self) -> bool {
    // check for null here is probably redundant, but allows for a short-circuit which may or may not be faster.
    !self.is_null() && unsafe { IsWindow(self.to_raw() as _) != 0 }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShowWindowResult {
  WasVisible,
  WasHidden,
}

impl Window {
  #[doc = "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow"]
  pub fn show_window(&self, cmd_show: i32) -> ShowWindowResult {
    match unsafe { ShowWindow(self.to_raw() as _, cmd_show) } {
      0 => ShowWindowResult::WasHidden,
      _ => ShowWindowResult::WasVisible,
    }
  }
}

impl Window {
  pub fn send_message(&self) {
    // TODO: somehow ensure these are always sent to the correct thread, even when called from a different thread.
    // maybe do it by storing the thread id?
    // Reference winit for this!
    todo!()
  }

  pub fn default_procedure(&self, message: &Message) -> LResult {
    unsafe { DefWindowProcW(self.to_ptr(), message.id().to_raw(), message.w().0, message.l().0) }.into()
  }

  pub fn destroy(&self) -> Result<()> {
    if unsafe { self.is_window() } {
      if let Some(state) = self.state() {
        state.set_destroying();
        reset_last_error();
        return match unsafe { DestroyWindow(self.to_ptr()) } {
          0 => Ok(()),
          _ => Err(get_last_error()),
        };
      }
    }
    Ok(())
  }

  pub fn quit(&self) {
    self.quit_with_code(0)
  }

  pub fn quit_with_code(&self, exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
  }

  pub fn set_window_text(&self, text: impl Into<String>) -> Result<()> {
    let text = OsString::from(text.into()).encode_wide().collect::<Vec<u16>>();
    reset_last_error();
    match unsafe { SetWindowTextW(self.to_ptr(), text.as_ptr()) } {
      0 => Ok(()),
      _ => Err(get_last_error()),
    }
  }

  // TODO: Migrate these functions to use conditionals for diff ptr widths
  pub(crate) fn get_window_ptr(&self, index: WindowPointerIndex) -> isize {
    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn inner(index: WindowPointerIndex) -> isize {
      unsafe {
        use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongW;
        GetWindowLongW(this.to_ptr(), index.to_raw()) as _
      }
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn inner(this: &Window, index: WindowPointerIndex) -> isize {
      unsafe {
        use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW;
        GetWindowLongPtrW(this.to_ptr(), index.to_raw()) as _
      }
    }

    inner(self, index)
  }

  pub(crate) fn set_window_ptr(&self, index: WindowPointerIndex, value: isize) -> Result<isize> {
    reset_last_error();

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn inner(index: WindowPointerIndex) -> isize {
      unsafe {
        use windows_sys::Win32::UI::WindowsAndMessaging::SetWindowLongW;
        SetWindowLongW(self.to_ptr(), index.to_raw(), value) as _
      }
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn inner(this: &Window, index: WindowPointerIndex, value: isize) -> isize {
      unsafe {
        use windows_sys::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW;
        SetWindowLongPtrW(this.to_ptr(), index.to_raw(), value) as _
      }
    }

    let result = inner(self, index, value);

    let error = get_last_error();
    match result == 0 && error.code() != HRESULT(0) {
      true => Err(error),
      false => Ok(result),
    }
  }

  pub(crate) fn state(&self) -> Option<&mut WindowState> {
    unsafe { (self.get_window_ptr(WindowPointerIndex::UserData) as *mut WindowState).as_mut() }
  }
}
