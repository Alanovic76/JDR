# AI_CONTEXT.md

Ce fichier est le point d'entrée pour toute IA (Claude, ChatGPT, Copilot, etc.)
qui contribue à ce projet. **Lis-le en entier avant de proposer du code.**

## Objectif du projet

JDR est un jeu de rôle 2D top-down inspiré de la série Ultima (en particulier
le style visuel d'Ultima V), écrit en Rust avec le moteur Bevy. L'objectif est
un jeu jouable sur Linux, avec une esthétique rétro assumée (tuiles carrées,
palette limitée, HUD façon CRPG classique).

## Stack technique (figée — ne pas dévier sans mise à jour de ce fichier)

- **Langage** : Rust, édition 2021
- **Moteur** : Bevy `0.19` (dernière version stable au moment de la rédaction :
  0.19.1, sortie le 13 août 2026)
- **Plateforme cible** : Linux (le code doit rester portable, mais aucun test
  n'est fait sur Windows/macOS pour l'instant)
- **Rendu** : 2D uniquement, caméra orthographique (`Camera2d`)

⚠️ Bevy change beaucoup son API entre versions mineures. **Ne jamais proposer
du code basé sur une version de Bevy antérieure à 0.19** (ex: `SpriteBundle`,
`NodeBundle`, `TextBundle` sont des patterns d'anciennes versions — Bevy 0.15+
utilise des composants directs avec "required components", pas des bundles
pour ces cas-là). En cas de doute sur une API, dis-le explicitement plutôt que
d'halluciner une signature.

## Conventions de code

- `cargo fmt` et `cargo clippy` doivent passer sans warning avant tout commit
- `cargo check` doit compiler avant de proposer un diff
- Un plugin Bevy par domaine de jeu (voir ARCHITECTURE.md)
- Chaque module de `src/` suit le pattern :
  ```
  module/
    mod.rs        (déclare le Plugin, ré-exporte les types publics)
    components.rs (Component, Resource, Event)
    systems.rs    (fonctions système)
  ```
- Messages de commit : `type(scope): description` (ex: `feat(player): ajoute le déplacement ZQSD`)
- Types/fonctions publics documentés avec `///`

## État actuel du projet

_(à tenir à jour à chaque évolution significative — dernière mise à jour : mise en place du squelette initial)_

- [x] Squelette du projet : `main.rs` + 5 plugins (`player`, `world`, `combat`, `enemies`, `ui`)
- [x] Déplacement du joueur (ZQSD + flèches)
- [x] Carte de test générée procéduralement (herbe, arbres en bordure, lac central, chemin de terre)
- [x] Un ennemi placeholder (slime) spawné en dur
- [x] HUD minimal affichant les PV du joueur
- [ ] Système de combat réel (l'event `AttackEvent` existe mais n'est pas encore résolu contre les ennemis)
- [ ] Chargement de cartes depuis des fichiers (`assets/maps/`)
- [ ] Sprites réels (actuellement : rectangles de couleur unie)
- [ ] Inventaire, PNJ, dialogues, dongeons

## Règles pour les IA contributrices

1. Ne jamais casser la compilation. Si tu ne peux pas vérifier avec `cargo check`,
   dis-le clairement au lieu d'affirmer que le code compile.
2. Lire `ARCHITECTURE.md` avant toute proposition de refonte structurelle.
3. Ne pas renommer/déplacer des modules ou fichiers sans mettre à jour ce fichier
   et `ARCHITECTURE.md` en conséquence.
4. Documenter tout nouveau système ou plugin Bevy ajouté (au moins une ligne
   `///` expliquant son rôle).
5. Ne pas ajouter de dépendance externe sans la justifier dans un commentaire
   ou dans `DECISIONS.md` — chaque crate ajoutée doit avoir une raison claire.
6. Respecter le style ZQSD (clavier français) pour les contrôles, avec les
   flèches en fallback.
7. En cas de choix architectural ambigu (ex: ECS vs état global, structure de
   la carte, système de sauvegarde...), proposer 1-2 options avec leurs
   compromis plutôt que trancher silencieusement.

## Glossaire du jeu

- **Tile** : case de la grille de jeu, actuellement 32×32 px (`TILE_SIZE`)
- **TileKind** : type de terrain (`Grass`, `Tree`, `Water`, `DirtPath`)
- **HUD** : affichage à l'écran des PV/statistiques du joueur
- **AttackEvent** : événement Bevy déclenché par l'appui sur `A`

## Contacts / contributeurs

- Alain (humain, product owner / dev)
- Claude (Anthropic) — architecture, revue de code, documentation
- ChatGPT (OpenAI) — contribue également au code ; se référer à ce fichier
  avant toute proposition
