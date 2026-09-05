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
3. **Outil Ligne.** Chaque clic prolonge la polyligne depuis le point
   précédent. `Échap` termine la chaîne sans quitter l'esquisse. Un clic à
   moins de 10 pixels d'un point existant réutilise ce point — c'est ce qui
   permet de fermer un contour.
4. **Outil Cote.** Cliquer un trait le sélectionne, puis on tape sa longueur en
   millimètres dans la barre du haut.
5. **Recadrer sur l'esquisse** remet la vue exactement face au plan et recadre
   le dessin. C'est le bouton à utiliser après avoir orbité pour regarder
   derrière.

Aujourd'hui les seuls plans proposés sont les trois plans d'origine (XY, XZ,
YZ) : il n'existe pas encore de solide, donc pas de face à cliquer. Le code de
sélection ne dépend pas de ce fait — il teste un rayon contre un `WorkPlane`,
et une face de pièce en sera un.

## Les cotes, et l'échelle

Une cote se comporte différemment selon qu'elle est la première du document :

- **La première cote définit l'échelle.** Rien ne bouge : dire qu'un trait fait
  100 mm apprend simplement au document combien de millimètres vaut une unité
  du monde. C'est ce qui permet de dessiner à vue puis de donner sa taille au
  dessin après coup, sans le déformer.
- **Les suivantes sont des contraintes.** La géométrie bouge pour respecter la
  longueur demandée.

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

L'esquisse est écrite dans le `.caopart` à chaque modification. Le fichier
porte un numéro de version de schéma, et les pièces créées avant l'esquisse se
rouvrent sans problème : les champs ajoutés depuis ont des valeurs par défaut.

## Ce qui manque encore

- Aucun accrochage à la grille ni aux alignements (horizontal, vertical) : seul
  l'accrochage aux points existants est fait.
- Pas d'annulation (`Ctrl+Z`), pas de suppression de trait.
- Pas de cotes angulaires ni de cotes entre deux points quelconques.
- L'esquisse ne produit encore aucun volume : l'extrusion est l'étape suivante.
