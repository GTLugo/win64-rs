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
  Null {
    w: WParam,
    l: LParam,
  },
  // #[strum_discriminants(id(0x0000..=0x03FF))]
  Reserved {
    msg: u32,
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id_range(0xC000..=0xFFFF))]
  Other {
    msg: u32,
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id_range(WindowsAndMessaging::WM_USER..=0x7FFF))]
  User {
    msg: u32,
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id_range(WindowsAndMessaging::WM_APP..=0xBFFF))]
  App {
    msg: u32,
    w: WParam,
    l: LParam,
  },
  Activate {
    w: WParam,
    l: LParam,
  },
  ActivateApp {
    w: WParam,
    l: LParam,
  },
  AfxFirst {
    w: WParam,
    l: LParam,
  },
  AfxLast {
    w: WParam,
    l: LParam,
  },
  AppCommand {
    w: WParam,
    l: LParam,
  },
  AskCbFormatName {
    w: WParam,
    l: LParam,
  },
  CancelJournal {
    w: WParam,
    l: LParam,
  },
  CancelMode {
    w: WParam,
    l: LParam,
  },
  CaptureChanged {
    w: WParam,
    l: LParam,
  },
  ChangeCbChain {
    w: WParam,
    l: LParam,
  },
  ChangeUiState {
    w: WParam,
    l: LParam,
  },
  Char {
    w: WParam,
    l: LParam,
  },
  CharToItem {
    w: WParam,
    l: LParam,
  },
  ChildActivate {
    w: WParam,
    l: LParam,
  },
  Clear {
    w: WParam,
    l: LParam,
  },
  ClipboardUpdate {
    w: WParam,
    l: LParam,
  },
  Close {
    w: WParam,
    l: LParam,
  },
  Command {
    w: WParam,
    l: LParam,
  },
  CommNotify {
    w: WParam,
    l: LParam,
  },
  Compacting {
    w: WParam,
    l: LParam,
  },
  CompareItem {
    w: WParam,
    l: LParam,
  },
  ContextMenu {
    w: WParam,
    l: LParam,
  },
  Copy {
    w: WParam,
    l: LParam,
  },
  CopyData {
    w: WParam,
    l: LParam,
  },
  Create {
    w: WParam,
    l: LParam,
  },
  CtlColorBtn {
    w: WParam,
    l: LParam,
  },
  CtlColorDlg {
    w: WParam,
    l: LParam,
  },
  CtlColorEdit {
    w: WParam,
    l: LParam,
  },
  CtlColorListBox {
    w: WParam,
    l: LParam,
  },
  CtlColorMsgBox {
    w: WParam,
    l: LParam,
  },
  CtlColorScrollBar {
    w: WParam,
    l: LParam,
  },
  CtlColorStatic {
    w: WParam,
    l: LParam,
  },
  Cut {
    w: WParam,
    l: LParam,
  },
  DeadChar {
    w: WParam,
    l: LParam,
  },
  DeleteItem {
    w: WParam,
    l: LParam,
  },
  Destroy {
    w: WParam,
    l: LParam,
  },
  DestroyClipboard {
    w: WParam,
    l: LParam,
  },
  DeviceChange {
    w: WParam,
    l: LParam,
  },
  DevModeChange {
    w: WParam,
    l: LParam,
  },
  DisplayChange {
    w: WParam,
    l: LParam,
  },
  DpiChanged {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_DPICHANGED_AFTERPARENT))]
  DpiChangedAfterParent {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_DPICHANGED_BEFOREPARENT))]
  #[strum_discriminants(id_range(0xC000..0xFFFF))]
  DpiChangedBeforeParent {
    w: WParam,
    l: LParam,
  },
  DrawClipboard {
    w: WParam,
    l: LParam,
  },
  DrawItem {
    w: WParam,
    l: LParam,
  },
  DropFiles {
    w: WParam,
    l: LParam,
  },
  DwmColorizationColorChanged {
    w: WParam,
    l: LParam,
  },
  DwmCompositionChanged {
    w: WParam,
    l: LParam,
  },
  DwmNcRenderingChanged {
    w: WParam,
    l: LParam,
  },
  DwmSendIconicLivePreviewBitmap {
    w: WParam,
    l: LParam,
  },
  DwmSendIconicThumbnail {
    w: WParam,
    l: LParam,
  },
  DwmWindowMaximizedChange {
    w: WParam,
    l: LParam,
  },
  Enable {
    w: WParam,
    l: LParam,
  },
  EndSession {
    w: WParam,
    l: LParam,
  },
  EnterIdle {
    w: WParam,
    l: LParam,
  },
  EnterMenuLoop {
    w: WParam,
    l: LParam,
  },
  EnterSizeMove {
    w: WParam,
    l: LParam,
  },
  EraseBkgnd {
    w: WParam,
    l: LParam,
  },
  ExitMenuLoop {
    w: WParam,
    l: LParam,
  },
  ExitSizeMove {
    w: WParam,
    l: LParam,
  },
  FontChange {
    w: WParam,
    l: LParam,
  },
  Gesture {
    w: WParam,
    l: LParam,
  },
  GestureNotify {
    w: WParam,
    l: LParam,
  },
  GetDlgCode {
    w: WParam,
    l: LParam,
  },
  GetDpiScaledSize {
    w: WParam,
    l: LParam,
  },
  GetFont {
    w: WParam,
    l: LParam,
  },
  GetHotKey {
    w: WParam,
    l: LParam,
  },
  GetIcon {
    w: WParam,
    l: LParam,
  },
  GetMinMaxInfo {
    w: WParam,
    l: LParam,
  },
  GetObject {
    w: WParam,
    l: LParam,
  },
  GetText {
    w: WParam,
    l: LParam,
  },
  GetTextLength {
    w: WParam,
    l: LParam,
  },
  GetTitleBarInfoEx {
    w: WParam,
    l: LParam,
  },
  HandheldFirst {
    w: WParam,
    l: LParam,
  },
  HandheldLast {
    w: WParam,
    l: LParam,
  },
  Help {
    w: WParam,
    l: LParam,
  },
  HotKey {
    w: WParam,
    l: LParam,
  },
  HScroll {
    w: WParam,
    l: LParam,
  },
  HScrollClipboard {
    w: WParam,
    l: LParam,
  },
  IconEraseBkgnd {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_CHAR))]
  ImeChar {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_COMPOSITION))]
  ImeComposition {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_COMPOSITIONFULL))]
  ImeCompositionFull {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_CONTROL))]
  ImeControl {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_ENDCOMPOSITION))]
  ImeEndComposition {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_KEYDOWN))]
  ImeKeyDown {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_KEYUP))]
  ImeKeyUp {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_NOTIFY))]
  ImeNotify {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_REQUEST))]
  ImeRequest {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_SELECT))]
  ImeSelect {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_SETCONTEXT))]
  ImeSetContext {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_IME_STARTCOMPOSITION))]
  ImeStartComposition {
    w: WParam,
    l: LParam,
  },
  InitDialog {
    w: WParam,
    l: LParam,
  },
  InitMenu {
    w: WParam,
    l: LParam,
  },
  InitMenuPopup {
    w: WParam,
    l: LParam,
  },
  Input {
    w: WParam,
    l: LParam,
  },
  InputLangChange {
    w: WParam,
    l: LParam,
  },
  InputLangChangeRequest {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_INPUT_DEVICE_CHANGE))]
  InputDeviceChange {
    w: WParam,
    l: LParam,
  },
  KeyDown {
    w: WParam,
    l: LParam,
  },
  KeyUp {
    w: WParam,
    l: LParam,
  },
  KillFocus {
    w: WParam,
    l: LParam,
  },
  LButtonDblClk {
    w: WParam,
    l: LParam,
  },
  LButtonDown {
    w: WParam,
    l: LParam,
  },
  LButtonUp {
    w: WParam,
    l: LParam,
  },
  MButtonDblClk {
    w: WParam,
    l: LParam,
  },
  MButtonDown {
    w: WParam,
    l: LParam,
  },
  MButtonUp {
    w: WParam,
    l: LParam,
  },
  MdiActivate {
    w: WParam,
    l: LParam,
  },
  MdiCascade {
    w: WParam,
    l: LParam,
  },
  MdiCreate {
    w: WParam,
    l: LParam,
  },
  MdiDestroy {
    w: WParam,
    l: LParam,
  },
  MdiGetActive {
    w: WParam,
    l: LParam,
  },
  MdiIconArrange {
    w: WParam,
    l: LParam,
  },
  MdiMaximize {
    w: WParam,
    l: LParam,
  },
  MdiNext {
    w: WParam,
    l: LParam,
  },
  MdiRefreshMenu {
    w: WParam,
    l: LParam,
  },
  MdiRestore {
    w: WParam,
    l: LParam,
  },
  MdiSetMenu {
    w: WParam,
    l: LParam,
  },
  MdiTile {
    w: WParam,
    l: LParam,
  },
  MeasureItem {
    w: WParam,
    l: LParam,
  },
  MenuChar {
    w: WParam,
    l: LParam,
  },
  MenuCommand {
    w: WParam,
    l: LParam,
  },
  MenuDrag {
    w: WParam,
    l: LParam,
  },
  MenuGetObject {
    w: WParam,
    l: LParam,
  },
  MenuRButtonUp {
    w: WParam,
    l: LParam,
  },
  MenuSelect {
    w: WParam,
    l: LParam,
  },
  MouseActivate {
    w: WParam,
    l: LParam,
  },
  MouseHWheel {
    w: WParam,
    l: LParam,
  },
  MouseMove {
    w: WParam,
    l: LParam,
  },
  MouseWheel {
    w: WParam,
    l: LParam,
  },
  Move {
    w: WParam,
    l: LParam,
  },
  Moving {
    w: WParam,
    l: LParam,
  },
  NcActivate {
    w: WParam,
    l: LParam,
  },
  NcCalcSize {
    w: WParam,
    l: LParam,
  },
  NcCreate {
    w: WParam,
    l: LParam,
  },
  NcDestroy {
    w: WParam,
    l: LParam,
  },
  NcHitTest {
    w: WParam,
    l: LParam,
  },
  NcLButtonDblClk {
    w: WParam,
    l: LParam,
  },
  NcLButtonDown {
    w: WParam,
    l: LParam,
  },
  NcLButtonUp {
    w: WParam,
    l: LParam,
  },
  NcMButtonDblClk {
    w: WParam,
    l: LParam,
  },
  NcMButtonDown {
    w: WParam,
    l: LParam,
  },
  NcMButtonUp {
    w: WParam,
    l: LParam,
  },
  NcMouseHover {
    w: WParam,
    l: LParam,
  },
  NcMouseLeave {
    w: WParam,
    l: LParam,
  },
  NcMouseMove {
    w: WParam,
    l: LParam,
  },
  NcPaint {
    w: WParam,
    l: LParam,
  },
  NcPointerDown {
    w: WParam,
    l: LParam,
  },
  NcPointerUp {
    w: WParam,
    l: LParam,
  },
  NcPointerUpdate {
    w: WParam,
    l: LParam,
  },
  NcRButtonDblClk {
    w: WParam,
    l: LParam,
  },
  NcRButtonDown {
    w: WParam,
    l: LParam,
  },
  NcRButtonUp {
    w: WParam,
    l: LParam,
  },
  NcXButtonDblClk {
    w: WParam,
    l: LParam,
  },
  NcXButtonDown {
    w: WParam,
    l: LParam,
  },
  NcXButtonUp {
    w: WParam,
    l: LParam,
  },
  NextDlgCtl {
    w: WParam,
    l: LParam,
  },
  NextMenu {
    w: WParam,
    l: LParam,
  },
  Notify {
    w: WParam,
    l: LParam,
  },
  NotifyFormat {
    w: WParam,
    l: LParam,
  },
  Paint {
    w: WParam,
    l: LParam,
  },
  PaintClipboard {
    w: WParam,
    l: LParam,
  },
  PaintIcon {
    w: WParam,
    l: LParam,
  },
  PaletteChanged {
    w: WParam,
    l: LParam,
  },
  PaletteIsChanging {
    w: WParam,
    l: LParam,
  },
  ParentNotify {
    w: WParam,
    l: LParam,
  },
  Paste {
    w: WParam,
    l: LParam,
  },
  PenWinFirst {
    w: WParam,
    l: LParam,
  },
  PenWinLast {
    w: WParam,
    l: LParam,
  },
  PointerActivate {
    w: WParam,
    l: LParam,
  },
  PointerCaptureChanged {
    w: WParam,
    l: LParam,
  },
  PointerDeviceChange {
    w: WParam,
    l: LParam,
  },
  PointerDeviceInRange {
    w: WParam,
    l: LParam,
  },
  PointerDeviceOutOfRange {
    w: WParam,
    l: LParam,
  },
  PointerDown {
    w: WParam,
    l: LParam,
  },
  PointerEnter {
    w: WParam,
    l: LParam,
  },
  PointerHWheel {
    w: WParam,
    l: LParam,
  },
  PointerLeave {
    w: WParam,
    l: LParam,
  },
  PointerRoutedAway {
    w: WParam,
    l: LParam,
  },
  PointerRoutedReleased {
    w: WParam,
    l: LParam,
  },
  PointerRoutedTo {
    w: WParam,
    l: LParam,
  },
  PointerUp {
    w: WParam,
    l: LParam,
  },
  PointerUpdate {
    w: WParam,
    l: LParam,
  },
  PointerWheel {
    w: WParam,
    l: LParam,
  },
  Power {
    w: WParam,
    l: LParam,
  },
  PowerBroadcast {
    w: WParam,
    l: LParam,
  },
  Print {
    w: WParam,
    l: LParam,
  },
  PrintClient {
    w: WParam,
    l: LParam,
  },
  QueryDragIcon {
    w: WParam,
    l: LParam,
  },
  QueryEndSession {
    w: WParam,
    l: LParam,
  },
  QueryNewPalette {
    w: WParam,
    l: LParam,
  },
  QueryOpen {
    w: WParam,
    l: LParam,
  },
  QueryUiState {
    w: WParam,
    l: LParam,
  },
  QueueSync {
    w: WParam,
    l: LParam,
  },
  Quit {
    w: WParam,
    l: LParam,
  },
  RButtonDblClk {
    w: WParam,
    l: LParam,
  },
  RButtonDown {
    w: WParam,
    l: LParam,
  },
  RButtonUp {
    w: WParam,
    l: LParam,
  },
  RenderAllFormats {
    w: WParam,
    l: LParam,
  },
  RenderFormat {
    w: WParam,
    l: LParam,
  },
  SetCursor {
    w: WParam,
    l: LParam,
  },
  SetFocus {
    w: WParam,
    l: LParam,
  },
  SetFont {
    w: WParam,
    l: LParam,
  },
  SetHotKey {
    w: WParam,
    l: LParam,
  },
  SetIcon {
    w: WParam,
    l: LParam,
  },
  SetRedraw {
    w: WParam,
    l: LParam,
  },
  SetText {
    w: WParam,
    l: LParam,
  },
  SettingChange {
    w: WParam,
    l: LParam,
  },
  ShowWindow {
    w: WParam,
    l: LParam,
  },
  Size {
    w: WParam,
    l: LParam,
  },
  SizeClipboard {
    w: WParam,
    l: LParam,
  },
  Sizing {
    w: WParam,
    l: LParam,
  },
  SpoolerStatus {
    w: WParam,
    l: LParam,
  },
  StyleChanged {
    w: WParam,
    l: LParam,
  },
  StyleChanging {
    w: WParam,
    l: LParam,
  },
  SyncPaint {
    w: WParam,
    l: LParam,
  },
  SysChar {
    w: WParam,
    l: LParam,
  },
  SysColorChange {
    w: WParam,
    l: LParam,
  },
  SysCommand {
    w: WParam,
    l: LParam,
  },
  SysDeadChar {
    w: WParam,
    l: LParam,
  },
  SysKeyDown {
    w: WParam,
    l: LParam,
  },
  SysKeyUp {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_TABLET_FIRST))]
  TabletFirst {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_TABLET_LAST))]
  TabletLast {
    w: WParam,
    l: LParam,
  },
  TCard {
    w: WParam,
    l: LParam,
  },
  ThemeChanged {
    w: WParam,
    l: LParam,
  },
  TimeChange {
    w: WParam,
    l: LParam,
  },
  Timer {
    w: WParam,
    l: LParam,
  },
  TooltipDismiss {
    w: WParam,
    l: LParam,
  },
  Touch {
    w: WParam,
    l: LParam,
  },
  TouchHitTesting {
    w: WParam,
    l: LParam,
  },
  Undo {
    w: WParam,
    l: LParam,
  },
  UniChar {
    w: WParam,
    l: LParam,
  },
  UninitMenuPopup {
    w: WParam,
    l: LParam,
  },
  UpdateUiState {
    w: WParam,
    l: LParam,
  },
  UserChanged {
    w: WParam,
    l: LParam,
  },
  VKeyToItem {
    w: WParam,
    l: LParam,
  },
  VScroll {
    w: WParam,
    l: LParam,
  },
  VScrollClipboard {
    w: WParam,
    l: LParam,
  },
  WindowPosChanged {
    w: WParam,
    l: LParam,
  },
  WindowPosChanging {
    w: WParam,
    l: LParam,
  },
  #[strum_discriminants(id(WindowsAndMessaging::WM_WTSSESSION_CHANGE))]
  WtsSessionChange {
    w: WParam,
    l: LParam,
  },
  XButtonDblClk {
    w: WParam,
    l: LParam,
  },
  XButtonDown {
    w: WParam,
    l: LParam,
  },
  XButtonUp {
    w: WParam,
    l: LParam,
  },
}

impl Message {
  #[inline]
  pub fn msg(&self) -> MessageType {
    self.discriminant()
  }

  // pub const fn is_key(&self) -> bool {
  //   self.id().is_key()
  // }

  // pub const fn is_mouse(&self) -> bool {
  //   self.id().is_mouse()
  // }

  // pub const fn quit_requested(&self) -> bool {
  //   matches!(self, Message::Destroy { .. })
  // }
}
