# Image Converter
Ce projet est un outil de ligne de commande écrit en Rust pour convertir des images PNG en format WebP. Il parcourt récursivement un répertoire d'entrée, convertit chaque image PNG trouvée, et enregistre les images converties dans un répertoire de sortie.

## Fonctionnalités
- Conversion récursive des images PNG en WebP. - Vérification pour éviter de reconvertir les images déjà traitées. - Affichage des messages de progression dans la console.
## Prérequis
-  [Rust](https://www.rust-lang.org/tools/install) doit être installé sur votre machine. -  [Cargo](https://doc.rust-lang.org/cargo/), le gestionnaire de paquets de Rust, doit également être installé.
## Installation
1. Clonez ce dépôt :   
```
    ssh  git clone https://github.com/votre-utilisateur/image-converter.git
    
    cd image-converter`
```

2.  Installez les dépendances :

```
    cargo build --release
```


Utilisation
-----------

1.  Modifiez les chemins d'entrée et de sortie dans le fichier `src/main.rs` pour correspondre à vos besoins :

```
    let input_dir =  Path::new("C:/Users/doria/Bureau/logos");  
    let output_dir =  Path::new("C:/Users/doria/Bureau/logos-webp");
```


2.  Exécutez le programme :
```
    cargo run
```

Exemple de sortie
-----------------

Le programme affichera des messages indiquant quelles images ont été converties ou sautées :


```
   Converted C:/Users/doria/Bureau/logos/image1.png to C:/Users/doria/Bureau/logos-webp/image1.webp 
    Skipping C:/Users/doria/Bureau/logos/image2.png, already converted
```


Dépendances
-----------

-   [image](https://crates.io/crates/image) : Utilisé pour lire et écrire des images.

