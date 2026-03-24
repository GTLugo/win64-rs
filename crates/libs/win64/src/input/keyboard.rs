#![allow(unused)]

pub mod key;
pub mod layout;

// Taken from winit as allowed by Apache 2.0 license

use {
  crate::{
    loword,
    primarylangid,
    user::LParam,
  },
  keyboard_types::{
    Code,
    Location,
  },
  std::mem::MaybeUninit,
  windows_sys::Win32::{
    System::SystemServices::LANG_KOREAN,
    UI::Input::KeyboardAndMouse::{
      self,
      GetAsyncKeyState,
      GetKeyState,
      GetKeyboardLayout,
      GetKeyboardState,
      HKL,
      MapVirtualKeyExW,
      VIRTUAL_KEY,
    },
  },
};

pub type ExScancode = u16;

#[derive(Debug, Copy, Clone)]
pub struct KeyLParam {
  pub scancode: u8,
  pub extended: bool,

  /// This is `previous_state XOR transition_state`. See the lParam for WM_KEYDOWN and WM_KEYUP
  /// for further details.
  pub is_repeat: bool,
}

pub fn destructure_key_lparam(LParam(lparam): LParam) -> KeyLParam {
  let previous_state = (lparam >> 30) & 0x01;
  let transition_state = (lparam >> 31) & 0x01;
  KeyLParam {
    scancode: ((lparam >> 16) & 0xff) as u8,
    extended: ((lparam >> 24) & 0x01) != 0,
    is_repeat: (previous_state ^ transition_state) != 0,
  }
}

#[inline]
pub fn new_ex_scancode(scancode: u8, extended: bool) -> ExScancode {
  (scancode as u16) | (if extended { 0xe000 } else { 0 })
}

#[inline]
pub fn ex_scancode_from_lparam(lparam: LParam) -> ExScancode {
  let lparam = destructure_key_lparam(lparam);
  new_ex_scancode(lparam.scancode, lparam.extended)
}

/// Gets the keyboard state as reported by messages that have been removed from the event queue.
/// See also: get_async_kbd_state
pub fn get_kbd_state() -> [u8; 256] {
  unsafe {
    let mut kbd_state: MaybeUninit<[u8; 256]> = MaybeUninit::uninit();
    GetKeyboardState(kbd_state.as_mut_ptr() as *mut u8);
    kbd_state.assume_init()
  }
}

/// Gets the current keyboard state regardless of whether the corresponding keyboard events have
/// been removed from the event queue. See also: get_kbd_state
#[allow(clippy::uninit_assumed_init)]
pub fn get_async_kbd_state() -> [u8; 256] {
  unsafe {
    let mut kbd_state: [u8; 256] = [0; 256];
    for (vk, state) in kbd_state.iter_mut().enumerate() {
      let vk = vk as VIRTUAL_KEY;
      let async_state = GetAsyncKeyState(vk as i32);
      let is_down = (async_state & (1 << 15)) != 0;
      *state = if is_down { 0x80 } else { 0 };

      if matches!(vk, KeyboardAndMouse::VK_CAPITAL | KeyboardAndMouse::VK_NUMLOCK | KeyboardAndMouse::VK_SCROLL) {
        // Toggle states aren't reported by `GetAsyncKeyState`
        let toggle_state = GetKeyState(vk as i32);
        let is_active = (toggle_state & 1) != 0;
        *state |= u8::from(is_active);
      }
    }
    kbd_state
  }
}

