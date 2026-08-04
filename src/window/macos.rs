use slint::{Weak, CloseRequestResponse, ComponentHandle};
use crate::AppWindow;
use rdev::{listen, Event, EventType, Key};

pub fn setup_behavior(ui: &AppWindow, ui_handle: Weak<AppWindow>) {
    // 1. Interception de la croix rouge pour cacher au lieu de tuer
    let ui_weak = ui_handle.clone();
    ui.window().on_close_requested(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let _ = ui.window().hide();
        }
        CloseRequestResponse::KeepWindowShown
    });

    // 2. Écouteur de raccourcis globaux (Cmd + Shift + V pour afficher, Echap pour cacher)
    std::thread::spawn(move || {
        let mut cmd_pressed = false;
        let mut shift_pressed = false;

        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(Key::MetaLeft) | EventType::KeyPress(Key::MetaRight) => cmd_pressed = true,
                EventType::KeyRelease(Key::MetaLeft) | EventType::KeyRelease(Key::MetaRight) => cmd_pressed = false,
                EventType::KeyPress(Key::ShiftLeft) | EventType::KeyPress(Key::ShiftRight) => shift_pressed = true,
                EventType::KeyRelease(Key::ShiftLeft) | EventType::KeyRelease(Key::ShiftRight) => shift_pressed = false,
                
                EventType::KeyPress(Key::KeyV) => {
                    if cmd_pressed && shift_pressed {
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