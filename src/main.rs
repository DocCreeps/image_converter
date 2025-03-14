use image::ImageFormat;
use std::fs;
use std::path::Path;

// Fonction récursive pour convertir les images dans un répertoire et ses sous-répertoires
fn convert_images_in_directory(input_dir: &Path, output_dir: &Path) {
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
        } else if path.extension().map_or(false, |ext| ext == "png") {
            // Lire l'image PNG
            let img = image::open(&path).unwrap();
            // Définir le chemin de sortie avec l'extension .webp
            let output_path = output_dir.join(path.file_stem().unwrap()).with_extension("webp");

            // Enregistrer l'image en format WebP
            img.save_with_format(&output_path, ImageFormat::WebP).unwrap();

            // Message indiquant que la conversion a été effectuée
            println!("Converted {} to {}", path.display(), output_path.display());
        }
    }
}

fn main() {
    // Chemin vers le dossier sur le bureau contenant les images PNG
    let input_dir = Path::new("C:/Users/doria/Bureau/logos");
    // Chemin vers le dossier de sortie pour les images WebP
    let output_dir = Path::new("C:/Users/doria/Bureau/logos-webp");

    // Appeler la fonction de conversion
    convert_images_in_directory(input_dir, output_dir);
}
