use std::process::{Command, Stdio};
use std::time::Duration;
use std::io::Write;
use std::rc::Rc;

use slint::{ModelRc, SharedString, VecModel, Weak};

use rusqlite::Connection;
use crate::db::{self, ClipboardItem};
use crate::AppWindow;

pub fn start_listening(ui_handle: Weak<AppWindow>) {
    std::thread::spawn(move || {
        println!("Écouteur (Mode Fedora/Wayland) démarré via wl-paste...");
        let mut last_text = String::new();

        loop {
            if let Ok(output) = Command::new("wl-paste").arg("--no-newline").output() {
                if let Ok(text) = String::from_utf8(output.stdout) {
                    if text != last_text && !text.trim().is_empty() {
                        last_text = text.clone();
                        println!("Nouveau texte : {}...", &text.chars().take(20).collect::<String>());
                        
                        if let Ok(conn) = Connection::open("clipboard_history.db") {
                            let item = ClipboardItem {
                                id: 0,
                                content_type: "text".to_string(),
                                content: Some(text),
                                media_path: None,
                                source_app: None,
                                is_pinned: false,
                            };
                            
                            if db::insert_item(&conn, &item).is_ok() {
                                let ui_clone = ui_handle.clone();
                                
                                let _ = slint::invoke_from_event_loop(move || {
                                    if let Some(ui) = ui_clone.upgrade() {
                                        if let Ok(conn_ui) = Connection::open("clipboard_history.db") {
                                            let items = db::get_recent_texts(&conn_ui, 20).unwrap_or_default();
                                            let slint_items: Vec<SharedString> = items.into_iter().map(|s| s.into()).collect();
                                            ui.set_history(ModelRc::from(Rc::new(VecModel::from(slint_items))));
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    });
}

pub fn write_text(text: String) {
    if let Ok(mut child) = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn() 
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
    }
    println!("Texte copié dans le presse-papier (Wayland).");
}