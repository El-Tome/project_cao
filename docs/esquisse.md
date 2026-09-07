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
| **Sélection** | Cliquer pour prendre, cliquer-glisser un point pour le déplacer, glisser dans le vide pour encadrer. |
| **Ligne** | Clics successifs, chaque trait prolonge le précédent. |
| **Rectangle** | Deux clics : deux coins opposés. |
| **Cercle** | Cinq façons de le poser, voir plus bas. |
| **Point** | Un clic pose un point isolé. |
| **Cote** | Deux clics : ce qu'on mesure, puis où l'annotation se pose. |

### Prendre plusieurs choses à la fois

Glisser **à partir du vide** tire une boîte, comme sur un bureau, et prend tout
ce qu'elle contient **entièrement** : un trait compte quand ses deux bouts y
sont. La moitié d'un trait ne se supprime pas, donc laisser la boîte s'en saisir
promettrait quelque chose que le dessin ne sait pas faire.

Qui répond au glissement est décidé **au départ du geste** et le reste jusqu'au
bout : un point sous le curseur au moment d'appuyer se déplace, sinon c'est une
boîte. Sans cela le geste changerait de nature en cours de route, dès que le
curseur passerait au-dessus d'un point.

`Cmd`/`Ctrl` (ou `Maj`) en cliquant **ajoute ou retire** un élément un par un,
et fonctionne aussi avec la boîte. C'est ce qui permet de désigner trois traits
qu'aucun cadre ne peut enfermer seuls.

`Suppr` efface tout ce qui est tenu **en une seule étape** d'historique : une
sélection supprimée est un seul geste, et une seule annulation la ramène.

### `Échap` recule d'un cran

Un appui abandonne ce qui est en cours : la chaîne de traits, le premier coin
d'une forme, la cote qui cherche sa place. Un second appui — quand il n'y a plus
rien à abandonner — **revient à l'outil Sélection**.

Un outil qui reste en main une fois son travail fait est un outil qui dessine un
trait perdu au clic suivant.

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

## Dessiner à une valeur

Pendant qu'une forme se dessine, deux champs suivent le curseur : la
**longueur** et l'**angle avec l'horizontale** pour un trait, la **largeur** et
la **hauteur** pour un rectangle.

Ils sont accrochés au pointeur, en bas à droite : posés sur le dessin, ils
finissaient sous le curseur, et un curseur sur les champs n'est plus un curseur
sur le canevas — la forme cessait de le suivre.

Le premier champ **prend le clavier dès qu'il apparaît**, valeur entière
sélectionnée : on tape, sans un seul Tab. Tab passe au second. Un champ auquel
on ne touche pas reste vide et montre la mesure en filigrane — garder la mesure
*dans* le champ faisait que la première frappe atterrissait derrière elle, et
« 40 » tapé sur « 0.000 » se lisait 0,00040.

Laissés tranquilles, ce sont de simples indications. **Tapés dedans, ce sont des
décisions** : le trait ne peut plus prendre une autre valeur, et la cote
correspondante est posée d'elle-même quand le trait est validé. Un trait dessiné
à une valeur n'a pas à être mesuré après coup.

Fixer l'un des deux laisse l'autre libre, ce qui est tout l'intérêt :

| Ce qui est tapé | Ce qui reste libre |
| --- | --- |
| Un angle | La longueur : le trait s'allonge et se raccourcit sur cette direction |
| Une longueur | La direction : le trait tourne à cette distance |
| Les deux | Rien ; le clic ne fait que valider |

Un rectangle marche pareil, côté par côté : une largeur tapée fige la largeur et
laisse la hauteur suivre le curseur. Les deux tailles arrivent alors comme cotes
sur la forme, avec ses angles droits.

Vider un champ reprend la décision. `Entrée` valide la forme sans avoir à
retrouver le canevas avec la souris.

Le signe suit le curseur : 30° tapé veut dire les 30° vers lesquels on pointe,
pas ceux d'en dessous. Et tant qu'une valeur est fixée, un point voisin n'est
plus accroché — cela redonnerait discrètement au trait une autre longueur.

## Les angles droits se posent tout seuls

En dessinant à la suite d'un trait, s'approcher à moins de 4° de la
perpendiculaire **cale le trait exactement à 90°**, affiche le petit carré du
dessin technique dans le coin, et pose la contrainte d'angle à la validation.