pub fn get_location(scancode: ExScancode, hkl: HKL) -> Location {
  let extension = 0xe000;
  let extended = (scancode & extension) == extension;
  let vkey = unsafe { MapVirtualKeyExW(scancode as u32, KeyboardAndMouse::MAPVK_VSC_TO_VK_EX, hkl) as VIRTUAL_KEY };

  // Use the native VKEY and the extended flag to cover most cases
  // This is taken from the `druid` GUI library, specifically
  // druid-shell/src/platform/windows/keyboard.rs
  match vkey {
    KeyboardAndMouse::VK_LSHIFT
    | KeyboardAndMouse::VK_LCONTROL
    | KeyboardAndMouse::VK_LMENU
    | KeyboardAndMouse::VK_LWIN => Location::Left,
    KeyboardAndMouse::VK_RSHIFT
    | KeyboardAndMouse::VK_RCONTROL
    | KeyboardAndMouse::VK_RMENU
    | KeyboardAndMouse::VK_RWIN => Location::Right,
    KeyboardAndMouse::VK_RETURN if extended => Location::Numpad,
    KeyboardAndMouse::VK_INSERT
    | KeyboardAndMouse::VK_DELETE
    | KeyboardAndMouse::VK_END
    | KeyboardAndMouse::VK_DOWN
    | KeyboardAndMouse::VK_NEXT
    | KeyboardAndMouse::VK_LEFT
    | KeyboardAndMouse::VK_CLEAR
    | KeyboardAndMouse::VK_RIGHT
    | KeyboardAndMouse::VK_HOME
    | KeyboardAndMouse::VK_UP
    | KeyboardAndMouse::VK_PRIOR => {
      if extended {
        Location::Standard
      } else {
        Location::Numpad
      }
    },
    KeyboardAndMouse::VK_NUMPAD0
    | KeyboardAndMouse::VK_NUMPAD1
    | KeyboardAndMouse::VK_NUMPAD2
    | KeyboardAndMouse::VK_NUMPAD3
    | KeyboardAndMouse::VK_NUMPAD4
    | KeyboardAndMouse::VK_NUMPAD5
    | KeyboardAndMouse::VK_NUMPAD6
    | KeyboardAndMouse::VK_NUMPAD7
    | KeyboardAndMouse::VK_NUMPAD8
    | KeyboardAndMouse::VK_NUMPAD9
    | KeyboardAndMouse::VK_DECIMAL
    | KeyboardAndMouse::VK_DIVIDE
    | KeyboardAndMouse::VK_MULTIPLY
    | KeyboardAndMouse::VK_SUBTRACT
    | KeyboardAndMouse::VK_ADD
    | KeyboardAndMouse::VK_ABNT_C2 => Location::Numpad,
    _ => Location::Standard,
  }
}

