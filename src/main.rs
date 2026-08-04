slint::include_modules!();

use slint::{ModelRc, VecModel};
use std::rc::Rc;

mod db;
mod clipboard;
mod window;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialisation de la base de données
    let conn = db::init_db().expect("Échec de l'initialisation de la base de données");
    let ui = AppWindow::new()?;

    // Chargement de l'historique initial
    let items = db::get_recent_texts(&conn, 20).unwrap_or_default();
    let slint_items: Vec<slint::SharedString> = items.into_iter().map(|s| s.into()).collect();
    let history_model = Rc::new(VecModel::from(slint_items));
    ui.set_history(ModelRc::from(history_model.clone()));

    // Démarrage de l'écouteur de presse-papier
    clipboard::start_listening(ui.as_weak());
    
    // Configuration de la fenêtre et des comportements spécifiques à l'OS
    window::setup_window_behavior(&ui, ui.as_weak());

    // Connexion des signaux de l'interface utilisateur aux actions appropriées
    ui.on_copy_item({
        move |text| {
            clipboard::write_text(text.to_string());
        }
    });
    
    // Démarrage de l'interface utilisateur
    ui.run()?;
    Ok(())
}