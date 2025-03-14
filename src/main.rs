use image::GenericImageView;
use std::fs;
use std::path::Path;
use webp::Encoder;

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
            let img = image::open(&path).unwrap();
            let output_path = output_dir.join(path.file_stem().unwrap()).with_extension("webp");

            // Convertir l'image en WebP
            let encoder = Encoder::from_image(&img).unwrap();
            let webp_data = encoder.encode(80.0);
            fs::write(output_path, webp_data).unwrap();

            println!("Converted {} to {}", path.display(), output_path.display());
        }
    }
}

fn main() {
    // Chemin vers le dossier sur le bureau
    let input_dir = Path::new("C:/Users/doria/Bureau/logos");
    let output_dir = Path::new("C:/Users/doria/Bureau/logos-webp");

    convert_images_in_directory(input_dir, output_dir);
}
