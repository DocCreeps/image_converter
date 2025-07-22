/// Application graphique pour convertir des images (PNG, JPG, JPEG, BMP) en WebP.
/// Permet de convertir une seule image, plusieurs images indépendantes ou toutes les images d'un répertoire et ses sous-répertoires.
/// Offre une interface moderne avec messages de confirmation, gestion d'erreurs, et tooltips pour l'accessibilité.
/// Les blocs de l'interface sont centrés horizontalement et verticalement dans la fenêtre.
/// Utilise `eframe` pour l'UI, `image` pour la conversion, et `rfd` pour les dialogues de fichiers.
/// Supporte Windows, macOS et Linux pour l'ouverture du dossier de sortie.

// Importe les modules externes nécessaires
use eframe::{egui, App, CreationContext, Frame, NativeOptions};
use std::path::PathBuf;
use std::sync::{Arc, Mutex}; // Pour la communication inter-threads

// Importe nos modules locaux
mod converter; // Contient la logique de conversion d'images et l'enum OverwriteMode
mod ui_helpers; // Contient des fonctions d'aide pour l'UI
mod platform_utils; // Contient des utilitaires spécifiques à la plateforme et de validation de chemin

/// Type d'entrée sélectionné par l'utilisateur : fichier unique, répertoire, ou plusieurs fichiers.
#[derive(Debug, PartialEq, Clone)]
pub enum InputType {
    SingleFile(PathBuf),
    Directory(PathBuf),
    MultipleFiles(Vec<PathBuf>), // Nouveau: pour la sélection de plusieurs fichiers indépendants
}

/// Structure principale de l'application, gérant l'entrée, le répertoire de sortie et l'état de l'UI.
pub struct ImageConverterApp {
    pub input: Option<InputType>,       // Fichier, répertoire ou liste de fichiers sélectionné.
    pub output_dir: PathBuf,            // Répertoire de sortie pour les images converties.
    pub dialog_message: Option<String>, // Message à afficher dans la fenêtre modale (succès ou erreur).
    pub show_dialog: bool,              // Contrôle l'affichage de la fenêtre modale.
    pub is_converting: bool,            // Indique si une conversion est en cours.
    pub conversion_progress: f32,       // Progrès de la conversion (0.0 à 1.0)
    pub overwrite_mode: converter::OverwriteMode, // Mode de gestion des fichiers existants.
    pub is_file_hovered: bool,          // Indique si un fichier est survolé pour le drag and drop
    pub show_toast: bool,               // Contrôle l'affichage du "toast" de notification.
    pub toast_message: String,          // Message du "toast".
    pub toast_is_error: bool,           // Vrai si le toast est un message d'erreur.
    // Nouveau: Pour la communication du résultat de la conversion depuis un thread secondaire
    pub conversion_result: Arc<Mutex<Option<Result<(), String>>>>,
}

/// Définit les valeurs par défaut pour `ImageConverterApp`, avec le dossier de sortie sur le bureau.
impl Default for ImageConverterApp {
    fn default() -> Self {
        let desktop_dir = dirs::desktop_dir().unwrap_or(PathBuf::from("."));
        Self {
            input: None,
            output_dir: desktop_dir.join("webp_converted"),
            dialog_message: None,
            show_dialog: false,
            is_converting: false,
            conversion_progress: 0.0,
            overwrite_mode: converter::OverwriteMode::Skip, // Par défaut, ignorer les fichiers existants
            is_file_hovered: false,
            show_toast: false,
            toast_message: String::new(),
            toast_is_error: false,
            conversion_result: Arc::new(Mutex::new(None)),
        }
    }
}

