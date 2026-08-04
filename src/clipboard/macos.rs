use rusqlite::Connection;
use slint::{ModelRc, SharedString, VecModel, Weak};
use std::rc::Rc;

use crate::db::{self, ClipboardItem};
use crate::AppWindow;

use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use arboard::Clipboard;

use std::process::{Command, Stdio};
use std::io::Write;

pub fn start_listening(ui_handle: Weak<AppWindow>) {
    struct ClipboardListener {
        ui: Weak<AppWindow>,
    }

    impl ClipboardHandler for ClipboardListener {
        fn on_clipboard_change(&mut self) -> CallbackResult {
            if let Ok(mut clipboard) = Clipboard::new() {
                if let Ok(text) = clipboard.get_text() {
                    if !text.trim().is_empty() {
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
                                let ui_clone = self.ui.clone();
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
            CallbackResult::Next
        }
        
        fn on_clipboard_error(&mut self, _error: std::io::Error) -> CallbackResult {
            CallbackResult::Next
        }
    }

    std::thread::spawn(move || {
        println!("Écouteur (Mode macOS) démarré via événements natifs...");
        let mut master = Master::new(ClipboardListener { ui: ui_handle }).unwrap();
        master.run().unwrap();
    });
}

pub fn write_text(text: String) {
    if let Ok(mut child) = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn() 
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
    }
    println!("Texte copié dans le presse-papier (macOS).");
}