slint::include_modules!();

mod db;
mod clipboard;
mod hotkeys;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database initialization
    let _conn = db::init_db().expect("Échec de l'initialisation de la base de données");
    println!("Base de données initialisée avec succès !");

    // Ui initialization and run app loop
    let ui = AppWindow::new()?;
    
    ui.run()?;
    Ok(())
}