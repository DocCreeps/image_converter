/// Ce module gère la conversion des images.
use image::{ImageReader, ImageFormat}; // Correction: Utilisation directe de ImageReader
use std::path::{Path, PathBuf};
use std::fs;
use std::io::BufWriter;
use walkdir::WalkDir; // Import de WalkDir

/// Mode de gestion des fichiers existants.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OverwriteMode {
    Skip,      // Ignorer si le fichier existe
    Overwrite, // Écraser le fichier existant
    Rename,    // Renommer le nouveau fichier (ex: image-1.webp)
}

/// Convertit un seul fichier image en WebP.
/// Retourne `Ok(())` en cas de succès, `Err(String)` en cas d'erreur.
pub fn convert_single_image(
    input_path: &Path,
    output_dir: &Path,
    overwrite_mode: &OverwriteMode,
) -> Result<(), String> {
    // Le parent_dir n'est pas utilisé dans convert_image_internal pour le renommage,
    // car le renommage se fait par rapport au output_dir déjà.
    convert_image_internal(input_path, output_dir, overwrite_mode)
}

/// Convertit plusieurs fichiers image en WebP.
pub fn convert_multiple_files(
    input_paths: &[PathBuf],
    output_dir: &Path,
    overwrite_mode: &OverwriteMode,
) -> Result<(), String> {
    // Crée le répertoire de sortie.
    fs::create_dir_all(output_dir).map_err(|e| format!("Échec de la création du répertoire de sortie : {}", e))?;

    for path in input_paths {
        convert_image_internal(path, output_dir, overwrite_mode)?;
    }
    Ok(())
}

/// Convertit toutes les images d'un répertoire et de ses sous-répertoires en WebP.
pub fn convert_images_in_directory(
    input_dir: &Path,
    output_base_dir: &Path, // Nouveau: Le répertoire racine où les sorties doivent être créées
    current_walk_dir: &Path, // Le répertoire actuellement traversé par walkdir
    overwrite_mode: &OverwriteMode,
) -> Result<(), String> {
    // Crée le répertoire de sortie de base s'il n'existe pas
    fs::create_dir_all(output_base_dir)
        .map_err(|e| format!("Impossible de créer le répertoire de sortie {}: {}", output_base_dir.display(), e))?;

    for entry in WalkDir::new(current_walk_dir) {
        let entry = entry.map_err(|e| format!("Erreur lors de la lecture du répertoire: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            // Vérifier si l'extension est celle d'une image supportée
            if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                if ["png", "jpg", "jpeg", "bmp"].contains(&extension.to_lowercase().as_str()) {
                    // Calculer le chemin de sortie relatif par rapport à input_dir
                    let relative_path = path.strip_prefix(input_dir)
                        .map_err(|e| format!("Erreur de chemin relatif : {}", e))?;

                    let output_file_dir = output_base_dir.join(relative_path.parent().unwrap_or_else(|| Path::new("")));

                    // S'assurer que le sous-répertoire de sortie existe
                    fs::create_dir_all(&output_file_dir)
                        .map_err(|e| format!("Impossible de créer le sous-répertoire de sortie {}: {}", output_file_dir.display(), e))?;

                    convert_image_internal(path, &output_file_dir, overwrite_mode)?;
                }
            }
        }
    }
    Ok(())
}

/// Fonction interne pour la logique de conversion unique, incluant la gestion du mode d'écrasement.
fn convert_image_internal(
    input_path: &Path,
    output_dir: &Path,
    overwrite_mode: &OverwriteMode,
) -> Result<(), String> {
    let image_name = input_path.file_stem().ok_or("Nom de fichier invalide")?;
    let mut output_file_name = format!("{}.webp", image_name.to_string_lossy());
    let mut output_full_path = output_dir.join(&output_file_name);

    match overwrite_mode {
        OverwriteMode::Skip => {
            if output_full_path.exists() {
                println!("Skipping existing file: {}", output_full_path.display());
                return Ok(()); // Ne rien faire si le fichier existe déjà
            }
        }
        OverwriteMode::Rename => {
            let mut counter = 1;
            while output_full_path.exists() {
                output_file_name = format!("{}-{}.webp", image_name.to_string_lossy(), counter);
                output_full_path = output_dir.join(&output_file_name);
                counter += 1;
            }
        }
        OverwriteMode::Overwrite => {
            // Pas d'action spécifique, le fichier sera écrasé par défaut
        }
    }

    let img = ImageReader::open(input_path)
        .map_err(|e| format!("Impossible d'ouvrir l'image {}: {}", input_path.display(), e))?
        .decode()
        .map_err(|e| format!("Impossible de décoder l'image {}: {}", input_path.display(), e))?;

    let file = fs::File::create(&output_full_path)
        .map_err(|e| format!("Impossible de créer le fichier de sortie {}: {}", output_full_path.display(), e))?;
    let mut writer = BufWriter::new(file);

    img.write_to(&mut writer, ImageFormat::WebP)
        .map_err(|e| format!("Impossible d'écrire l'image WebP dans {}: {}", output_full_path.display(), e))?;

    Ok(())
}
