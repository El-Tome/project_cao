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

### Sur quoi on dessine

Les trois plans d'origine, et — dès qu'il y a de la matière — **n'importe
quelle face plane de la pièce**. Les faces passent devant les trois plans là où
elles sont, celles-ci restant disponibles partout ailleurs. Voir
[extrusion.md](extrusion.md).

## Les outils

| Outil | Geste |
| --- | --- |
| **Sélection** | Cliquer-glisser un point pour le déplacer. |
| **Ligne** | Clics successifs, chaque trait prolonge le précédent. `Échap` termine la chaîne. |
| **Rectangle** | Deux clics : deux coins opposés. |
| **Cercle** | Deux clics : le centre puis un point du bord. |
| **Point** | Un clic pose un point isolé. |
| **Cote** | Cote intelligente, voir plus bas. |

Ligne, rectangle et cercle **réutilisent les points déjà présents** quand le
curseur en survole un : les formes se tiennent entre elles au lieu d'empiler
des points au même endroit. Rien n'oblige jamais à poser ces points d'abord —
l'outil Point est là pour les cas où on le veut explicitement.

L'outil Sélection déplace aussi les **cotes elles-mêmes** : attraper une
annotation la décale, ce qui permet de la sortir de là où elle gêne. Le
décalage est enregistré avec la cote.

Déplacer un point avec l'outil Sélection ne casse pas les cotes déjà posées :
le dessin se réajuste autour de lui. Le déplacement n'est enregistré qu'au
**relâchement** — pendant le glissement le point suit simplement le curseur, ce
qui évite de remplir l'historique de milliers d'entrées disant la même chose.

## La cote intelligente

Un seul outil, qui mesure ce qu'on lui montre :

| Ce qu'on clique | Ce qu'on obtient |
| --- | --- |
| Un trait | Sa longueur |
| Deux points | La distance entre eux, reliés ou non |
| Deux traits qui se touchent | L'angle entre eux |
| Un trait puis un axe de l'esquisse | L'angle avec cette direction |
| Un axe puis un trait | Le même, dans l'autre ordre |
| Un cercle | Son rayon |

Un point l'emporte sur un trait sous le même curseur : c'est la plus petite
cible, donc la viser est un acte délibéré.

**Un trait posé sur un axe** se sélectionne en cliquant deux fois dessus : le
premier clic prend le trait, le second — qui retombe forcément sur le même
trait — est lu comme « et maintenant l'axe sur lequel il repose ». Sans ça un
rectangle dessiné le long des axes ne pouvait jamais être contraint.

Quand deux choses se superposent et que la mauvaise l'emporte, la rangée
**Mesurer** force le type : *Intelligente*, *Point à point*, *Trait*, *Angle*,
*Rayon*.

## L'aimantation

Le curseur est attiré par ce dont il est proche, dans cet ordre :

1. **Un point existant**, à moins de 10 pixels — c'est ce qui permet de fermer
   un contour, de rattacher une forme à une autre, et de se poser sur
   l'origine.
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

## Ce que les cotes dessinent

Une cote n'est pas qu'un nombre posé à côté du dessin : elle est **tracée**,
avec ses lignes d'attache, sa ligne de cote et ses flèches pour une longueur,
un arc fléché pour un angle, un rayon fléché pour un cercle. La valeur s'écrit
sur le tracé.

C'est du dessin vectoriel produit par le code, pas des images : quelques
segments par cote, qui suivent la géométrie quand elle bouge et restent nets à
n'importe quel zoom. Une image devrait être refaite pour chaque valeur et
chaque angle.

Une cote en lecture seule est tracée plus discrètement, en gris : elle rend
compte, elle ne décide pas.

### Déplacer une cote

Avec l'outil Sélection, attraper une cote la décale, et le décalage est
enregistré avec elle. C'est **toute l'annotation** qui bouge — la ligne, ses
flèches et sa valeur ensemble — les lignes d'attache s'étirant pour suivre : un
nombre qui s'éloignerait seul de sa ligne se lirait comme une étiquette égarée.

Un angle reste accroché au coin qu'il mesure : le tirer ouvre son arc au lieu de
l'arracher. Un rayon tourne autour de son cercle.

## Les couleurs : où en est le dessin

| Couleur | Ce que ça veut dire |
| --- | --- |
| **Jaune** | Il reste de la liberté : cet élément peut encore bouger. |
| **Vert** | Entièrement contraint : ce point ne peut plus se déplacer du tout. |
| **Gris** | Une autre esquisse que celle en cours d'édition. |

La couleur est **par élément, pas par esquisse** : un contour peut être
entièrement figé pendant que son voisin flotte encore, et c'est justement ce
qui montre ce qu'il reste à faire. Un trait n'est vert que si ses deux
extrémités le sont.

### Le point d'origine

