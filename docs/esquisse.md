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

Un clic à moins de 10 pixels d'un point existant réutilise ce point — c'est ce
qui permet de fermer un contour, et de rattacher une forme à une autre.

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
| **Jaune** | Il reste de la liberté : la forme peut encore bouger ou changer de taille. |
| **Vert** | Entièrement contrainte : plus rien à déterminer. |
| **Gris** | Une autre esquisse que celle en cours d'édition. |

Une forme perd sa liberté à mesure qu'on la cote, et un point posé **sur
l'origine** de l'esquisse la fige sur place — c'est ce qui enlève la dernière
liberté de translation.

### Comment c'est calculé, et ses limites

C'est un **comptage**, pas une analyse de rang : chaque point vaut deux
inconnues, chaque cercle ajoute son rayon, chaque cote pilotante en retire une,
un point sur l'origine en retire deux. Quand le compte tombe à zéro, la forme
est déclarée contrainte.

Ce comptage ne sait pas voir que deux contraintes disent la même chose
autrement. Un dessin qu'il annonce contraint peut donc, dans un montage
inhabituel, l'être imparfaitement. C'est suffisant pour colorer le dessin et
signaler la redondance évidente, et ce n'est volontairement pas présenté comme
davantage.

## Les cotes en trop

Poser une cote sur une forme qui n'a plus de liberté n'apporte rien. Plutôt que
de la refuser, l'application la pose **en lecture seule** : elle affiche la
valeur mesurée, entre parenthèses et en gris, et son champ n'est pas
modifiable. Un message le dit au moment de la poser.

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

La règle est volontairement simple et prévisible :

- le **point de départ** du trait est l'ancre, il ne bouge jamais ;
- le **point d'arrivée** glisse le long du trait jusqu'à la bonne longueur ;
- tout ce qui est accroché à ce point d'arrivée le suit **en bloc**, en gardant
  ses propres longueurs et angles.

Ancre veut dire ancré *à ce qui le tient* : si une cote ultérieure déplace la
géométrie à laquelle ce point appartient, il suit, parce que c'est le même
point partagé.

**Cas du contour fermé.** Si le dessin reboucle sur l'ancre — un carré, par
exemple — aucun déplacement en bloc ne peut satisfaire la longueur sans casser
la boucle. Seul le point d'arrivée est alors déplacé, ce qui déforme les traits
voisins, et l'application le dit : « Contour fermé : seul le point d'arrivée a
bougé ».

### Ce que ce n'est pas

Ce n'est **pas** un solveur de contraintes général. Il n'y a ni parallélisme,
ni perpendicularité, ni tangence, ni résolution numérique itérative ; poser
deux cotes contradictoires ne déclenche aucun diagnostic. C'est une règle de
propagation déterministe, suffisante pour dessiner et donner ses dimensions à
un contour, et qui devra être remplacée par un vrai solveur quand les
contraintes géométriques arriveront.

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
- L'esquisse ne produit encore aucun volume : l'extrusion est l'étape suivante.
