# Architecture — JDR

## Moteur
- Rust
- Bevy 0.19

## Modules

### `src/main.rs`
Point d'entrée et assemblage des plugins.

### `src/player/`
Gestion du joueur, déplacement, collisions, sortie et déclenchement des rencontres.

### `src/world/`
Génération aléatoire de la carte, rochers, végétation, sortie, emplacements invisibles des monstres et brouillard de guerre.

### `src/combat/`
Caractéristiques du joueur, types de monstres, D20 et résolution du combat.

### `src/ui/`
Interface d'exploration et écran de combat.

## Génération du niveau

La carte mesure 25 x 15 cases. Les bordures sont bloquantes. À chaque lancement, le jeu choisit le départ et la sortie puis génère les obstacles. Un chemin protégé est conservé pour garantir que la sortie reste accessible.

## Brouillard de guerre

Chaque case possède un voile noir placé au-dessus du décor. Le voile disparaît définitivement lorsque la case entre dans le rayon de vision du joueur (1 case autour du personnage).

## Monstres

4 à 6 rencontres invisibles sont placées sur des cases praticables. En entrant sur l'une de ces cases, un type de monstre est choisi et le combat démarre automatiquement.
