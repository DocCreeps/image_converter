# Convertisseur d'Images WebP

Ce projet est une application graphique conviviale, développée en Rust à l'aide de la bibliothèque `eframe`, conçue pour simplifier la conversion d'images (PNG, JPG, JPEG, BMP) au format WebP. Son interface utilisateur moderne et intuitive permet de sélectionner des fichiers individuels ou des répertoires entiers, offrant une expérience de conversion fluide et efficace. L'interface est conçue avec des blocs centrés et des infobulles (tooltips) pour une meilleure accessibilité.

## Fonctionnalités Clés

-   **Interface Utilisateur Graphique Moderne (GUI)** : Propulsée par `eframe` et `egui`, l'application offre une expérience utilisateur agréable avec une mise en page soignée, des éléments centrés verticalement et horizontalement, et un style visuel personnalisé.
-   **Sélection Flexible de l'Entrée** : Choisissez de convertir un seul fichier image ou tous les fichiers pris en charge au sein d'un répertoire et de ses sous-répertoires grâce à des boutons dédiés ("📄 Fichier" et "📁 Répertoire").
-   **Conversion Récursive Intelligente** : Traite automatiquement tous les fichiers image pertinents (PNG, JPG, JPEG, BMP) dans le répertoire sélectionné et sa structure de sous-dossiers, en conservant l'arborescence d'origine dans le répertoire de sortie.
-   **Formats d'Image Pris en Charge** : Convertit les images aux formats PNG, JPG, JPEG et BMP vers le format WebP, optimisé pour le web.
-   **Gestion des Fichiers Existants** : Vérifie si un fichier WebP de destination existe déjà pour éviter les conversions inutiles, optimisant ainsi le temps de traitement et affichant un message dans le terminal pour les fichiers ignorés.
-   **Personnalisation du Répertoire de Sortie** : Permet de sélectionner facilement le répertoire de destination des images converties via une boîte de dialogue intuitive, accessible en cliquant sur le bouton "✏️ Changer". Le répertoire actuel est toujours affiché.
-   **Rétroaction Visuelle Claire** : Affiche des messages de confirmation (succès ou erreur) dans une fenêtre modale après la tentative de conversion, offrant une indication claire du résultat.
-   **Ouverture Automatique du Répertoire de Sortie (Optionnelle)** : Offre la possibilité d'ouvrir automatiquement le répertoire contenant les images WebP converties à la fin du processus via un bouton dans la fenêtre de résultat. Cliquer sur ce bouton fermera l'application après l'ouverture du dossier.
-   **Compatibilité Multiplateforme** : Fonctionne de manière native sur Windows, macOS et Linux, assurant une expérience cohérente quel que soit votre système d'exploitation pour la sélection des fichiers et l'ouverture du répertoire de sortie.

## Prérequis

-   **Rust et Cargo** : Assurez-vous que Rust et son gestionnaire de paquets Cargo sont installés sur votre système. Vous pouvez les télécharger et les installer facilement depuis [rustup.rs](https://rustup.rs/).

## Installation

1.  **Cloner le dépôt** :

    ```bash
    git clone [https://github.com/votre-utilisateur/image-converter.git](https://github.com/votre-utilisateur/image-converter.git)
    cd image-converter
    ```

2.  **Compiler le projet en mode Release** (pour des performances optimales) :

    ```bash
    cargo build --release
    ```

## Utilisation

1.  **Exécuter l'application** :

    ```bash
    cargo run --release
    ```

2.  **Interface Graphique Intuitive** :

    -   Cliquez sur le bouton "📄 Fichier" pour sélectionner un seul fichier image à convertir. Une boîte de dialogue s'ouvrira pour choisir le fichier.
    -   Cliquez sur le bouton "📁 Répertoire" pour choisir un dossier contenant les images à convertir (y compris les sous-dossiers). Une boîte de dialogue s'ouvrira pour sélectionner le répertoire.
    -   Le répertoire de sortie actuel est affiché. Cliquez sur le bouton "✏️ Changer" pour modifier le dossier de destination des images WebP via une nouvelle boîte de dialogue.
    -   Une fois un fichier ou un répertoire sélectionné, le bouton "🚀 Convertir" s'active. Cliquez dessus pour lancer le processus de conversion.

3.  **Résultat de la Conversion** :

    -   Une fenêtre intitulée "Résultat de la conversion" s'affichera avec un message indiquant si la conversion a réussi ou s'il y a eu une erreur.
    -   Dans cette fenêtre, un bouton "Ouvrir le dossier" vous permettra d'accéder directement au répertoire de sortie. Cliquer sur ce bouton fermera l'application après l'ouverture du dossier.
    -   Un bouton "Fermer" vous permettra de simplement fermer la fenêtre de résultat.
    -   Les images WebP converties seront enregistrées dans le répertoire de sortie, en conservant l'arborescence des dossiers d'origine si vous avez converti un répertoire. Des messages indiquant la progression de la conversion s'afficheront également dans le terminal.

## Dépendances

Ce projet utilise les bibliothèques Rust suivantes :

-   [eframe](https://crates.io/crates/eframe) : Framework pour la création d'applications graphiques natives.
-   [egui](https://crates.io/crates/egui) : Bibliothèque d'interface utilisateur immédiate pour `eframe`.
-   [image](https://crates.io/crates/image) : Bibliothèque pour le chargement et la sauvegarde de différents formats d'image.
-   [rfd](https://crates.io/crates/rfd) : Fournit des boîtes de dialogue natives pour la sélection de fichiers et de répertoires.
-   [dirs](https://crates.io/crates/dirs) : Permet d'accéder aux répertoires spécifiques à l'utilisateur, comme le bureau pour définir le répertoire de sortie par défaut.

## Notes Importantes

-   Par défaut, si aucun répertoire de sortie n'est spécifié, un dossier "webp" sera créé sur votre bureau pour enregistrer les images converties.
-   L'application préserve la structure des répertoires du dossier d'entrée lors de la conversion par lots.
-   La gestion des doublons permet d'éviter de ré-encoder des images WebP déjà présentes dans le répertoire de sortie, avec une notification dans le terminal.
-   Assurez-vous d'avoir correctement sélectionné le fichier ou le répertoire d'entrée ainsi que le répertoire de sortie pour garantir une conversion réussie. Les messages d'erreur s'afficheront dans la fenêtre de résultat en cas de problème.
