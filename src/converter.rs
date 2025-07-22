/// Ce module contient les fonctions responsables de la conversion d'images.
use image::ImageFormat;
use std::fs;
use std::path::PathBuf;

/// Convertit récursivement les images (PNG, JPG, JPEG, BMP) d'un répertoire en WebP.
/// Ignore les fichiers déjà convertis et conserve la structure des dossiers.
/// Retourne un Result indiquant le succès ou une erreur.
pub fn convert_images_in_directory(input_dir: &PathBuf, output_dir: &PathBuf, base_input_dir: &PathBuf) -> Result<(), String> {
    // Crée le répertoire de sortie.
    fs::create_dir_all(output_dir).map_err(|e| format!("Échec de la création du répertoire de sortie : {}", e))?;

    // Lit les entrées du répertoire d'entrée.
    let entries = fs::read_dir(input_dir).map_err(|e| format!("Échec de la lecture du répertoire : {}", e))?;

    // Parcourt chaque entrée (fichier ou dossier).
    for entry in entries {
        let entry = entry.map_err(|e| format!("Échec de la lecture de l'entrée : {}", e))?;
        let path = entry.path();

        // Traite les sous-répertoires récursivement.
        if path.is_dir() {
            let relative_path = path.strip_prefix(base_input_dir).map_err(|e| format!("Échec du calcul du chemin relatif : {}", e))?;
            let new_output_dir = output_dir.join(relative_path);
            convert_images_in_directory(&path, &new_output_dir, base_input_dir)?;
        } else if let Some(extension) = path.extension() {
            // Convertit les images PNG, JPG, JPEG, BMP en WebP.
            let extension_str = extension.to_str().ok_or("Extension de fichier invalide")?.to_lowercase();
            if is_supported_image_extension(&extension_str) {
                let output_path = output_dir.join(path.file_stem().ok_or("Nom de fichier invalide")?).with_extension("webp");

                // Ignore si le fichier WebP existe déjà.
                if output_path.exists() {
                    println!("Ignorer {}, déjà converti", path.display());
                    continue;
                }

                // Charge et sauvegarde l'image en WebP.
                let img = image::open(&path).map_err(|e| format!("Échec de l'ouverture de {} : {}", path.display(), e))?;
                img.save_with_format(&output_path, ImageFormat::WebP)
                    .map_err(|e| format!("Échec de la sauvegarde de {} : {}", output_path.display(), e))?;
                println!("Converti {} en {}", path.display(), output_path.display());
            }
        }
    }
    Ok(())
}

/// Convertit une seule image (PNG, JPG, JPEG, BMP) en WebP dans le répertoire de sortie.
/// Retourne un Result indiquant le succès ou une erreur.
pub fn convert_single_image(input_file: &PathBuf, output_dir: &PathBuf) -> Result<(), String> {
    // Crée le répertoire de sortie.
    fs::create_dir_all(output_dir).map_err(|e| format!("Échec de la création du répertoire de sortie : {}", e))?;

    // Vérifie si le fichier est une image prise en charge.
    if let Some(extension) = input_file.extension() {
        let extension_str = extension.to_str().ok_or("Extension de fichier invalide")?.to_lowercase();
        if is_supported_image_extension(&extension_str) {
            let output_path = output_dir.join(input_file.file_stem().ok_or("Nom de fichier invalide")?).with_extension("webp");

            // Ignore si le fichier WebP existe déjà.
            if output_path.exists() {
                println!("Ignorer {}, déjà converti", input_file.display());
                return Ok(());
            }

            // Charge et sauvegarde l'image en WebP.
            let img = image::open(input_file).map_err(|e| format!("Échec de l'ouverture de {} : {}", input_file.display(), e))?;
            img.save_with_format(&output_path, ImageFormat::WebP)
                .map_err(|e| format!("Échec de la sauvegarde de {} : {}", output_path.display(), e))?;
            println!("Converti {} en {}", input_file.display(), output_path.display());
        } else {
            return Err(format!("Format de fichier non pris en charge : {}", extension_str));
        }
    } else {
        return Err("Le fichier n'a pas d'extension".to_string());
    }
    Ok(())
}

/// Vérifie si l'extension de fichier est une extension d'image prise en charge.
fn is_supported_image_extension(extension: &str) -> bool {
    matches!(extension, "png" | "jpg" | "jpeg" | "bmp")
}
