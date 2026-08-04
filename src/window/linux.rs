use slint::{Weak, CloseRequestResponse, ComponentHandle};
use crate::AppWindow;
use rdev::{listen, Event, EventType, Key};

pub fn setup_behavior(ui: &AppWindow, ui_handle: Weak<AppWindow>) {
    // 1. Interception propre de la croix pour masquer la fenêtre
    let ui_weak = ui_handle.clone();
    ui.window().on_close_requested(move || {
        if let Some(ui) = ui_weak.upgrade() {
            // On cache explicitement la fenêtre
            let _ = ui.window().hide();
        }
        // On indique à Slint de ne surtout pas détruire la fenêtre ni couper l'app
        CloseRequestResponse::KeepWindowShown
    });

    // 2. Écouteur de raccourcis globaux (Super + Shift + V pour afficher, Echap pour cacher)
    std::thread::spawn(move || {
        let mut meta_pressed = false;
        let mut shift_pressed = false;

        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(Key::MetaLeft) | EventType::KeyPress(Key::MetaRight) => meta_pressed = true,
                EventType::KeyRelease(Key::MetaLeft) | EventType::KeyRelease(Key::MetaRight) => meta_pressed = false,
                EventType::KeyPress(Key::ShiftLeft) | EventType::KeyPress(Key::ShiftRight) => shift_pressed = true,
                EventType::KeyRelease(Key::ShiftLeft) | EventType::KeyRelease(Key::ShiftRight) => shift_pressed = false,
                
                EventType::KeyPress(Key::KeyV) => {
                    if meta_pressed && shift_pressed {
                        let ui_clone = ui_handle.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_clone.upgrade() {
                                let _ = ui.window().show();
                            }
                        });
                    }
                },
                EventType::KeyPress(Key::Escape) => {
                    let ui_clone = ui_handle.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_clone.upgrade() {
                            let _ = ui.window().hide();
                        }
                    });
                },
                _ => (),
            }
        };

        let _ = listen(callback);
    });
}