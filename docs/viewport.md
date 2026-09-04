# Le viewport 3D

Ce que l'utilisateur voit quand une pièce est ouverte : un espace 3D avec les
axes X/Y/Z, un cube d'orientation dans un coin, et une grille quand on est
posé sur un plan.

Voir aussi : [rendu GPU](rendu.md) · [navigation](navigation.md) ·
[configuration](configuration.md)

## Les deux modes

Le viewport a exactement deux états, décrits par `ViewMode`
(`crates/app/src/screens/viewport.rs`) :

| Mode | Ce qui est affiché | Comment on y entre |
| --- | --- | --- |
| `Free` | Seulement les 3 axes colorés | En orbitant (dès qu'on tourne la vue) |
| `Plane(plan)` | Les axes **et** la grille du plan | En cliquant une **face** du cube d'orientation |

La règle est volontairement simple : orbiter fait forcément sortir du mode
plan, puisque la vue n'est alors plus alignée sur un plan. En revanche le pan
et le zoom conservent le mode : cadrer ou zoomer sur un plan est un geste
normal.

Le cube se clique sur trois types de zones, découpées comme une grille 3×3 sur
chaque face :

| Zone cliquée | Vue obtenue | Mode |
| --- | --- | --- |
| **Face** (centre) | Vue droite sur le plan | Grille |
| **Arête** (bord) | Vue à 45° entre deux faces | Traits |
| **Coin** | Vue isométrique | Traits |

Seule une face correspond à un plan de travail : une vue d'arête ou de coin est
oblique, donc par définition alignée sur aucun plan — elle reste en mode
traits. Survoler une arête ou un coin le met en surbrillance sur toutes les
faces qu'il touche à la fois.

| Face cliquée | Vue | Plan de grille |
| --- | --- | --- |
| DESSUS / DESSOUS | ±Z | XY |
| FACE / ARRIÈRE | ∓Y | XZ |
| DROITE / GAUCHE | ±X | YZ |

Le passage à la vue est animé (~0.35 s, voir `ViewTransition`) pour qu'on
comprenne comment la pièce a tourné plutôt que de subir un saut.

## La grille adaptative

Le pas de la grille suit la suite 1 – 2 – 5 – 10 : c'est le plus petit pas dont
l'espacement à l'écran reste au-dessus de `grid_pixel_spacing` (48 px par
défaut). En zoomant, une graduation de 10 devient 5, puis 2, puis 1 ; en
dézoomant l'inverse. Une ligne sur dix est plus marquée.

La grille est centrée sur la cible de la caméra (arrondie au pas) et non sur
l'origine, pour qu'elle suive le pan sans jamais s'arrêter net ; son alpha
décroît avec la distance au centre, ce qui évite un bord franc. Les lignes qui
tomberaient exactement sur un axe sont sautées, sinon elles doubleraient la
ligne colorée de l'axe.

## Le cube d'orientation

Le cube tourne avec la caméra et indique donc comment on regarde la pièce. Ses
6 faces portent une étiquette (DESSUS, FACE, DROITE...) dessinée par egui et
non par le GPU : afficher du texte demanderait un atlas de police côté rendu,
alors qu'egui en a déjà un.

Sa position est configurable (`cube_corner`, coin haut-droit par défaut), tout
comme sa taille et sa marge. Le survol met la zone visée en surbrillance.

La détection de la zone survolée est un lancer de rayon sur le CPU
(`cube::pick_zone`), pas une lecture de pixel GPU : le cube est axis-aligned en
projection orthographique, l'intersection tient donc en quelques lignes de
calcul et reste synchrone avec l'affichage.

## La règle (barre d'échelle)

En bas à gauche, une barre longue d'exactement un carreau de la grille, avec sa
valeur (« 10 mm »). Elle répond à deux questions d'un coup d'œil : quelle est la
taille d'un carreau, et à quelle vitesse on zoome — la valeur change en
sautant de 1 à 2, 5, 10, ce qui rend le zoom lisible.

Son coin est configurable (`ruler_corner`), et elle peut être masquée
(`ruler_visible`).

## Repère et unités

Convention **Z vers le haut** (usuelle en CAO mécanique) : le plan XY est le
plan « du sol », vu de dessus.

Une unité du monde vaut **un millimètre** pour l'instant, et la règle affiche
donc des mm. À terme l'échelle devra s'adapter à la première cote posée : si on
déclare qu'un trait fait 100 mm, 5 m ou 5 mm, le visuel ne doit pas bouger,
c'est l'échelle du document qui est redéfinie. Ce n'est pas encore implémenté —
seul le type `LengthUnit` est en place pour l'accueillir.
