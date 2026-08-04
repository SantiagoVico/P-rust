use slint::Weak;
use crate::AppWindow;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

pub fn setup_window_behavior(ui: &AppWindow, ui_handle: Weak<AppWindow>) {
    #[cfg(target_os = "linux")]
    linux::setup_behavior(ui, ui_handle);

    #[cfg(target_os = "macos")]
    macos::setup_behavior(ui, ui_handle);
}