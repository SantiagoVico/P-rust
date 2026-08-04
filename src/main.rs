slint::include_modules!();

use slint::{ModelRc, VecModel};
use std::rc::Rc;

mod db;
mod clipboard;
mod window;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialisation de la connexion principale à la base de données
    let conn = db::init_db().expect("Échec de l'initialisation de la base de données");
    
    // Création de l'interface graphique Slint
    let ui = AppWindow::new()?;

    // Chargement de l'historique initial (les 20 derniers éléments)
    let items = db::get_recent_texts(&conn, 20).unwrap_or_default();
    
    // Conversion des textes standards en chaînes compatibles avec Slint (SharedString)
    let slint_items: Vec<slint::SharedString> = items.into_iter().map(|s| s.into()).collect();
    let history_model = Rc::new(VecModel::from(slint_items));
    ui.set_history(ModelRc::from(history_model.clone()));

    // Démarrage de l'écouteur du presse-papier système en arrière-plan
    clipboard::start_listening(ui.as_weak());
    
    // Configuration de la fenêtre (transparence, positionnement, etc.) selon l'OS
    window::setup_window_behavior(&ui, ui.as_weak());

    // --- CONNEXION DES ÉVÉNEMENTS DE L'INTERFACE ---

    // Événement : Clic sur le bouton "Copier"
    ui.on_copy_item({
        move |text| {
            // Envoie le texte au presse-papier du système (Wayland/macOS)
            clipboard::write_text(text.to_string());
        }
    });

    // Préparation d'un pointeur faible (weak handle) vers l'interface.
    // Cela permet de mettre à jour l'UI depuis le bloc de suppression sans bloquer la mémoire.
    let ui_handle = ui.as_weak();

    // Événement : Clic sur le bouton "Supprimer"
    ui.on_delete_item({
        move |_index, text| {
            if let Ok(local_conn) = db::init_db() {
                
                // 1. Suppression de l'entrée textuelle dans la base
                if let Err(e) = db::delete_item(&local_conn, text.as_str()) {
                    eprintln!("Erreur lors de la suppression en DB : {}", e);
                    return;
                }
                println!("Élément supprimé avec succès.");
                
                // 2. Récupération de la liste mise à jour (les 50 derniers éléments)
                if let Ok(new_history) = db::get_recent_texts(&local_conn, 50) {
                    
                    // --- CORRECTION ICI : Conversion des String en SharedString ---
                    let slint_items: Vec<slint::SharedString> = new_history
                        .into_iter()
                        .map(|s| s.into())
                        .collect();

                    // 3. Mise à jour de l'interface graphique via le pointeur faible
                    if let Some(ui_instance) = ui_handle.upgrade() {
                        let slint_model = Rc::new(VecModel::from(slint_items));
                        ui_instance.set_history(ModelRc::from(slint_model));
                    }
                }
            } else {
                eprintln!("Impossible d'ouvrir la base de données pour la suppression.");
            }
        }
    });
    
    // Démarrage de la boucle principale de l'interface utilisateur (bloquante)
    ui.run()?;
    
    Ok(())
}