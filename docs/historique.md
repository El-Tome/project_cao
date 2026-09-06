# Historique, annulation et format de fichier

Voir aussi : [esquisse](esquisse.md) · [architecture](ARCHITECTURE.md)

## Le principe : la pièce est sa liste d'opérations

La géométrie d'une pièce n'est **pas** stockée. Ce qui est stocké, c'est la
suite des opérations qui l'ont produite : « esquisse sur le plan XY », « trait
de A à B », « cote de 100 mm ». La géométrie est reconstruite en rejouant cette
liste.

Ce choix rend trois fonctions identiques, au lieu de trois mécanismes séparés
qui finiraient par se contredire :

| Ce que fait l'utilisateur | Ce qui se passe |
| --- | --- |
| Annuler | Le curseur recule d'un cran |
| Rétablir | Le curseur avance d'un cran |
| Cliquer une étape de l'historique | Le curseur va à cette position |

Dans les trois cas, la pièce est ensuite reconstruite depuis le début. Il n'y a
donc aucun moyen que l'affichage et l'historique divergent.

Un test vérifie explicitement qu'appliquer une opération en direct donne le même
résultat que la rejouer : sans quoi un dessin pourrait changer d'aspect au
simple fait de fermer et rouvrir la pièce.

Une cote emporte **où son annotation se pose** dans la même opération. Poser une
cote est un seul geste de l'utilisateur ; lire « Cote 60 mm » puis « Cote
déplacée » à chaque clic n'aurait rien dit de plus. Un déplacement ultérieur, à
la souris, reste une opération à part.

## Le curseur et la branche abandonnée

L'historique retient toutes les opérations et une position : ce qui est avant
est appliqué, ce qui est après attend d'être rétabli. Cette queue est
**enregistrée dans le fichier**, donc le « rétablir » survit à la fermeture du
logiciel.

Dessiner quelque chose de neuf après une annulation efface cette queue : la
pièce a pris une autre direction, et garder l'ancienne branche laisserait un
« rétablir » qui ne découle plus de ce qui est à l'écran.

## Les points, et pourquoi ils ne sont pas recalculés

Une opération « trait » ne garde pas deux positions mais deux **références** :
soit un point existant, soit un point à créer à telle position.

C'est important : l'accrochage dépend du zoom au moment du clic (10 pixels à
l'écran valent plus ou moins de millimètres selon la distance). Rejouer
l'accrochage plus tard pourrait donc souder des points différents et
reconstruire un autre dessin. La décision est prise une fois, au clic, et
conservée.

## L'arbre

Le panneau de gauche liste les opérations, groupées sous celle qui a ouvert la
fonction en cours — une ligne par esquisse, dépliable. Les étapes annulées
apparaissent en grisé sous la position courante. Cliquer une ligne remet la
pièce dans l'état où elle était juste après cette étape.

## L'extrusion dans l'historique

Une extrusion est une opération comme une autre : elle ouvre sa propre ligne
dans l'arbre, et revenir avant elle rend la pièce à l'état de dessin. Le volume
n'est jamais stocké — il est reconstruit en rejouant les opérations, exactement
comme la géométrie de l'esquisse.

L'aire extrudée est retenue par **la position cliquée** et non par son rang,
pour la même raison que les points d'un trait : un rang bougerait dès qu'une
autre forme est dessinée. Voir [extrusion.md](extrusion.md).

## Supprimer ne retire rien de la liste

Une suppression est une opération comme les autres, et elle **marque** ce qui
disparaît au lieu de le retirer. Sortir un trait du milieu de la liste
décalerait le rang de tous les suivants, et chaque cote enregistrée contre ces
rangs désignerait alors un autre morceau du dessin — silencieusement.

C'est ce qui permet d'annuler une suppression comme n'importe quelle autre
étape, et de la rejouer à l'identique. Voir [esquisse.md](esquisse.md).

## Le format de fichier

Un `.caopart` est une **archive zip**, et non plus un seul objet JSON :

| Fichier | Contenu |
| --- | --- |
| `part.json` | Identité de la pièce : identifiant, nom, dates, version de schéma |
| `history.json` | La liste des opérations et la position du curseur |

Séparer les fichiers permet de faire évoluer chaque partie indépendamment, et
laisse la place à ce qui viendra s'ajouter (miniature de la pièce, matériaux,
maillages exportés) sans réécrire le reste à chaque enregistrement.

### Les versions antérieures ne sont pas converties

Un fichier écrit par une version antérieure est **refusé**, avec la raison, au
lieu d'être converti. Tant que l'outil bouge autant, une conversion aurait plus
de chances de reconstruire une pièce de travers que de sauver quoi que ce soit
d'utile.

## Ce qui manque

- L'historique n'est pas modifiable : on ne peut ni supprimer une étape au
  milieu, ni réordonner, ni éditer les paramètres d'une opération passée.
- Pas de branches : une seule ligne d'historique, avec une seule queue de
  rétablissement.
- Une pièce très longue est reconstruite entièrement à chaque déplacement du
  curseur. C'est instantané aux tailles actuelles ; il faudra des états
  intermédiaires mis en cache le jour où ça ne le sera plus.
