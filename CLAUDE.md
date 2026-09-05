# Règles pour ce projet

Voir [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) pour la vision complète
avant toute modification structurelle.

## Modularité — non négociable

- `cao_core` ne dépend jamais d'une crate UI (`egui`, `eframe`, ...). C'est la
  seule façon de garder ce crate réutilisable par un futur front-end
  tablette/web.
- Un nouveau mode (croquis, assemblage, ...) = une nouvelle variante de
  `Screen` (`crates/app/src/screens/mod.rs`) + son propre module dans
  `screens/`. Ne jamais entasser plusieurs modes dans un seul fichier/match.
- Dès qu'un mode dépasse un simple écran (logique métier non triviale), il
  doit devenir son propre crate (`cao_sketch`, `cao_assembly`, ...) plutôt
  que de grossir `cao_app`.
- `cao_app` doit rester un shell fin : fenêtre + routage entre modes, pas de
  logique métier.

## Portée

- Ne pas anticiper le collaboratif temps réel ni le format de fichier final
  (arbre de fonctions) tant que le besoin n'est pas concret — ce sont des
  décisions explicitement remises à plus tard (voir ARCHITECTURE.md).
- Ne pas ajouter de fonctionnalité, de crate ou d'abstraction non demandée.
  Ce projet grossit par petites étapes explicitement demandées par
  l'utilisateur.

## Style

- Pas de commentaires sauf pour une raison non évidente (contrainte cachée,
  workaround). Le code doit se suffire à lui-même.
- Les textes visibles par l'utilisateur (UI, messages) sont en français mais un systeme de traduction sera neccessaire.
- Ne pas ésiter a faire des commits et des branches avec git pour revenir en arrière au cas où il y aurait un probleme tu peux également push sur le dépot.
- Créer une doc sur plusieur fichier en parrallèle pour faciliter la compréntion du code et des fonctions pour faciliter l'intervention dans les fichiers.
- n'hésite pas a demander au moindre moment ou tu ne comprends pas ou que la demande n'est pas très clair ou qu'il manque des informations avant de faire la moindre action
