# ARCHITECTURE.md

## Vue d'ensemble

Le jeu est structuré en **plugins Bevy**, un par domaine fonctionnel. Chaque
plugin est responsable de ses propres composants, événements et systèmes.
`main.rs` se contente d'assembler les plugins — il ne doit pas contenir de
logique de jeu.

```
src/
├── main.rs          # Point d'entrée, assemble les plugins
├── player/           # Le joueur : déplacement, statistiques
│   ├── mod.rs         # PlayerPlugin
│   ├── components.rs  # Player, Speed, Health
│   └── systems.rs     # spawn_player, movement_system
├── world/            # La carte : tuiles, terrain
│   ├── mod.rs         # WorldPlugin
│   ├── components.rs  # Tile, TileKind
│   └── systems.rs     # spawn_world (génération procédurale temporaire)
├── combat/           # Système de combat
│   ├── mod.rs         # CombatPlugin
│   ├── components.rs  # Attack, AttackEvent
│   └── systems.rs     # attack_input_system
├── enemies/           # Ennemis
│   ├── mod.rs         # EnemiesPlugin
│   ├── components.rs  # Enemy, EnemyHealth
│   └── systems.rs     # spawn_enemies
└── ui/                # HUD et interface
    ├── mod.rs          # UiPlugin
    ├── components.rs   # HealthText (marqueurs UI)
    └── systems.rs      # spawn_hud, update_health_text
```

## Convention par module

Chaque module suit le même schéma :

- **`mod.rs`** : déclare le `struct XPlugin`, implémente `Plugin::build`,
  ré-exporte (`pub use`) les types que d'autres modules doivent consommer.
- **`components.rs`** : tous les `#[derive(Component)]`, `Resource` et
  `Event` du domaine.
- **`systems.rs`** : les fonctions systèmes (`fn ...(query: Query<...>, ...)`).

Cette séparation permet à plusieurs contributeurs (humains ou IA) de
travailler sur des modules différents sans conflits de fusion.

## Communication entre modules

- Les modules communiquent via des **événements Bevy** (`Event` +
  `EventWriter`/`EventReader`) plutôt que des appels de fonction directs,
  pour rester découplés. Exemple : `combat::AttackEvent`.
- Un module peut importer les types publics d'un autre (ex: `combat` importe
  `player::Player` pour identifier qui attaque), mais évite les dépendances
  circulaires entre plugins.

## Système de coordonnées

- Grille logique : `(grid_x, grid_y)` en `i32`, origine en bas à gauche de la carte.
- Position monde : `Transform.translation`, en pixels, avec `TILE_SIZE = 32.0`.
- La conversion grille → monde se fait dans `world::systems::spawn_world`
  (voir ce fichier pour la formule exacte).

## Prochaines évolutions structurelles prévues

- `world` : remplacer la génération procédurale par un chargement de fichier
  de carte (format à définir — probablement RON ou un format tile-based
  simple type CSV/JSON).
- `combat` : ajouter une résolution réelle de l'`AttackEvent` (détection de
  collision avec les ennemis à portée, application des dégâts).
- Ajout probable d'un module `inventory/` et `dialogue/` une fois les
  fondations stabilisées.

Toute modification de cette structure doit être répercutée ici **et** dans
`AI_CONTEXT.md`.