Le carré est montré **avant** de valider : une contrainte qui apparaît sans
prévenir est une mauvaise surprise. La bande est assez large pour être facile à
viser, assez étroite pour ne pas voler un angle vraiment voulu à 80°.

## La cote intelligente

Un seul outil, qui mesure ce qu'on lui montre :

| Ce qu'on clique | Ce qu'on obtient |
| --- | --- |
| Un trait | Sa longueur, sa largeur ou sa hauteur, selon où la cote se pose |
| Deux points | La distance entre eux, reliés ou non |
| Deux traits qui se touchent | L'angle entre eux |
| Un trait puis un point | La distance du point à la droite, prise d'équerre |
| Un point puis un trait | La même, dans l'autre ordre |
| Un trait puis un axe de l'esquisse | L'angle avec cette direction |
| Un axe puis un trait | Le même, dans l'autre ordre |
| Un cercle | Son **diamètre** |
| Un cercle puis son centre | Son rayon |

Le premier clic prend une entité et **montre déjà ce qu'elle mesure toute
seule** — la longueur d'un trait. Cliquer une seconde entité avant de poser la
cote la transforme : un autre trait en fait un angle, un point en fait une
distance à la droite. C'est ce qu'on attend d'une cote dite intelligente, et ça
évite d'aller chercher le type dans la rangée *Mesurer*.

Une cote d'un point à une droite se lit comme si un segment perpendiculaire
descendait du point jusqu'à la droite. C'est bien la **droite** qui est mesurée,
pas le bout de trait dessiné : quand le pied tombe au-delà de l'extrémité, un
trait fin prolonge le segment jusque-là, comme sur un plan.

Un point l'emporte sur un trait sous le même curseur : c'est la plus petite
cible, donc la viser est un acte délibéré.

### Un cercle donne son diamètre

Un simple clic sur un cercle prend son **diamètre** : c'est la taille à laquelle
un trou est percé et celle à laquelle un rond est tourné. Le rayon se demande
exprès, en cliquant ensuite le **centre** — la seule chose que le centre puisse
ajouter à un cercle déjà pris.

### On clique ce qu'on mesure, puis où la cote se pose

Le premier clic prend la géométrie ; l'annotation **suit ensuite le curseur**,
valeur comprise, jusqu'au second clic qui la pose là. Une cote lâchée d'office
par-dessus la forme qu'elle mesure doit de toute façon en être écartée à la
main : autant qu'elle arrive où elle doit être.

La cote est posée avec la valeur que la géométrie mesure déjà, donc **poser une
cote ne déforme jamais rien**. C'est en tapant une autre valeur qu'on déplace le
dessin.

### Un trait en diagonale se lit de trois façons

Sa longueur, sa largeur ou sa hauteur. Laquelle est choisie dépend simplement
**d'où l'on pose la cote**, les deux bouts du trait délimitant une boîte :

| Où va le curseur | Ce qui est coté |
| --- | --- |
| Au-dessus ou en dessous de la boîte | La **largeur** — écart horizontal |
| À gauche ou à droite | La **hauteur** — écart vertical |
| Dans la boîte, ou au-delà d'un coin | La **longueur**, en diagonale |

L'aperçu montre laquelle avant le clic. Une cote de largeur est dessinée à
l'horizontale, ses deux lignes d'attache descendant chacune de son bout : elles
n'ont donc pas la même longueur, comme sur un plan.

Les trois peuvent cohabiter sur un même trait — largeur et hauteur ensemble le
fixent complètement, et le solveur les tient séparément : une largeur laisse le
trait libre de coulisser à la verticale.

Seul un trait **posé d'aplomb sur un axe** — 0, 90, 180, 270° — ne se voit pas
proposer le choix : sa largeur *est* sa longueur, et deux noms pour une seule
mesure est un nom de trop. Tout le reste, si peu incliné soit-il, l'a.

### Une mesure, une seule cote

Recliquer ce qui est déjà coté **rouvre la cote existante** au lieu d'en poser
une deuxième par-dessus. Cliquer une annotation directement fait la même chose,
avec l'outil Cote comme avec l'outil Sélection : c'est le geste évident pour
changer un nombre qu'on a déjà sous le curseur.

Les deux façons de désigner une même mesure — deux traits dans un sens ou dans
l'autre, deux points dans un sens ou dans l'autre — sont ramenées à une seule
avant d'être enregistrées. Sans ça, la même cote existerait deux fois, en double
exemplaire superposé.

