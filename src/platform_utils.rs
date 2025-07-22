/// Ce module contient des utilitaires spécifiques à la plateforme et des fonctions de validation de chemin.
use std::path::{Path, PathBuf};
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

/// Valide que le répertoire de sortie n'est pas le même que le répertoire d'entrée,
/// ni un sous-répertoire de celui-ci.
pub fn validate_paths(input_path: &Path, output_path: &Path) -> Result<(), String> {
    // Résoudre les chemins pour obtenir les chemins absolus et canoniques
    let canonical_input = input_path.canonicalize().map_err(|e| format!("Impossible de canoniser le chemin d'entrée : {}", e))?;
    let canonical_output = output_path.canonicalize().map_err(|e| format!("Impossible de canoniser le chemin de sortie : {}", e))?;

    if canonical_input == canonical_output {
        return Err("Le répertoire de sortie ne peut pas être le même que le répertoire d'entrée.".to_string());
    }

    // Vérifier si le répertoire de sortie est un sous-répertoire de l'entrée
    if canonical_output.starts_with(&canonical_input) {
        return Err("Le répertoire de sortie ne peut pas être un sous-répertoire du répertoire d'entrée.".to_string());
    }

    Ok(())
}
