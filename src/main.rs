slint::include_modules!();

use slint::{ModelRc, VecModel};
use std::rc::Rc;

mod db;
mod clipboard;
mod window;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = db::init_db().expect("Échec de l'initialisation de la base de données");
    
    // Nettoyage automatique au démarrage : on ne conserve que les 100 éléments les plus récents
    let _ = db::garbage_collect(&conn, 100);

    let ui = AppWindow::new()?;

    let items = db::get_recent_texts(&conn, 20).unwrap_or_default();
    let slint_items: Vec<slint::SharedString> = items.into_iter().map(|s| s.into()).collect();
    let history_model = Rc::new(VecModel::from(slint_items));
    ui.set_history(ModelRc::from(history_model.clone()));

    clipboard::start_listening(ui.as_weak());
    window::setup_window_behavior(&ui, ui.as_weak());

    // --- CALLBACKS ---

    ui.on_copy_item({
        move |text| {
            clipboard::write_text(text.to_string());
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_delete_item({
        move |_index, text| {
            if let Ok(local_conn) = db::init_db() {
                if let Err(e) = db::delete_item(&local_conn, text.as_str()) {
                    eprintln!("Erreur lors de la suppression en DB : {}", e);
                    return;
                }
                
                if let Ok(new_history) = db::get_recent_texts(&local_conn, 50) {
                    let slint_items: Vec<slint::SharedString> = new_history
                        .into_iter()
                        .map(|s| s.into())
                        .collect();

                    if let Some(ui_instance) = ui_handle.upgrade() {
                        let slint_model = Rc::new(VecModel::from(slint_items));
                        ui_instance.set_history(ModelRc::from(slint_model));
                    }
                }
            }
        }
    });

    let ui_handle_search = ui.as_weak();
    ui.on_search_changed(move |query| {
        if let Some(ui_instance) = ui_handle_search.upgrade() {
            if let Ok(conn) = db::init_db() {
                let items = if query.is_empty() {
                    db::get_recent_texts(&conn, 20).unwrap_or_default()
                } else {
                    db::search_texts(&conn, query.as_str(), 50).unwrap_or_default()
                };

                let slint_items: Vec<slint::SharedString> = items
                    .into_iter()
                    .map(|s| s.into())
                    .collect();
                
                let history_model = Rc::new(VecModel::from(slint_items));
                ui_instance.set_history(ModelRc::from(history_model));
            }
        }
    });
    
    ui.run()?;
    Ok(())
}