Revalider une valeur qui est déjà celle en vigueur ne fait **rien** : appuyer
deux fois sur ✔ ne doit pas laisser deux étapes identiques dans l'historique.

### Où la cote se tient

Ce qui est enregistré est la position de l'annotation **en unités du dessin**,
pas en pixels. Une cote posée quelque part y reste : à l'ancienne, un décalage
en pixels ramenait toutes les annotations sur la forme dès qu'on dézoomait. Les
cotes automatiques du trait et du rectangle sont figées de la même façon au
moment où elles sont posées.

Seule la taille de ce qui doit rester lisible — le texte, les pointes de flèche
— continue de se compter en pixels.

### Le petit trait de rappel

Une valeur tirée sur le côté, au-delà des extrémités de la cote, n'a plus rien
qui dise à quoi elle appartient. Un **trait de rappel** prolonge alors la ligne
de cote jusque sous le nombre. Même chose pour un angle dont la valeur est
sortie de l'ouverture des deux traits.

**Un trait posé sur un axe** se sélectionne en cliquant deux fois dessus : le
premier clic prend le trait, le second — qui retombe forcément sur le même
trait — est lu comme « et maintenant l'axe sur lequel il repose ». Sans ça un
rectangle dessiné le long des axes ne pouvait jamais être contraint.

Quand deux choses se superposent et que la mauvaise l'emporte, la rangée
**Mesurer** force le type : *Intelligente*, *Point à point*, *Trait*, *Angle*,
*Rayon*.

## Les cercles

Le menu **Cercles**, dans la rangée Dessin, en propose cinq façons :

| Façon | Ce qu'on clique |
| --- | --- |
| **Centre et diamètre** | Le centre, puis un point du bord |
| **Deux points du bord** | Deux points opposés ; le centre est entre les deux |
| **Deux points puis le centre** | Deux points du bord, puis le centre |
| **Tangent à deux droites** | Deux traits, puis le centre |
| **Tangent à trois droites** | Trois traits : il ne reste rien à choisir |

Les deux façons qui finissent par le centre ne le prennent pas où l'on clique :
un centre à égale distance de deux points ne peut être que sur leur
**médiatrice**, et un centre à égale distance de deux droites que sur leur
**bissectrice**. Le clic y est ramené — l'utilisateur dit à peu près où, la
géométrie dit exactement où.

Un cercle posé contre des traits **y reste** : la tangence est enregistrée comme
contrainte, puisque c'est tout l'intérêt de les avoir désignés.

### Les endroits cliqués deviennent des points

Un cercle tracé par des points garde ces points : ils sont des points du dessin
comme les autres, tenus sur le bord. C'est ce qui donne des **poignées** à un
cercle, qui n'en avait que son centre — on peut les attraper, mesurer depuis
eux, s'y aimanter.

- **Tirer une poignée du bord agrandit le cercle** : le rayon est une inconnue
  du solveur, et le point tenu sur le bord la fait bouger.
- **Tirer le centre déplace tout le cercle**, ses poignées avec lui.

Une tangence emmène de même son **point de contact**. Il n'est pas libre : il est
sur la droite, et d'aplomb sous le centre. Sans cette seconde moitié il
glisserait le long de la droite — faire glisser un point le long d'un cercle
qu'il touche ne change rien du tout au premier ordre, et le solveur n'aurait
donc rien à corriger.

Ce point est ce qu'on attrape pour **faire glisser un cercle le long de la
droite qu'il touche**, sans casser la tangence.

Le **diamètre se tape** pendant le geste, comme la longueur d'un trait ou les
côtés d'un rectangle, et devient une cote sur le cercle. Une taille trop petite
pour atteindre les deux points est retenue à la plus petite qui les atteint :
taper 150 passe par 1 et par 15 en chemin, et un cercle qui disparaît à la
première frappe emporte avec lui le champ dans lequel on écrit.

## Les contraintes

Une cote dit **combien** ; une contrainte dit **comment**. Les deux enlèvent de
la liberté au dessin et comptent pareil quand il s'agit de savoir ce qui est
encore libre — elles sont tenues à part uniquement parce que l'une porte une
valeur que l'utilisateur tape et l'autre non.

Le menu **Contraintes**, dans la rangée Dessin, en propose neuf :

