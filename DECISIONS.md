# DECISIONS.md

Journal court des décisions techniques tranchées, pour éviter que deux IA (ou
un humain et une IA) à des sessions différentes ne se contredisent ou ne
reviennent sur un choix déjà fait sans le savoir.

Format : `## AAAA-MM-JJ — Titre` suivi d'une explication brève.

---

## 2026-09-08 — Choix du moteur et de la version

Bevy `0.19` (dernière stable : 0.19.1, 13 août 2026). Rust édition 2021.
Justification : Bevy est l'écosystème ECS Rust le plus actif, avec un bon
support 2D natif. Pas de moteur alternatif envisagé pour l'instant.

## 2026-09-08 — Un crate unique, pas de workspace

Le projet reste un seul crate binaire (`jdr`) tant qu'aucune partie du code
n'a besoin d'être réutilisée indépendamment (ex: un outil d'édition de carte
séparé). Pas de workspace multi-crate pour l'instant — inutile de complexifier
avant d'en avoir besoin.

## 2026-09-08 — Structure des modules : plugin + components + systems

Chaque domaine de jeu (`player`, `world`, `combat`, `enemies`, `ui`) suit le
pattern `mod.rs` (Plugin) / `components.rs` / `systems.rs`. Voir
`ARCHITECTURE.md` pour le détail. Choisi pour permettre à plusieurs
contributeurs de travailler sur des fichiers différents sans conflits.

## 2026-09-08 — Carte de départ procédurale, pas de fichier

Le premier "monde" (`world::systems::spawn_world`) est généré en dur dans le
code (bordure d'arbres, lac central) pour avoir quelque chose de jouable
immédiatement. Le chargement de carte depuis un fichier est prévu mais pas
encore implémenté — le format (RON / JSON / CSV) reste à trancher.

## 2026-09-08 — Contrôles ZQSD par défaut

Clavier français (ZQSD) en contrôle principal, flèches directionnelles en
fallback. Décision reprise du prototype ncurses précédent du projet.

---

_Ajoutez une nouvelle entrée à chaque fois qu'un choix structurant est fait
(dépendance ajoutée, format de données choisi, abandon d'une piste, etc.)._
