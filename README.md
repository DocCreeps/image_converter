# Convertisseur d'Images WebP

Ce projet est une application graphique écrite en Rust, utilisant la bibliothèque `eframe` pour convertir des images (PNG, JPG, JPEG, BMP) au format WebP. Il offre une interface utilisateur intuitive permettant de sélectionner un répertoire d'entrée, un répertoire de sortie, et de lancer la conversion en quelques clics.

## Fonctionnalités Principales

-   **Interface Utilisateur Graphique Conviviale (GUI)** : Utilisation de `eframe` et `egui` pour une expérience utilisateur simple et agréable.
-   **Conversion Récursive** : Traitement de tous les fichiers image dans le répertoire sélectionné et ses sous-répertoires.
-   **Formats d'Image Supportés** : Conversion des images aux formats PNG, JPG, JPEG et BMP vers WebP.
-   **Gestion des Doublons** : Vérification de l'existence des fichiers WebP pour éviter les conversions redondantes.
-   **Ouverture Automatique du Répertoire de Sortie** : Accès direct aux images converties après la conversion.
-   **Personnalisation du Répertoire de Sortie** : Choix du répertoire de destination via une boîte de dialogue.
-   **Indicateurs de Progression** : Affichage des images converties en temps réel dans le terminal.
-   **Compatibilité Multiplateforme** : Fonctionne sur Windows, macOS et Linux.

## Prérequis

-   **Rust et Cargo** : Assurez-vous d'avoir Rust et son gestionnaire de paquets Cargo installés sur votre système. Vous pouvez les télécharger depuis [rustup.rs](https://rustup.rs/).

## Installation

1.  **Cloner le dépôt** :

    ```bash
    git clone [https://github.com/votre-utilisateur/image-converter.git](https://github.com/votre-utilisateur/image-converter.git)
    cd image-converter
    ```

2.  **Compiler le projet** :

    ```bash
    cargo build --release
    ```

## Utilisation

1.  **Exécuter l'application** :

    ```bash
    cargo run --release
    ```

2.  **Interface Graphique** :

    -   Cliquez sur "Sélectionner..." pour choisir le répertoire contenant les images à convertir.
    -   Le répertoire de sortie actuel est affiché. Cliquez sur "Changer le répertoire de sortie" pour le modifier.
    -   Cliquez sur "Convertir les images" pour lancer la conversion.

3.  **Résultat** :

    -   Le répertoire de sortie s'ouvrira automatiquement après la conversion.
    -   Les images converties seront enregistrées au format WebP, en conservant la structure des répertoires d'origine.
    -   Le terminal affichera un message pour chaque image convertie.

## Dépendances

-   `eframe` : Framework pour la création d'interfaces graphiques en Rust.
-   `egui` : Bibliothèque d'interface utilisateur pour `eframe`.
-   `image` : Bibliothèque pour la lecture et l'écriture d'images.
-   `rfd` : Pour les boîtes de dialogue de sélection de fichiers et répertoires.
-   `dirs` : Pour accéder aux répertoires spécifiques à l'utilisateur.

## Notes Importantes

-   Par défaut, l'application crée un dossier "webp" sur le bureau si aucun répertoire de sortie n'est spécifié.
-   Les images converties conservent l'arborescence des répertoires du répertoire d'entrée.
-   L'application gère les doublons en vérifiant l'existence des fichiers WebP dans le répertoire de sortie.
-   Assurez-vous que les répertoires d'entrée et de sortie sont correctement définis pour éviter les erreurs de conversion.
