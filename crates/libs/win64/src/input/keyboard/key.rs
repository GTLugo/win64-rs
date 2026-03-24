// Taken from winit as allowed by Apache 2.0 license

use {
  keyboard_types::NamedKey,
  windows_sys::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY,
};

/// Key represents the meaning of a keypress.
///
/// This is a superset of the UI Events Specification's [`KeyboardEvent.key`] with
/// additions:
/// - All simple variants are wrapped under the `Named` variant
/// - The `Unidentified` variant here, can still identify a key through it's `NativeKeyCode`.
/// - The `Dead` variant here, can specify the character which is inserted when pressing the
///   dead-key twice.
///
/// [`KeyboardEvent.key`]: https://w3c.github.io/uievents-key/
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Key {
  /// A simple (unparameterised) action
  Named(NamedKey),

  /// A key string that corresponds to the character typed by the user, taking into account the
  /// user’s current locale setting, and any system-level keyboard mapping overrides that are in
  /// effect.
  Character(String),

  /// This variant is used when the key cannot be translated to any other variant.
  ///
  /// The native key is provided (if available) in order to allow the user to specify keybindings
  /// for keys which are not defined by this API, mainly through some sort of UI.
  Unidentified(VIRTUAL_KEY),

  /// Contains the text representation of the dead-key when available.
  ///
  /// ## Platform-specific
  /// - **Web:** Always contains `None`
  Dead(Option<char>),
}

impl From<NamedKey> for Key {
  #[inline]
  fn from(action: NamedKey) -> Self {
    Key::Named(action)
  }
}

impl From<VIRTUAL_KEY> for Key {
  #[inline]
  fn from(code: VIRTUAL_KEY) -> Self {
    Key::Unidentified(code)
  }
}

impl PartialEq<NamedKey> for Key {
  #[inline]
  fn eq(&self, rhs: &NamedKey) -> bool {
    match self {
      Key::Named(a) => a == rhs,
      _ => false,
    }
  }
}

impl PartialEq<str> for Key {
  #[inline]
  fn eq(&self, rhs: &str) -> bool {
    match self {
      Key::Character(s) => s == rhs,
      _ => false,
    }
  }
}

impl PartialEq<&str> for Key {
  #[inline]
  fn eq(&self, rhs: &&str) -> bool {
    self == *rhs
  }
}

impl PartialEq<VIRTUAL_KEY> for Key {
  #[inline]
  fn eq(&self, rhs: &VIRTUAL_KEY) -> bool {
    match self {
      Key::Unidentified(code) => code == rhs,
      _ => false,
    }
  }
}

impl PartialEq<Key> for VIRTUAL_KEY {
  #[inline]
  fn eq(&self, rhs: &Key) -> bool {
    rhs == self
  }
}
