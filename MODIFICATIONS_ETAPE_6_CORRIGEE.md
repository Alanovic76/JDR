# Étape 6 corrigée
- Les coffres restent cachés sous le brouillard puis deviennent visibles dès que leur case est découverte.
- Un coffre nécessite un jet de DEX (ESPACE) ; en cas d'échec il reste disponible pour une nouvelle tentative.
- Saisie du prénom au lancement.
- Tableau permanent des 10 meilleurs scores dans `jdr_highscores.txt`, trié par monstres tués puis or.
- Police Unicode chargée depuis Linux Mint (DejaVu Sans, avec Liberation Sans en repli) sans embarquer de fichier de police.
- Conservation du choix des armes, surprise et avantage/désavantage de l'Étape 6.

## Correctif compilation Bevy 0.19.1
- Corrige les littéraux flottants Rust (0.xx).
- Adapte Font::from_bytes, FontSource::Handle et FontSize::Px à Bevy 0.19.1.
- Corrige le conflit d’emprunt dans le message de dégâts du combat.