| Règle | Ce qu'on clique | Ce qu'elle tient |
| --- | --- | --- |
| **Perpendiculaire** | Deux traits | Ils restent d'équerre |
| **Parallèle** | Deux traits | Ils gardent la même direction |
| **Égalité** | Deux traits, ou deux cercles | Le second prend la taille du premier |
| **Coïncidence** | Un point et un trait, ou deux points | Le point reste sur la droite ; deux points n'en font qu'un ; un point posé sur un cercle reste sur son bord |
| **Colinéaire** | Deux traits, ou un trait et un axe du repère | Ils reposent sur la même droite |
| **Tangence** | Un cercle et un trait | Le trait effleure le cercle, et le point de contact est posé |
| **Milieu** | Un point et un trait | Le point reste à mi-longueur |
| **Fixe** | N'importe quoi : un point, un trait, un cercle | Il ne bouge plus de sa place (sa taille n'est pas fixée pour autant) |
| **Concentrique** | Deux cercles | Ils partagent un seul centre |

L'ordre des clics est libre : un point et un trait font la même coïncidence
dans un sens comme dans l'autre. Ce qui compte est **ce qui** a été cliqué, donc
la règle est construite à partir des types récoltés et non de leur ordre.

**Une exception : l'égalité.** Le premier trait cliqué est celui dont la
longueur convient ; le second vient la prendre. Une règle qui déplacerait les
deux laisserait aucun des deux à la taille demandée. Un coin que les deux
partagent ne bouge pas non plus, sinon étirer le second tordrait le premier.

Ce qui est **fixé** se dessine dans sa propre couleur, réglable comme les
autres : ce qui ne bouge plus doit se lire d'un coup d'œil, pas se deviner en
essayant de le déplacer. Fixer un trait tient ses deux bouts ; fixer un cercle
tient son centre et lui seul, son rayon reste libre.

Deux d'entre elles ne sont pas des règles mais des **fusions** : deux points
amenés à coïncider, et deux cercles ramenés sur un centre unique. Les tenir à
distance nulle par une équation laisserait deux points superposés pour
toujours — exactement ce que le dessin ne veut pas.

### Comment elles sont tenues

Chacune devient une équation de plus dans le même système que les cotes :
perpendicularité et parallélisme sont un produit scalaire ou vectoriel à
annuler, l'égalité une différence de longueurs, la coïncidence une distance à
une droite, le milieu deux équations — être au milieu est deux affirmations,
pas une.

**Fixe** sort du solveur : elle passe par les épingles, un point fixé n'ayant
simplement nulle part où aller, comme le point d'origine.

### La taille d'un cercle est une inconnue comme une autre

Le système compte deux inconnues par point **et une par cercle**. Un cercle tenu
contre un trait cède sur sa taille aussi volontiers que sur sa place, et une
règle qui ne pourrait que le déplacer devrait être rompue pour l'agrandir.

C'est ce qui fait marcher trois choses d'un coup :

- tirer un coin d'un triangle **agrandit son cercle inscrit** au lieu de le
  laisser en travers ;
- changer la taille d'un cercle tangent **le fait glisser** pour qu'il continue
  de toucher, au lieu d'attendre le geste suivant ;
- coter la distance du centre à la droite est **une vraie cote**, qui pilote le
  dessin. Tant que le rayon n'était pas une inconnue, l'analyse de rang la
  voyait identique à la tangence et la posait en lecture seule.

Rayon, diamètre et égalité de rayons sont donc devenus des équations ordinaires,
et les deux passes qui les rattrapaient à la main ont disparu.

Une règle disparaît d'elle-même quand ce dont elle parle est supprimé.

### Les marques

Chaque règle écrit sa marque (`|_`, `//`, `=`, `+`, `--`, `T`, `1/2`, `X`)
**sur chacune des choses qu'elle tient** : pointer l'une d'elles dit à quoi elle
est prise. Un parallélisme en pose donc une sur chaque trait, et non une seule
entre les deux.

L'angle droit fait exception : sa marque va **dans le coin**, seul endroit où
elle se lit comme un angle plutôt que comme une note à propos de deux traits.
Une tangence va de même **au point de contact** : les trois tangences d'un
cercle inscrit se poseraient sinon toutes au même endroit.

