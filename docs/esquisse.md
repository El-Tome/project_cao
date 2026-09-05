# Le mode esquisse

Dessiner en 2D sur un plan, avant de passer en 3D. C'est la première brique du
cycle croquis → extrusion.

Voir aussi : [viewport](viewport.md) · [architecture](ARCHITECTURE.md)

## Le déroulé

1. **Esquisse → Nouvelle esquisse**. Les plans disponibles apparaissent en
   carrés translucides dans la vue.
2. **Cliquer un plan.** La caméra pivote pour se mettre face à lui et cadre la
   zone de travail, la grille s'affiche. Tout plan fonctionne, y compris un
   plan oblique : la vue se met face à sa normale quelle qu'elle soit.
3. **Dessiner** avec les outils ci-dessous. La forme à venir se dessine en
   continu jusqu'au curseur et suit l'accrochage, donc on voit où elle tombera
   avant de cliquer.
4. **Coter.** Cliquer ce qu'on veut coter, puis saisir la valeur dans la barre
   du haut.
5. **Recadrer** remet la vue exactement face au plan et recadre le dessin.
   C'est le bouton à utiliser après avoir orbité pour regarder derrière.

## Les outils

| Outil | Geste |
| --- | --- |
| **Ligne** | Clics successifs, chaque trait prolonge le précédent. `Échap` termine la chaîne. |
| **Rectangle** | Deux clics : deux coins opposés. |
| **Cercle** | Deux clics : le centre puis un point du bord. |
| **Point** | Un clic pose un point isolé. |
| **Cote** | Cliquer un trait ou un cercle, puis saisir la longueur ou le rayon en millimètres. |
| **Angle** | Cliquer deux traits qui se touchent, puis saisir l'angle en degrés. |

## L'aimantation

Le curseur est attiré par ce dont il est proche, dans cet ordre :

1. **Un point existant**, à moins de 10 pixels — c'est ce qui permet de fermer
   un contour et de rattacher une forme à une autre.
2. **La grille**, à moins de 12 pixels, sur des quarts de carreau.

L'aimant de la grille est ce qui rend faciles le tracé sur l'origine et l'angle
droit à main levée : il suffit de viser à peu près. Il ne mord qu'à quelques
pixels, donc une position libre volontaire reste possible. Tout est réglable
(`grid_snap`, `grid_snap_divisions`, `grid_snap_pixels`).

Attention : aimanter n'est pas contraindre. Un trait posé bien horizontalement
grâce à la grille reste libre de tourner tant qu'aucune cote d'angle ne le
tient.

Un rectangle est **une seule opération** dans l'historique, pas quatre traits :
c'est ce qu'on veut voir en relisant la construction.

## Rouvrir une esquisse

Le panneau Historique affiche un bouton ✏ **Modifier** sous chaque esquisse.
Il la rouvre pour y ajouter des traits, même après avoir cliqué « Terminer » ou
fermé la pièce. La vue se remet face au plan et cadre le dessin existant.

Aujourd'hui les seuls plans proposés sont les trois plans d'origine (XY, XZ,
YZ) : il n'existe pas encore de solide, donc pas de face à cliquer. Le code de
sélection ne dépend pas de ce fait — il teste un rayon contre un `WorkPlane`,
et une face de pièce en sera un.

## Les couleurs : où en est le dessin

| Couleur | Ce que ça veut dire |
| --- | --- |
| **Jaune** | Il reste de la liberté : le dessin peut encore bouger ou changer de forme. |
| **Vert** | Entièrement contraint : plus rien à déterminer. |
| **Gris** | Une autre esquisse que celle en cours d'édition. |

### Ce qu'il faut pour arriver au vert

Trois choses, et il en manque souvent une :

1. **Toutes les valeurs de forme** — les longueurs et les angles nécessaires.
   Attention, « nécessaires » et non « toutes » : dans un triangle dont deux
   côtés et l'angle entre eux sont donnés, le troisième côté **suit** et ne
   peut plus être imposé.
2. **Un sommet sur l'origine** de l'esquisse. C'est ce qui l'empêche de
   glisser. *(Plus tard, en 3D, ce pourra aussi être un sommet d'une pièce
   existante.)*
3. **Une direction fixe** : une cote d'angle prise avec un axe de l'esquisse.
   Poser un point sur l'origine enlève les deux façons de glisser, jamais la
   façon de tourner — sans référence de direction, le dessin peut pivoter
   autour de son ancre et toutes les cotes restent vraies.

### Comment c'est calculé

