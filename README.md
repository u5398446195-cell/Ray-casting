# 🕹️ Rust Raycasting 3D (Raylib)

Un moteur de raycasting 3D rétro fait maison, écrit entièrement en **Rust** et propulsé par la bibliothèque **Raylib**. Le projet est conçu pour tourner nativement sur desktop et être compilé vers le Web (**WebAssembly / WASM**).

## 🚀 Fonctionnalités

- **Moteur Raycasting 3D personnalisé** : Inspiré des classiques du genre (style *Wolfenstein 3D*), gérant les textures de murs, le ombrage (*shading*) et le sol/plafond.
- **HUD & Minimap en temps réel** : Affichage d'une mini-carte dynamique avec la position et l'orientation du joueur, ainsi qu'un compteur de score.
- **Écran de victoire** : Overlay de fin de partie stylisé.
- **Architecture modulaire** : Code séparé proprement (`main.rs`, `player.rs`, `map.rs`).

## 🎮 Contrôles

| Touche | Action |
| :--- | :--- |
| **Flèches directionnelles** | Se déplacer et pivoter dans le labyrinthe |

## 🛠️ Stack technique

- **Langage** : [Rust](https://www.rust-lang.org/)
- **Graphismes / Fenêtrage** : [Raylib](https://www.raylib.com/) (via la crate `raylib-rs`)
- **Compilation Web** : Emscripten / WebAssembly

## 📁 Structure du projet

```text
rust_raylib/
├── src/
│   ├── main.rs      # Boucle de jeu, rendu 3D et gestion globale
│   ├── player.rs    # Gestion de la position, des mouvements et collisions
│   └── map.rs       # Définition de la grille du labyrinthe et des tuiles
├── Cargo.toml       # Dépendances du projet
└── README.md
