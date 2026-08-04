#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::init_autostart;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::init_autostart;

// Fallback pour d'autres plateformes si nécessaire
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn init_autostart() -> std::io::Result<()> {
    Ok(())
}