use bitflags::bitflags;
use windows_sys::Win32::UI::WindowsAndMessaging::{self, WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASS_STYLES};

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct WindowClassStyle: u32 {
    const ByteAlignClient = WindowsAndMessaging::CS_BYTEALIGNCLIENT;
    const ByteAlignWindow = WindowsAndMessaging::CS_BYTEALIGNWINDOW;
    const DeviceContext = WindowsAndMessaging::CS_CLASSDC;
    const DoubleClicks = WindowsAndMessaging::CS_DBLCLKS;
    const DropShadow = WindowsAndMessaging::CS_DROPSHADOW;
    const Global = WindowsAndMessaging::CS_GLOBALCLASS;
    const HorizontalRedraw = WindowsAndMessaging::CS_HREDRAW;
    const NoClose = WindowsAndMessaging::CS_NOCLOSE;
    const OwnDeviceContext = WindowsAndMessaging::CS_OWNDC;
    const ParentDeviceContext = WindowsAndMessaging::CS_PARENTDC;
    const SaveBits = WindowsAndMessaging::CS_SAVEBITS;
    const VerticalRedraw = WindowsAndMessaging::CS_VREDRAW;
  }
}

impl WindowClassStyle {
  #[inline]
  pub const fn to_raw(self) -> WNDCLASS_STYLES {
    self.bits()
  }
}

impl Default for WindowClassStyle {
  fn default() -> Self {
    Self::empty()
  }
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct WindowStyle: u32 {
    const Border = WindowsAndMessaging::WS_BORDER;
    const Caption = WindowsAndMessaging::WS_CAPTION;
    const Child = WindowsAndMessaging::WS_CHILD;
    const ChildWindow = WindowsAndMessaging::WS_CHILDWINDOW;
    const ClipChildren = WindowsAndMessaging::WS_CLIPCHILDREN;
    const ClipSiblings = WindowsAndMessaging::WS_CLIPSIBLINGS;
    const Disabled = WindowsAndMessaging::WS_DISABLED;
    const DialogFrame = WindowsAndMessaging::WS_DLGFRAME;
    const Group = WindowsAndMessaging::WS_GROUP;
    const HorizontalScroll = WindowsAndMessaging::WS_HSCROLL;
    const Iconic = WindowsAndMessaging::WS_ICONIC;
    const Maximize = WindowsAndMessaging::WS_MAXIMIZE;
    const MaximizeBox = WindowsAndMessaging::WS_MAXIMIZEBOX;
    const Minimize = WindowsAndMessaging::WS_MINIMIZE;
    const MinimizeBox = WindowsAndMessaging::WS_MINIMIZEBOX;
    const Overlapped = WindowsAndMessaging::WS_OVERLAPPED;
    const OverlappedWindow = WindowsAndMessaging::WS_OVERLAPPEDWINDOW;
    const PopUp = WindowsAndMessaging::WS_POPUP;
    const PopUpWindow = WindowsAndMessaging::WS_POPUPWINDOW;
    const SizeBox = WindowsAndMessaging::WS_SIZEBOX;
    const SystemMenu = WindowsAndMessaging::WS_SYSMENU;
    const TabStop = WindowsAndMessaging::WS_TABSTOP;
    const ThickFrame = WindowsAndMessaging::WS_THICKFRAME;
    const Tiled = WindowsAndMessaging::WS_TILED;
    const TiledWindow = WindowsAndMessaging::WS_TILEDWINDOW;
    const Visible = WindowsAndMessaging::WS_VISIBLE;
    const VerticalScroll = WindowsAndMessaging::WS_VSCROLL;
  }
}

impl WindowStyle {
  #[inline]
  pub const fn to_raw(self) -> WINDOW_STYLE {
    self.bits()
  }
}

impl Default for WindowStyle {
  fn default() -> Self {
    Self::empty()
  }
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct ExtendedWindowStyle: u32 {
    const AcceptFiles = WindowsAndMessaging::WS_EX_ACCEPTFILES;
    const AppWindow = WindowsAndMessaging::WS_EX_APPWINDOW;
    const ClientEdge = WindowsAndMessaging::WS_EX_CLIENTEDGE;
    const Composited = WindowsAndMessaging::WS_EX_COMPOSITED;
    const ContextHelp = WindowsAndMessaging::WS_EX_CONTEXTHELP;
    const ControlParent = WindowsAndMessaging::WS_EX_CONTROLPARENT;
    const DialogModalFrame = WindowsAndMessaging::WS_EX_DLGMODALFRAME;
    const Layered = WindowsAndMessaging::WS_EX_LAYERED;
    const LayoutRightToLeft = WindowsAndMessaging::WS_EX_LAYOUTRTL;
    const Left = WindowsAndMessaging::WS_EX_LEFT;
    const LeftScrollbar = WindowsAndMessaging::WS_EX_LEFTSCROLLBAR;
    const LeftToRightReading = WindowsAndMessaging::WS_EX_LTRREADING;
    const MDIChild = WindowsAndMessaging::WS_EX_MDICHILD;
    const NoActivate = WindowsAndMessaging::WS_EX_NOACTIVATE;
    const NoInheritLayout = WindowsAndMessaging::WS_EX_NOINHERITLAYOUT;
    const NoParentNotify = WindowsAndMessaging::WS_EX_NOPARENTNOTIFY;
    const NoRedirectionBitmap = WindowsAndMessaging::WS_EX_NOREDIRECTIONBITMAP;
    const OverlappedWindow = WindowsAndMessaging::WS_EX_OVERLAPPEDWINDOW;
    const PaletteWindow = WindowsAndMessaging::WS_EX_PALETTEWINDOW;
    const Right = WindowsAndMessaging::WS_EX_RIGHT;
    const RightScrollbar = WindowsAndMessaging::WS_EX_RIGHTSCROLLBAR;
    const RightToLeftReading = WindowsAndMessaging::WS_EX_RTLREADING;
    const StaticEdge = WindowsAndMessaging::WS_EX_STATICEDGE;
    const ToolWindow = WindowsAndMessaging::WS_EX_TOOLWINDOW;
    const TopMost = WindowsAndMessaging::WS_EX_TOPMOST;
    const Transparent = WindowsAndMessaging::WS_EX_TRANSPARENT;
    const WindowEdge = WindowsAndMessaging::WS_EX_WINDOWEDGE;
  }
}

impl ExtendedWindowStyle {
  #[inline]
  pub const fn to_raw(self) -> WINDOW_EX_STYLE {
    self.bits()
  }
}

impl Default for ExtendedWindowStyle {
  fn default() -> Self {
    Self::empty()
  }
}
