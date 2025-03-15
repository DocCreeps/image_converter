# Convertisseur d'Images WebP

Ce projet est une application graphique écrite en Rust utilisant eframe pour convertir des images (PNG, JPG, JPEG, BMP) au format WebP. Il permet de sélectionner un répertoire d'entrée, un répertoire de sortie, et de lancer la conversion de manière intuitive.

## Fonctionnalités

-   **Interface graphique conviviale** : Sélection facile des répertoires d'entrée et de sortie via une interface graphique.
-   **Conversion récursive** : Convertit toutes les images dans le répertoire d'entrée et ses sous-répertoires.
-   **Formats supportés** : Prend en charge les formats d'image PNG, JPG, JPEG et BMP.
-   **Prévention des doublons** : Vérifie si une image a déjà été convertie pour éviter les conversions inutiles.
-   **Ouverture automatique du répertoire de sortie** : Ouvre le répertoire de sortie une fois la conversion terminée.
-   **Configuration du répertoire de sortie** : Possibilité de choisir le répertoire de sortie via une boîte de dialogue.

## Prérequis

-   [Rust](https://www.rust-lang.org/tools/install) doit être installé sur votre machine.
-   [Cargo](https://doc.rust-lang.org/cargo/), le gestionnaire de paquets de Rust, doit également être installé.

## Installation

1.  Clonez ce dépôt :

    ```bash
    git clone [https://github.com/votre-utilisateur/image-converter.git](https://github.com/votre-utilisateur/image-converter.git)
    cd image-converter
    ```

2.  Installez les dépendances et compilez le projet :

    ```bash
    cargo build --release
    ```

## Utilisation

1.  Exécutez l'application :

    ```bash
    cargo run --release
    ```

2.  Une fenêtre s'ouvrira, vous permettant de :

    -   Sélectionner le répertoire contenant les images à convertir en cliquant sur "Sélectionner...".
    -   Visualiser le répertoire de sortie actuel.
    -   Changer le répertoire de sortie en cliquant sur "Changer le répertoire de sortie".
    -   Lancer la conversion en cliquant sur "Convertir les images".

3.  Une fois la conversion terminée, le répertoire de sortie s'ouvrira automatiquement.

## Dépendances

-   [eframe](https://crates.io/crates/eframe) : Framework pour la création d'interfaces graphiques.
-   [egui](https://crates.io/crates/egui) : Bibliothèque d'interface utilisateur pour eframe.
-   [image](https://crates.io/crates/image) : Utilisé pour lire et écrire des images.
-   [rfd](https://crates.io/crates/rfd) : Boîte de dialogue pour la sélection de fichiers et de répertoires.
-   [dirs](https://crates.io/crates/dirs) : Pour obtenir les répertoires spécifiques à l'utilisateur.

## Notes

-   L'application crée un dossier "webp" sur le bureau par défaut si aucun répertoire de sortie n'est spécifié.
-   Les images converties conservent la même structure de répertoires que le répertoire d'entrée.
-   L'application gère les doublons en vérifiant si une image a déjà été convertie dans le répertoire de sortie.
