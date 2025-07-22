# Convertisseur d'Images WebP

Ce projet est une application graphique conviviale, développée en Rust à l'aide de la bibliothèque `eframe`, conçue pour simplifier la conversion d'images (PNG, JPG, JPEG, BMP) au format WebP. Son interface utilisateur moderne et intuitive permet de sélectionner des fichiers individuels, plusieurs images indépendantes ou des répertoires entiers, offrant une expérience de conversion fluide et efficace. L'interface est conçue avec des blocs centrés et des infobulles (tooltips) pour une meilleure accessibilité.

## Fonctionnalités Clés

-   **Interface Utilisateur Graphique (GUI)** : Propulsée par `eframe` et `egui`, l'application offre une expérience utilisateur agréable avec une mise en page soignée, des éléments centrés verticalement et horizontalement, et un style visuel personnalisé.
-   **Sélection Flexible de l'Entrée** : Choisissez de convertir un seul fichier image, plusieurs fichiers indépendants ou tous les fichiers pris en charge au sein d'un répertoire et de ses sous-répertoires grâce à des boutons dédiés ("📄 Fichier Unique", "📂 Plusieurs Fichiers" et "📁 Répertoire").
-   **Glisser-déposer (Drag & Drop)** : Déposez simplement des fichiers ou des dossiers directement dans la fenêtre de l'application pour une sélection rapide et intuitive.
-   **Conversion Récursive Intelligente** : Traite automatiquement tous les fichiers image pertinents (PNG, JPG, JPEG, BMP) dans le répertoire sélectionné et sa structure de sous-dossiers, en conservant l'arborescence d'origine dans le répertoire de sortie.
-   **Formats d'Image Pris en Charge** : Convertit les images aux formats PNG, JPG, JPEG et BMP vers le format WebP, optimisé pour le web.
-   **Gestion des Fichiers Existants** : Configurez comment l'application doit gérer les fichiers WebP déjà présents dans le répertoire de sortie :
    * **Ignorer** : Ne pas convertir si le fichier WebP existe déjà, avec une notification dans le terminal.
    * **Écraser** : Remplacer le fichier WebP existant.
    * **Renommer** : Créer une nouvelle version du fichier avec un suffixe (ex: `image-1.webp`).
-   **Personnalisation du Répertoire de Sortie** : Permet de sélectionner facilement le répertoire de destination des images converties via une boîte de dialogue intuitive, accessible en cliquant sur le bouton "📁 Changer". Le répertoire actuel est toujours affiché.
-   **Rétroaction Visuelle Claire** : Affiche des messages de confirmation (succès ou erreur) via des "toasts" de notification temporaires et une fenêtre modale pour les erreurs critiques ou les messages importants.
-   **Barre de Progression** : Une barre de progression visuelle indique l'état de la conversion, offrant un retour en temps réel sur l'avancement du processus.
-   **Ouverture Automatique du Répertoire de Sortie (Optionnelle)** : Offre la possibilité d'ouvrir automatiquement le répertoire contenant les images WebP converties à la fin du processus via un bouton dans la fenêtre de résultat. Cliquer sur ce bouton fermera l'application après l'ouverture du dossier.
-   **Compatibilité Multiplateforme** : Fonctionne de manière native sur Windows, macOS et Linux, assurant une expérience cohérente quel que soit votre système d'exploitation pour la sélection des fichiers et l'ouverture du répertoire de sortie.

## Prérequis

-   **Rust et Cargo** : Assurez-vous que Rust et son gestionnaire de paquets Cargo sont installés sur votre système. Vous pouvez les télécharger et les installer facilement depuis [rustup.rs](https://rustup.rs/).

## Installation

1.  **Cloner le dépôt** :

    ```bash
    git clone [https://github.com/DocCreeps/image-converter.git](https://github.com/DocCreeps/image-converter.git)
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

    * **Sélection de l'entrée** :
        * **Glisser-déposer** : Faites glisser un ou plusieurs fichiers images, ou un dossier, directement dans la zone dédiée.
        * Cliquez sur le bouton "📄 **Fichier Unique**" pour sélectionner un seul fichier image (PNG, JPG, JPEG, BMP).
        * Cliquez sur le bouton "📂 **Plusieurs Fichiers**" pour choisir plusieurs images indépendantes.
        * Cliquez sur le bouton "📁 **Répertoire**" pour sélectionner un dossier contenant les images à convertir (y compris les sous-dossiers).
    * **Répertoire de sortie** : Le répertoire de sortie actuel est affiché. Cliquez sur le bouton "📁 **Changer**" pour modifier le dossier de destination des images WebP.
    * **Gestion des doublons** : Sélectionnez l'option souhaitée (Ignorer, Écraser, Renommer) pour gérer les fichiers WebP existants dans le répertoire de sortie.
    * **Lancer la conversion** : Une fois un fichier, des fichiers multiples ou un répertoire sélectionné, le bouton "🚀 **Convertir les images**" s'active. Cliquez dessus pour lancer le processus de conversion. Une barre de progression s'affichera pendant la conversion.

3.  **Résultat de la Conversion** :

    * Un "toast" de notification apparaîtra brièvement en bas à droite de la fenêtre, indiquant le succès ou l'échec de la conversion.
    * En cas de succès ou d'erreur critique, une fenêtre "Information" s'affichera avec un message détaillé.
    * Dans cette fenêtre, un bouton "Ouvrir le dossier" vous permettra d'accéder directement au répertoire de sortie. Cliquer sur ce bouton fermera l'application après l'ouverture du dossier.
    * Un bouton "Fermer" vous permettra de simplement fermer la fenêtre de résultat.
    * Les images WebP converties seront enregistrées dans le répertoire de sortie, en conservant l'arborescence des dossiers d'origine si vous avez converti un répertoire.

## Dépendances

Ce projet utilise les bibliothèques Rust suivantes :

-   [eframe](https://crates.io/crates/eframe) : Framework pour la création d'applications graphiques natives.
-   [egui](https://crates.io/crates/egui) : Bibliothèque d'interface utilisateur immédiate pour `eframe`.
-   [image](https://crates.io/crates/image) : Bibliothèque pour le chargement et la sauvegarde de différents formats d'image.
-   [rfd](https://crates.io/crates/rfd) : Fournit des boîtes de dialogue natives pour la sélection de fichiers et de répertoires.
-   [dirs](https://crates.io/crates/dirs) : Permet d'accéder aux répertoires spécifiques à l'utilisateur, comme le bureau pour définir le répertoire de sortie par défaut.
-   [walkdir](https://crates.io/crates/walkdir) : Utilisé pour parcourir les répertoires de manière récursive.

## Notes Importantes

-   Par défaut, le répertoire de sortie sera `webp_converted` sur votre bureau. Vous pouvez le changer à tout moment.
-   L'application préserve la structure des répertoires du dossier d'entrée lors de la conversion par lots (mode "Répertoire").
-   La validation du chemin de sortie empêche la conversion vers le même répertoire ou un sous-répertoire du chemin d'entrée pour éviter des problèmes de boucle ou d'écrasement involontaire.