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
| **Ligne** | Clics successifs, chaque trait prolonge le précédent. |
| **Rectangle** | Deux clics : deux coins opposés. |
| **Cercle** | Deux clics : le centre puis un point du bord. |
| **Point** | Un clic pose un point isolé. |
| **Cote** | Deux clics : ce qu'on mesure, puis où l'annotation se pose. |

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
| Un trait | Sa longueur |
| Deux points | La distance entre eux, reliés ou non |
| Deux traits qui se touchent | L'angle entre eux |
| Un trait puis un point | La distance du point à la droite, prise d'équerre |
| Un point puis un trait | La même, dans l'autre ordre |
| Un trait puis un axe de l'esquisse | L'angle avec cette direction |
| Un axe puis un trait | Le même, dans l'autre ordre |
| Un cercle | Son rayon |

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

### On clique ce qu'on mesure, puis où la cote se pose

Le premier clic prend la géométrie ; l'annotation **suit ensuite le curseur**,
valeur comprise, jusqu'au second clic qui la pose là. Une cote lâchée d'office
par-dessus la forme qu'elle mesure doit de toute façon en être écartée à la
main : autant qu'elle arrive où elle doit être.

La cote est posée avec la valeur que la géométrie mesure déjà, donc **poser une
cote ne déforme jamais rien**. C'est en tapant une autre valeur qu'on déplace le
dessin.

### Une mesure, une seule cote

Recliquer ce qui est déjà coté **rouvre la cote existante** au lieu d'en poser
une deuxième par-dessus. Cliquer une annotation directement fait la même chose,
avec l'outil Cote comme avec l'outil Sélection : c'est le geste évident pour
changer un nombre qu'on a déjà sous le curseur.

Les deux façons de désigner une même mesure — deux traits dans un sens ou dans
l'autre, deux points dans un sens ou dans l'autre — sont ramenées à une seule
avant d'être enregistrées. Sans ça, la même cote existerait deux fois, en double
exemplaire superposé.

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
