/// Application graphique pour convertir des images (PNG, JPG, JPEG, BMP) en WebP.
/// Permet de convertir une seule image ou toutes les images d'un répertoire et ses sous-répertoires.
/// Offre une interface moderne avec messages de confirmation, gestion d'erreurs, et tooltips pour l'accessibilité.
/// Les blocs de l'interface sont centrés horizontalement et verticalement dans la fenêtre.
/// Utilise `eframe` pour l'UI, `image` pour la conversion, et `rfd` pour les dialogues de fichiers.
/// Supporte Windows, macOS et Linux pour l'ouverture du dossier de sortie.
use eframe::{egui, App, CreationContext, Frame, NativeOptions};
use image::ImageFormat;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};

/// Type d'entrée sélectionné par l'utilisateur : fichier unique ou répertoire.
enum InputType {
    File(PathBuf),
    Directory(PathBuf),
}

/// Convertit récursivement les images (PNG, JPG, JPEG, BMP) d'un répertoire en WebP.
/// Ignore les fichiers déjà convertis et conserve la structure des dossiers.
/// Retourne un Result indiquant le succès ou une erreur.
fn convert_images_in_directory(input_dir: &PathBuf, output_dir: &PathBuf, base_input_dir: &PathBuf) -> Result<(), String> {
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
            let extension = extension.to_str().ok_or("Extension de fichier invalide")?.to_lowercase();
            if extension == "png" || extension == "jpg" || extension == "jpeg" || extension == "bmp" {
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
fn convert_single_image(input_file: &PathBuf, output_dir: &PathBuf) -> Result<(), String> {
    // Crée le répertoire de sortie.
    fs::create_dir_all(output_dir).map_err(|e| format!("Échec de la création du répertoire de sortie : {}", e))?;

    // Vérifie si le fichier est une image prise en charge.
    if let Some(extension) = input_file.extension() {
        let extension = extension.to_str().ok_or("Extension de fichier invalide")?.to_lowercase();
        if extension == "png" || extension == "jpg" || extension == "jpeg" || extension == "bmp" {
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
        }
    }
    Ok(())
}

/// Structure principale de l'application, gérant l'entrée, le répertoire de sortie et l'état de l'UI.
struct ImageConverterApp {
    input: Option<InputType>,       // Fichier ou répertoire sélectionné.
    output_dir: PathBuf,            // Répertoire de sortie pour les images converties.
    dialog_message: Option<String>, // Message à afficher dans la fenêtre modale (succès ou erreur).
    show_dialog: bool,              // Contrôle l'affichage de la fenêtre modale.
}

/// Définit les valeurs par défaut pour `ImageConverterApp`, avec le dossier de sortie sur le bureau.
impl Default for ImageConverterApp {
    fn default() -> Self {
        let desktop_dir = dirs::desktop_dir().unwrap_or(PathBuf::from("."));
        Self {
            input: None,
            output_dir: desktop_dir.join("webp"),
            dialog_message: None,
            show_dialog: false,
        }
    }
}

/// Implémente l'interface graphique avec `eframe`.
impl App for ImageConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Personnalise le style visuel de l'application.
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(220, 220, 220);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(200, 200, 200);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(180, 180, 180);

        // Change la couleur du texte des boutons
        style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)); 
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)); 
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)); 

        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Centre verticalement et horizontalement le contenu.
            ui.vertical_centered(|ui| {
                // Ajoute un espace en haut pour équilibrer la mise en page.
                ui.add_space(50.0);

                // Titre stylé avec une police plus grande.
                ui.style_mut().text_styles.get_mut(&egui::TextStyle::Heading).unwrap().size = 24.0;
                ui.heading("Convertisseur d'Images")
                    .on_hover_text("Application pour convertir des images en WebP");

                ui.add_space(20.0);

                // Conteneur pour limiter la largeur des blocs et centrer.
                ui.scope(|ui| {
                    ui.set_max_width(400.0); // Limite la largeur pour un look compact.

                    // Section pour l'entrée, avec un cadre stylé.
                    egui::Frame::default()
                        .inner_margin(10.0)
                        .fill(egui::Color32::from_rgb(240, 240, 240))
                        .corner_radius(5.0)
                        .show(ui, |ui| {
                            ui.label("Sélectionner une entrée :")
                                .on_hover_text("Choisissez un fichier ou un répertoire à convertir");
                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                // Bouton pour sélectionner un fichier.
                                if ui
                                    .button("📄 Fichier")
                                    .on_hover_text("Sélectionner une image (PNG, JPG, JPEG, BMP)")
                                    .clicked()
                                {
                                    if let Some(path) = FileDialog::new()
                                        .add_filter("Images", &["png", "jpg", "jpeg", "bmp"])
                                        .pick_file()
                                    {
                                        self.input = Some(InputType::File(path));
                                    }
                                }

                                // Bouton pour sélectionner un répertoire.
                                if ui
                                    .button("📁 Répertoire")
                                    .on_hover_text("Sélectionner un dossier contenant des images")
                                    .clicked()
                                {
                                    if let Some(path) = FileDialog::new().pick_folder() {
                                        self.input = Some(InputType::Directory(path));
                                    }
                                }
                            });

                            // Affiche le chemin de l'entrée sélectionnée.
                            if let Some(input) = &self.input {
                                ui.add_space(5.0);
                                match input {
                                    InputType::File(path) => ui.label(format!("Fichier sélectionné : {}", path.display())),
                                    InputType::Directory(path) => ui.label(format!("Répertoire sélectionné : {}", path.display())),
                                }
                                    .on_hover_text("Chemin de l'entrée choisie");
                            }
                        });

                    ui.add_space(15.0);

                    // Section pour le répertoire de sortie.
                    egui::Frame::default()
                        .inner_margin(10.0)
                        .fill(egui::Color32::from_rgb(240, 240, 240))
                        .corner_radius(5.0)
                        .show(ui, |ui| {
                            ui.label(format!("Répertoire de sortie : {}", self.output_dir.display()))
                                .on_hover_text("Dossier où les images WebP seront sauvegardées");
                            if ui
                                .button("✏️ Changer")
                                .on_hover_text("Modifier le dossier de sortie")
                                .clicked()
                            {
                                if let Some(path) = FileDialog::new().pick_folder() {
                                    self.output_dir = path;
                                }
                            }
                        });

                    ui.add_space(20.0);

                    // Bouton de conversion, désactivé si aucune entrée n'est sélectionnée.
                    ui.style_mut().text_styles.get_mut(&egui::TextStyle::Button).unwrap().size = 16.0;
                    let convert_button = ui.add_enabled(
                        self.input.is_some(),
                        egui::Button::new("🚀 Convertir")
                            .fill(egui::Color32::from_rgb(100, 150, 255))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)),
                    );

                    if convert_button
                        .on_hover_text("Lancer la conversion des images en WebP")
                        .clicked()
                    {
                        if let Some(input) = &self.input {
                            let result = match input {
                                InputType::File(file_path) => convert_single_image(file_path, &self.output_dir),
                                InputType::Directory(dir_path) => {
                                    match dir_path.file_name() {
                                        Some(dir_name) => {
                                            let final_output_dir = self.output_dir.join(dir_name);
                                            convert_images_in_directory(dir_path, &final_output_dir, dir_path)
                                        }
                                        None => Err("Nom de répertoire invalide".to_string()),
                                    }
                                }
                            };

                            // Affiche un message dans une fenêtre modale.
                            match result {
                                Ok(()) => {
                                    self.dialog_message = Some("Conversion terminée avec succès !".to_string());
                                    self.show_dialog = true;
                                }
                                Err(e) => {
                                    self.dialog_message = Some(format!("Erreur lors de la conversion : {}", e));
                                    self.show_dialog = true;
                                }
                            }
                        }
                    }
                });

                // Ajoute un espace en bas pour équilibrer la mise en page.
                ui.add_space(50.0);
            });
        });

        // Fenêtre modale pour les messages de succès ou d'erreur.
        if self.show_dialog {
            egui::Window::new("Résultat de la conversion")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(self.dialog_message.as_ref().unwrap());
                    ui.add_space(10.0);
                    if ui.button("Ouvrir le dossier").clicked() {
                        // Ouvre le répertoire de sortie (Windows, macOS, Linux).
                        let result = if cfg!(target_os = "windows") {
                            Command::new("explorer")
                                .arg(self.output_dir.to_str().unwrap())
                                .spawn()
                        } else if cfg!(target_os = "macos") {
                            Command::new("open")
                                .arg(self.output_dir.to_str().unwrap())
                                .spawn()
                        } else {
                            Command::new("xdg-open")
                                .arg(self.output_dir.to_str().unwrap())
                                .spawn()
                        };

                        if let Err(e) = result {
                            self.dialog_message = Some(format!("Erreur lors de l'ouverture du dossier : {}", e));
                            self.show_dialog = true;
                        } else {
                            // Ferme l'application après l'ouverture du dossier.
                            exit(0);
                        }
                    }
                    if ui.button("Fermer").clicked() {
                        self.show_dialog = false;
                        self.dialog_message = None;
                    }
                });
        }
    }
}


/// Point d'entrée du programme, configure et lance l'application graphique.
fn main() {
    // Configure les options de la fenêtre (taille 500x400 pour plus d'espace).
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 400.0]),
        ..Default::default()
    };

    // Lance l'application `eframe` avec `ImageConverterApp`.
    eframe::run_native(
        "Convertisseur d'Images",
        native_options,
        Box::new(|_cc: &CreationContext| Ok(Box::new(ImageConverterApp::default()))),
    )
        .unwrap();
}