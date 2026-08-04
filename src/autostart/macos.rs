use std::fs;
use std::path::PathBuf;

pub fn init_autostart() -> std::io::Result<()> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Dossier utilisateur introuvable")
    })?;
    
    let launch_agents_dir = home_dir.join("Library").join("LaunchAgents");
    fs::create_dir_all(&launch_agents_dir)?;

    let plist_path = launch_agents_dir.join("com.p-rust.app.plist");

    if !plist_path.exists() {
        let current_exe = std::env::current_exe()?
            .to_string_lossy()
            .to_string();

        let plist_content = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
            <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
            <plist version=\"1.0\">\n\
            <dict>\n\
                <key>Label</key>\n\
                <string>com.p-rust.app</string>\n\
                <key>ProgramArguments</key>\n\
                <array>\n\
                    <string>{}</string>\n\
                </array>\n\
                <key>RunAtLoad</key>\n\
                <true/>\n\
            </dict>\n\
            </plist>\n",
            current_exe
        );

        fs::write(plist_path, plist_content)?;
        println!("Fichier d'autostart macOS généré avec succès !");
    }

    Ok(())
}