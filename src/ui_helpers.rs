/// Ce module contient des fonctions d'aide pour la construction de l'interface utilisateur.
use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
// use std::process::exit; // Déplacé à l'intérieur de render_dialog_window
use std::time::Duration;

// Importe les enums InputType et OverwriteMode du module parent (main.rs et converter.rs)
use super::InputType;
use crate::converter::OverwriteMode;
use crate::platform_utils; // Importe le module platform_utils

/// Applique un style personnalisé à l'interface utilisateur.
pub fn set_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(15.0, 8.0); // Plus de padding pour les boutons

    // Couleurs de fond des boutons (bleu)
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(50, 150, 250); // Bleu vif
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(70, 170, 255); // Bleu plus clair au survol
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(30, 130, 230);  // Bleu plus foncé à l'activation

    // Couleur du texte des boutons (blanc)
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255));
    style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255));
    style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255));

    // Couleur de fond générale de l'application
    style.visuals.window_fill = egui::Color32::from_rgb(245, 245, 245); // Un gris très clair pour l'arrière-plan

    // Bordures des fenêtres et des cadres
    style.visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 180));
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 200)); // Bordure des éléments inactifs
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(150, 150, 150)); // Bordure des éléments survolés

    // Coins arrondis (utilisation de CornerRadius et f32/u8 selon le champ)
    style.visuals.window_corner_radius = egui::CornerRadius::same(8); // u8

    // Les arrondis pour les widgets sont définis ici:
    style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(6); // u8
    style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(6); // u8
    style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(6); // u8
    style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(6); // u8
    style.visuals.widgets.open.corner_radius = egui::CornerRadius::same(6); // u8


    ctx.set_style(style);
}

/// Rend le titre de l'application.
pub fn render_title(ui: &mut egui::Ui) {
    ui.style_mut().text_styles.get_mut(&egui::TextStyle::Heading).unwrap().size = 28.0; // Titre plus grand
    ui.heading(egui::RichText::new("Convertisseur WebP").strong()) // Titre en gras
        .on_hover_text("Convertissez vos images en WebP rapidement !");
}

/// Rend la zone de Drag & Drop pour la sélection des fichiers.
pub fn render_drag_drop_area(ui: &mut egui::Ui, _input: &mut Option<InputType>, is_file_hovered: bool) -> egui::Response {
    let mut rect = ui.available_rect_before_wrap();
    rect.set_height(150.0); // Hauteur fixe pour la zone

    let (response, painter) = ui.allocate_painter(rect.size(), egui::Sense::hover()); // Sense::hover() car le clic est géré par les boutons
    let is_hovering_files = ui.input(|i| !i.raw.hovered_files.is_empty());

    let fill_color = if is_file_hovered || is_hovering_files {
        egui::Color32::from_rgb(200, 230, 255) // Couleur de survol pour feedback
    } else {
        egui::Color32::from_rgb(230, 230, 230) // Couleur de fond normale
    };

    let stroke_color = if is_file_hovered || is_hovering_files {
        egui::Color32::from_rgb(50, 150, 250)
    } else {
        egui::Color32::from_rgb(180, 180, 180)
    };

    painter.rect(
        response.rect,
        8.0, // Coins arrondis (f32)
        fill_color,
        egui::Stroke::new(2.0, stroke_color), // Bordure un peu plus épaisse
        egui::StrokeKind::Outside, // Utilisation de StrokeKind::Outside
    );

    let text = if is_hovering_files {
        "Relâchez les fichiers ici !"
    } else {
        "Déposez vos fichiers ou dossiers ici (PNG, JPG, BMP)"
    };

    let text_color = egui::Color32::from_rgb(80, 80, 80); // Texte gris foncé

    let font_size = if is_hovering_files { 20.0 } else { 16.0 };
    painter.text(
        response.rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(font_size),
        text_color,
    );

    // Correction: Chaîner on_hover_text directement à la valeur de retour
    response.on_hover_text("Déposez des images/dossiers ici")
}

/// Rend les boutons de sélection de fichiers/dossiers.
pub fn render_file_selection_buttons(ui: &mut egui::Ui, input: &mut Option<InputType>) {
    ui.horizontal(|ui| {
        // Bouton pour sélectionner un fichier unique.
        if ui
            .button("📄 Fichier Unique")
            .on_hover_text("Sélectionner une seule image (PNG, JPG, JPEG, BMP)")
            .clicked()
        {
            if let Some(path) = FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "bmp"])
                .pick_file()
            {
                *input = Some(InputType::SingleFile(path));
            }
        }

        // Bouton pour sélectionner plusieurs fichiers.
        if ui
            .button("📂 Plusieurs Fichiers")
            .on_hover_text("Sélectionner plusieurs images (PNG, JPG, JPEG, BMP)")
            .clicked()
        {
            if let Some(paths) = FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "bmp"])
                .pick_files()
            {
                *input = Some(InputType::MultipleFiles(paths));
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
}


