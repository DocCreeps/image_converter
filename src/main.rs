use image::ImageFormat;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;

// Fonction récursive pour convertir les images dans un répertoire et ses sous-répertoires
fn convert_images_in_directory(input_dir: &PathBuf, output_dir: &PathBuf) {
    // Créer le dossier de sortie s'il n'existe pas
    fs::create_dir_all(output_dir).unwrap();

    // Lire les fichiers dans le dossier d'entrée
    let entries = fs::read_dir(input_dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            // Si c'est un dossier, appeler récursivement la fonction
            let new_output_dir = output_dir.join(path.file_name().unwrap());
            convert_images_in_directory(&path, &new_output_dir);
        } else if let Some(extension) = path.extension() {
            let extension = extension.to_str().unwrap().to_lowercase();
            if extension == "png" || extension == "jpg" || extension == "jpeg" || extension == "bmp" {
                // Définir le chemin de sortie avec l'extension .webp
                let output_path = output_dir.join(path.file_stem().unwrap()).with_extension("webp");

                // Vérifier si le fichier de sortie existe déjà
                if output_path.exists() {
                    println!("Skipping {}, already converted", path.display());
                    continue;
                }

                // Lire l'image
                let img = image::open(&path).unwrap();

                // Enregistrer l'image en format WebP
                img.save_with_format(&output_path, ImageFormat::WebP).unwrap();

                // Afficher un message indiquant que la conversion a été effectuée
                println!("Converted {} to {}", path.display(), output_path.display());
            }
        }
    }
}

fn main() {
    // Demander à l'utilisateur de sélectionner le répertoire d'entrée
    let input_dir = FileDialog::new()
        .set_title("Select Input Directory")
        .pick_folder()
        .expect("Failed to select input directory");

    // Demander à l'utilisateur de sélectionner le répertoire de sortie
    let output_dir = FileDialog::new()
        .set_title("Select Output Directory")
        .pick_folder()
        .expect("Failed to select output directory");

    // Appeler la fonction de conversion
    convert_images_in_directory(&input_dir, &output_dir);
}