pub fn code_to_scancode(code: Code) -> Option<u32> {
  // See `scancode_to_physicalkey` for more info

  let hkl = unsafe { GetKeyboardLayout(0) };

  let primary_lang_id = primarylangid(loword(hkl as u32));
  let is_korean = primary_lang_id as u32 == LANG_KOREAN;

  // let code = match physical_key {
  //   PhysicalKey::Code(code) => code,
  //   PhysicalKey::Unidentified(code) => {
  //     return match code {
  //       NativeKeyCode::Windows(scancode) => Some(scancode as u32),
  //       _ => None,
  //     };
  //   },
  // };

  match code {
    Code::Backquote => Some(0x0029),
    Code::Backslash => Some(0x002b),
    Code::Backspace => Some(0x000e),
    Code::BracketLeft => Some(0x001a),
    Code::BracketRight => Some(0x001b),
    Code::Comma => Some(0x0033),
    Code::Digit0 => Some(0x000b),
    Code::Digit1 => Some(0x0002),
    Code::Digit2 => Some(0x0003),
    Code::Digit3 => Some(0x0004),
    Code::Digit4 => Some(0x0005),
    Code::Digit5 => Some(0x0006),
    Code::Digit6 => Some(0x0007),
    Code::Digit7 => Some(0x0008),
    Code::Digit8 => Some(0x0009),
    Code::Digit9 => Some(0x000a),
    Code::Equal => Some(0x000d),
    Code::IntlBackslash => Some(0x0056),
    Code::IntlRo => Some(0x0073),
    Code::IntlYen => Some(0x007d),
    Code::KeyA => Some(0x001e),
    Code::KeyB => Some(0x0030),
    Code::KeyC => Some(0x002e),
    Code::KeyD => Some(0x0020),
    Code::KeyE => Some(0x0012),
    Code::KeyF => Some(0x0021),
    Code::KeyG => Some(0x0022),
    Code::KeyH => Some(0x0023),
    Code::KeyI => Some(0x0017),
    Code::KeyJ => Some(0x0024),
    Code::KeyK => Some(0x0025),
    Code::KeyL => Some(0x0026),
    Code::KeyM => Some(0x0032),
    Code::KeyN => Some(0x0031),
    Code::KeyO => Some(0x0018),
    Code::KeyP => Some(0x0019),
    Code::KeyQ => Some(0x0010),
    Code::KeyR => Some(0x0013),
    Code::KeyS => Some(0x001f),
    Code::KeyT => Some(0x0014),
    Code::KeyU => Some(0x0016),
    Code::KeyV => Some(0x002f),
    Code::KeyW => Some(0x0011),
    Code::KeyX => Some(0x002d),
    Code::KeyY => Some(0x0015),
    Code::KeyZ => Some(0x002c),
    Code::Minus => Some(0x000c),
    Code::Period => Some(0x0034),
    Code::Quote => Some(0x0028),
    Code::Semicolon => Some(0x0027),
    Code::Slash => Some(0x0035),
    Code::AltLeft => Some(0x0038),
    Code::AltRight => Some(0xe038),
    Code::CapsLock => Some(0x003a),
    Code::ContextMenu => Some(0xe05d),
    Code::ControlLeft => Some(0x001d),
    Code::ControlRight => Some(0xe01d),
    Code::Enter => Some(0x001c),
    Code::MetaLeft => Some(0xe05b),
    Code::MetaRight => Some(0xe05c),
    Code::ShiftLeft => Some(0x002a),
    Code::ShiftRight => Some(0x0036),
    Code::Space => Some(0x0039),
    Code::Tab => Some(0x000f),
    Code::Convert => Some(0x0079),
    Code::Lang1 => {
      if is_korean {
        Some(0xe0f2)
      } else {
        Some(0x0072)
      }
    },
    Code::Lang2 => {
      if is_korean {
        Some(0xe0f1)
      } else {
        Some(0x0071)
      }
    },
    Code::KanaMode => Some(0x0070),
    Code::NonConvert => Some(0x007b),
    Code::Delete => Some(0xe053),
    Code::End => Some(0xe04f),
    Code::Home => Some(0xe047),
    Code::Insert => Some(0xe052),
    Code::PageDown => Some(0xe051),
    Code::PageUp => Some(0xe049),
    Code::ArrowDown => Some(0xe050),
    Code::ArrowLeft => Some(0xe04b),
    Code::ArrowRight => Some(0xe04d),
    Code::ArrowUp => Some(0xe048),
    Code::NumLock => Some(0xe045),
    Code::Numpad0 => Some(0x0052),
    Code::Numpad1 => Some(0x004f),
    Code::Numpad2 => Some(0x0050),
    Code::Numpad3 => Some(0x0051),
    Code::Numpad4 => Some(0x004b),
    Code::Numpad5 => Some(0x004c),
    Code::Numpad6 => Some(0x004d),
    Code::Numpad7 => Some(0x0047),
    Code::Numpad8 => Some(0x0048),
    Code::Numpad9 => Some(0x0049),
    Code::NumpadAdd => Some(0x004e),
    Code::NumpadComma => Some(0x007e),
    Code::NumpadDecimal => Some(0x0053),
    Code::NumpadDivide => Some(0xe035),
    Code::NumpadEnter => Some(0xe01c),
    Code::NumpadEqual => Some(0x0059),
    Code::NumpadMultiply => Some(0x0037),
    Code::NumpadSubtract => Some(0x004a),
    Code::Escape => Some(0x0001),
    Code::F1 => Some(0x003b),
    Code::F2 => Some(0x003c),
    Code::F3 => Some(0x003d),
    Code::F4 => Some(0x003e),
    Code::F5 => Some(0x003f),
    Code::F6 => Some(0x0040),
    Code::F7 => Some(0x0041),
    Code::F8 => Some(0x0042),
    Code::F9 => Some(0x0043),
    Code::F10 => Some(0x0044),
    Code::F11 => Some(0x0057),
    Code::F12 => Some(0x0058),
    Code::F13 => Some(0x0064),
    Code::F14 => Some(0x0065),
    Code::F15 => Some(0x0066),
    Code::F16 => Some(0x0067),
    Code::F17 => Some(0x0068),
    Code::F18 => Some(0x0069),
    Code::F19 => Some(0x006a),
    Code::F20 => Some(0x006b),
    Code::F21 => Some(0x006c),
    Code::F22 => Some(0x006d),
    Code::F23 => Some(0x006e),
    Code::F24 => Some(0x0076),
    Code::PrintScreen => Some(0xe037),
    // KeyCode::PrintScreen => Some(0x0054), // Alt + PrintScreen
    Code::ScrollLock => Some(0x0046),
    Code::Pause => Some(0x0045),
    // KeyCode::Pause => Some(0xE046), // Ctrl + Pause
    Code::BrowserBack => Some(0xe06a),
    Code::BrowserFavorites => Some(0xe066),
    Code::BrowserForward => Some(0xe069),
    Code::BrowserHome => Some(0xe032),
    Code::BrowserRefresh => Some(0xe067),
    Code::BrowserSearch => Some(0xe065),
    Code::BrowserStop => Some(0xe068),
    Code::LaunchApp1 => Some(0xe06b),
    Code::LaunchApp2 => Some(0xe021),
    Code::LaunchMail => Some(0xe06c),
    Code::MediaPlayPause => Some(0xe022),
    Code::MediaSelect => Some(0xe06d),
    Code::MediaStop => Some(0xe024),
    Code::MediaTrackNext => Some(0xe019),
    Code::MediaTrackPrevious => Some(0xe010),
    Code::Power => Some(0xe05e),
    Code::AudioVolumeDown => Some(0xe02e),
    Code::AudioVolumeMute => Some(0xe020),
    Code::AudioVolumeUp => Some(0xe030),

    // Extra from Chromium sources:
    // https://chromium.googlesource.com/chromium/src.git/+/3e1a26c44c024d97dc9a4c09bbc6a2365398ca2c/ui/events/keycodes/dom/dom_code_data.inc
    Code::Lang4 => Some(0x0077),
    Code::Lang3 => Some(0x0078),
    Code::Undo => Some(0xe008),
    Code::Paste => Some(0xe00a),
    Code::Cut => Some(0xe017),
    Code::Copy => Some(0xe018),
    Code::Eject => Some(0xe02c),
    Code::Help => Some(0xe03b),
    Code::Sleep => Some(0xe05f),
    Code::WakeUp => Some(0xe063),

    _ => None,
  }
}

