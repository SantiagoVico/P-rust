mod db;
mod autostart;

use rdev::{listen, Event, EventType, Key};
use std::thread;
use std::time::Duration;
use std::process::Command;
use std::net::TcpStream;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Enregistrement automatique au démarrage
    autostart::init_autostart().ok();

    // 2. Initialisation de la base de données et nettoyage
    let conn = db::init_db().expect("Échec de l'initialisation de la base de données");
    let _ = db::garbage_collect(&conn, 100);
    drop(conn);

    // 3. Thread d'arrière-plan avec Fallback Wayland natif
    thread::spawn(|| {
        let mut last_text = String::new();
        println!("[Daemon] Démarrage de l'écoute du presse-papier (Compatible Wayland/macOS)...");
        
        loop {
            let mut current_text = None;

            // Tentative 1 : Via arboard (Fonctionne parfaitement sur macOS et X11)
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if let Ok(text) = clipboard.get_text() {
                    current_text = Some(text);
                }
            }

            // Tentative 2 : Fallback spécifique pour Linux/Wayland (KDE Plasma)
            #[cfg(target_os = "linux")]
            if current_text.is_none() {
                if let Ok(output) = Command::new("wl-paste").output() {
                    if output.status.success() {
                        if let Ok(text) = String::from_utf8(output.stdout) {
                            current_text = Some(text);
                        }
                    }
                }
            }

            // Si on a réussi à récupérer du texte (peu importe la méthode)
            if let Some(text) = current_text {
                if text != last_text && !text.trim().is_empty() {
                    println!("[Daemon] Texte copié détecté : {}", text);
                    last_text = text.clone();
                    
                    let item = db::ClipboardItem {
                        id: 0,
                        content_type: "text".to_string(),
                        content: Some(text),
                        media_path: None,
                        source_app: None,
                        is_pinned: false,
                    };

                    if let Ok(conn) = db::init_db() {
                        if let Err(e) = db::insert_item(&conn, &item) {
                            eprintln!("[Daemon] Erreur insertion DB : {:?}", e);
                        } else {
                            println!("[Daemon] Texte inséré avec succès dans la DB !");
                        }
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(800));
        }
    });

    // 4. Écouteur global de raccourcis (Meta + Shift + V)
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
                    // On essaie de contacter l'UI existante via TCP
                    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:48292") {
                        let _ = stream.write_all(b"show");
                    } else {
                        // Si l'UI ne tourne pas, on la lance
                        if let Ok(current_exe) = std::env::current_exe() {
                            if let Some(dir) = current_exe.parent() {
                                let ui_binary = dir.join("p-rust-ui");
                                let _ = Command::new(ui_binary).spawn();
                            }
                        }
                    }
                }
            },
            _ => (),
        }
    };

    if let Err(error) = listen(callback) {
        eprintln!("Erreur de l'écouteur global : {:?}", error);
    }

    Ok(())
}