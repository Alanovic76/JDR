# JDR — Niveau 01

Premier prototype jouable du projet JDR en Rust + Bevy.

## Ce prototype contient

- Une carte 2D en grille de 25 x 15 cases
- Un personnage contrôlable
- Déplacement avec **ZQSD** ou **WASD**, ainsi que les flèches
- Collision avec les murs
- Une sortie de niveau
- Un PNJ décoratif
- Une interface indiquant les commandes et l'objectif
- Une structure de projet prévue pour être développée par plusieurs IA

## Lancer le jeu

Il faut avoir Rust installé.

```bash
cargo run
```

Pour une version optimisée :

```bash
cargo run --release
```

## Structure

```text
src/
├── main.rs
├── player/
│   └── mod.rs
├── world/
│   └── mod.rs
└── ui/
    └── mod.rs

assets/
└── README.md

tests/
└── README.md
```

## Objectif du niveau

Partir du point de départ en haut à gauche et atteindre la porte de sortie en bas à droite sans traverser les murs.

## Étape suivante

Ce niveau est volontairement simple. Il servira de base pour ajouter progressivement :
- caméra plus évoluée
- vraie carte de jeu
- sprites
- ennemis
- combat
- inventaire
- dialogues
- sauvegarde
- quêtes
