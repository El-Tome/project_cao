# Naviguer dans la vue

Voir aussi : [viewport](viewport.md) · [configuration](configuration.md)

## À la souris (préréglage Fusion 360)

| Geste | Action |
| --- | --- |
| Molette | Zoom |
| Clic molette + glisser | Déplacement (pan) |
| Maj + clic molette + glisser | Orbite |
| Clic sur le cube | Voir [viewport](viewport.md) |

## Au trackpad

| Geste | Action |
| --- | --- |
| Deux doigts | Déplacement (pan) |
| Maj + deux doigts | Orbite |
| Pincer | Zoom |
| Alt + clic gauche + glisser | Orbite |
| Alt + Maj + clic gauche + glisser | Déplacement |

Molette de souris et défilement à deux doigts arrivent dans le même flux
d'événements ; ils sont distingués par leur unité (lignes pour une molette,
pixels pour un trackpad), ce qui permet de garder « molette = zoom » sans que
le trackpad zoome à toute vitesse.

C'est aussi pour ça que la molette a **sa propre sensibilité**
(`wheel_zoom_sensitivity`) : un cran de molette vaut *une ligne*, quand un
geste de trackpad vaut des centaines de *pixels*. Partager un même réglage
rendait forcément l'un des deux inutilisable — la molette avançait d'un
demi-pourcent par cran pendant que Ctrl + molette faisait des bonds.

Ctrl (ou Cmd) + molette et Ctrl + deux doigts zooment aussi, à la même vitesse
que le geste sans modificateur : ce n'est plus un raccourci « turbo ».

Les gestes du trackpad sont réglables (`TrackpadConfig`) : chacun des deux
défilements peut être `Pan`, `Orbit`, `Zoom` ou `Ignore`.

## Autres préréglages

`NavigationPreset` propose aussi `SolidWorks` (clic molette = orbite, Ctrl +
clic molette = pan) et `Blender` (clic molette = orbite, Maj + clic molette =
pan). Le préréglage est un champ de la configuration du viewport ; il n'y a pas
encore d'écran de préférences pour en changer.

## Comportement de la caméra

La caméra est **orbitale** : elle tourne autour d'un point cible, avec un
lacet (yaw), un tangage (pitch) et une distance. Elle ne peut donc jamais
rouler, et le tangage est borné à ±90°.

Ce paramétrage évite un piège classique : avec une matrice `look_at` et un
vecteur « haut » fixe, regarder droit vers le bas est une position dégénérée
qui fait basculer l'image. Ici, la vue de dessus est un angle comme un autre.

- **Zoom** : exponentiel, donc un cran de molette a le même effet visuel qu'on
  soit à 1 mm ou à 10 m de la pièce. Les plans proche et lointain suivent la
  distance, ce qui garde la précision de profondeur utilisable à toute échelle.
  La distance est bornée (`min_distance`, `max_distance`, par défaut 1 µm à
  1 000 km) : une borne doit exister, les positions étant en `f32`, mais elle
  est réglable et assez lointaine pour cadrer un assemblage entier.
- **Pan** : converti en unités monde selon la distance, pour que la pièce suive
  exactement le curseur.
- **Orbite** : fait toujours repasser en vue 3D libre (voir
  [viewport](viewport.md)).

Une orbite ou un pan commencé sur le canvas continue même si le curseur en
sort, comme dans tout logiciel de CAO.

## Ce qui n'existe pas encore

Le tactile et le stylet ne sont pas gérés : c'est prévu pour le portage
tablette, et cela demandera un vrai jeu de gestes (pincer pour zoomer, deux
doigts pour orbiter) en plus des liaisons souris décrites ici.
