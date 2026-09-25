# Architecture

## Moteur

- Rust
- Bevy 0.19

## Modules

### `src/main.rs`
Point d'entrée et assemblage des systèmes.

### `src/player/`
Gestion du joueur et du déplacement.

### `src/world/`
Carte, cases, murs, sortie et PNJ.

### `src/ui/`
Interface utilisateur.

## Principe

La carte est décrite par une grille ASCII dans `world/mod.rs`.

L'idée est de pouvoir remplacer plus tard cette carte codée en dur par :
- un fichier JSON/TOML
- Tiled
- un éditeur de niveau
- ou un format propriétaire.

Le joueur utilise la grille pour déterminer les cases bloquées.
