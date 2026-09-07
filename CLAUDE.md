# Règles pour ce projet

Voir [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) pour la vision complète
avant toute modification structurelle, et
[`docs/carte-du-code.md`](docs/carte-du-code.md) pour savoir où vit quoi.

## Comment travailler

Cinq skills portent le détail, dans `.claude/skills/` :

| Skill | Quand |
| --- | --- |
| `ouvrir-une-tache` | au tout début, avant de lire du code |
| `carte-du-code` | pour trouver où intervenir |
| `rust-tdd` | pour écrire le test avant le code |
| `architecture-rust` | avant d'ajouter une crate, un module, une dépendance ou de l'I/O |
| `revue-rust` | avant de commiter |

Le sous-agent `revue-archi-rust` relit un diff dans un contexte séparé.

Sur un clone neuf, activer le hook git une fois pour toutes :

```sh
git config core.hooksPath .githooks
```

`git commit` déclenche alors `clippy -D warnings` puis
`cargo test --workspace` (~12 s). En cas d'échec, le commit n'est pas exécuté.
La soupape `CAO_SKIP_GATE=1` existe pour les travaux en cours : elle appartient
à l'humain, un agent ne la pose jamais de lui-même.

Pour une question d'API sur `egui`, `wgpu` ou `glam`, utiliser `context7`
plutôt que sa mémoire : le projet est sur `egui 0.36`, `wgpu 30` et
`glam 0.33`, des crates dont l'API casse à chaque version mineure.

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
- Attention au nom : malgré ce qu'affirme sa documentation, `cao_core` n'est
  pas le domaine. Il dépend de `cao_sketch` et `cao_solid` et orchestre
  esquisse, solide, historique et persistance — c'est la couche application.
  Les vrais domaines sont `cao_sketch` et `cao_solid`. Une règle métier de
  géométrie va dans l'un des deux.

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
- avant chaque début de tache il faut faire un pull et check si tu n'as pas une branche en cour qui pourrai faire la feature demander car il y a plusieur personne qui travail sur ce projet
