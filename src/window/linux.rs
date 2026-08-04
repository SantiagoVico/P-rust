use slint::{Weak, ComponentHandle};
use crate::AppWindow;

pub fn setup_behavior(_ui: &AppWindow, _ui_handle: Weak<AppWindow>) {
    println!("Configuration de la fenêtre chargée pour Linux (Wayland).");
}