/// Implémente l'interface graphique avec `eframe`.
impl App for ImageConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Personnalise le style visuel de l'application.
        ui_helpers::set_custom_style(ctx);

        // Gère le glisser-déposer de fichiers
        self.is_file_hovered = !ctx.input(|i| i.raw.hovered_files.is_empty());
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let dropped_paths: Vec<PathBuf> = ctx.input(|i| i.raw.dropped_files.clone())
                .into_iter()
                .filter_map(|file| file.path)
                .collect();

            if dropped_paths.len() == 1 {
                if dropped_paths[0].is_file() {
                    self.input = Some(InputType::SingleFile(dropped_paths[0].clone()));
                } else if dropped_paths[0].is_dir() {
                    self.input = Some(InputType::Directory(dropped_paths[0].clone()));
                }
            } else if dropped_paths.len() > 1 {
                self.input = Some(InputType::MultipleFiles(dropped_paths));
            }
        }


        egui::CentralPanel::default().show(ctx, |ui| {
            // Centre verticalement et horizontalement le contenu.
            ui.vertical_centered(|ui| {
                ui.add_space(30.0); // Espace en haut

                // Titre
                ui_helpers::render_title(ui);
                ui.add_space(20.0);

                // Zone principale centrée
                ui.allocate_ui_with_layout(
                    egui::vec2(500.0, ui.available_height() - 60.0), // Largeur fixe, hauteur flexible
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        // Zone de Drag & Drop
                        ui_helpers::render_drag_drop_area(ui, &mut self.input, self.is_file_hovered);
                        ui.add_space(10.0);

                        // Boutons de sélection de fichiers/dossiers
                        ui_helpers::render_file_selection_buttons(ui, &mut self.input);
                        ui.add_space(20.0);

                        // Affichage du chemin sélectionné
                        ui_helpers::render_selected_input_display(ui, &self.input);
                        ui.add_space(10.0);

                        // Section Répertoire de sortie
                        ui_helpers::render_output_section(ui, &mut self.output_dir);
                        ui.add_space(10.0);

                        // Section Overwrite Mode
                        ui_helpers::render_overwrite_options(ui, &mut self.overwrite_mode);
                        ui.add_space(20.0);

                        // Bouton de Conversion
                        let convert_button_enabled = self.input.is_some() && !self.is_converting;
                        if ui_helpers::render_convert_button(ui, convert_button_enabled).clicked() {
                            if let Some(input) = &self.input {
                                // Validation du chemin de sortie
                                if let Some(input_path) = input.get_path_for_validation() {
                                    if let Err(e) = platform_utils::validate_paths(input_path, &self.output_dir) {
                                        self.dialog_message = Some(format!("Erreur de validation du chemin : {}", e));
                                        self.show_dialog = true; // Afficher la modale pour l'erreur critique
                                        return;
                                    }
                                }

                                self.is_converting = true;
                                self.conversion_progress = 0.0; // Réinitialiser la progression

                                // Lancer la conversion dans un thread séparé
                                let input_clone = input.clone();
                                let output_dir_clone = self.output_dir.clone();
                                let overwrite_mode_clone = self.overwrite_mode.clone();
                                let ctx_clone = ctx.clone();
                                let conversion_result_clone = Arc::clone(&self.conversion_result);

                                std::thread::spawn(move || {
                                    let thread_result = match input_clone {
                                        InputType::SingleFile(file_path) => {
                                            converter::convert_single_image(&file_path, &output_dir_clone, &overwrite_mode_clone)
                                        }
                                        InputType::MultipleFiles(file_paths) => {
                                            // TODO: Pour la barre de progression, il faudrait modifier convert_multiple_files
                                            // pour qu'il prenne un callback de progression. Pour l'instant, la barre progressera après la fin.
                                            converter::convert_multiple_files(&file_paths, &output_dir_clone, &overwrite_mode_clone)
                                        }
                                        InputType::Directory(dir_path) => {
                                            let final_output_dir = output_dir_clone.join(
                                                dir_path.file_name().unwrap_or_default()
                                            );
                                            converter::convert_images_in_directory(&dir_path, &final_output_dir, &dir_path, &overwrite_mode_clone)
                                        }
                                    };

                                    // Envoyer le résultat au thread UI
                                    *conversion_result_clone.lock().unwrap() = Some(thread_result);
                                    ctx_clone.request_repaint(); // Demander au thread UI de se rafraîchir
                                });
                            }
                        }

                        // Vérifier le résultat de la conversion une once qu'elle est terminée
                        if let Some(result) = self.conversion_result.lock().unwrap().take() {
                            self.is_converting = false;
                            self.conversion_progress = 1.0; // Marquer comme terminé

                            match result {
                                Ok(()) => {
                                    self.toast_message = "Conversion terminée avec succès !".to_string();
                                    self.show_toast = true;
                                    self.toast_is_error = false;
                                }
                                Err(e) => {
                                    self.dialog_message = Some(format!("Erreur lors de la conversion : {}", e)); // Wrap in Some
                                    self.show_dialog = true; // Afficher la modale pour les erreurs de conversion
                                    self.toast_message = "Erreur lors de la conversion !".to_string();
                                    self.show_toast = true;
                                    self.toast_is_error = true;
                                }
                            }
                        }

                        // Afficher un indicateur de chargement si une conversion est en cours
                        if self.is_converting {
                            ui.add_space(10.0);
                            ui.add(egui::ProgressBar::new(self.conversion_progress).show_percentage());
                            ui.label("Conversion en cours...");
                            ctx.request_repaint(); // Demander un rafraîchissement continu pendant la conversion
                        }
                    },
                ); // Fin allocate_ui_with_layout
            }); // Fin vertical_centered
        }); // Fin CentralPanel

        // Fenêtre modale pour les erreurs critiques ou l'ouverture du dossier
        if self.show_dialog {
            ui_helpers::render_dialog_window(ctx, &mut self.show_dialog, &mut self.dialog_message, &self.output_dir);
        }

        // Afficher le toast de notification
        if self.show_toast {
            ui_helpers::render_toast(ctx, &mut self.show_toast, &self.toast_message, self.toast_is_error);
        }
    }
}

// Ajout d'une méthode utilitaire à InputType pour faciliter la validation de chemin
impl InputType {
    fn get_path_for_validation(&self) -> Option<&PathBuf> {
        match self {
            InputType::SingleFile(path) => Some(path),
            InputType::Directory(path) => Some(path),
            InputType::MultipleFiles(_) => None, // La validation pour plusieurs fichiers est plus complexe, à implémenter si nécessaire
        }
    }
}

/// Point d'entrée du programme, configure et lance l'application graphique.
fn main() {
    // Configure les options de la fenêtre (taille 600x500 pour plus d'espace).
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 500.0]),
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
