# Configuration

L'objectif du projet est que le maximum de choses soit réglable plutôt que codé
en dur. Les réglages vivent dans `cao_core::config`, sans dépendance à
l'interface, et sont sérialisables (serde) pour être persistés le jour où un
écran de préférences existera.

Voir aussi : [navigation](navigation.md) · [viewport](viewport.md)

## `ViewportConfig`

| Réglage | Défaut | Effet |
| --- | --- | --- |
| `cube_corner` | `TopRight` | Coin où se place le cube d'orientation |
| `cube_size` | 96 pt | Côté du cube |
| `cube_margin` | 12 pt | Marge entre le cube et les bords du canvas |
| `navigation` | `Fusion360` | Préréglage des gestes souris |
| `orbit_sensitivity` | 0.008 rad/px | Vitesse de rotation à l'orbite |
| `zoom_sensitivity` | 0.0015 | Zoom par pixel de défilement trackpad |
| `wheel_zoom_sensitivity` | 0.12 | Zoom par cran de molette (un cran = une ligne, pas 50 pixels) |
| `min_distance` | 0.001 mm | Distance caméra minimale |
| `max_distance` | 1e9 mm | Distance caméra maximale (1 000 km) |
| `grid_pixel_spacing` | 48 px | Espacement minimal des lignes avant que le pas de grille grossisse |
| `ruler_corner` | `BottomLeft` | Coin où se place la barre d'échelle |
| `ruler_visible` | `true` | Afficher ou non la barre d'échelle |
| `unit` | `Auto` | Unité de la règle : `Auto` choisit µm/mm/m/km, ou `Fixed(unité)` |

## `TrackpadConfig`

| Réglage | Défaut | Effet |
| --- | --- | --- |
| `scroll` | `Pan` | Ce que fait un défilement à deux doigts |
| `shift_scroll` | `Orbit` | Idem avec Maj enfoncée |
| `pinch_zooms` | `true` | Le pincement zoome |
| `scroll_sensitivity` | 1.0 | Multiplicateur du défilement trackpad |

## Ce qui n'est pas encore configurable

Ces valeurs existent mais sont pour l'instant fixées dans le code de rendu, et
devront migrer vers la configuration quand le besoin se posera :

- les couleurs des axes et de la grille, et leurs épaisseurs
  (`AxisStyle`, `GridStyle` dans `cao_render::geometry`) ;
- la couleur de fond du viewport ;
- la durée de l'animation de changement de vue (0.35 s) ;
- la largeur des bords du cube qui sélectionnent une arête ou un coin
  (`ZONE_BORDER`, 22 % du côté) ;
- le champ de vision de la caméra (45°).

## Il n'y a pas encore de fichier de configuration

Aucune de ces valeurs n'est lue ni écrite sur disque : l'application démarre
toujours sur les valeurs par défaut. Le seul état persisté aujourd'hui est la
liste des pièces récentes, dans le dossier de configuration de l'OS.
