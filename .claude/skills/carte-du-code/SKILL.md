---
name: carte-du-code
description: Trouver où intervenir dans le code CAO. À utiliser dès qu'on cherche quel fichier ou quelle fonction porte un comportement — esquisse, cotation, solveur, contraintes, extrusion, révolution, booléens, historique, annulation, viewport, caméra, cube d'orientation, grille, rendu wgpu, réglages, profils, raccourcis, barre d'outils, format .caopart, menu de démarrage.
---

# Où vit quoi

`docs/` raconte ce que fait le logiciel. Ce skill dit où c'est écrit.
La référence complète, lisible aussi par un humain : `docs/carte-du-code.md`.

Ici on cherche **où un comportement est déjà écrit**. Pour savoir **où poser un
fichier neuf** — quel dossier, ce qu'il a le droit d'importer —, c'est
`docs/code-layout.md`, et le test d'architecture le vérifie.

## Les cinq crates et le sens des dépendances

```
cao_app  ──►  cao_core  ──►  cao_sketch
   │             └────────►  cao_solid
   └──────────►  cao_render
```

`cao_sketch` et `cao_solid` ne dépendent de rien d'autre que `glam` et `serde`.
`cao_render` ne connaît que `wgpu`, `glam` et `bytemuck` — aucun framework
d'interface. `cao_app` est le seul à voir `egui`/`eframe`.

**Attention au nom.** `cao_core` se présente comme « les types de domaine »,
mais il dépend de `cao_sketch` et `cao_solid` et orchestre esquisse, solide,
historique et persistance : c'est la couche **application**. Les vrais domaines
sont `cao_sketch` et `cao_solid`. Voir le skill `architecture-rust`.

## Comportement → fichier

### Dessiner et coter — `cao_sketch`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| Le modèle d'esquisse, points, traits, cercles | `sketch/src/sketch.rs` | `Sketch`, `live_points`, `live_segments`, `live_circles` |
| Effacer un élément et ses dépendants | `sketch/src/sketch.rs` | `Sketch::erase`, `Erased` |
| Poser ou retirer une contrainte | `sketch/src/sketch.rs` | `add_constraint`, `add_tangency`, `erase_constraint` |
| Les types de contraintes et de cotes | `sketch/src/constraints.rs` | `Constraint`, `Dimension`, `DimensionTarget`, `Freedom` |
| **Le solveur** — rendre toutes les valeurs vraies ensemble | `sketch/src/solver.rs` | `solve(millimeters_per_unit)` → `SolveOutcome` |
| Les cinq constructions de cercle | `sketch/src/construct.rs` | `centre_through`, `centre_touching_two`, `circle_touching_three` |
| Le plan de travail, 2D ↔ 3D | `sketch/src/plane.rs` | `WorkPlane::to_world`, `to_local`, `ray_intersection` |
| Les aires fermées, pour extruder | `sketch/src/regions.rs` | `Sketch::regions()` → `Vec<Region>` |

### Volumes — `cao_solid`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| Le maillage, les faces, le lancer de rayon | `solid/src/mesh.rs` | `Mesh`, `Polygon`, `ray_hit`, `bounds` |
| Extruder une aire en prisme | `solid/src/mesh.rs` | `prism(...)` |
| Faire tourner une aire autour d'un axe | `solid/src/mesh.rs` | `revolution(...)` |
| Ajouter ou enlever de la matière | `solid/src/boolean.rs` | `Mesh::union`, `Mesh::difference` (arbre BSP) |

### Historique et persistance — `cao_core`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| La liste des opérations, annuler, refaire | `core/src/history.rs` | `History`, `Operation`, `applied_operations` |
| **Rejouer l'historique pour obtenir la géométrie** | `core/src/state.rs` | `PartState::rebuild`, `PartState::apply` |
| Le fichier `.caopart` (zip), lecture et écriture | `core/src/document.rs` | `PartDocument`, `SCHEMA_VERSION = 3` |
| Les dix pièces récentes | `core/src/recents.rs` | `RecentList` |
| Chemins, dossier des pièces, journal de plantage | `core/src/storage.rs` | `default_projects_dir`, `record_panics` |
| Les commandes de l'interface | `core/src/command.rs` | `Command`, `label`, `hint`, `family` |
| Réglages et profils nommés | `core/src/settings.rs` | `Settings`, `Profile`, `Profiles` |
| Réglages du viewport et de la navigation | `core/src/config.rs` | `ViewportConfig`, `Binding`, `NavigationPreset` |
| Couleurs, dégradés | `core/src/theme.rs` | `Theme`, `Background`, `Rgba`, `Stop` |
| Raccourcis clavier | `core/src/shortcuts.rs` | `Shortcuts`, `Chord`, `Key` |
| Arrangement de la barre d'outils | `core/src/toolbar.rs` | `ToolbarLayout`, `Item`, `Edge` |

