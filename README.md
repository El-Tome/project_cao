# CAO

Un logiciel de conception 3D (façon SolidWorks / Fusion 360) entièrement écrit
en Rust, pensé pour être modulaire et multiplateforme dès le départ.

## État actuel

Seul le **menu de démarrage** existe : créer une nouvelle pièce, ou rouvrir
une des 10 dernières pièces ouvertes. Aucun mode d'édition (croquis,
extrusion, assemblage) n'est encore implémenté — voir
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) pour la vision d'ensemble et la
feuille de route.

## Lancer l'application

```sh
cargo run -p cao_app
```

## Structure du workspace

- `crates/core` (`cao_core`) — types de domaine et persistance, sans aucune
  dépendance UI. Réutilisable tel quel par un futur front-end web/tablette.
- `crates/app` (`cao_app`) — interface desktop (egui/eframe), pour l'instant
  limitée au menu de démarrage.

## Licence

Double licence MIT / Apache-2.0, voir [`LICENSE-MIT`](LICENSE-MIT) et
[`LICENSE-APACHE`](LICENSE-APACHE). Une offre professionnelle additionnelle
est envisagée à terme (non définie pour l'instant).
