// A lot of code here is pulled from external sources including, but not limited to, `winit` and StackOverflow.

pub fn is_os_dark_mode() -> bool {
  // based on https://stackoverflow.com/a/70753913

  let key = windows_registry::CURRENT_USER
    .open(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize")
    .expect("failed to open registry key");

  let light_theme = key
    .get_u32("AppsUseLightTheme")
    .expect("failed to read value from registry key");

  light_theme == 0
}