### Rendu GPU — `cao_render`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| Les pipelines wgpu, la passe de rendu | `render/src/renderer.rs` | `SceneRenderer::prepare`, `paint`, `SceneFrame` |
| Caméra orbite, transitions de vue | `render/src/camera.rs` | `OrbitCamera`, `ViewTransition`, `view_angles_towards` |
| Le cube d'orientation, faces/arêtes/coins | `render/src/cube.rs` | `push_faces`, `zone_at`, `is_visible` |
| Axes, grille adaptative, fond, solides | `render/src/geometry.rs` | `push_axes`, `push_grid`, `push_background`, `push_solid`, `adaptive_step` |
| Rendu hors fenêtre, contrôle visuel | `render/examples/offscreen.rs` | `cargo run -p cao_render --example offscreen -- /tmp` |

### Interface — `cao_app`

| Ce qu'on cherche | Fichier | Point d'entrée |
| --- | --- | --- |
| L'état de l'application, la boucle de trame | `app/src/app.rs` | `CaoApp`, `impl eframe::App` |
| Le routage entre modes | `app/src/screens/mod.rs` | `enum Screen`, `struct OpenPart` |
| **Le canvas : gestes, hit-test, dessin** | `app/src/screens/viewport.rs` — le plus gros fichier du dépôt | `show(ui, state, sketch)`, `ViewportState`, `ViewMode` |
| L'outil d'esquisse, saisie au clavier | `app/src/screens/sketch.rs` | `SketchEditor`, `LiveInput`, `CircleMode`, `Selection` |
| Le placement des cotes à l'écran | `app/src/screens/annotations.rs` | `push(...)`, `Placement`, `Style` |
| Extrusion et révolution, côté interface | `app/src/screens/extrusion.rs` | `ExtrusionState` |
| Le panneau Historique | `app/src/screens/history_tree.rs` | `show(...)` → `HistoryAction` |
| La barre d'outils | `app/src/screens/ribbon.rs` | `Ribbon::show`, `is_enabled` |
| L'écran de réglages | `app/src/screens/settings.rs` | `show(ui, profiles, editor)` |
| Le menu de démarrage | `app/src/screens/start_menu.rs` | `show(...)` → `StartMenuAction` |

## Les invariants — ne pas les casser

**Les nombres.** Le noyau — esquisse, solveur, solide, booléens — calcule en
`f64`. La caméra, le rendu et l'interface sont en `f32`, parce que c'est ce que
le GPU et `egui` prennent. La conversion se fait **au dernier moment**, à chaque
passage de frontière. Le `f32` ne garde que sept chiffres : une pièce d'un mètre
décrite en millimètres n'a plus qu'un pas de 6·10⁻⁵ mm, et l'erreur s'accumule
dans les booléens — c'est ce qui avait fait boucler la partition de l'espace.

**La géométrie n'est jamais enregistrée.** Un `.caopart` contient les
métadonnées et l'historique des opérations, rien d'autre. La géométrie est
reconstruite par `PartState::rebuild`, qui rejoue les opérations. C'est ce qui
fait de l'annulation, du rétablissement et du retour à une étape la même
opération. Toute géométrie qu'on garderait à côté finirait par diverger de
l'historique : il n'y a **qu'un seul** endroit où la géométrie est produite,
`PartState::apply`.

**Un mode = une variante de `Screen`.** Un nouveau mode (assemblage, ...) ajoute
une variante à `enum Screen` et son propre module dans `screens/`. Jamais une
branche greffée sur un module existant.

**`cao_core` ne dépend d'aucune crate UI.** C'est la condition pour qu'un futur
front-end tablette ou web le réutilise tel quel. Ni `egui`, ni `eframe`, ni
`winit`, ni `wgpu`.

**`cao_app` est un shell fin.** Fenêtre et routage entre modes. Dès qu'un mode
porte une logique métier non triviale, il devient son propre crate.

## Les zones sans filet

Trois endroits n'ont **aucun test** :

- `sketch/src/solver.rs` — le cœur algorithmique, et quatre correctifs récents
  portent dessus (`fix/solver-anchoring`, `fix/tangent-circles`,
  `fix/circle-handling`, `fix/dimension-handling`) ;
- `sketch/src/constraints.rs` ;
- `crates/app/` entier.

Y intervenir demande d'écrire d'abord un test qui caractérise l'existant. Voir
le skill `rust-tdd`.
