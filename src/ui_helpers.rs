/// Ce module contient des fonctions d'aide pour la construction de l'interface utilisateur.
use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use std::process::exit;

// Importe l'enum InputType du module parent (main.rs)
use crate::InputType;
use crate::platform_utils; // Importe le module platform_utils

/// Applique un style personnalisé à l'interface utilisateur.
pub fn set_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(100, 150, 255); // Couleur de fond des boutons
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(80, 130, 230); // Couleur de fond des boutons au survol
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 110, 210); // Couleur de fond des boutons actifs

    // Change la couleur du texte des boutons
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255));
    style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255));
    style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255));

    ctx.set_style(style);
}

/// Rend le titre de l'application.
pub fn render_title(ui: &mut egui::Ui) {
    ui.style_mut().text_styles.get_mut(&egui::TextStyle::Heading).unwrap().size = 24.0;
    ui.heading("Convertisseur d'Images")
        .on_hover_text("Application pour convertir des images en WebP");
}

/// Rend la section de sélection de l'entrée (fichier ou répertoire).
pub fn render_input_section(ui: &mut egui::Ui, input: &mut Option<InputType>) {
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
                        *input = Some(InputType::File(path));
                    }
                }

                // Bouton pour sélectionner un répertoire.
                if ui
                    .button("📁 Répertoire")
                    .on_hover_text("Sélectionner un dossier contenant des images")
                    .clicked()
                {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        *input = Some(InputType::Directory(path));
                    }
                }
            });

            // Affiche le chemin de l'entrée sélectionnée.
            if let Some(selected_input) = &input {
                ui.add_space(5.0);
                match selected_input {
                    InputType::File(path) => ui.label(format!("Fichier sélectionné : {}", path.display())),
                    InputType::Directory(path) => ui.label(format!("Répertoire sélectionné : {}", path.display())),
                }
                    .on_hover_text("Chemin de l'entrée choisie");
            }
        });
}

/// Rend la section du répertoire de sortie.
pub fn render_output_section(ui: &mut egui::Ui, output_dir: &mut PathBuf) {
    egui::Frame::default()
        .inner_margin(10.0)
        .fill(egui::Color32::from_rgb(240, 240, 240))
        .corner_radius(5.0)
        .show(ui, |ui| {
            ui.label(format!("Répertoire de sortie : {}", output_dir.display()))
                .on_hover_text("Dossier où les images WebP seront sauvegardées");
            if ui
                .button("✏️ Changer")
                .on_hover_text("Modifier le dossier de sortie")
                .clicked()
            {
                if let Some(path) = FileDialog::new().pick_folder() {
                    *output_dir = path;
                }
            }
        });
}

/// Rend le bouton de conversion.
pub fn render_convert_button(ui: &mut egui::Ui, enabled: bool) -> egui::Response {
    ui.style_mut().text_styles.get_mut(&egui::TextStyle::Button).unwrap().size = 16.0;
    ui.add_enabled(
        enabled,
        egui::Button::new("🚀 Convertir")
            .fill(egui::Color32::from_rgb(100, 150, 255)) // Couleur de fond du bouton
            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)), // Bordure blanche
    )
        .on_hover_text("Lancer la conversion des images en WebP")
}

/// Rend la fenêtre modale pour les messages de succès ou d'erreur.
pub fn render_dialog_window(
    ctx: &egui::Context,
    show_dialog: &mut bool,
    dialog_message: &mut Option<String>,
    output_dir: &PathBuf,
) {
    egui::Window::new("Résultat de la conversion")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label(dialog_message.as_ref().unwrap());
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Ouvrir le dossier").clicked() {
                    // Ouvre le répertoire de sortie via la fonction utilitaire de la plateforme
                    let result = platform_utils::open_output_directory(output_dir);

                    if let Err(e) = result {
                        *dialog_message = Some(format!("Erreur lors de l'ouverture du dossier : {}", e));
                        // show_dialog reste true pour afficher le message d'erreur
                    } else {
                        // Ferme l'application après l'ouverture du dossier.
                        exit(0);
                    }
                }
                if ui.button("Fermer").clicked() {
                    *show_dialog = false;
                    *dialog_message = None;
                }
            });
        });
}
