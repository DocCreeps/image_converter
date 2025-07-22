/// Application graphique pour convertir des images (PNG, JPG, JPEG, BMP) en WebP.
/// Permet de convertir une seule image ou toutes les images d'un répertoire et ses sous-répertoires.
/// Offre une interface moderne avec messages de confirmation, gestion d'erreurs, et tooltips pour l'accessibilité.
/// Les blocs de l'interface sont centrés horizontalement et verticalement dans la fenêtre.
/// Utilise `eframe` pour l'UI, `image` pour la conversion, et `rfd` pour les dialogues de fichiers.
/// Supporte Windows, macOS et Linux pour l'ouverture du dossier de sortie.

// Importe les modules externes nécessaires
use eframe::{egui, App, CreationContext, Frame, NativeOptions};

use std::path::PathBuf;

// Importe nos modules locaux
mod converter; // Contient la logique de conversion d'images
mod ui_helpers; // Contient des fonctions d'aide pour l'UI
mod platform_utils; // Contient des utilitaires spécifiques à la plateforme

/// Type d'entrée sélectionné par l'utilisateur : fichier unique ou répertoire.
#[derive(Debug, PartialEq, Clone)] // Ajout de Debug, PartialEq, Clone pour faciliter la manipulation de l'enum
pub enum InputType {
    File(PathBuf),
    Directory(PathBuf),
}

/// Structure principale de l'application, gérant l'entrée, le répertoire de sortie et l'état de l'UI.
pub struct ImageConverterApp {
    pub input: Option<InputType>,       // Fichier ou répertoire sélectionné.
    pub output_dir: PathBuf,            // Répertoire de sortie pour les images converties.
    pub dialog_message: Option<String>, // Message à afficher dans la fenêtre modale (succès ou erreur).
    pub show_dialog: bool,              // Contrôle l'affichage de la fenêtre modale.
}

/// Définit les valeurs par défaut pour `ImageConverterApp`, avec le dossier de sortie sur le bureau.
impl Default for ImageConverterApp {
    fn default() -> Self {
        let desktop_dir = dirs::desktop_dir().unwrap_or(PathBuf::from("."));
        Self {
            input: None,
            output_dir: desktop_dir.join("webp_converted"), // Nom de dossier plus spécifique
            dialog_message: None,
            show_dialog: false,
        }
    }
}

/// Implémente l'interface graphique avec `eframe`.
impl App for ImageConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Personnalise le style visuel de l'application.
        ui_helpers::set_custom_style(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Centre verticalement et horizontalement le contenu.
            ui.vertical_centered(|ui| {
                // Ajoute un espace en haut pour équilibrer la mise en page.
                ui.add_space(50.0);

                // Titre stylé avec une police plus grande.
                ui_helpers::render_title(ui);

                ui.add_space(20.0);

                // Conteneur pour limiter la largeur des blocs et centrer.
                ui.scope(|ui| {
                    ui.set_max_width(400.0); // Limite la largeur pour un look compact.

                    // Section pour l'entrée.
                    ui_helpers::render_input_section(ui, &mut self.input);

                    ui.add_space(15.0);

                    // Section pour le répertoire de sortie.
                    ui_helpers::render_output_section(ui, &mut self.output_dir);

                    ui.add_space(20.0);

                    // Bouton de conversion.
                    if ui_helpers::render_convert_button(ui, self.input.is_some()).clicked() {
                        if let Some(input) = &self.input {
                            let result = match input {
                                InputType::File(file_path) => converter::convert_single_image(file_path, &self.output_dir),
                                InputType::Directory(dir_path) => {
                                    // Crée un sous-dossier dans le répertoire de sortie avec le nom du dossier source
                                    let final_output_dir = self.output_dir.join(
                                        dir_path.file_name().unwrap_or_default()
                                    );
                                    converter::convert_images_in_directory(dir_path, &final_output_dir, dir_path)
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
            ui_helpers::render_dialog_window(ctx, &mut self.show_dialog, &mut self.dialog_message, &self.output_dir);
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
