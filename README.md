# JDR

Jeu de rôle 2D top-down inspiré de la série *Ultima* (esthétique Ultima V),
écrit en **Rust** avec le moteur **[Bevy](https://bevy.org)**.

## Prérequis

- [Rust](https://rustup.rs/) (édition 2021, toolchain stable récente)
- Sur Linux, les dépendances système de Bevy (voir la
  [doc officielle](https://bevy.org/learn/quick-start/getting-started/setup/#linux))

## Lancer le jeu

```bash
cargo run
```

Pour une build optimisée (recommandé pour tester les performances réelles) :

```bash
cargo run --release
```

## Contrôles

| Action    | Touche(s)          |
|-----------|---------------------|
| Déplacement | Z Q S D ou flèches |
| Attaque   | A                    |
| Repos     | R                    |
| Quitter   | Échap                |

## Structure du projet

Voir [ARCHITECTURE.md](ARCHITECTURE.md) pour le détail de l'organisation du code.

## Documentation

- [AI_CONTEXT.md](AI_CONTEXT.md) — contexte et règles pour les IA contribuant au projet (Claude, ChatGPT, etc.)
- [ARCHITECTURE.md](ARCHITECTURE.md) — organisation technique du code
- [GAME_DESIGN.md](GAME_DESIGN.md) — vision et mécaniques de jeu
- [DECISIONS.md](DECISIONS.md) — journal des décisions techniques

## Contribuer

Ce projet est développé avec l'aide de plusieurs IA en plus d'un développeur
humain. Toute IA (ou humain) contribuant doit lire `AI_CONTEXT.md` avant de
proposer du code.

## Licence

MIT — voir [LICENSE](LICENSE).
