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

## Le format de fichier

Un `.caopart` est une **archive zip**, et non plus un seul objet JSON :

| Fichier | Contenu |
| --- | --- |
| `part.json` | Identité de la pièce : identifiant, nom, dates, version de schéma |
| `history.json` | La liste des opérations et la position du curseur |

Séparer les fichiers permet de faire évoluer chaque partie indépendamment, et
laisse la place à ce qui viendra s'ajouter (miniature de la pièce, matériaux,
maillages exportés) sans réécrire le reste à chaque enregistrement.

### Les anciens fichiers

Les pièces écrites au format précédent (un JSON unique) s'ouvrent toujours :
le chargeur reconnaît un fichier qui ne commence pas par la signature d'une
archive, et **convertit son dessin en opérations** — une par esquisse, une par
trait, une par cote. L'ancienne pièce arrive donc avec un historique comme les
autres, y compris ses coins partagés. Elle est réenregistrée au nouveau format
à la première modification.

## Ce qui manque

- L'historique n'est pas modifiable : on ne peut ni supprimer une étape au
  milieu, ni réordonner, ni éditer les paramètres d'une opération passée.
- Pas de branches : une seule ligne d'historique, avec une seule queue de
  rétablissement.
- Une pièce très longue est reconstruite entièrement à chaque déplacement du
  curseur. C'est instantané aux tailles actuelles ; il faudra des états
  intermédiaires mis en cache le jour où ça ne le sera plus.