Par le **rang** du système d'équations, pas en comptant les cotes. Chaque cote
donne une équation ; on regarde combien d'entre elles disent quelque chose de
neuf. C'est la seule façon de voir que le troisième côté d'un triangle découle
des autres — un comptage ne le verrait jamais.

## Les cotes en trop

Poser une cote dont la valeur découle déjà des autres n'apporte rien.
L'application le détecte et la pose **en lecture seule** plutôt que de la
refuser : elle affiche la valeur mesurée, entre parenthèses et en gris, et son
champ n'est pas modifiable. Un message le dit au moment de la poser.

C'est utile : lire une longueur reste intéressant même quand la fixer n'a pas
de sens. Et comme elle affiche toujours ce que la géométrie mesure, elle reste
juste quand le dessin bouge ensuite.

## Les cotes, et l'échelle

Une cote se comporte différemment selon qu'elle est la première du document :

- **La première cote définit l'échelle.** Rien ne bouge : dire qu'un trait fait
  100 mm apprend simplement au document combien de millimètres vaut une unité
  du monde. C'est ce qui permet de dessiner à vue puis de donner sa taille au
  dessin après coup, sans le déformer.
- **Les suivantes sont des contraintes.** La géométrie bouge pour respecter la
  longueur demandée.

### Les angles

Une cote d'angle se pose sur deux traits qui se touchent, et fait tourner le
second autour du point commun jusqu'à l'angle demandé, en entraînant ce qui y
est accroché — même esprit qu'une longueur. Le sens d'ouverture est conservé :
demander 30° sur un coin qui tourne dans un sens ne le retourne pas.

Un angle ne peut jamais définir l'échelle du document : des degrés ne disent
rien d'une taille.

### Comment la géométrie bouge

Toutes les cotes sont **re-résolues ensemble** à chaque changement. C'est ce
qui fait qu'une valeur reste vraie après en avoir modifié une autre : le
système entier est repris, au lieu d'appliquer chaque cote une fois puis de
l'oublier.

La méthode est la projection : chaque équation est corrigée un peu, à tour de
rôle, jusqu'à ce que plus rien ne bouge. Les points posés sur l'origine ne
bougent jamais. Le procédé est déterministe — même dessin, même ordre, même
nombre d'itérations — ce qui permet de reconstruire une pièce à l'identique en
rejouant son historique.

Si les valeurs se contredisent, le solveur s'arrête au bout de son quota
d'itérations et le signale au lieu de s'arrêter en silence sur l'une d'elles.

### Les angles

Une cote d'angle se pose sur deux traits qui se touchent. Avec l'outil Angle,
le second clic peut aussi tomber sur **un axe de l'esquisse** : l'angle est
alors mesuré par rapport à cette direction fixe, et c'est ce qui empêche le
dessin de tourner.

Le sens d'ouverture est conservé : demander 30° sur un coin qui tourne dans un
sens ne le retourne pas. Un angle ne peut jamais définir l'échelle du document :
des degrés ne disent rien d'une taille.

### Ce que ce n'est pas

Les seules contraintes sont les cotes : il n'y a pas de parallélisme, de
perpendicularité ni de tangence à poser explicitement. Un angle de 90° fait le
travail d'une perpendicularité, mais il faut le poser.

Le solveur est de type projection, pas de Newton : il converge bien sur les
dessins de cette taille, mais il n'y a ni détection de conflit avant coup, ni
diagnostic expliquant *quelles* cotes se contredisent — seulement le constat
qu'il n'y est pas arrivé.

## Enregistrement

Chaque geste devient une opération enregistrée dans le `.caopart` — une archive
zip, décrite dans [historique.md](historique.md). Le dessin lui-même n'est pas
stocké : il est reconstruit en rejouant ces opérations.

## Annuler

`Ctrl+Z` annule, `Ctrl+Y` (ou `Ctrl+Maj+Z`) rétablit. Le panneau Historique
permet en plus de revenir directement à n'importe quelle étape. Voir
[historique.md](historique.md).

## Ce qui manque encore

- Aucun accrochage à la grille ni aux alignements (horizontal, vertical) : seul
  l'accrochage aux points existants est fait.
- Pas de suppression d'un trait déjà tracé autrement qu'en revenant en arrière
  dans l'historique.
- Pas de cotes entre deux points quelconques, ni de cotes de diamètre.
- Les contraintes géométriques (parallèle, perpendiculaire, tangent) n'existent
  pas : seules les cotes contraignent.
- Le solveur ne dit pas *quelles* cotes se contredisent quand il n'y arrive
  pas.
- L'esquisse ne produit encore aucun volume : l'extrusion est l'étape suivante.
