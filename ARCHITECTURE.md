# Architecture

## Moteur
- Rust
- Bevy 0.19

## Modules

### `src/main.rs`
Point d'entrée et assemblage des plugins.

### `src/player/`
Gestion du joueur, déplacement, collisions et arrivée.

### `src/world/`
Carte, terrain, rochers, végétation, sortie et emplacement du monstre.

### `src/combat/`
Caractéristiques du joueur, types de monstres, D20 et résolution du combat.

### `src/ui/`
Interface d'exploration et fenêtre de combat.

## Niveau

Le niveau est maintenant un terrain ouvert décrit par une grille ASCII :
- `#` = bord / rocher gris
- `R` = rocher gris
- `V` = végétation verte
- `P` = départ
- `E` = sortie jaune
- `.` = terrain praticable

Le monstre est placé aléatoirement sur une case praticable à l'intérieur du terrain.

## Combat

Le joueur choisit un monstre avec les touches `1` à `4`, puis lance le D20 avec `ESPACE`.
