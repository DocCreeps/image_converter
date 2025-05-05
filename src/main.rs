/// Application graphique pour convertir des images (PNG, JPG, JPEG, BMP) en WebP.
/// Permet de convertir soit une seule image, soit toutes les images d'un répertoire et ses sous-répertoires.
/// Utilise `eframe` pour l'interface utilisateur, `image` pour la conversion, et `rfd` pour les dialogues de fichiers.
/// Supporte Windows, macOS et Linux pour l'ouverture du dossier de sortie.
use eframe::{egui, App, CreationContext, Frame, NativeOptions};
use image::ImageFormat;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};
use dirs;

/// Type d'entrée sélectionné par l'utilisateur : fichier unique ou répertoire.
enum InputType {
    File(PathBuf),
    Directory(PathBuf),
}

/// Convertit récursivement les images (PNG, JPG, JPEG, BMP) d'un répertoire et ses sous-répertoires en WebP.
/// Ignore les fichiers déjà convertis et conserve la structure des dossiers.
fn convert_images_in_directory(input_dir: &PathBuf, output_dir: &PathBuf, base_input_dir: &PathBuf) {
    // Crée le répertoire de sortie, panique en cas d'erreur.
    fs::create_dir_all(output_dir).unwrap();

    // Lit les entrées du répertoire d'entrée.
    let entries = fs::read_dir(input_dir).unwrap();

    // Parcourt chaque entrée (fichier ou dossier).
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        // Traite les sous-répertoires récursivement.
        if path.is_dir() {
            let relative_path = path.strip_prefix(base_input_dir).unwrap();
            let new_output_dir = output_dir.join(relative_path);
            convert_images_in_directory(&path, &new_output_dir, base_input_dir);
        } else if let Some(extension) = path.extension() {
            // Convertit les images PNG, JPG, JPEG, BMP en WebP.
            let extension = extension.to_str().unwrap().to_lowercase();
            if extension == "png" || extension == "jpg" || extension == "jpeg" || extension == "bmp" {
                let output_path = output_dir.join(path.file_stem().unwrap()).with_extension("webp");

                // Ignore si le fichier WebP existe déjà.
                if output_path.exists() {
                    println!("Ignorer {}, déjà converti", path.display());
                    continue;
                }

                // Charge l'image et la sauvegarde en WebP. Panique si l'opération échoue.
                let img = image::open(&path).unwrap();
                img.save_with_format(&output_path, ImageFormat::WebP).unwrap();
                println!("Converti {} en {}", path.display(), output_path.display());
            }
        }
    }
}

/// Convertit une seule image (PNG, JPG, JPEG, BMP) en WebP dans le répertoire de sortie.
fn convert_single_image(input_file: &PathBuf, output_dir: &PathBuf) {
    // Crée le répertoire de sortie, panique en cas d'erreur.
    fs::create_dir_all(output_dir).unwrap();

    // Vérifie si le fichier est une image prise en charge.
    if let Some(extension) = input_file.extension() {
        let extension = extension.to_str().unwrap().to_lowercase();
        if extension == "png" || extension == "jpg" || extension == "jpeg" || extension == "bmp" {
            let output_path = output_dir.join(input_file.file_stem().unwrap()).with_extension("webp");

            // Ignore si le fichier WebP existe déjà.
            if output_path.exists() {
                println!("Ignorer {}, déjà converti", input_file.display());
                return;
            }

            // Charge l'image et la sauvegarde en WebP. Panique si l'opération échoue.
            let img = image::open(input_file).unwrap();
            img.save_with_format(&output_path, ImageFormat::WebP).unwrap();
            println!("Converti {} en {}", input_file.display(), output_path.display());
        }
    }
}

/// Structure principale de l'application, gérant l'entrée (fichier ou répertoire) et le répertoire de sortie.
struct ImageConverterApp {
    input: Option<InputType>, // Fichier ou répertoire sélectionné par l'utilisateur.
    output_dir: PathBuf,      // Répertoire de sortie pour les images converties.
}

