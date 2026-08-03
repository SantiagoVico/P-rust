slint::include_modules!();

use slint::{ModelRc, VecModel};
use std::rc::Rc;

mod db;
mod clipboard;
mod hotkeys;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialisation de la base de données
    let conn = db::init_db().expect("Échec de l'initialisation de la base de données");

    // 2. Création de l'interface
    let ui = AppWindow::new()?;
    
    // 3. Premier chargement de l'historique
    let items = db::get_recent_texts(&conn, 20).unwrap_or_default();
    let slint_items: Vec<slint::SharedString> = items.into_iter().map(|s| s.into()).collect();
    let history_model = Rc::new(VecModel::from(slint_items));
    ui.set_history(ModelRc::from(history_model.clone()));

    // 4. Démarrage de l'écouteur avec un pointeur vers l'interface
    clipboard::start_listening(ui.as_weak());
    
    // 5. Lancement de la boucle principale
    ui.run()?;
    
    Ok(())
}