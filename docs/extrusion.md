# Extrusion : de l'aire dessinée au volume

Voir aussi : [esquisse](esquisse.md) · [historique](historique.md) ·
[rendu](rendu.md) · [architecture](ARCHITECTURE.md)

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
du prisme à la fin.

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

## Ce qui manque encore

- Pas de « jusqu'à la face suivante » ni de « traversant tout » : seule une
  hauteur donnée existe.
- Pas de dépouille, pas de révolution, pas de balayage.
- Une extrusion n'est pas modifiable après coup : il faut revenir en arrière
  dans l'historique et la refaire.
- Les faces du volume ne sont pas sélectionnables : on ne peut pas encore
  esquisser sur une face de la pièce, seulement sur les trois plans d'origine.
- Le maillage n'est pas exporté (pas de STL/STEP).
- Les booléens travaillent en `f32` : deux faces exactement coplanaires peuvent
  laisser des éclats de surface. Rien de visible aux tailles courantes, mais
  c'est la première chose à revoir si un volume devient bizarre.
