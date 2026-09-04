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

Aucun mode d'édition (croquis, extrusion, assemblage) n'est encore implémenté —
voir [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) pour la vision d'ensemble
et la feuille de route.

## Lancer l'application

```sh
cargo run -p cao_app
```

## Structure du workspace

- `crates/core` (`cao_core`) — types de domaine, persistance et configuration,
  sans aucune dépendance UI. Réutilisable tel quel par un futur front-end
  web/tablette.
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
- [Viewport](docs/viewport.md) — les deux modes du canvas, la grille, le cube
- [Rendu](docs/rendu.md) — pipelines wgpu, lignes épaisses, rendu hors fenêtre
- [Navigation](docs/navigation.md) — gestes souris, caméra
- [Configuration](docs/configuration.md) — réglages disponibles
- [Compilation](docs/build.md) — exécutable Windows, autres plateformes

## Tests

```sh
cargo test --workspace
cargo run -p cao_render --example offscreen -- /tmp   # rend 3 PNG de contrôle
```

## Licence

Double licence MIT / Apache-2.0, voir [`LICENSE-MIT`](LICENSE-MIT) et
[`LICENSE-APACHE`](LICENSE-APACHE). Une offre professionnelle additionnelle
est envisagée à terme (non définie pour l'instant).