/// Définit les valeurs par défaut pour `ImageConverterApp`, avec le dossier de sortie sur le bureau.
impl Default for ImageConverterApp {
    fn default() -> Self {
        let desktop_dir = dirs::desktop_dir().unwrap_or(PathBuf::from("."));
        Self {
            input: None,
            output_dir: desktop_dir.join("webp"), // Crée le dossier "webp" sur le bureau.
        }
    }
}

/// Implémente l'interface graphique avec `eframe`.
impl App for ImageConverterApp {
    /// Met à jour l'interface graphique à chaque frame, gérant la sélection de l'entrée et la conversion.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Affiche le titre de l'application.
            ui.heading("Convertisseur d'images");
            ui.add_space(10.0);

            // Groupe pour sélectionner l'entrée (fichier ou répertoire).
            egui::Frame::group(&ctx.style()).show(ui, |ui| {
                ui.label("Entrée :");
                ui.horizontal(|ui| {
                    // Bouton pour sélectionner un fichier unique.
                    if ui.button("Sélectionner un fichier...").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Images", &["png", "jpg", "jpeg", "bmp"])
                            .pick_file()
                        {
                            self.input = Some(InputType::File(path));
                        }
                    }
                    // Bouton pour sélectionner un répertoire.
                    if ui.button("Sélectionner un répertoire...").clicked() {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            self.input = Some(InputType::Directory(path));
                        }
                    }
                    // Affiche le chemin de l'entrée sélectionnée.
                    if let Some(input) = &self.input {
                        match input {
                            InputType::File(path) => ui.label(format!("Fichier: {}", path.display())),
                            InputType::Directory(path) => ui.label(format!("Répertoire: {}", path.display())),
                        };
                    }
                });
            });

            ui.add_space(10.0);

            // Affiche le répertoire de sortie actuel.
            ui.label(format!("Répertoire de sortie : {}", self.output_dir.display()));

            // Bouton pour modifier le répertoire de sortie.
            if ui.button("Changer le répertoire de sortie").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.output_dir = path;
                }
            }

            ui.add_space(20.0);

            // Bouton lançant la conversion et ouvrant le dossier de sortie.
            if ui.button("Convertir").clicked() {
                if let Some(input) = &self.input {
                    match input {
                        InputType::File(file_path) => {
                            // Convertit une seule image.
                            convert_single_image(file_path, &self.output_dir);
                        }
                        InputType::Directory(dir_path) => {
                            // Convertit toutes les images du répertoire.
                            let dir_name = dir_path.file_name().unwrap();
                            let final_output_dir = self.output_dir.join(dir_name);
                            convert_images_in_directory(dir_path, &final_output_dir, dir_path);
                        }
                    }

                    // Ouvre le répertoire de sortie dans l'explorateur de fichiers (Windows, macOS, Linux).
                    if cfg!(target_os = "windows") {
                        Command::new("explorer")
                            .arg(self.output_dir.to_str().unwrap())
                            .spawn()
                            .unwrap();
                    } else if cfg!(target_os = "macos") {
                        Command::new("open")
                            .arg(self.output_dir.to_str().unwrap())
                            .spawn()
                            .unwrap();
                    } else {
                        Command::new("xdg-open")
                            .arg(self.output_dir.to_str().unwrap())
                            .spawn()
                            .unwrap();
                    }

                    // Termine le programme après la conversion et l'ouverture du dossier.
                    exit(0);
                }
            }
        });
    }
}

/// Point d'entrée du programme, configure et lance l'application graphique.
fn main() {
    // Configure les options de la fenêtre (taille 400x300 pixels).
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    // Lance l'application `eframe` avec `ImageConverterApp`.
    eframe::run_native(
        "Convertisseur d'images",
        native_options,
        Box::new(|_cc: &CreationContext| Ok(Box::new(ImageConverterApp::default()))),
    )
        .unwrap();
}