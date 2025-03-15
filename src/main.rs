use eframe::{egui, App, CreationContext, Frame, NativeOptions};
use image::ImageFormat;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;

// Fonction pour convertir les images dans un répertoire et ses sous-répertoires en format WebP.
fn convert_images_in_directory(input_dir: &PathBuf, output_dir: &PathBuf) {
    // Créer le répertoire de sortie s'il n'existe pas.
    fs::create_dir_all(output_dir).unwrap();

    // Lire les entrées (fichiers et répertoires) du répertoire d'entrée.
    let entries = fs::read_dir(input_dir).unwrap();

    // Itérer sur chaque entrée.
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        // Si l'entrée est un répertoire, appeler récursivement la fonction.
        if path.is_dir() {
            let new_output_dir = output_dir.join(path.file_name().unwrap());
            convert_images_in_directory(&path, &new_output_dir);
        } else if let Some(extension) = path.extension() {
            // Si l'entrée est un fichier image (png, jpg, jpeg, bmp), le convertir en WebP.
            let extension = extension.to_str().unwrap().to_lowercase();
            if extension == "png" || extension == "jpg" || extension == "jpeg" || extension == "bmp" {
                let output_path = output_dir.join(path.file_stem().unwrap()).with_extension("webp");

                // Si le fichier de sortie existe déjà, ignorer la conversion.
                if output_path.exists() {
                    println!("Ignorer {}, déjà converti", path.display());
                    continue;
                }

                // Ouvrir l'image et la convertir en WebP.
                let img = image::open(&path).unwrap();
                img.save_with_format(&output_path, ImageFormat::WebP).unwrap();
                println!("Converti {} en {}", path.display(), output_path.display());
            }
        }
    }
}

// Structure pour l'application de conversion d'images.
struct ImageConverterApp {
    input_dir: Option<PathBuf>,
    output_dir: Option<PathBuf>,
}

// Implémentation de la valeur par défaut pour ImageConverterApp.
impl Default for ImageConverterApp {
    fn default() -> Self {
        Self {
            input_dir: None,
            output_dir: None,
        }
    }
}

// Implémentation du trait App pour ImageConverterApp.
impl App for ImageConverterApp {
    // Fonction appelée à chaque mise à jour de l'interface utilisateur.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Titre de l'application.
            ui.heading("Convertisseur d'images");
            // Ajout d'un espace vertical.
            ui.add_space(10.0);

            // Groupe pour le répertoire d'entrée.
            egui::Frame::group(&ctx.style()).show(ui, |ui| {
                // Étiquette du groupe.
                ui.label("Répertoire d'entrée :");
                // Disposition horizontale des éléments.
                ui.horizontal(|ui| {
                    // Bouton pour sélectionner le répertoire d'entrée.
                    if ui.button("Sélectionner...").clicked() {
                        // Ouvre une boîte de dialogue pour choisir un répertoire.
                        if let Some(path) = FileDialog::new().pick_folder() {
                            // Enregistre le chemin du répertoire sélectionné.
                            self.input_dir = Some(path);
                        }
                    }
                    // Affiche le chemin du répertoire sélectionné.
                    if let Some(path) = &self.input_dir {
                        ui.label(path.display().to_string());
                    }
                });
            });

            // Ajout d'un espace vertical.
            ui.add_space(10.0);

            // Groupe pour le répertoire de sortie.
            egui::Frame::group(&ctx.style()).show(ui, |ui| {
                // Étiquette du groupe.
                ui.label("Répertoire de sortie :");
                // Disposition horizontale des éléments.
                ui.horizontal(|ui| {
                    // Bouton pour sélectionner le répertoire de sortie.
                    if ui.button("Sélectionner...").clicked() {
                        // Ouvre une boîte de dialogue pour choisir un répertoire.
                        if let Some(path) = FileDialog::new().pick_folder() {
                            // Enregistre le chemin du répertoire sélectionné.
                            self.output_dir = Some(path);
                        }
                    }
                    // Affiche le chemin du répertoire sélectionné.
                    if let Some(path) = &self.output_dir {
                        ui.label(path.display().to_string());
                    }
                });
            });

            // Ajout d'un espace vertical.
            ui.add_space(20.0);

            // Bouton pour lancer la conversion.
            if ui.button("Convertir les images").clicked() {
                // Vérifie si les répertoires d'entrée et de sortie sont sélectionnés.
                if let (Some(input_dir), Some(output_dir)) = (&self.input_dir, &self.output_dir) {
                    // Appelle la fonction de conversion.
                    convert_images_in_directory(input_dir, output_dir);
                }
            }
        });
    }
}

// Fonction principale du programme.
fn main() {
    // Configuration des options natives de l'application.
    let native_options = NativeOptions {
        // Configuration de la fenêtre de l'application.
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    // Lancement de l'application eframe.
    eframe::run_native(
        "Convertisseur d'images", // Titre de l'application.
        native_options,
        // Fonction de création de l'application.
        Box::new(|_cc: &CreationContext| Ok(Box::new(ImageConverterApp::default()))),
    )
        .unwrap();
}