Plusieurs règles peuvent tenir le même endroit — un milieu et une
perpendicularité, par exemple. Les marques qui se poseraient l'une sur l'autre
sont donc **écartées côte à côte**.

Elles suivent le dessin **pendant** un déplacement, comme les valeurs des cotes :
lues sur le dessin enregistré, elles restaient en arrière et ne rattrapaient
leur place qu'au relâchement.

Les symboles du dessin technique — ⊥, ∥, ½ — ne sont pas dans les polices
livrées avec l'interface, et une marque qui sort en carré vide dit moins que
rien.

## L'aimantation

Le curseur est attiré, dans cet ordre :

| Ce qui attire | Pourquoi en premier |
| --- | --- |
| **Un point existant** | C'est ce qu'on vise le plus souvent, et rater d'un cheveu laisse une géométrie qui n'a l'air jointe que de loin |
| **Le milieu d'un trait** | On y vise exprès, et rien à l'écran ne dit qu'on est exactement à mi-longueur : un **petit triangle** l'annonce |
| **Le corps d'un trait** | Dessiner sur un trait déjà là est bien plus courant que dessiner à côté |
| **La grille** | Le filet de sécurité, avec la portée la plus courte |

Un trait déjà dessiné attire donc **plus fort que la grille** : sa portée est
réglable à part ([configuration.md](configuration.md)).

La portée d'un **clic** (ce qu'on attrape, ce qu'on cote) est de 18 pixels
physiques, soit 9 points sur un écran haute densité. À dix, il fallait viser un
point à quatre points près : bien plus fin que ce que quiconque vise.

### Deux sommets superposés n'en font qu'un

Lâcher un point sur un autre les **fusionne** : tout ce qui pointait vers celui
qui part pointe désormais vers celui qui reste, les cotes comprises. Deux bouts
posés l'un sur l'autre sont un seul coin, pas deux — sans quoi le contour a
l'air fermé sans l'être, et rien ne s'extrude.

Un trait dont les deux bouts deviennent le même point s'en va : il n'a plus ni
longueur ni direction. Et le point d'origine n'est jamais celui qui cède.

La décision est prise au lâcher et **enregistrée**, comme l'accrochage : la
distance qui compte dépend du zoom du moment, donc la refaire au rejeu pourrait
joindre une autre paire, ou aucune.

L'aimant de la grille mord sur des quarts de carreau : viser à peu près suffit
pour se poser sur l'origine. Aucun aimant ne mord au-delà de quelques pixels,
donc une position libre volontaire reste possible.

Attention : **aimanter n'est pas contraindre**. Un trait posé bien
horizontalement grâce à la grille reste libre de tourner tant qu'aucune cote
d'angle ne le tient.

Un rectangle est **une seule opération** dans l'historique, pas quatre traits :
c'est ce qu'on veut voir en relisant la construction.

### Un rectangle arrive coté

Le dessiner puis devoir dire quatre fois que ses coins sont droits, c'est de la
corvée : c'est ce qu'un rectangle *est*. Il reçoit donc tout seul **trois angles
droits** — le quatrième suit — et **une longueur sur deux côtés voisins**, ce
qui le fige exactement.

Une valeur qui n'apporterait rien est laissée de côté, comme pour le trait. Et
comme la forme est aussitôt entièrement contrainte, une cote posée dessus après
coup est en lecture seule : pour changer une taille, on retape la cote qui est
déjà là.

## Supprimer

Avec l'outil **Sélection**, cliquer un trait, un point, un cercle ou une cote le
met en surbrillance ; `Suppr` ou `Retour arrière` l'efface. Le plus petit gagne :
un point avant un trait avant un cercle avant une cote, puisque plus la cible est
petite, plus il est difficile de la viser exprès.

Supprimer **emporte ce qui s'appuyait dessus**. Un trait sans son point n'est pas
de la géométrie, et une cote qui mesure ce qui n'est plus là ne rend compte de
rien. Le point d'origine, lui, ne s'efface pas : c'est ce depuis quoi tout le
reste est mesuré.

### Pourquoi rien n'est vraiment retiré

Ce qui est supprimé est **marqué**, pas sorti de la liste. Un trait retiré du
milieu décalerait le rang de tous les suivants, et chaque cote déjà enregistrée
contre ces rangs se mettrait silencieusement à désigner un autre morceau du
dessin.

C'est aussi ce qui fait qu'une suppression se rejoue et s'annule comme n'importe
quelle autre étape ([historique.md](historique.md)).

