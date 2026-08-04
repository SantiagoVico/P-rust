use slint::Weak;
use crate::AppWindow;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

pub fn start_listening(ui_handle: Weak<AppWindow>) {
    #[cfg(target_os = "linux")]
    linux::start_listening(ui_handle);

    #[cfg(target_os = "macos")]
    macos::start_listening(ui_handle);
}

pub fn write_text(text: String) {
    #[cfg(target_os = "linux")]
    linux::write_text(text);

    #[cfg(target_os = "macos")]
    macos::write_text(text);
}