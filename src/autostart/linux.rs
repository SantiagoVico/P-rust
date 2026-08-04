use std::fs;
use std::path::PathBuf;

pub fn init_autostart() -> std::io::Result<()> {
    let mut autostart_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from(".config"));
    autostart_dir.push("autostart");
    fs::create_dir_all(&autostart_dir)?;

    let desktop_file_path = autostart_dir.join("p-rust.desktop");

    if !desktop_file_path.exists() {
        let current_exe = std::env::current_exe()?
            .to_string_lossy()
            .to_string();

        let desktop_content = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=Paste Clone\n\
             Exec={}\n\
             Terminal=false\n\
             X-KDE-autostart-enabled=true\n",
            current_exe
        );

        fs::write(desktop_file_path, desktop_content)?;
        println!("Fichier d'autostart KDE Plasma généré avec succès !");
    }

    Ok(())
}