## Rouvrir une esquisse

Le panneau Historique affiche un bouton ✏ **Modifier** sous chaque esquisse.
Il la rouvre pour y ajouter des traits, même après avoir cliqué « Terminer » ou
fermé la pièce. La vue se remet face au plan et cadre le dessin existant.

Aujourd'hui les seuls plans proposés sont les trois plans d'origine (XY, XZ,
YZ) : il n'existe pas encore de solide, donc pas de face à cliquer. Le code de
sélection ne dépend pas de ce fait — il teste un rayon contre un `WorkPlane`,
et une face de pièce en sera un.

## Tout se voit avant d'être posé

Chaque outil montre ce qu'un clic ferait, avant de le faire : le trait suit le
curseur, le rectangle et le cercle se dessinent en clair, le point a son
marqueur, et **la cote intelligente trace l'annotation qu'elle poserait** — au
bon endroit, avec ses flèches et ses lignes d'attache.

L'aperçu de la cote est calculé par **la même lecture du curseur** que la pose
elle-même. Deux lectures séparées finiraient par diverger, et un aperçu qui ment
est pire que pas d'aperçu du tout.

### Pendant qu'on déplace un point

Le point tenu sous le curseur **ne cède pas** : le dessin se pose *autour* de
lui. Le solveur l'épingle le temps du geste, exactement comme le point
d'origine. Sans cela les contraintes le ramènent en partie en arrière, la forme
sort de sous le curseur — c'est ce qui rendait un cercle tangent si pénible à
déplacer.

Le dessin est montré **tel qu'il se posera** si on lâche là : le solveur tourne
à chaque image, et les valeurs déjà données tirent le reste de la forme avec le
point. Auparavant seul le point suivait le curseur pendant que le reste ne
bougeait pas ; la forme paraissait déchirée, et on ne voyait rien de là où elle
allait atterrir.

Les cotes suivent : leurs lignes, leurs flèches **et leurs valeurs** sont lues
sur ce même dessin en train de se poser, sans quoi les nombres resteraient en
arrière pendant que les lignes auxquelles ils appartiennent s'en vont.

Rien n'est enregistré pour autant : l'historique ne reçoit qu'une seule
opération, au lâcher.

### Quand le geste est impossible

Tenir le point n'est pas toujours à la portée du dessin : un coin tiré là où
aucune tangence ne peut le suivre, par exemple. Les valeurs déjà données
l'emportent alors sur le curseur — tout revient en place et se pose de la façon
ordinaire, le point allant aussi loin que le dessin le lui permet.

Et si même cela écrase un trait jusqu'à n'en plus rien laisser, le geste est
**refusé** : le point ne va pas là. Un trait de longueur nulle n'est pas de la
géométrie, et ses équations ne peuvent même plus s'écrire — le système se
déclarerait satisfait alors que le dessin est tombé en morceaux.

### Déplacer toute une figure d'un bloc

Un glisser qui commence **sur quelque chose de déjà sélectionné** emporte toute
la sélection, comme un bureau déplace un groupe d'icônes. Chaque point nommé
avance du même pas : la forme est portée, jamais étirée, et le reste du dessin
se pose autour d'elle. Une seule ligne dans l'historique pour tout le bloc.

Un glisser qui commence ailleurs reste ce qu'il était : un point sous le
curseur, ou une boîte de sélection.

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
flèches et sa valeur ensemble.

L'annotation suit le curseur pendant tout le geste, alors que rien n'est
enregistré avant le lâcher : sans cela elle resterait immobile et sauterait à la
fin, et le déplacement aurait l'air de n'avoir rien fait.

Attraper une cote suppose de savoir où elle est dessinée : elle se tient à un
nombre fixe de **pixels** de ce qu'elle mesure, donc la chercher à une autre
échelle que celle de l'écran la place là où elle n'est pas — et elle devient
alors impossible à attraper.

Une cote de longueur ne s'éloigne que **perpendiculairement** à ce qu'elle
mesure : la part du déplacement le long du trait est écartée. La ligne de cote
reste donc parallèle à ce qu'elle mesure, avec ses deux lignes d'attache
perpendiculaires et de même longueur. Autrement c'est une paire de flèches de
travers, qui ne se lit plus comme une mesure. Seule la valeur peut encore
glisser le long de la ligne, ce qui permet à deux cotes de même direction de ne
plus se recouvrir.

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

