# CAO

Un logiciel de conception 3D (façon SolidWorks / Fusion 360) entièrement écrit
en Rust, pensé pour être modulaire et multiplateforme dès le départ.

## État actuel

- **Menu de démarrage** : créer une nouvelle pièce, ou rouvrir une des 10
  dernières pièces ouvertes.
- **Viewport 3D** : espace 3D avec les axes X/Y/Z, un cube d'orientation dont
  les faces, arêtes et coins sont cliquables, une grille adaptative quand on se
  pose sur un plan, et une barre d'échelle en mm. Navigation souris et
  trackpad. Voir [`docs/viewport.md`](docs/viewport.md).

- **Esquisse** : choisir un plan, puis tracer lignes, rectangles, cercles et
  points, coter des longueurs, des rayons et des angles. Un trait se dessine à
  la longueur et à l'angle voulus, tapés à côté du curseur, et se cote tout
  seul ; les angles droits se posent d'eux-mêmes ; tout s'efface avec `Suppr`. Le dessin se colore
  selon ce qu'il lui reste comme liberté. La première cote définit l'échelle,
  les suivantes déforment la géométrie. Voir [`docs/sketch.md`](docs/sketch.md).

- **Historique** : chaque geste est une opération enregistrée. Annulation
  (`Ctrl+Z`), rétablissement, et retour direct à n'importe quelle étape depuis
  le panneau Historique. Voir [`docs/historique.md`](docs/historique.md).

- **Extrusion** : après avoir terminé une esquisse, choisir une à plusieurs
  aires fermées et leur donner une hauteur — ou un angle et un axe, pour une
  révolution — en ajoutant ou en enlevant de la matière. Deux cercles l'un dans
  l'autre donnent un tube, pas un barreau. Les faces planes de la pièce servent
  ensuite de plans d'esquisse. Voir [`docs/extrusion.md`](docs/extrusion.md).

- **Réglages** : un écran de préférences pour le viewport, la navigation, les
  couleurs et le fond (dégradés compris), les raccourcis clavier et
  l'arrangement de la barre d'outils. Le tout en profils nommés, conservés entre
  les sessions, remis à zéro d'un bouton et partageables en un fichier. Voir
  [`docs/configuration.md`](docs/configuration.md).

L'assemblage reste à faire — voir
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) pour la vision d'ensemble et la
feuille de route.

## Lancer l'application

```sh
cargo run -p cao_app
```

## Structure du workspace

- `crates/core` (`cao_core`) — la couche applicative : elle orchestre esquisse,
  solide, historique et persistance. Ce n'est pas le domaine, malgré son nom.
  Sans aucune dépendance UI, donc réutilisable telle quelle par un futur
  front-end web/tablette.
- `crates/prefs` (`cao_prefs`) — thème, raccourcis, barre d'outils, profils,
  fichiers récents, sans géométrie ni UI.
- `crates/sketch` (`cao_sketch`) — modèle d'esquisse et application des cotes,
  sans rendu ni UI.
- `crates/solid` (`cao_solid`) — volumes, extrusion et opérations booléennes,
  sans rendu ni UI.
- `crates/render` (`cao_render`) — rendu GPU du viewport (wgpu), sans
  dépendance UI non plus.
- `crates/app` (`cao_app`) — interface desktop (egui/eframe) : menu de
  démarrage et viewport.

## Exécutable Windows

```sh
./scripts/build-windows.sh
```

Produit un `.exe` autonome depuis macOS ou Linux — voir
[`docs/build.md`](docs/build.md).

## Documentation

- [Architecture](docs/ARCHITECTURE.md) — vision, découpage, feuille de route
- [Contextes](docs/contexts.md) — où sont les coutures, et où elles vont
- [Où va un fichier](docs/code-layout.md) — les dossiers, ce qu'ils importent
- [Carte du code](docs/carte-du-code.md) — quel fichier porte quel comportement
- [Glossaire](docs/glossary.md) — les mots, et ce qu'ils veulent dire ici
- [Esquisse](docs/sketch.md) — dessiner, coter, la règle d'échelle
- [Historique](docs/historique.md) — opérations, annulation, format `.caopart`
- [Interface](docs/interface.md) — barre d'outils détachable, panneaux
- [Viewport](docs/viewport.md) — les deux modes du canvas, la grille, le cube
- [Rendu](docs/render.md) — pipelines wgpu, lignes épaisses, rendu hors fenêtre
- [Navigation](docs/navigation.md) — gestes souris, caméra
- [Configuration](docs/configuration.md) — réglages disponibles
- [Compilation](docs/build.md) — exécutable Windows, autres plateformes

## Tests

```sh
cargo test --workspace
cargo run -p cao_render --example offscreen -- /tmp   # rend 3 PNG de contrôle
```

`scripts/verifier.sh` enchaîne `cargo fmt --all --check`, `clippy -D warnings`
puis `cargo test --workspace` ; les deux hooks locaux l'appellent avant chaque
commit.
`crates/app/tests/architecture.rs` y vérifie les règles d'architecture — graphe
des crates, dossiers, budget de 400 lignes par fichier — et
`crates/app/tests/gate.rs` que la CI contrôle bien les mêmes choses que lui.

## Licence

Double licence MIT / Apache-2.0, voir [`LICENSE-MIT`](LICENSE-MIT) et
[`LICENSE-APACHE`](LICENSE-APACHE). Une offre professionnelle additionnelle
est envisagée à terme (non définie pour l'instant).
