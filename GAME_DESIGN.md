# GAME_DESIGN.md

## Vision

Un RPG 2D top-down qui capture l'esprit des premiers *Ultima* (en particulier
*Ultima V*) : monde ouvert vu du dessus, tuiles carrées bien lisibles, combat
tour par tour ou en temps réel simplifié, progression par niveaux/XP.

Ce document décrit l'intention de jeu. Les détails techniques d'implémentation
vivent dans `ARCHITECTURE.md`.

## Piliers du jeu

1. **Lisibilité avant tout** — chaque tuile, sprite et élément de HUD doit être
   immédiatement compréhensible, dans l'esprit CRPG rétro.
2. **Exploration** — le monde encourage à se déplacer, découvrir des lieux,
   plutôt qu'un couloir linéaire.
3. **Combat simple mais tactique** — pas de complexité inutile, mais des
   choix significatifs (attaquer, fuir, se reposer).

## Mécaniques prévues (par ordre de priorité)

### Déjà en place (squelette)
- Déplacement du joueur sur la carte (ZQSD/flèches)
- Terrain avec plusieurs types de tuiles (herbe, arbres, eau, chemin)
- PV du joueur affichés en HUD
- Un ennemi de test

### Court terme
- Résolution réelle du combat (dégâts, mort des ennemis, PV qui baissent)
- Plusieurs types d'ennemis avec des statistiques différentes
- Système de repos (`R`) pour régénérer les PV

### Moyen terme
- Chargement de cartes définies dans des fichiers (plusieurs "écrans"/zones)
- Système d'expérience et de montée de niveau
- Inventaire basique (objets ramassables, équipement simple)
- PNJ avec dialogue simple

### Long terme / exploratoire
- Donjons (cartes séparées, plus denses en ennemis)
- Quêtes simples
- Sauvegarde / chargement de partie

## Références visuelles et d'ambiance

- *Ultima V: Warriors of Destiny* — palette de couleurs, disposition du HUD
  (panneau de statistiques sur le côté), style des tuiles carrées.
- Toute nouvelle mécanique proposée doit rester cohérente avec cette
  esthétique rétro — éviter les ajouts qui tireraient le jeu vers un style
  visuel ou mécanique trop moderne sans discussion préalable.

## Notes ouvertes

_(Section à utiliser pour noter les idées non tranchées, questions de design
en suspens, etc. — toute IA ou contributeur peut y ajouter une entrée horodatée.)_
