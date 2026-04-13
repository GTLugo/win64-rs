use {
  crate::{
    high_word,
    signed_high_word,
    signed_low_word,
    user::{
      LParam,
      WParam,
    },
  },
  dpi::PhysicalPosition,
  keyboard_types::Modifiers,
  mouse_types::{
    button::MouseButton,
    event::MouseEvent,
    state::ButtonState,
  },
  windows_sys::Win32::{
    System::SystemServices::{
      MK_LBUTTON,
      MK_MBUTTON,
      MK_RBUTTON,
      MK_XBUTTON1,
      MK_XBUTTON2,
    },
    UI::WindowsAndMessaging,
  },
};

pub(crate) fn mouse_event(message: u32, w_param: WParam, l_param: LParam) -> MouseEvent {
  let flags = w_param.0 as u32;

  let button: MouseButton = {
    match message {
      WindowsAndMessaging::WM_LBUTTONDBLCLK
      | WindowsAndMessaging::WM_LBUTTONDOWN
      | WindowsAndMessaging::WM_LBUTTONUP => MouseButton::Left,
      WindowsAndMessaging::WM_MBUTTONDBLCLK
      | WindowsAndMessaging::WM_MBUTTONDOWN
      | WindowsAndMessaging::WM_MBUTTONUP => MouseButton::Middle,
      WindowsAndMessaging::WM_RBUTTONDBLCLK
      | WindowsAndMessaging::WM_RBUTTONDOWN
      | WindowsAndMessaging::WM_RBUTTONUP => MouseButton::Right,
      WindowsAndMessaging::WM_XBUTTONDBLCLK
      | WindowsAndMessaging::WM_XBUTTONDOWN
      | WindowsAndMessaging::WM_XBUTTONUP => {
        let hi_flags = high_word(flags);
        if (hi_flags & WindowsAndMessaging::XBUTTON1) == WindowsAndMessaging::XBUTTON1 {
          MouseButton::Back
        } else {
          MouseButton::Forward
        }
      },
      _ => unimplemented!("Unexpected mouse message: {message}"),
    }
  };

  let is_double_click = matches!(
    message,
    WindowsAndMessaging::WM_LBUTTONDBLCLK
      | WindowsAndMessaging::WM_MBUTTONDBLCLK
      | WindowsAndMessaging::WM_RBUTTONDBLCLK
      | WindowsAndMessaging::WM_XBUTTONDBLCLK
  );

  let state = {
    let mod_flags = flags;
    let is_l_down = (mod_flags & MK_LBUTTON) == MK_LBUTTON;
    let is_m_down = (mod_flags & MK_MBUTTON) == MK_MBUTTON;
    let is_r_down = (mod_flags & MK_RBUTTON) == MK_RBUTTON;
    let is_x1_down = (mod_flags & MK_XBUTTON1) == MK_XBUTTON1;
    let is_x2_down = (mod_flags & MK_XBUTTON2) == MK_XBUTTON2;

    let is_down = match message {
      WindowsAndMessaging::WM_LBUTTONDBLCLK | WindowsAndMessaging::WM_LBUTTONDOWN if is_l_down => true,
      WindowsAndMessaging::WM_MBUTTONDBLCLK | WindowsAndMessaging::WM_MBUTTONDOWN if is_m_down => true,
      WindowsAndMessaging::WM_RBUTTONDBLCLK | WindowsAndMessaging::WM_RBUTTONDOWN if is_r_down => true,
      WindowsAndMessaging::WM_XBUTTONDBLCLK | WindowsAndMessaging::WM_XBUTTONDOWN if is_x1_down || is_x2_down => true,
      _ => false,
    };

    if is_down { ButtonState::Down } else { ButtonState::Up }
  };

  let (x, y) = (signed_low_word(l_param.0 as i32), signed_high_word(l_param.0 as i32));

  let position = PhysicalPosition::new(x as i32, y as i32);

  MouseEvent {
    position,
    state,
    button,
    modifiers: Modifiers::empty(), // TODO: Get modifiers from w_param
    is_double_click,
  }
}