pub fn scancode_to_code(scancode: u32) -> Code {
  // See: https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html
  // and: https://www.w3.org/TR/uievents-code/
  // and: The widget/NativeKeyToDOMCodeName.h file in the firefox source

  match scancode {
    0x0029 => Code::Backquote,
    0x002b => Code::Backslash,
    0x000e => Code::Backspace,
    0x001a => Code::BracketLeft,
    0x001b => Code::BracketRight,
    0x0033 => Code::Comma,
    0x000b => Code::Digit0,
    0x0002 => Code::Digit1,
    0x0003 => Code::Digit2,
    0x0004 => Code::Digit3,
    0x0005 => Code::Digit4,
    0x0006 => Code::Digit5,
    0x0007 => Code::Digit6,
    0x0008 => Code::Digit7,
    0x0009 => Code::Digit8,
    0x000a => Code::Digit9,
    0x000d => Code::Equal,
    0x0056 => Code::IntlBackslash,
    0x0073 => Code::IntlRo,
    0x007d => Code::IntlYen,
    0x001e => Code::KeyA,
    0x0030 => Code::KeyB,
    0x002e => Code::KeyC,
    0x0020 => Code::KeyD,
    0x0012 => Code::KeyE,
    0x0021 => Code::KeyF,
    0x0022 => Code::KeyG,
    0x0023 => Code::KeyH,
    0x0017 => Code::KeyI,
    0x0024 => Code::KeyJ,
    0x0025 => Code::KeyK,
    0x0026 => Code::KeyL,
    0x0032 => Code::KeyM,
    0x0031 => Code::KeyN,
    0x0018 => Code::KeyO,
    0x0019 => Code::KeyP,
    0x0010 => Code::KeyQ,
    0x0013 => Code::KeyR,
    0x001f => Code::KeyS,
    0x0014 => Code::KeyT,
    0x0016 => Code::KeyU,
    0x002f => Code::KeyV,
    0x0011 => Code::KeyW,
    0x002d => Code::KeyX,
    0x0015 => Code::KeyY,
    0x002c => Code::KeyZ,
    0x000c => Code::Minus,
    0x0034 => Code::Period,
    0x0028 => Code::Quote,
    0x0027 => Code::Semicolon,
    0x0035 => Code::Slash,
    0x0038 => Code::AltLeft,
    0xe038 => Code::AltRight,
    0x003a => Code::CapsLock,
    0xe05d => Code::ContextMenu,
    0x001d => Code::ControlLeft,
    0xe01d => Code::ControlRight,
    0x001c => Code::Enter,
    0xe05b => Code::MetaLeft,
    0xe05c => Code::MetaRight,
    0x002a => Code::ShiftLeft,
    0x0036 => Code::ShiftRight,
    0x0039 => Code::Space,
    0x000f => Code::Tab,
    0x0079 => Code::Convert,
    0x0072 => Code::Lang1, // for non-Korean layout
    0xe0f2 => Code::Lang1, // for Korean layout
    0x0071 => Code::Lang2, // for non-Korean layout
    0xe0f1 => Code::Lang2, // for Korean layout
    0x0070 => Code::KanaMode,
    0x007b => Code::NonConvert,
    0xe053 => Code::Delete,
    0xe04f => Code::End,
    0xe047 => Code::Home,
    0xe052 => Code::Insert,
    0xe051 => Code::PageDown,
    0xe049 => Code::PageUp,
    0xe050 => Code::ArrowDown,
    0xe04b => Code::ArrowLeft,
    0xe04d => Code::ArrowRight,
    0xe048 => Code::ArrowUp,
    0xe045 => Code::NumLock,
    0x0052 => Code::Numpad0,
    0x004f => Code::Numpad1,
    0x0050 => Code::Numpad2,
    0x0051 => Code::Numpad3,
    0x004b => Code::Numpad4,
    0x004c => Code::Numpad5,
    0x004d => Code::Numpad6,
    0x0047 => Code::Numpad7,
    0x0048 => Code::Numpad8,
    0x0049 => Code::Numpad9,
    0x004e => Code::NumpadAdd,
    0x007e => Code::NumpadComma,
    0x0053 => Code::NumpadDecimal,
    0xe035 => Code::NumpadDivide,
    0xe01c => Code::NumpadEnter,
    0x0059 => Code::NumpadEqual,
    0x0037 => Code::NumpadMultiply,
    0x004a => Code::NumpadSubtract,
    0x0001 => Code::Escape,
    0x003b => Code::F1,
    0x003c => Code::F2,
    0x003d => Code::F3,
    0x003e => Code::F4,
    0x003f => Code::F5,
    0x0040 => Code::F6,
    0x0041 => Code::F7,
    0x0042 => Code::F8,
    0x0043 => Code::F9,
    0x0044 => Code::F10,
    0x0057 => Code::F11,
    0x0058 => Code::F12,
    0x0064 => Code::F13,
    0x0065 => Code::F14,
    0x0066 => Code::F15,
    0x0067 => Code::F16,
    0x0068 => Code::F17,
    0x0069 => Code::F18,
    0x006a => Code::F19,
    0x006b => Code::F20,
    0x006c => Code::F21,
    0x006d => Code::F22,
    0x006e => Code::F23,
    0x0076 => Code::F24,
    0xe037 => Code::PrintScreen,
    0x0054 => Code::PrintScreen, // Alt + PrintScreen
    0x0046 => Code::ScrollLock,
    0x0045 => Code::Pause,
    0xe046 => Code::Pause, // Ctrl + Pause
    0xe06a => Code::BrowserBack,
    0xe066 => Code::BrowserFavorites,
    0xe069 => Code::BrowserForward,
    0xe032 => Code::BrowserHome,
    0xe067 => Code::BrowserRefresh,
    0xe065 => Code::BrowserSearch,
    0xe068 => Code::BrowserStop,
    0xe06b => Code::LaunchApp1,
    0xe021 => Code::LaunchApp2,
    0xe06c => Code::LaunchMail,
    0xe022 => Code::MediaPlayPause,
    0xe06d => Code::MediaSelect,
    0xe024 => Code::MediaStop,
    0xe019 => Code::MediaTrackNext,
    0xe010 => Code::MediaTrackPrevious,
    0xe05e => Code::Power,
    0xe02e => Code::AudioVolumeDown,
    0xe020 => Code::AudioVolumeMute,
    0xe030 => Code::AudioVolumeUp,

    // Extra from Chromium sources:
    // https://chromium.googlesource.com/chromium/src.git/+/3e1a26c44c024d97dc9a4c09bbc6a2365398ca2c/ui/events/Codes/dom/dom_code_data.inc
    0x0077 => Code::Lang4,
    0x0078 => Code::Lang3,
    0xe008 => Code::Undo,
    0xe00a => Code::Paste,
    0xe017 => Code::Cut,
    0xe018 => Code::Copy,
    0xe02c => Code::Eject,
    0xe03b => Code::Help,
    0xe05f => Code::Sleep,
    0xe063 => Code::WakeUp,

    _ => Code::Unidentified,
  }
}