/// Rend l'affichage du chemin d'entrée sélectionné.
pub fn render_selected_input_display(ui: &mut egui::Ui, input: &Option<InputType>) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Entrée sélectionnée:").strong()); // Utilisation de RichText
        if let Some(selected_input) = input {
            let display_text = match selected_input {
                InputType::SingleFile(path) => format!("Fichier: {}", path.file_name().unwrap_or_default().to_string_lossy()),
                InputType::Directory(path) => format!("Répertoire: {}", path.file_name().unwrap_or_default().to_string_lossy()),
                InputType::MultipleFiles(paths) => format!("{} fichiers", paths.len()),
            };
            ui.label(display_text).on_hover_text(format!("Chemin complet: {}", selected_input.get_display_path()));

            // Bouton pour effacer la sélection (si implémenté, sinon masqué ou désactivé)
            // ui.add_enabled_ui(input.is_some(), |ui| {
            //     if ui.button("X").on_hover_text("Effacer la sélection").clicked() {
            //         *input = None; // Nécessiterait de rendre 'input' mutable ici
            //     // }
            // });
        } else {
            ui.label(egui::RichText::new("Aucune").weak()); // Utilisation de RichText
        }
    });
}

// Ajout d'une méthode utilitaire pour l'affichage des chemins
impl InputType {
    fn get_display_path(&self) -> String {
        match self {
            InputType::SingleFile(path) => path.display().to_string(),
            InputType::Directory(path) => path.display().to_string(),
            InputType::MultipleFiles(paths) => {
                paths.iter()
                    .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            }
        }
    }
}


/// Rend la section du répertoire de sortie.
pub fn render_output_section(ui: &mut egui::Ui, output_dir: &mut PathBuf) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Répertoire de sortie:").strong()); // Utilisation de RichText
        ui.label(output_dir.display().to_string())
            .on_hover_text("Dossier où les images WebP seront sauvegardées");
        if ui
            .button("📁 Changer") // Icône de dossier
            .on_hover_text("Modifier le dossier de sortie")
            .clicked()
        {
            if let Some(path) = FileDialog::new().pick_folder() {
                *output_dir = path;
            }
        }
    });
}

/// Rend la section des options de gestion des fichiers existants.
pub fn render_overwrite_options(ui: &mut egui::Ui, overwrite_mode: &mut OverwriteMode) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Si le fichier existe:").strong()); // Utilisation de RichText
        ui.radio_value(overwrite_mode, OverwriteMode::Skip, "Ignorer")
            .on_hover_text("Ne pas convertir si le fichier WebP existe déjà.");
        ui.radio_value(overwrite_mode, OverwriteMode::Overwrite, "Écraser")
            .on_hover_text("Écraser le fichier WebP existant.");
        ui.radio_value(overwrite_mode, OverwriteMode::Rename, "Renommer")
            .on_hover_text("Créer un nouveau fichier avec un suffixe (ex: image-1.webp).");
    });
}

/// Rend le bouton de conversion.
pub fn render_convert_button(ui: &mut egui::Ui, enabled: bool) -> egui::Response {
    ui.style_mut().text_styles.get_mut(&egui::TextStyle::Button).unwrap().size = 18.0; // Bouton plus grand
    ui.add_enabled(
        enabled,
        egui::Button::new("🚀 Convertir les images")
            .fill(egui::Color32::from_rgb(50, 150, 250)) // Couleur de fond du bouton (bleu vif)
            .stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT)), // Pas de bordure visible
    )
        .on_hover_text("Lancer la conversion des images en WebP")
}

/// Rend la fenêtre modale pour les messages critiques (erreurs graves ou demande d'ouvrir dossier).
pub fn render_dialog_window(
    ctx: &egui::Context,
    show_dialog: &mut bool,
    dialog_message: &mut Option<String>,
    output_dir: &PathBuf,
) {
    use std::process::exit; // Déplacé ici

    egui::Window::new("Information")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label(dialog_message.as_ref().unwrap());
            ui.add_space(15.0);
            ui.horizontal(|ui| {
                if ui.button("Ouvrir le dossier").clicked() {
                    let result = platform_utils::open_output_directory(output_dir);
                    if let Err(e) = result {
                        *dialog_message = Some(format!("Erreur lors de l'ouverture du dossier : {}", e));
                        // show_dialog reste true pour afficher le nouveau message d'erreur
                    } else {
                        *show_dialog = false; // Ferme la modale si l'ouverture réussit
                        exit(0); // Quitte l'application
                    }
                }
                if ui.button("Fermer").clicked() {
                    *show_dialog = false;
                    *dialog_message = None;
                }
            });
        });
}

/// Rend un "toast" de notification temporaire.
pub fn render_toast(ctx: &egui::Context, show_toast: &mut bool, message: &str, is_error: bool) {
    let toast_color = if is_error {
        egui::Color32::from_rgb(255, 100, 100) // Rouge pour les erreurs
    } else {
        egui::Color32::from_rgb(100, 200, 100) // Vert pour les succès
    };

    let text_color = egui::Color32::WHITE;

    egui::Window::new("")
        .id(egui::Id::new("toast_window"))
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-20.0, -20.0)) // Coin inférieur droit
        .frame(egui::Frame::window(&ctx.style()).fill(toast_color).stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT)).corner_radius(8.0)) // Correction de Stroke::none() et rounding()
        .show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new(message).color(text_color).strong());
                ui.add_space(10.0);
            });
            ui.add_space(5.0);
        });

    // Optionnel: faire disparaître le toast après quelques secondes
    if *show_toast {
        let current_time = ctx.input(|i| i.time);
        let start_time_id = egui::Id::new("toast_start_time");
        let start_time: f64 = ctx.data_mut(|data| *data.get_temp_mut_or_insert_with(start_time_id, || current_time));

        if current_time - start_time > 3.0 { // Toast disparaît après 3 secondes
            *show_toast = false;
            ctx.data_mut(|data| data.remove::<f64>(start_time_id)); // Supprime le temps de début
        } else {
            ctx.request_repaint_after(Duration::from_millis(50)); // Redessine pour le timer
        }
    }
}
