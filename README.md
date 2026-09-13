# 🕹️ Rust Raycasting 3D (Raylib)

Un moteur de raycasting 3D rétro fait maison, écrit entièrement en **Rust** et propulsé par la bibliothèque **Raylib**. Le projet est conçu pour tourner sur desktop et être compilé pour le Web.

## 🚀 Fonctionnalités

- **Moteur Raycasting 3D personnalisé** : Inspiré des classiques du genre (style *Wolfenstein 3D*), gérant les murs, le ombrage (*shading*) et le sol/plafond.
- **HUD & Minimap en temps réel** : Affichage d'une mini-carte dynamique avec la position et l'orientation du joueur, ainsi qu'un compteur de score.
- **Écran de victoire** : Overlay de fin de partie stylisé.
- **Architecture modulaire** : Code séparé proprement (`main.rs`, `player.rs`, `map.rs`).

## 🎮 Contrôles

| Touche | Action |
| :--- | :--- |
| **Flèches directionnelles** | Se déplacer et pivoter dans le labyrinthe |

## 🛠️ Stack technique & Choix d'architecture

- **Langage** : [Rust](https://www.rust-lang.org/)
- **Graphismes / Fenêtrage** : [Raylib](https://www.raylib.com/) (via la crate `raylib-rs`)
- **Support Web (WASM)** : Étant donné que la cible WebAssembly pure (`wasm32-unknown-unknown`) ne prend pas en charge directement Raylib (qui repose sur du code C bas niveau nécessitant un contexte OpenGL et une gestion de fenêtrage), le projet utilise **Emscripten** (`wasm32-unknown-emscripten`) couplé à `cmake` pour compiler et lier proprement le code C d'origine vers le Web.

## 📁 Structure du projet

```text
rust_raylib/
├── src/
│   ├── main.rs      # Boucle de jeu, rendu 3D et gestion globale
│   ├── player.rs    # Gestion de la position, des mouvements et collisions
│   └── map.rs       # Définition de la grille du labyrinthe et des tuiles
├── Cargo.toml       # Dépendances du projet
└── README.md
