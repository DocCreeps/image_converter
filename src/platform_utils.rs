/// Ce module contient des utilitaires spécifiques à la plateforme.
use std::path::PathBuf;
use std::process::Command;

/// Ouvre le répertoire de sortie en utilisant la commande appropriée pour le système d'exploitation.
/// Supporte Windows, macOS et Linux.
pub fn open_output_directory(path: &PathBuf) -> Result<(), String> {
    let result = if cfg!(target_os = "windows") {
        Command::new("explorer")
            .arg(path.to_str().ok_or("Chemin invalide pour explorer")?)
            .spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(path.to_str().ok_or("Chemin invalide pour open")?)
            .spawn()
    } else {
        // Pour Linux et autres Unix-like
        Command::new("xdg-open")
            .arg(path.to_str().ok_or("Chemin invalide pour xdg-open")?)
            .spawn()
    };

    result.map_err(|e| format!("Échec de l'ouverture du dossier : {}", e))?;
    Ok(())
}
