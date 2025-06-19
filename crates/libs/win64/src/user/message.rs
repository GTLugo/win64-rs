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
  AfxFirst,
  AfxLast,
  #[params(w, l)]
  AppCommand,
  #[params(w, l)]
  AskCbFormatName,
  CancelJournal,
  CancelMode,
  #[params(l)]
  CaptureChanged,
  ChangeCbChain,
  ChangeUiState,
  Char,
  CharToItem,
  ChildActivate,
  Clear,
  ClipboardUpdate,
  Close,
  Command,
  CommNotify,
  Compacting,
  CompareItem,
  ContextMenu,
  Copy,
  CopyData,
  Create,
  CtlColorBtn,
  CtlColorDlg,
  CtlColorEdit,
  CtlColorListBox,
  CtlColorMsgBox,
  CtlColorScrollBar,
  CtlColorStatic,
  Cut,
  DeadChar,
  DeleteItem,
  Destroy,
  DestroyClipboard,
  DeviceChange,
  DevModeChange,
  DisplayChange,
  DpiChanged,
  #[id(WindowsAndMessaging::WM_DPICHANGED_AFTERPARENT)]
  DpiChangedAfterParent,
  #[id(WindowsAndMessaging::WM_DPICHANGED_BEFOREPARENT)]
  DpiChangedBeforeParent,
  DrawClipboard,
  DrawItem,
  DropFiles,
  DwmColorizationColorChanged,
  DwmCompositionChanged,
  DwmNcRenderingChanged,
  DwmSendIconicLivePreviewBitmap,
  DwmSendIconicThumbnail,
  DwmWindowMaximizedChange,
  Enable,
  EndSession,
  EnterIdle,
  EnterMenuLoop,
  EnterSizeMove,
  EraseBkgnd,
  ExitMenuLoop,
  ExitSizeMove,
  FontChange,
  Gesture,
  GestureNotify,
  GetDlgCode,
  GetDpiScaledSize,
  GetFont,
  #[id(WindowsAndMessaging::MN_GETHMENU)]
  GetHMenu,
  GetHotKey,
  GetIcon,
  GetMinMaxInfo,
  GetObject,
  GetText,
  GetTextLength,
  GetTitleBarInfoEx,
  HandheldFirst,
  HandheldLast,
  Help,
  HotKey,
  HScroll,
  HScrollClipboard,
  IconEraseBkgnd,
  #[id(WindowsAndMessaging::WM_IME_CHAR)]
  ImeChar,
  #[id(WindowsAndMessaging::WM_IME_COMPOSITION)]
  ImeComposition,
  #[id(WindowsAndMessaging::WM_IME_COMPOSITIONFULL)]
  ImeCompositionFull,
  #[id(WindowsAndMessaging::WM_IME_CONTROL)]
  ImeControl,
  #[id(WindowsAndMessaging::WM_IME_ENDCOMPOSITION)]
  ImeEndComposition,
  #[id(WindowsAndMessaging::WM_IME_KEYDOWN)]
  ImeKeyDown,
  #[id(WindowsAndMessaging::WM_IME_KEYUP)]
  ImeKeyUp,
  #[id(WindowsAndMessaging::WM_IME_NOTIFY)]
  ImeNotify,
  #[id(WindowsAndMessaging::WM_IME_REQUEST)]
  ImeRequest,
  #[id(WindowsAndMessaging::WM_IME_SELECT)]
  ImeSelect,
  #[id(WindowsAndMessaging::WM_IME_SETCONTEXT)]
  ImeSetContext,
  #[id(WindowsAndMessaging::WM_IME_STARTCOMPOSITION)]
  ImeStartComposition,
  InitDialog,
  InitMenu,
  InitMenuPopup,
  Input,
  InputLangChange,
  InputLangChangeRequest,
  #[id(WindowsAndMessaging::WM_INPUT_DEVICE_CHANGE)]
  InputDeviceChange,
  KeyDown,
  KeyUp,
  KillFocus,
  LButtonDblClk,
  LButtonDown,
  LButtonUp,
  MButtonDblClk,
  MButtonDown,
  MButtonUp,
  MdiActivate,
  MdiCascade,
  MdiCreate,
  MdiDestroy,
  MdiGetActive,
  MdiIconArrange,
  MdiMaximize,
  MdiNext,
  MdiRefreshMenu,
  MdiRestore,
  MdiSetMenu,
  MdiTile,
  MeasureItem,
  MenuChar,
  MenuCommand,
  MenuDrag,
  MenuGetObject,
  MenuRButtonUp,
  MenuSelect,
  MouseActivate,
  MouseHWheel,
  MouseMove,
  MouseWheel,
  Move,
  Moving,
  NcActivate,
  NcCalcSize,
  NcCreate,
  NcDestroy,
  NcHitTest,
  NcLButtonDblClk,
  NcLButtonDown,
  NcLButtonUp,
  NcMButtonDblClk,
  NcMButtonDown,
  NcMButtonUp,
  NcMouseHover,
  NcMouseLeave,
  NcMouseMove,
  NcPaint,
  NcPointerDown,
  NcPointerUp,
  NcPointerUpdate,
  NcRButtonDblClk,
  NcRButtonDown,
  NcRButtonUp,
  NcXButtonDblClk,
  NcXButtonDown,
  NcXButtonUp,
  NextDlgCtl,
  NextMenu,
  Notify,
  NotifyFormat,
  Paint,
  PaintClipboard,
  PaintIcon,
  PaletteChanged,
  PaletteIsChanging,
  ParentNotify,
  Paste,
  PenWinFirst,
  PenWinLast,
  PointerActivate,
  PointerCaptureChanged,
  PointerDeviceChange,
  PointerDeviceInRange,
  PointerDeviceOutOfRange,
  PointerDown,
  PointerEnter,
  PointerHWheel,
  PointerLeave,
  PointerRoutedAway,
  PointerRoutedReleased,
  PointerRoutedTo,
  PointerUp,
  PointerUpdate,
  PointerWheel,
  Power,
  PowerBroadcast,
  Print,
  PrintClient,
  QueryDragIcon,
  QueryEndSession,
  QueryNewPalette,
  QueryOpen,
  QueryUiState,
  QueueSync,
  Quit,
  RButtonDblClk,
  RButtonDown,
  RButtonUp,
  RenderAllFormats,
  RenderFormat,
  SetCursor,
  SetFocus,
  SetFont,
  SetHotKey,
  SetIcon,
  SetRedraw,
  SetText,
  SettingChange,
  ShowWindow,
  Size,
  SizeClipboard,
  Sizing,
  SpoolerStatus,
  StyleChanged,
  StyleChanging,
  SyncPaint,
  SysChar,
  SysColorChange,
  SysCommand,
  SysDeadChar,
  SysKeyDown,
  SysKeyUp,
  #[id(WindowsAndMessaging::WM_TABLET_FIRST)]
  TabletFirst,
  #[id(WindowsAndMessaging::WM_TABLET_LAST)]
  TabletLast,
  TCard,
  ThemeChanged,
  TimeChange,
  Timer,
  TooltipDismiss,
  Touch,
  TouchHitTesting,
  Undo,
  UniChar,
  UninitMenuPopup,
  UpdateUiState,
  UserChanged,
  VKeyToItem,
  VScroll,
  VScrollClipboard,
  WindowPosChanged,
  WindowPosChanging,
  #[id(WindowsAndMessaging::WM_WTSSESSION_CHANGE)]
  WtsSessionChange,
  XButtonDblClk,
  XButtonDown,
  XButtonUp,
}

/*
 TODO:
   Look into inverting this so that the Message enum is the one generated via attr
   that way the individual MessageType variants are easier to handle along with the different
   wparam vs lparam vs none requirements. The params would become attrs.
*/

impl Message {
  //   pub const KEYS: RangeInclusive<u32> = WindowsAndMessaging::WM_KEYFIRST..=WindowsAndMessaging::WM_KEYLAST;
  //   pub const MOUSES: RangeInclusive<u32> = WindowsAndMessaging::WM_MOUSEFIRST..=WindowsAndMessaging::WM_MOUSELAST;

  //   #[inline]
  //   pub fn msg(&self) -> MessageType {
  //     self.discriminant()
  //   }

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
