use std::ops::RangeInclusive;

use windows_sys::Win32::UI::WindowsAndMessaging;

pub mod data;
// pub mod id;
// pub mod pump;
// pub mod thread;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WParam(pub usize);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LParam(pub isize);

const REGISTERED_MESSAGES: u32 = 0xC000;

#[derive(win64_macro::Message, Default, Debug, Clone, PartialEq, Eq)]
pub enum MessageId {
  #[default]
  Null,
  /// Any common messages falling into this category should be reported so they may be elevated to proper variants.
  #[fallback]
  #[params(w, l)]
  Other(u32),
  #[id(WindowsAndMessaging::WM_USER..WindowsAndMessaging::WM_APP)]
  #[params(w, l)]
  User(u32),
  #[id(WindowsAndMessaging::WM_APP..REGISTERED_MESSAGES)]
  #[params(w, l)]
  App(u32),
  #[id(REGISTERED_MESSAGES..=0xFFFF)]
  #[params(w, l)]
  Registered(u32),
  #[params(w, l)]
  Activate,
  #[params(w, l)]
  ActivateApp,
  #[params(w, l)]
  AppCommand,
  #[params(w, l)]
  AskCbFormatName,
  CancelJournal,
  CancelMode,
  #[params(l)]
  CaptureChanged,
  #[params(w, l)]
  ChangeCbChain,
  #[params(w)]
  ChangeUiState,
  #[params(w, l)]
  Char,
  #[params(w, l)]
  CharToItem,
  ChildActivate,
  Clear,
  ClipboardUpdate,
  Close,
  Command,
  CommNotify,
  #[params(w)]
  Compacting,
  #[params(w, l)]
  CompareItem,
  #[params(w, l)]
  ContextMenu,
  Copy,
  #[params(w, l)]
  CopyData,
  #[params(l)]
  Create,
  #[params(w, l)]
  CtlColorBtn,
  #[params(w, l)]
  CtlColorDlg,
  #[params(w, l)]
  CtlColorEdit,
  #[params(w, l)]
  CtlColorListBox,
  #[params(w, l)]
  CtlColorMsgBox,
  #[params(w, l)]
  CtlColorScrollBar,
  #[params(w, l)]
  CtlColorStatic,
  Cut,
  #[params(w, l)]
  DeadChar,
  #[params(w, l)]
  DeleteItem,
  Destroy,
  DestroyClipboard,
  #[params(w, l)]
  DeviceChange,
  #[params(l)]
  DevModeChange,
  #[params(w, l)]
  DisplayChange,
  #[params(w, l)]
  DpiChanged,
  #[id(WindowsAndMessaging::WM_DPICHANGED_AFTERPARENT)]
  DpiChangedAfterParent,
  #[id(WindowsAndMessaging::WM_DPICHANGED_BEFOREPARENT)]
  DpiChangedBeforeParent,
  DrawClipboard,
  #[params(w, l)]
  DrawItem,
  #[params(w)]
  DropFiles,
  #[params(w, l)]
  DwmColorizationColorChanged,
  DwmCompositionChanged,
  #[params(w)]
  DwmNcRenderingChanged,
  DwmSendIconicLivePreviewBitmap,
  #[params(l)]
  DwmSendIconicThumbnail,
  #[params(w)]
  DwmWindowMaximizedChange,
  #[params(w)]
  Enable,
  #[params(w, l)]
  EndSession,
  #[params(w, l)]
  EnterIdle,
  #[params(w)]
  EnterMenuLoop,
  EnterSizeMove,
  #[params(w)]
  EraseBkgnd,
  ExitMenuLoop,
  ExitSizeMove,
  FontChange,
  #[params(w, l)]
  Gesture,
  #[params(l)]
  GestureNotify,
  #[params(w, l)]
  GetDlgCode,
  #[params(w, l)]
  GetDpiScaledSize,
  GetFont,
  #[id(WindowsAndMessaging::MN_GETHMENU)]
  GetHMenu,
  GetHotKey,
  #[params(w, l)]
  GetIcon,
  #[params(l)]
  GetMinMaxInfo,
  #[params(w, l)]
  GetObject,
  #[params(w, l)]
  GetText,
  GetTextLength,
  #[params(l)]
  GetTitleBarInfoEx,
  #[params(l)]
  Help,
  #[params(w, l)]
  HotKey,
  #[params(w, l)]
  HScroll,
  #[params(w, l)]
  HScrollClipboard,
  #[params(w)]
  IconEraseBkgnd,
  #[id(WindowsAndMessaging::WM_IME_CHAR)]
  #[params(w, l)]
  ImeChar,
  #[id(WindowsAndMessaging::WM_IME_COMPOSITION)]
  #[params(w, l)]
  ImeComposition,
  #[id(WindowsAndMessaging::WM_IME_COMPOSITIONFULL)]
  ImeCompositionFull,
  #[id(WindowsAndMessaging::WM_IME_CONTROL)]
  #[params(w, l)]
  ImeControl,
  #[id(WindowsAndMessaging::WM_IME_ENDCOMPOSITION)]
  ImeEndComposition,
  #[id(WindowsAndMessaging::WM_IME_KEYDOWN)]
  #[params(w, l)]
  ImeKeyDown,
  #[id(WindowsAndMessaging::WM_IME_KEYUP)]
  #[params(w, l)]
  ImeKeyUp,
  #[id(WindowsAndMessaging::WM_IME_NOTIFY)]
  #[params(w, l)]
  ImeNotify,
  #[id(WindowsAndMessaging::WM_IME_REQUEST)]
  #[params(w, l)]
  ImeRequest,
  #[id(WindowsAndMessaging::WM_IME_SELECT)]
  #[params(w, l)]
  ImeSelect,
  #[id(WindowsAndMessaging::WM_IME_SETCONTEXT)]
  #[params(w, l)]
  ImeSetContext,
  #[id(WindowsAndMessaging::WM_IME_STARTCOMPOSITION)]
  ImeStartComposition,
  #[params(w, l)]
  InitDialog,
  #[params(w)]
  InitMenu,
  InitMenuPopup,
  #[params(w, l)]
  Input,
  #[params(w, l)]
  InputLangChange,
  #[params(w, l)]
  InputLangChangeRequest,
  #[id(WindowsAndMessaging::WM_INPUT_DEVICE_CHANGE)]
  #[params(w, l)]
  InputDeviceChange,
  #[params(w, l)]
  KeyDown,
  #[params(w, l)]
  KeyUp,
  #[params(w)]
  KillFocus,
  #[params(w, l)]
  LButtonDblClk,
  #[params(w, l)]
  LButtonDown,
  #[params(w, l)]
  LButtonUp,
  #[params(w, l)]
  MButtonDblClk,
  #[params(w, l)]
  MButtonDown,
  #[params(w, l)]
  MButtonUp,
  #[params(w)]
  MdiActivate,
  #[params(w)]
  MdiCascade,
  #[params(l)]
  MdiCreate,
  #[params(w)]
  MdiDestroy,
  #[params(l)]
  MdiGetActive,
  MdiIconArrange,
  #[params(w)]
  MdiMaximize,
  #[params(w, l)]
  MdiNext,
  MdiRefreshMenu,
  #[params(w)]
  MdiRestore,
  #[params(w, l)]
  MdiSetMenu,
  #[params(w)]
  MdiTile,
  #[params(w, l)]
  MeasureItem,
  #[params(w, l)]
  MenuChar,
  #[params(w, l)]
  MenuCommand,
  #[params(w, l)]
  MenuDrag,
  #[params(l)]
  MenuGetObject,
  #[params(w, l)]
  MenuRButtonUp,
  #[params(w, l)]
  MenuSelect,
  #[params(w, l)]
  MouseActivate,
  #[params(w, l)]
  MouseHWheel,
  #[id(0x02A1)]
  #[params(w, l)]
  MouseHover,
  #[id(0x02A3)]
  MouseLeave,
  #[params(w, l)]
  MouseMove,
  #[params(w, l)]
  MouseWheel,
  #[params(l)]
  Move,
  #[params(l)]
  Moving,
  #[params(w, l)]
  NcActivate,
  #[params(w, l)]
  NcCalcSize,
  #[params(l)]
  NcCreate,
  NcDestroy,
  #[params(l)]
  NcHitTest,
  #[params(w, l)]
  NcLButtonDblClk,
  #[params(w, l)]
  NcLButtonDown,
  #[params(w, l)]
  NcLButtonUp,
  #[params(w, l)]
  NcMButtonDblClk,
  #[params(w, l)]
  NcMButtonDown,
  #[params(w, l)]
  NcMButtonUp,
  #[params(w, l)]
  NcMouseHover,
  NcMouseLeave,
  #[params(w, l)]
  NcMouseMove,
  #[params(w)]
  NcPaint,
  #[params(w, l)]
  NcPointerDown,
  #[params(w, l)]
  NcPointerUp,
  #[params(w, l)]
  NcPointerUpdate,
  #[params(w, l)]
  NcRButtonDblClk,
  #[params(w, l)]
  NcRButtonDown,
  #[params(w, l)]
  NcRButtonUp,
  #[params(w, l)]
  NcXButtonDblClk,
  #[params(w, l)]
  NcXButtonDown,
  #[params(w, l)]
  NcXButtonUp,
  #[params(w, l)]
  NextDlgCtl,
  #[params(w, l)]
  NextMenu,
  #[params(w, l)]
  Notify,
  #[params(w, l)]
  NotifyFormat,
  Paint,
  #[params(w, l)]
  PaintClipboard,
  PaintIcon,
  #[params(w)]
  PaletteChanged,
  #[params(w)]
  PaletteIsChanging,
  #[params(w, l)]
  ParentNotify,
  Paste,
  #[params(w, l)]
  PointerActivate,
  #[params(w, l)]
  PointerCaptureChanged,
  #[params(w, l)]
  PointerDeviceChange,
  #[params(w, l)]
  PointerDeviceInRange,
  #[params(w, l)]
  PointerDeviceOutOfRange,
  #[params(w, l)]
  PointerDown,
  #[params(w, l)]
  PointerEnter,
  #[params(w, l)]
  PointerHWheel,
  #[params(w, l)]
  PointerLeave,
  PointerRoutedAway,
  PointerRoutedReleased,
  PointerRoutedTo,
  #[params(w, l)]
  PointerUp,
  #[params(w, l)]
  PointerUpdate,
  #[params(w, l)]
  PointerWheel,
  #[params(w)]
  Power,
  #[params(w, l)]
  PowerBroadcast,
  #[params(w, l)]
  Print,
  #[params(w, l)]
  PrintClient,
  QueryDragIcon,
  #[params(l)]
  QueryEndSession,
  QueryNewPalette,
  QueryOpen,
  QueryUiState,
  QueueSync,
  #[params(w)]
  Quit,
  #[params(w, l)]
  RButtonDblClk,
  #[params(w, l)]
  RButtonDown,
  #[params(w, l)]
  RButtonUp,
  RenderAllFormats,
  #[params(w)]
  RenderFormat,
  #[params(w, l)]
  SetCursor,
  #[params(w)]
  SetFocus,
  #[params(w, l)]
  SetFont,
  #[params(w)]
  SetHotKey,
  #[params(w, l)]
  SetIcon,
  #[params(w)]
  SetRedraw,
  #[params(l)]
  SetText,
  #[params(w, l)]
  SettingChange,
  #[params(w, l)]
  ShowWindow,
  #[params(w, l)]
  Size,
  #[params(w, l)]
  SizeClipboard,
  #[params(w, l)]
  Sizing,
  #[params(w, l)]
  SpoolerStatus,
  #[params(w, l)]
  StyleChanged,
  #[params(w, l)]
  StyleChanging,
  SyncPaint,
  #[params(w, l)]
  SysChar,
  SysColorChange,
  #[params(w, l)]
  SysCommand,
  #[params(w, l)]
  SysDeadChar,
  #[params(w, l)]
  SysKeyDown,
  #[params(w, l)]
  SysKeyUp,
  #[params(w, l)]
  TCard,
  ThemeChanged,
  TimeChange,
  #[params(w, l)]
  Timer,
  #[params(w, l)]
  TooltipDismiss,
  #[params(w, l)]
  Touch,
  #[params(l)]
  TouchHitTesting,
  Undo,
  #[params(w, l)]
  UniChar,
  #[params(w, l)]
  UninitMenuPopup,
  #[params(w)]
  UpdateUiState,
  UserChanged,
  #[params(w, l)]
  VKeyToItem,
  #[params(w, l)]
  VScroll,
  #[params(w, l)]
  VScrollClipboard,
  #[params(l)]
  WindowPosChanged,
  #[params(l)]
  WindowPosChanging,
  #[id(WindowsAndMessaging::WM_WTSSESSION_CHANGE)]
  #[params(w, l)]
  WtsSessionChange,
  #[params(w, l)]
  XButtonDblClk,
  #[params(w, l)]
  XButtonDown,
  #[params(w, l)]
  XButtonUp,
}

impl Message {
  pub const KEYS: RangeInclusive<u32> = WindowsAndMessaging::WM_KEYFIRST..=WindowsAndMessaging::WM_KEYLAST;
  pub const MOUSES: RangeInclusive<u32> = WindowsAndMessaging::WM_MOUSEFIRST..=WindowsAndMessaging::WM_MOUSELAST;

  //   pub fn is_key(&self) -> bool {
  //     let id_range = self.discriminant().to_id_range();
  //     Self::KEYS.start() <= id_range.end() && id_range.start() <= Self::KEYS.end()
  //   }

  //   pub fn is_mouse(&self) -> bool {
  //     let id_range = self.discriminant().to_id_range();
  //     Self::MOUSES.start() <= id_range.end() && id_range.start() <= Self::MOUSES.end()
  //   }

  //   // pub const fn is_key(&self) -> bool {
  //   //   self.id().is_key()
  //   // }

  //   // pub const fn is_mouse(&self) -> bool {
  //   //   self.id().is_mouse()
  //   // }

  //   pub const fn quit_requested(&self) -> bool {
  //     matches!(self, Message::Destroy)
  //   }
}
