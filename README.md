# JDR — Prototype 2D Rust + Bevy

Petit jeu de rôle 2D développé en Rust avec Bevy 0.19.

## État actuel — Étape 3

Le prototype contient maintenant :

- déplacement avec ZQSD / WASD / flèches ;
- personnage avec FOR, INT, CON, WIS, DEX et CHA ;
- carte générée aléatoirement à chaque lancement ;
- rochers gris et végétation verte bloquants ;
- chemin garanti entre le départ et la sortie ;
- sortie jaune ;
- brouillard de guerre persistant ;
- vision du personnage sur sa case et une case autour de lui ;
- 4 à 6 monstres invisibles placés aléatoirement ;
- rencontres avec Gobelin, Renard, Loup ou Hobgobelin ;
- combat simple au D20 ;
- PV du joueur et des monstres.

## Commandes

### Exploration

- ZQSD / WASD / flèches : déplacement

### Combat

- ESPACE : lancer le D20

Les monstres ne sont pas visibles sur la carte. Le combat commence automatiquement lorsque le joueur marche sur une case contenant un monstre.

## Brouillard de guerre

Au début, seule la zone proche du personnage est visible. En se déplaçant, le joueur révèle progressivement la carte. Une case déjà découverte reste visible.

## Carte aléatoire

Une nouvelle disposition est créée à chaque lancement :

- position verticale du départ et de la sortie ;
- rochers ;
- végétation ;
- emplacement des monstres.

Un passage libre est toujours conservé entre le départ et la sortie pour éviter une carte impossible.

## Lancer le jeu

```bash
cargo run
```

Vérifier la compilation :

```bash
cargo check
```

## Technologies

- Rust
- Bevy 0.19

## Prochaines idées

- améliorer les règles de combat ;
- utiliser davantage INT, WIS et CHA ;
- expérience et niveaux ;
- inventaire et objets ;
- plusieurs niveaux ;
- sauvegarde.

## Étape 4
L'interface affiche maintenant l'état complet du personnage et, pendant un combat, celui du monstre. À la fin d'une partie, un écran FIN apparaît : ENTRÉE génère une nouvelle partie et ÉCHAP quitte le jeu.

## Étape 5

- Cases agrandies d'environ 50 % (48 px au lieu de 32 px).
- Vitesse et taille du personnage adaptées à la nouvelle échelle.
- Lancement en plein écran fenêtré (sans bordure) sur l'écran principal.
- Cadre PERSONNAGE permanent à gauche.
- Cadre MONSTRE permanent à droite ; les informations apparaissent lors d'une rencontre.
- Bandeau de commandes placé en bas pour laisser la carte dégagée.