Trois choses :

1. **Les valeurs de forme nécessaires** — longueurs et angles. « Nécessaires »
   et non « toutes » : dans un triangle dont deux côtés et l'angle entre eux
   sont donnés, le troisième côté **suit** et ne peut plus être imposé.
2. **Un rattachement à l'origine**, par accrochage ou par cote.
3. **De quoi dire dans quel sens la forme est posée** — voir plus bas.

**Exemple, un rectangle** dont un coin est sur l'origine : deux côtés et
**trois** angles droits (le quatrième suit) suffisent. Avec un seul angle droit
le quadrilatère peut encore se déformer en parallélogramme.

### Seule l'origine rattache

Un dessin qui ne tient à rien peut être n'importe où sur le plan, et le déclarer
fini serait dire qu'il est terminé alors qu'il n'est accroché à rien. Le seul
point qui rattache est donc **l'origine** — et, plus tard en 3D, un sommet ou
une face de la pièce.

**Fixe** ne rattache pas. Elle tient un élément immobile pendant que le dessin
se pose — c'est à cela qu'elle sert — mais elle ne dit pas *où* : la figure
qu'elle seule retient pourrait être ailleurs, et elle ne la fait donc pas
passer au vert.

### L'orientation, quand la forme la dit d'elle-même

Faire tourner un dessin entier autour de l'origine ne change aucune longueur et
aucun angle : **aucune cote ne peut voir cette rotation**. Il fallait donc
autrefois poser une cote d'angle à 0° sur un axe, uniquement pour dire « et ça
reste dans ce sens-là ».

Ce n'est plus nécessaire **quand la forme le dit d'elle-même** : un trait posé
le long d'un axe dit dans quel sens la figure est couchée. Carré au repère est
un sens comme un autre, et le plus courant. La figure garde alors le sens dans
lequel elle a été dessinée, sans qu'on ait rien à écrire.

Une forme **penchée** ne dit rien. Dès qu'aucun de ses traits n'est à 0, 90, 180
ou 270°, il faut le dire : une cote d'angle contre un axe, et elle passe au
vert. Sans cela, elle serait déclarée finie alors qu'on peut encore la faire
pivoter.

La règle vaut **par groupe de géométrie reliée** : deux formes dessinées à
l'écart peuvent tourner l'une par rapport à l'autre, donc chacune répond de son
sens de son côté. Une seule règle commune laisserait les deux libres de pivoter
l'une contre l'autre, et aucune ne serait jamais figée.

Poser un angle avec un axe sur une forme déjà carrée n'enlève rien : dès qu'une
cote dit dans quel sens une forme est posée, la règle implicite s'efface pour ce
groupe — sans quoi la même liberté serait retirée deux fois et un dessin encore
libre de glisser passerait pour figé.

### Comment c'est calculé

Par le **rang** du système d'équations, pas en comptant les cotes. Chaque cote
donne une équation ; on regarde combien d'entre elles disent quelque chose de
neuf. C'est la seule façon de voir que le troisième côté d'un triangle découle
des autres — un comptage ne le verrait jamais.

Pour savoir si un point précis est figé, on calcule les **mouvements encore
possibles** (le noyau du système) : si aucun d'eux ne déplace ce point, il ne
peut plus bouger.

Ce qui est compté en face, ce sont les **inconnues** : deux par point, une par
cercle. Seule l'origine n'en fait pas partie — c'est le seul point dont on
sache d'avance qu'il ne bougera pas.

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

### Le champ de valeur est sur la cote

Une fois posée, la cote porte son champ de saisie **juste à côté d'elle**, dans
le viewport. Il était auparavant dans la barre de titre, à un bras du dessin :
l'œil devait quitter la forme mesurée pour retrouver le nombre qui lui
appartient.

Le champ **prend le clavier** dès que la cote est posée, valeur sélectionnée :
on tape la nouvelle et c'est tout — y arriver avec Tab voudrait dire traverser
toute la barre d'outils d'abord, et sans la sélection « 40 » tapé sur « 60.88 »
se lirait 60,8840.

`Entrée` valide, et **cette frappe-là est consommée sur place** : le champ vient
de rendre le clavier, donc sans cela le même appui déclencherait aussi le
raccourci qui lui est lié — et terminerait l'esquisse.

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

#### On recommence tant que ça avance

