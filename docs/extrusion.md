# Extrusion : de l'aire dessinée au volume

Voir aussi : [esquisse](sketch.md) · [historique](historique.md) ·
[rendu](render.md) · [architecture](ARCHITECTURE.md)

## Le déroulé

1. On dessine une esquisse et on clique **Terminer**. L'esquisse n'a **pas**
   besoin d'être entièrement contrainte : on extrude ce qui est là.
2. La barre bascule d'elle-même sur la catégorie **Extrusion**, proposant les
   deux outils. C'est là que l'envie d'extruder arrive, plutôt que d'avoir à
   retrouver l'outil ensuite.
3. On choisit **Ajout de matière** ou **Enlèvement de matière**.
4. On clique **une à n aires fermées** du dessin. Un second clic sur une aire
   déjà prise la retire.
5. On donne une hauteur en millimètres, éventuellement en sens inverse, et on
   applique.
6. La vue bascule en oblique : vu de face depuis son propre plan, un prisme
   ressemble exactement au dessin dont il sort.

Les deux outils sont le même travail : ils ne diffèrent que par ce qu'ils font
du volume à la fin.

## Droite ou révolution

La même paire d'outils fabrique le volume de deux façons :

| Forme | Ce qu'on donne |
| --- | --- |
| **Droite** | Une hauteur, en millimètres. La matière part perpendiculairement au plan. |
| **Révolution** | Un angle, en degrés, et un axe. L'aire tourne autour de cet axe. |

L'axe est soit l'un des deux axes de l'esquisse, soit **un trait qu'on a tracé
soi-même** : en mode révolution, cliquer un trait le prend comme axe. Un trait
est une cible bien plus petite qu'une aire, donc il est proposé en premier.

Un tour complet se referme sur lui-même et n'a pas d'extrémités ; un tour
partiel est fermé aux deux bouts par le profil lui-même.

Le profil doit tenir **entièrement d'un seul côté de l'axe**. À cheval, il
passerait à travers lui-même en tournant, et aucune précaution ensuite ne
rattrape une forme obtenue comme ça : rien n'est produit, et l'application le
dit.

## Ce qu'est une aire, et pourquoi le tube marche

Une aire, c'est un contour fermé **moins ce qui est dessiné directement
dedans**. Deux cercles l'un dans l'autre donnent donc :

| Ce qu'on clique | L'aire obtenue |
| --- | --- |
| L'anneau (la teinte claire) | Le tube : le milieu reste vide |
| Le milieu (la teinte foncée) | Le disque intérieur seul |

C'est exactement la règle « on sélectionne l'aire de la même couleur » : ce qui
est teinté d'un ton est une aire, ce qui est teinté plus fort en est une autre.
Ce qui est dessiné **dans un trou** est de la matière à nouveau, et forme sa
propre aire.

L'aire choisie est remplie à l'écran de la couleur de la matière qu'elle est sur
le point de devenir — vert pour un ajout, rouge pour un enlèvement — trous
compris, donc ce qui est montré plein est exactement ce qui deviendra plein.

### Découper une aire trouée

Un anneau ne peut pas être découpé en triangles tel quel : aucun tour ne laisse
à la fois le trou vide et le contour fermé. On creuse donc un **couloir** du
trou jusqu'au contour, et on le parcourt en descendant d'un côté et en
remontant de l'autre. Les deux bords du couloir sont confondus : il n'a aucune
aire, et la face est inchangée.

## L'aire est nommée par un point, pas par son rang

L'opération enregistrée ne retient pas « la deuxième aire » mais **la position
cliquée**. Un rang bougerait dès qu'on dessine autre chose dans l'esquisse, et
l'extrusion se mettrait silencieusement à s'appliquer ailleurs. Au rejeu, l'aire
est retrouvée comme celle qui contient ce point — la plus intérieure s'il y en a
plusieurs.

C'est le même principe que pour les points d'un trait
([historique.md](historique.md)) : la décision est prise au clic et conservée.

## Ajouter et enlever de la matière

La pièce est **un seul volume**, pas un tas de morceaux : une poche creusée dans
un bloc doit vraiment être un trou dans ce bloc.

Les deux opérations passent par une **partition binaire de l'espace** (BSP) :
chaque volume devient un arbre de plans pris sur ses propres faces, et les faces
de l'autre volume y sont poussées. Chacune ressort étiquetée dedans ou dehors,
et coupée en deux là où le plan la traverse. L'union et la différence sont
ensuite affaire de garder les bonnes moitiés et de retourner un volume.

