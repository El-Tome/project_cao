# Carte du code

Les autres pages de `docs/` racontent ce que fait le logiciel. Celle-ci dit où
c'est écrit. Elle sert à quiconque arrive sur le projet et cherche par où
entrer — humain ou agent.

Elle répond à « où est-ce déjà écrit ». Pour « où poser un fichier neuf » — quel
dossier, ce qu'il a le droit d'importer, quelle taille il ne dépasse pas —,
c'est [`code-layout.md`](code-layout.md), et le test d'architecture le vérifie.

## Les six crates

```
cao_app  ──►  cao_core  ──►  cao_sketch
   │             └────────►  cao_solid
   ├──────────►  cao_prefs
   └──────────►  cao_render
```

| Crate | Rôle | Dépend de |
| --- | --- | --- |
| `cao_sketch` | modèle d'esquisse, contraintes, solveur | `glam`, `serde` |
| `cao_solid` | maillage, extrusion, booléens | `glam`, `serde` |
| `cao_render` | rendu GPU du viewport | `wgpu`, `glam`, `bytemuck` |
| `cao_core` | document, historique, persistance d'une pièce | les deux domaines |
| `cao_prefs` | thème, raccourcis, barre d'outils, profils, récents | `serde`, `directories` |
| `cao_app` | shell desktop, routage entre modes | tout |

Une flèche vers la gauche est interdite : `cao_sketch` ne connaîtra jamais
`cao_core`, `cao_render` ne connaîtra jamais `cao_app`.

Aucune longueur n'est chiffrée ici : un nombre dans la prose est exact au commit
qui l'écrit et faux au suivant. Les seules longueurs qui portent une règle — les
fichiers au-dessus du budget de 400 lignes — sont tenues par
`crates/app/tests/architecture.rs`, qui échoue quand elles bougent.

**Une mise en garde sur le nom.** `cao_core` se présente comme « les types de
domaine », mais il dépend de `cao_sketch` et `cao_solid` et orchestre esquisse,
solide, historique et persistance : c'est en réalité la couche **application**.
Les vrais domaines, ceux qui ne dépendent de rien, sont `cao_sketch` et
`cao_solid`. Une règle métier de géométrie va dans l'un des deux, pas dans
`cao_core`.

## Dessiner et coter — `cao_sketch`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| Modèle d'esquisse : points, traits, cercles | `sketch/src/sketch.rs` | `Sketch`, `live_points`, `live_segments`, `live_circles` |
| Effacer un élément et ses dépendants | `sketch/src/sketch.rs` | `Sketch::erase` |
| Poser ou retirer une contrainte | `sketch/src/sketch.rs` | `add_constraint`, `add_tangency`, `erase_constraint` |
| Types de contraintes et de cotes | `sketch/src/constraints.rs` | `Constraint`, `Dimension`, `DimensionTarget`, `Freedom` |
| Le solveur | `sketch/src/solver.rs` | `solve(millimeters_per_unit)` → `SolveOutcome` |
| Les cinq constructions de cercle | `sketch/src/construct.rs` | `centre_through`, `centre_touching_two`, `circle_touching_three` |
| Plan de travail, passage 2D ↔ 3D | `sketch/src/plane.rs` | `WorkPlane::to_world`, `to_local`, `ray_intersection` |
| Aires fermées, pour extruder | `sketch/src/regions.rs` | `Sketch::regions()` |

Détail fonctionnel : [`sketch.md`](sketch.md).

## Volumes — `cao_solid`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| Maillage, faces, lancer de rayon | `solid/src/mesh.rs` | `Mesh`, `Polygon`, `ray_hit`, `bounds` |
| Extruder une aire en prisme | `solid/src/mesh.rs` | `prism(...)` |
| Faire tourner une aire autour d'un axe | `solid/src/mesh.rs` | `revolution(...)` |
| Ajouter ou enlever de la matière | `solid/src/boolean.rs` | `Mesh::union`, `Mesh::difference` (arbre BSP) |

Détail fonctionnel : [`extrusion.md`](extrusion.md).

## Historique et persistance — `cao_core`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| Liste des opérations, annuler, refaire | `core/src/history.rs` | `History`, `Operation` |
| Rejouer l'historique pour la géométrie | `core/src/state.rs` | `PartState::rebuild`, `PartState::apply` |
| Fichier `.caopart` (zip) | `core/src/document.rs` | `PartDocument`, `SCHEMA_VERSION = 3` |
| Ce qui rate à l'ouverture d'une pièce | `core/src/errors.rs` | `PartFileError` |

## Réglages, profils et récents — `cao_prefs`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| Les dix pièces récentes | `prefs/src/recents.rs` | `RecentList` |
| Chemins, journal de plantage | `prefs/src/storage.rs` | `project_dirs`, `default_projects_dir`, `record_panics` |
| Commandes de l'interface | `prefs/src/command.rs` | `Command` |
| Réglages et profils nommés | `prefs/src/settings.rs` | `Settings`, `Profile`, `Profiles` |
| Réglages viewport et navigation | `prefs/src/config.rs` | `ViewportConfig`, `Binding`, `NavigationPreset` |
| Couleurs et dégradés | `prefs/src/theme.rs` | `Theme`, `Background`, `Rgba`, `Stop` |
| Raccourcis clavier | `prefs/src/shortcuts.rs` | `Shortcuts`, `Chord`, `Key` |
| Barre d'outils | `prefs/src/toolbar.rs` | `ToolbarLayout`, `Item`, `Edge` |