Chaque esquisse possède, dès sa création, **un point à son origine**. On ne le
place pas : il est là. Il se distingue des autres par un losange, ne se déplace
jamais, et sert de référence à tout le reste.

C'est lui qui empêche un dessin de glisser, de deux façons :

- en **accrochant** un sommet dessus — un clic à proximité le rejoint plutôt
  que de poser un second point au même endroit ;
- en **mesurant depuis lui** — une cote point à point entre l'origine et un
  sommet le positionne sans qu'il ait à le toucher.

*(Plus tard, en 3D, un sommet d'une pièce existante pourra jouer le même
rôle.)*

### Ce qu'il faut pour arriver au vert

Deux choses :

1. **Les valeurs de forme nécessaires** — longueurs et angles. « Nécessaires »
   et non « toutes » : dans un triangle dont deux côtés et l'angle entre eux
   sont donnés, le troisième côté **suit** et ne peut plus être imposé.
2. **Un rattachement à l'origine**, par accrochage ou par cote.

**Exemple, un rectangle** dont un coin est sur l'origine : deux côtés et
**trois** angles droits (le quatrième suit) suffisent. Avec un seul angle droit
le quadrilatère peut encore se déformer en parallélogramme.

### L'orientation est implicite, comme le point d'origine

Faire tourner un dessin entier autour de l'origine ne change aucune longueur et
aucun angle : **aucune cote ne peut voir cette rotation**. Il fallait donc
autrefois poser une cote d'angle à 0° sur un axe, uniquement pour dire « et ça
reste dans ce sens-là ».

Ce n'est plus nécessaire : un dessin **garde le sens dans lequel il a été
dessiné**, exactement comme il possède un point d'origine sans qu'on le place.

La règle vaut **par groupe de géométrie reliée** : deux formes dessinées à
l'écart peuvent tourner l'une par rapport à l'autre, donc chacune garde son
sens de son côté. Une seule règle commune laisserait les deux libres de pivoter
l'une contre l'autre, et aucune ne serait jamais figée.

Poser malgré tout un angle avec un axe reste possible et n'enlève rien : dès
qu'une cote dit dans quel sens une forme est posée, la règle implicite s'efface
pour ce groupe — sans quoi la même liberté serait retirée deux fois et un
dessin encore libre de glisser passerait pour figé.

### Comment c'est calculé

Par le **rang** du système d'équations, pas en comptant les cotes. Chaque cote
donne une équation ; on regarde combien d'entre elles disent quelque chose de
neuf. C'est la seule façon de voir que le troisième côté d'un triangle découle
des autres — un comptage ne le verrait jamais.

Pour savoir si un point précis est figé, on calcule les **mouvements encore
possibles** (le noyau du système) : si aucun d'eux ne déplace ce point, il ne
peut plus bouger.

### Un point figé ne se déplace plus à la souris

Un sommet vert ne répond pas à l'outil Sélection. Le tirer reviendrait à défaire
en silence une valeur qui a été tapée ; pour le bouger, on change ce qui le
retient.

## Les surfaces fermées

Dès qu'un contour se referme, l'aire qu'il enclôt est **légèrement teintée**.
Quatre traits séparés deviennent une face, et on voit d'un coup d'œil si une
forme est vraiment fermée.

Une forme dessinée **dans une autre** est teintée plus franchement : sans cela
un contour et la poche qui s'y trouve se fondraient l'un dans l'autre.

Les contours sont trouvés comme une carte trouve ses pays : on longe chaque
trait en tournant toujours le plus serré possible, et le parcours revient sur
lui-même autour d'une aire exactement. Compter les traits ne suffirait pas — un
même côté appartient à deux aires quand deux formes le partagent.

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
alors mesuré par rapport à cette direction fixe. Ce n'est plus obligatoire pour
figer un dessin — l'orientation est implicite — mais c'est ce qui sert à poser
une forme à un angle voulu.

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

- Pas d'accrochage aux alignements (horizontal, vertical) d'un point existant :
  l'aimantation ne tient qu'aux points et à la grille.
- Une forme dessinée dans une autre est teintée comme une aire à part, pas
  traitée comme un trou : la zone commune reste remplie deux fois.
- Un contour qui se recoupe lui-même n'est pas teinté.
- Pas de cotes de diamètre, ni de cotes horizontales/verticales séparées (une
  cote point à point mesure toujours la distance directe).
- On ne peut pas supprimer un point ni un trait autrement qu'en revenant en
  arrière dans l'historique.
- Les contraintes géométriques (parallèle, perpendiculaire, tangent) n'existent
  pas : seules les cotes contraignent.
- Le solveur ne dit pas *quelles* cotes se contredisent quand il n'y arrive
  pas.
- L'esquisse ne produit encore aucun volume : l'extrusion est l'étape suivante.