Cette méthode marche sur n'importe quelle forme, convexe ou non — une poche dans
un bloc est justement le cas qui casse les méthodes plus simples.

Quand plusieurs aires sont extrudées ensemble, elles sont d'abord réunies en un
seul outil, puis appliquées d'un coup : deux aires extrudées ensemble doivent se
comporter comme une seule forme.

## Le sens

La matière part le long de la **normale du plan** de l'esquisse. La case *Sens
inverse* la pousse de l'autre côté ; c'est souvent ce qu'il faut pour creuser,
puisque la matière n'est pas toujours du côté où pointe le plan.

Un enlèvement qui ne rencontre rien le dit, au lieu de laisser croire à un outil
en panne.

## L'échelle

La hauteur est donnée en millimètres, comme les cotes. Elle est convertie en
unités du monde avec l'échelle du document, donc une extrusion de 25 mm reste
25 mm même si une cote redéfinit l'échelle ensuite… à ceci près que le volume
est reconstruit en rejouant l'historique, donc c'est l'échelle **au moment du
rejeu** qui s'applique.

## Esquisser sur une face de la pièce

Une fois qu'il y a de la matière, **ses faces planes sont des plans
d'esquisse**. Le choix d'un plan les propose en premier là où elles sont, et
les trois plans d'origine restent disponibles partout ailleurs — ils s'effacent
visuellement pour ne plus masquer la pièce.

La face du dessus est prioritaire sur les trois plans plutôt que « le plus
proche de la caméra » : les plans d'origine sont des feuilles infinies qui
traversent la pièce, et le plus proche serait presque toujours l'un d'eux.

Toutes les faces qui partagent le même plan s'allument ensemble : une surface
courbe et une surface coupée sont l'une comme l'autre stockées en plusieurs
morceaux plats, et n'en allumer qu'un se lirait comme choisir un fragment.

L'origine de l'esquisse tombe là où l'origine du monde se projette sur la face,
et la vue se cale sur l'endroit cliqué. Une esquisse posée sur une face est
enregistrée avec **son plan complet**, pas avec une référence à la face : si la
pièce change ensuite, le dessin reste où il a été fait plutôt que de suivre une
face qui n'existe peut-être plus.

## Pourquoi l'application se fermait

Un cas rencontré à l'usage : un cylindre de révolution, puis une poche creusée
dedans depuis sa propre face — l'application se fermait d'un coup, sans message,
et pas toujours.

La cause était dans la partition de l'espace. Elle trie les faces par le plan
sur lequel elles se posent, et le premier plan vient d'une face prise au
hasard : cette face-là est sur ce plan par définition. Sauf qu'en `f32` — ce
qu'utilisait alors le noyau —, à trente unités de l'origine, le produit scalaire
porte déjà quelques millionièmes d'erreur, plus que la tolérance qui servait. Un triangle ressortait donc
**des deux côtés de son propre plan**, était coupé en deux, et chaque moitié
recommençait : un découpage sans fin, qui remplissait la pile et arrêtait le
programme.

Trois choses ont changé :

- la tolérance est **relative** à la distance à l'origine, et non plus fixe ;
- la face qui a donné le plan est mise de côté au lieu d'être triée, ce qui
  garantit qu'à chaque tour il reste strictement moins de faces à placer ;
- le parcours de l'arbre ne passe plus par la pile d'appels du tout — les nœuds
  vivent dans un tableau et se désignent par leur position. Une pièce faite de
  centaines de facettes donne un arbre en chaîne, et une descente récursive y
  finit par déborder même sans erreur de calcul.

Le cas exact, avec ses vraies mesures, est un test.

## Ce qui manque encore

- Pas de « jusqu'à la face suivante » ni de « traversant tout » : seule une
  hauteur, ou un angle, donné.
- Pas de dépouille et pas de balayage le long d'une courbe.
- Une esquisse posée sur une face ne suit pas cette face si la pièce change :
  elle reste sur le plan où elle a été faite.
- Une extrusion n'est pas modifiable après coup : il faut revenir en arrière
  dans l'historique et la refaire.
- Le maillage n'est pas exporté (pas de STL/STEP).
- Deux faces exactement coplanaires peuvent encore laisser des éclats de
  surface. Le passage du noyau en `f64` a ramené la tolérance de coplanarité
  d'un millionième à un milliardième, donc il en reste mille fois moins, mais
  le cas n'est pas traité pour lui-même.
