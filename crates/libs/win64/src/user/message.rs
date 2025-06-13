use std::ops::RangeInclusive;

use strum::{EnumDiscriminants, IntoDiscriminant};
use win64_macro::{FromRaw, Id};
use windows_sys::Win32::UI::WindowsAndMessaging;

pub mod data;
// pub mod id;
// pub mod pump;
// pub mod thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WParam(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LParam(pub isize);

#[derive(Debug, Clone, PartialEq, Eq, EnumDiscriminants, FromRaw)]
#[strum_discriminants(name(MessageType))]
#[strum_discriminants(derive(Id))]
pub enum Message {
  Null(WParam, LParam),
  // #[strum_discriminants(id(0x0000..=0x03FF))]
  Reserved(u32, WParam, LParam),
  #[strum_discriminants(id_range(0xC000..=0xFFFF))]
  Other(u32, WParam, LParam),
  #[strum_discriminants(id_range(WindowsAndMessaging::WM_USER..=0x7FFF))]
  User(u32, WParam, LParam),
  #[strum_discriminants(id_range(WindowsAndMessaging::WM_APP..=0xBFFF))]
  App(u32, WParam, LParam),
  Activate(WParam, LParam),
  ActivateApp(WParam, LParam),
  AfxFirst(WParam, LParam),
  AfxLast(WParam, LParam),
  AppCommand(WParam, LParam),
  AskCbFormatName(WParam, LParam),
  CancelJournal(WParam, LParam),
  CancelMode(WParam, LParam),
  CaptureChanged(WParam, LParam),
  ChangeCbChain(WParam, LParam),
  ChangeUiState(WParam, LParam),
  Char(WParam, LParam),
  CharToItem(WParam, LParam),
  ChildActivate(WParam, LParam),
  Clear(WParam, LParam),
  ClipboardUpdate(WParam, LParam),
  Close(WParam, LParam),
  Command(WParam, LParam),
  CommNotify(WParam, LParam),
  Compacting(WParam, LParam),
  CompareItem(WParam, LParam),
  ContextMenu(WParam, LParam),
  Copy(WParam, LParam),
  CopyData(WParam, LParam),
  Create(WParam, LParam),
  CtlColorBtn(WParam, LParam),
  CtlColorDlg(WParam, LParam),
  CtlColorEdit(WParam, LParam),
  CtlColorListBox(WParam, LParam),
  CtlColorMsgBox(WParam, LParam),
  CtlColorScrollBar(WParam, LParam),
  CtlColorStatic(WParam, LParam),
  Cut(WParam, LParam),
  DeadChar(WParam, LParam),
  DeleteItem(WParam, LParam),
  Destroy(WParam, LParam),
  DestroyClipboard(WParam, LParam),
  DeviceChange(WParam, LParam),
  DevModeChange(WParam, LParam),
  DisplayChange(WParam, LParam),
  DpiChanged(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_DPICHANGED_AFTERPARENT))]
  DpiChangedAfterParent(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_DPICHANGED_BEFOREPARENT))]
  #[strum_discriminants(id_range(0xC000..0xFFFF))]
  DpiChangedBeforeParent(WParam, LParam),
  DrawClipboard(WParam, LParam),
  DrawItem(WParam, LParam),
  DropFiles(WParam, LParam),
  DwmColorizationColorChanged(WParam, LParam),
  DwmCompositionChanged(WParam, LParam),
  DwmNcRenderingChanged(WParam, LParam),
  DwmSendIconicLivePreviewBitmap(WParam, LParam),
  DwmSendIconicThumbnail(WParam, LParam),
  DwmWindowMaximizedChange(WParam, LParam),
  Enable(WParam, LParam),
  EndSession(WParam, LParam),
  EnterIdle(WParam, LParam),
  EnterMenuLoop(WParam, LParam),
  EnterSizeMove(WParam, LParam),
  EraseBkgnd(WParam, LParam),
  ExitMenuLoop(WParam, LParam),
  ExitSizeMove(WParam, LParam),
  FontChange(WParam, LParam),
  Gesture(WParam, LParam),
  GestureNotify(WParam, LParam),
  GetDlgCode(WParam, LParam),
  GetDpiScaledSize(WParam, LParam),
  GetFont(WParam, LParam),
  GetHotKey(WParam, LParam),
  GetIcon(WParam, LParam),
  GetMinMaxInfo(WParam, LParam),
  GetObject(WParam, LParam),
  GetText(WParam, LParam),
  GetTextLength(WParam, LParam),
  GetTitleBarInfoEx(WParam, LParam),
  HandheldFirst(WParam, LParam),
  HandheldLast(WParam, LParam),
  Help(WParam, LParam),
  HotKey(WParam, LParam),
  HScroll(WParam, LParam),
  HScrollClipboard(WParam, LParam),
  IconEraseBkgnd(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_CHAR))]
  ImeChar(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_COMPOSITION))]
  ImeComposition(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_COMPOSITIONFULL))]
  ImeCompositionFull(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_CONTROL))]
  ImeControl(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_ENDCOMPOSITION))]
  ImeEndComposition(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_KEYDOWN))]
  ImeKeyDown(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_KEYUP))]
  ImeKeyUp(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_NOTIFY))]
  ImeNotify(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_REQUEST))]
  ImeRequest(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_SELECT))]
  ImeSelect(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_SETCONTEXT))]
  ImeSetContext(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_STARTCOMPOSITION))]
  ImeStartComposition(WParam, LParam),
  InitDialog(WParam, LParam),
  InitMenu(WParam, LParam),
  InitMenuPopup(WParam, LParam),
  Input(WParam, LParam),
  InputLangChange(WParam, LParam),
  InputLangChangeRequest(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_INPUT_DEVICE_CHANGE))]
  InputDeviceChange(WParam, LParam),
  KeyDown(WParam, LParam),
  KeyUp(WParam, LParam),
  KillFocus(WParam, LParam),
  LButtonDblClk(WParam, LParam),
  LButtonDown(WParam, LParam),
  LButtonUp(WParam, LParam),
  MButtonDblClk(WParam, LParam),
  MButtonDown(WParam, LParam),
  MButtonUp(WParam, LParam),
  MdiActivate(WParam, LParam),
  MdiCascade(WParam, LParam),
  MdiCreate(WParam, LParam),
  MdiDestroy(WParam, LParam),
  MdiGetActive(WParam, LParam),
  MdiIconArrange(WParam, LParam),
  MdiMaximize(WParam, LParam),
  MdiNext(WParam, LParam),
  MdiRefreshMenu(WParam, LParam),
  MdiRestore(WParam, LParam),
  MdiSetMenu(WParam, LParam),
  MdiTile(WParam, LParam),
  MeasureItem(WParam, LParam),
  MenuChar(WParam, LParam),
  MenuCommand(WParam, LParam),
  MenuDrag(WParam, LParam),
  MenuGetObject(WParam, LParam),
  MenuRButtonUp(WParam, LParam),
  MenuSelect(WParam, LParam),
  MouseActivate(WParam, LParam),
  MouseHWheel(WParam, LParam),
  MouseMove(WParam, LParam),
  MouseWheel(WParam, LParam),
  Move(WParam, LParam),
  Moving(WParam, LParam),
  NcActivate(WParam, LParam),
  NcCalcSize(WParam, LParam),
  NcCreate(WParam, LParam),
  NcDestroy(WParam, LParam),
  NcHitTest(WParam, LParam),
  NcLButtonDblClk(WParam, LParam),
  NcLButtonDown(WParam, LParam),
  NcLButtonUp(WParam, LParam),
  NcMButtonDblClk(WParam, LParam),
  NcMButtonDown(WParam, LParam),
  NcMButtonUp(WParam, LParam),
  NcMouseHover(WParam, LParam),
  NcMouseLeave(WParam, LParam),
  NcMouseMove(WParam, LParam),
  NcPaint(WParam, LParam),
  NcPointerDown(WParam, LParam),
  NcPointerUp(WParam, LParam),
  NcPointerUpdate(WParam, LParam),
  NcRButtonDblClk(WParam, LParam),
  NcRButtonDown(WParam, LParam),
  NcRButtonUp(WParam, LParam),
  NcXButtonDblClk(WParam, LParam),
  NcXButtonDown(WParam, LParam),
  NcXButtonUp(WParam, LParam),
  NextDlgCtl(WParam, LParam),
  NextMenu(WParam, LParam),
  Notify(WParam, LParam),
  NotifyFormat(WParam, LParam),
  Paint(WParam, LParam),
  PaintClipboard(WParam, LParam),
  PaintIcon(WParam, LParam),
  PaletteChanged(WParam, LParam),
  PaletteIsChanging(WParam, LParam),
  ParentNotify(WParam, LParam),
  Paste(WParam, LParam),
  PenWinFirst(WParam, LParam),
  PenWinLast(WParam, LParam),
  PointerActivate(WParam, LParam),
  PointerCaptureChanged(WParam, LParam),
  PointerDeviceChange(WParam, LParam),
  PointerDeviceInRange(WParam, LParam),
  PointerDeviceOutOfRange(WParam, LParam),
  PointerDown(WParam, LParam),
  PointerEnter(WParam, LParam),
  PointerHWheel(WParam, LParam),
  PointerLeave(WParam, LParam),
  PointerRoutedAway(WParam, LParam),
  PointerRoutedReleased(WParam, LParam),
  PointerRoutedTo(WParam, LParam),
  PointerUp(WParam, LParam),
  PointerUpdate(WParam, LParam),
  PointerWheel(WParam, LParam),
  Power(WParam, LParam),
  PowerBroadcast(WParam, LParam),
  Print(WParam, LParam),
  PrintClient(WParam, LParam),
  QueryDragIcon(WParam, LParam),
  QueryEndSession(WParam, LParam),
  QueryNewPalette(WParam, LParam),
  QueryOpen(WParam, LParam),
  QueryUiState(WParam, LParam),
  QueueSync(WParam, LParam),
  Quit(WParam, LParam),
  RButtonDblClk(WParam, LParam),
  RButtonDown(WParam, LParam),
  RButtonUp(WParam, LParam),
  RenderAllFormats(WParam, LParam),
  RenderFormat(WParam, LParam),
  SetCursor(WParam, LParam),
  SetFocus(WParam, LParam),
  SetFont(WParam, LParam),
  SetHotKey(WParam, LParam),
  SetIcon(WParam, LParam),
  SetRedraw(WParam, LParam),
  SetText(WParam, LParam),
  SettingChange(WParam, LParam),
  ShowWindow(WParam, LParam),
  Size(WParam, LParam),
  SizeClipboard(WParam, LParam),
  Sizing(WParam, LParam),
  SpoolerStatus(WParam, LParam),
  StyleChanged(WParam, LParam),
  StyleChanging(WParam, LParam),
  SyncPaint(WParam, LParam),
  SysChar(WParam, LParam),
  SysColorChange(WParam, LParam),
  SysCommand(WParam, LParam),
  SysDeadChar(WParam, LParam),
  SysKeyDown(WParam, LParam),
  SysKeyUp(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_TABLET_FIRST))]
  TabletFirst(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_TABLET_LAST))]
  TabletLast(WParam, LParam),
  TCard(WParam, LParam),
  ThemeChanged(WParam, LParam),
  TimeChange(WParam, LParam),
  Timer(WParam, LParam),
  TooltipDismiss(WParam, LParam),
  Touch(WParam, LParam),
  TouchHitTesting(WParam, LParam),
  Undo(WParam, LParam),
  UniChar(WParam, LParam),
  UninitMenuPopup(WParam, LParam),
  UpdateUiState(WParam, LParam),
  UserChanged(WParam, LParam),
  VKeyToItem(WParam, LParam),
  VScroll(WParam, LParam),
  VScrollClipboard(WParam, LParam),
  WindowPosChanged(WParam, LParam),
  WindowPosChanging(WParam, LParam),
  #[strum_discriminants(id(WindowsAndMessaging::WM_WTSSESSION_CHANGE))]
  WtsSessionChange(WParam, LParam),
  XButtonDblClk(WParam, LParam),
  XButtonDown(WParam, LParam),
  XButtonUp(WParam, LParam),
}

impl Message {
  pub const KEYS: RangeInclusive<u32> = WindowsAndMessaging::WM_KEYFIRST..=WindowsAndMessaging::WM_KEYLAST;
  pub const MOUSES: RangeInclusive<u32> = WindowsAndMessaging::WM_MOUSEFIRST..=WindowsAndMessaging::WM_MOUSELAST;

  #[inline]
  pub fn msg(&self) -> MessageType {
    self.discriminant()
  }

  pub fn is_key(&self) -> bool {
    let id_range = self.discriminant().to_id_range();
    Self::KEYS.start() <= id_range.end() && id_range.start() <= Self::KEYS.end()
  }

  pub fn is_mouse(&self) -> bool {
    let id_range = self.discriminant().to_id_range();
    Self::MOUSES.start() <= id_range.end() && id_range.start() <= Self::MOUSES.end()
  }

  // pub const fn is_key(&self) -> bool {
  //   self.id().is_key()
  // }

  // pub const fn is_mouse(&self) -> bool {
  //   self.id().is_mouse()
  // }

  pub const fn quit_requested(&self) -> bool {
    matches!(self, Message::Destroy(..))
  }
}