Une passe ne suffit pas toujours. Ce qui est tenu rigide et ce qui a le droit
de céder est lu **sur la forme telle qu'elle est** ; une fois le dessin
déplacé, cette lecture est périmée, et recommencer en prend une neuve. Le
solveur repart donc du début tant que chaque tour enlève au moins un dixième de
l'erreur restante, et s'arrête dès qu'il n'avance plus.

C'est ce qui arrivait auparavant par accident : un dessin laissé à mi-chemin se
remettait droit dès que le changement suivant lui donnait un nouveau quota. On
voyait alors un cercle tangent rester visiblement hors de forme jusqu'à ce
qu'on touche autre chose. Il se pose maintenant au lâcher.

#### Ce que le changement n'a pas à déformer

Une valeur tapée, un point déplacé : quelques équations ne sont plus vraies, et
**elles seules disent où le dessin a le droit de céder**. Un angle s'ouvre entre
ses deux traits ; une longueur étire son propre trait. Tout le reste est soudé.

Le solveur découpe donc le dessin en **blocs** — des paquets de traits soudés
entre eux — et ne leur laisse qu'un déplacement d'ensemble : porté, tourné,
jamais plié. Un bloc qui partage un point avec un bloc déjà en place **tourne
autour de ce point** : c'est ce qui fait qu'une figure pivote autour de son coin
au lieu de s'étirer. Un bloc tenant un point qui ne peut pas bouger — l'origine
— ne bouge pas du tout.

Concrètement, sur une chaîne posée sur l'origine avec un angle coté au milieu :
changer cet angle laisse le côté ancré exactement où il est et fait pivoter
l'autre, formes conservées, au lieu de déformer tout le dessin d'un peu partout.

Cette mise en forme est appliquée **à chaque pas** de correction, et non en
retouche à la fin : une correction étalée sur les points puis redressée après
coup est une correction jetée aux trois quarts, et le dessin met alors des
dizaines de tours à converger.

Garder les formes n'est pas toujours possible — la valeur demandée peut vouloir
exactement ce qu'on tenait, comme la hauteur d'un rectangle. Le dessin est alors
résolu à l'ancienne, en pliant là où il faut.

#### Les angles se jugent en angles

L'écart d'une équation est comparé à la taille du dessin, ce qui a du sens pour
une longueur et pas pour un angle. Un dessin de 150 unités pouvait ainsi être
déclaré résolu avec un coin à un dixième de degré près. Un angle est maintenant
jugé sur lui-même, en radians.

#### Le dessin ne dérive pas

Un groupe que rien ne tient droit peut être tourné sans casser une seule cote,
donc **le solveur est libre de le tourner** — et il le faisait : chaque
correction est un pas de taille finie, et ce que chacun laisse derrière lui
s'additionne. Un rectangle dont on changeait la hauteur ressortait de plusieurs
degrés de travers, en se déclarant toujours entièrement contraint — et il
l'était : il avait simplement tourné.

Le sens de chaque groupe libre de tourner est donc **relevé avant de résoudre**
(la direction de son premier trait) et **rétabli après**. Faire tourner en bloc
un groupe que rien n'oriente laisse toutes ses cotes exactement comme elles
étaient — c'est la définition même de « libre de tourner » — donc le dessin est
redressé sans que rien de ce qu'il mesure ne change.

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
- La saisie en direct n'existe que pour le trait : le rectangle et le cercle se
  cotent après coup.
- Le seul angle posé tout seul est l'angle droit ; il n'y a ni parallélisme ni
  tangence automatiques.
- On ne peut pas sélectionner plusieurs éléments à la fois pour les supprimer
  d'un coup.
- Une forme dessinée dans une autre est teintée comme une aire à part, pas
  traitée comme un trou : la zone commune reste remplie deux fois.
- Un contour qui se recoupe lui-même n'est pas teinté.
- Pas de cotes de diamètre, ni de cotes horizontales/verticales séparées (une
  cote point à point mesure toujours la distance directe).
- Les contraintes géométriques (parallèle, tangent) n'existent pas : seules les
  cotes contraignent — l'angle droit posé tout seul est une cote d'angle comme
  une autre.
- Le solveur ne dit pas *quelles* cotes se contredisent quand il n'y arrive
  pas.
- L'esquisse ne produit encore aucun volume : l'extrusion est l'étape suivante.