Détail fonctionnel : [`historique.md`](historique.md),
[`configuration.md`](configuration.md).

## Rendu GPU — `cao_render`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| Pipelines wgpu, passe de rendu | `render/src/renderer.rs` | `SceneRenderer::prepare`, `paint` |
| Caméra orbite, transitions | `render/src/camera.rs` | `OrbitCamera`, `ViewTransition` |
| Cube d'orientation | `render/src/cube.rs` | `push_faces`, `zone_at`, `is_visible` |
| Axes, grille, fond, solides | `render/src/geometry.rs` | `push_axes`, `push_grid`, `push_background`, `push_solid` |
| Contrôle visuel hors fenêtre | `render/examples/offscreen.rs` | `cargo run -p cao_render --example offscreen -- /tmp` |

Détail fonctionnel : [`rendu.md`](rendu.md), [`viewport.md`](viewport.md).

## Interface — `cao_app`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| État de l'application, boucle de trame | `app/src/app.rs` | `CaoApp`, `impl eframe::App` |
| Routage entre modes | `app/src/screens/mod.rs` | `enum Screen`, `struct OpenPart` |
| Canvas : gestes, hit-test, dessin | `app/src/screens/viewport.rs` — le plus gros fichier du dépôt | `show(...)`, `ViewportState`, `ViewMode` |
| Outil d'esquisse, saisie clavier | `app/src/screens/sketch.rs` | `SketchEditor`, `LiveInput`, `CircleMode` |
| Placement des cotes à l'écran | `app/src/screens/annotations.rs` | `push(...)`, `Placement`, `Style` |
| Extrusion et révolution, côté UI | `app/src/screens/extrusion.rs` | `ExtrusionState` |
| Panneau Historique | `app/src/screens/history_tree.rs` | `show(...)` → `HistoryAction` |
| Barre d'outils | `app/src/screens/ribbon.rs` | `Ribbon::show`, `is_enabled` |
| Écran de réglages | `app/src/screens/settings.rs` | `show(...)` |
| Menu de démarrage | `app/src/screens/start_menu.rs` | `show(...)` → `StartMenuAction` |

Détail fonctionnel : [`interface.md`](interface.md),
[`navigation.md`](navigation.md).

## Les invariants

**Les nombres.** Le noyau — esquisse, solveur, solide, booléens — calcule en
`f64`. La caméra, le rendu et l'interface sont en `f32`, parce que c'est ce que
le GPU et `egui` prennent. La conversion se fait au dernier moment, à chaque
passage de frontière. Le `f32` ne garde que sept chiffres : une pièce d'un mètre
décrite en millimètres n'a plus qu'un pas de 6·10⁻⁵ mm, et l'erreur s'accumule
dans les booléens — c'est ce qui avait fait boucler la partition de l'espace.

**La géométrie n'est jamais enregistrée.** Un `.caopart` contient ses
métadonnées et son historique d'opérations, rien d'autre. La géométrie est
reconstruite par `PartState::rebuild`, qui rejoue les opérations — c'est ce qui
fait de l'annulation, du rétablissement et du retour à une étape la même
opération. Il n'y a **qu'un seul** endroit où la géométrie est produite :
`PartState::apply`. Toute copie gardée à côté finirait par diverger.

**Un mode = une variante de `Screen`.** Un nouveau mode ajoute une variante à
`enum Screen` et son propre module dans `screens/`, jamais une branche greffée
sur un module existant.

**`cao_core` ne dépend d'aucune crate UI.** C'est la condition pour qu'un futur
front-end tablette ou web le réutilise tel quel.

**`cao_app` doit rester un shell fin.** C'est une visée, pas un constat : c'est
la plus grosse crate du dépôt, et `viewport.rs` y tient à lui seul la caméra, le
hit-test, le clavier, les gestes et le dessin des annotations. Dès qu'un mode
porte une logique métier non triviale, il devient son propre crate.

## Ce qui n'a pas de tests

| Zone | Tests |
| --- | ---: |
| `sketch/src/solver.rs` | 0 |
| `sketch/src/constraints.rs` | 0 |
| `crates/app/src/` | 0 |

Les deux fichiers de tests de la crate portent sur le dépôt, pas sur
l'interface : `crates/app/tests/architecture.rs` en teste la forme — graphe des
crates, dossiers, budget de lignes, français sous l'interface — et
`crates/app/tests/gate.rs` vérifie que `scripts/verifier.sh` et
`.github/workflows/ci.yml` contrôlent bien les mêmes choses.

Le solveur est le cœur algorithmique et l'essentiel de son historique est fait
de correctifs successifs (`git log -- crates/sketch/src/solver.rs`), sans aucun
filet. Y intervenir demande d'écrire d'abord un test qui caractérise l'existant.

Ailleurs le dépôt est testé, et chaque test vit dans le fichier qu'il couvre.
`cargo test --workspace` en donne le compte du jour.

## Vérifier

```sh
scripts/verifier.sh          # fmt --check, clippy -D warnings, cargo test --workspace, ~12 s
cargo test -p cao_sketch     # une seule crate, pendant la boucle
cargo run -p cao_app         # lancer l'application
```
