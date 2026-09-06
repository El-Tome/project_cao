# Réglages, profils et personnalisation

Voir aussi : [navigation](navigation.md) · [viewport](viewport.md) ·
[interface](interface.md) · [architecture](ARCHITECTURE.md)

L'objectif du projet est que le maximum de choses soit réglable plutôt que codé
en dur. Tout vit dans `cao_core`, sans dépendance à l'interface, et tout est
sérialisable : un réglage qui ne peut pas être écrit dans un fichier ne peut ni
être conservé ni être partagé.

## L'écran de préférences

Bouton ⚙ de la barre d'outils, ou `Cmd/Ctrl + ,`. Six sections :

| Section | Ce qu'on y règle |
| --- | --- |
| **Profils** | Changer, dupliquer, supprimer, importer, exporter, tout remettre par défaut |
| **Viewport** | Cube d'orientation, grille et aimants, règle, limites de la caméra |
| **Navigation** | Habitudes souris, sensibilités, gestes du trackpad |
| **Apparence** | Le fond, et toutes les couleurs et épaisseurs |
| **Raccourcis** | La touche de chaque commande |
| **Barre d'outils** | L'emplacement, le logo, et l'arrangement des boutons |

Tout est appliqué immédiatement et écrit sur disque dans la foulée : il n'y a
pas de bouton « valider », donc pas de réglage perdu parce qu'on a fermé la
fenêtre.

## Les profils

Un profil est un **jeu de réglages complet**, avec un nom. C'est l'unité de
tout le reste : ce qu'on enregistre, ce qu'on remet à zéro, ce qu'on donne à
quelqu'un.

- Le profil **Par défaut** existe toujours et ne peut pas être supprimé : il
  reste de quoi revenir quand une expérience tourne mal.
- Deux profils ne peuvent pas porter le même nom — ils seraient impossibles à
  distinguer dans la liste. Un nom déjà pris devient « … 2 ».
- **Tout remettre par défaut** ne touche que le profil actif.

### Partager

Un profil s'exporte dans un fichier `.caoprofile` (du JSON), et s'importe
depuis un tel fichier — y compris celui de quelqu'un d'autre. Un profil importé
arrive comme un profil de plus, il ne remplace rien.

Chaque champ a une valeur par défaut, donc **un profil écrit par une version qui
connaissait moins de réglages se charge quand même** : ceux qu'il ignore gardent
leur valeur d'origine. Seul un numéro de version différent est refusé.

Les raccourcis utilisent un modificateur « commande » qui vaut Ctrl sur Windows
et Linux, Cmd sur macOS. Un profil partagé entre machines se lit donc
correctement des deux côtés.

Tout est écrit dans `settings.json`, dans le dossier de configuration de l'OS.
Un fichier illisible n'empêche pas l'application de démarrer : elle repart des
valeurs par défaut, faute de quoi l'utilisateur n'aurait aucun moyen d'entrer
pour le réparer.

## L'apparence

### Le fond

Uni, dégradé linéaire à n'importe quel angle, ou dégradé radial dont on place le
centre et le rayon. Dans les deux cas la couleur est décrite par une **liste
d'arrêts** — on en ajoute et on en retire à volonté. Les dégradés à deux
couleurs sont ce qu'on demande en premier et ceux à trois juste après ; une
liste ne coûte pas plus cher à dessiner.

Le fond est émis en coordonnées écran, sans caméra : il ne bouge pas quand la
vue tourne. Changer de type de dégradé conserve les couleurs déjà choisies,
sinon essayer les trois serait fastidieux.

### Les couleurs

Tout ce que le viewport dessine : les trois axes et leur épaisseur, les deux
niveaux de grille, les trois états d'une esquisse (libre, contrainte, autre
esquisse), les cotes, la teinte des aires, la matière, le survol, et les deux
couleurs d'extrusion. Plus aucune couleur du viewport n'est codée en dur : une
couleur qui vit dans une constante quelque part est une couleur que personne ne
peut changer.

Les couleurs sont stockées en sRGB — l'espace des sélecteurs de couleur — et
converties une fois, là où la géométrie est construite.

## Les raccourcis

Cliquer un raccourci puis appuyer sur la touche voulue ; les modificateurs tenus
au moment de la frappe sont pris avec. Une touche déjà utilisée est **retirée à
l'autre commande** : deux commandes sur la même touche en rendraient une
inatteignable sans dire pourquoi.

Un raccourci ne part jamais pendant qu'un champ de texte a le clavier : taper
« 50 » dans une cote ne doit pas aussi déclencher ce à quoi 5 et 0 sont liés. Et
un raccourci dont le bouton est grisé ne fait rien non plus.

## La barre d'outils

L'emplacement est libre : en haut, en bas, à gauche, à droite, ou flottante.

Les boutons sont un **arbre**. Un groupe contient des commandes, des séparateurs
et **d'autres groupes, aussi profond qu'on veut**. Les groupes du premier niveau
sont les onglets ; un niveau plus bas, un groupe est étalé sur place avec son nom
à côté ; plus profond encore, il devient un menu qui s'ouvre au clic — étaler un
troisième niveau pousserait tout le reste hors de la barre, et il n'y a pas de
fond à la profondeur possible.

L'éditeur permet de sélectionner une entrée, la monter, la descendre, la faire
entrer dans le groupe juste au-dessus, l'en sortir, la retirer, créer des
groupes, les renommer, et ajouter n'importe quelle commande depuis la palette.

Une barre arrangée à la main **reçoit les outils ajoutés plus tard**, chacun
dans le groupe où la barre standard le place. Sans cela, une barre réorganisée
une fois n'entendrait plus jamais parler d'un nouvel outil : les boutons neufs
existeraient pour un profil neuf et pour personne d'autre. Ce que l'utilisateur
a arrangé n'est pas touché ; seul ce qui manque est ajouté.

Un emplacement pour le **logo** existe déjà, avec son texte, en attendant qu'il
y ait une image à y mettre.

## Une seule liste de commandes

Un bouton et un raccourci sont deux façons de demander la même chose, donc il
n'y a qu'une liste : `cao_core::Command`. Les deux chemins aboutissent à la même
fonction. Deux chemins séparés finiraient par diverger, et un raccourci qui fait
*presque* ce que fait son bouton est pire que pas de raccourci.

C'est aussi ce qui fait qu'ajouter une commande est un seul geste : elle
apparaît d'elle-même dans la palette de la barre d'outils et dans la liste des
raccourcis.

## Ce qui n'est pas encore réglable

- La durée de l'animation de changement de vue (0,35 s) et le champ de vision de
  la caméra (45°).
- La largeur des bords du cube qui sélectionnent une arête ou un coin (22 %).
- Le logo est un texte : il n'y a pas encore de fichier image à charger.
- Les couleurs de l'interface elle-même (panneaux, boutons) sont celles d'egui ;
  seul le viewport suit le thème.
- Les gestes souris passent par des préréglages (Fusion, SolidWorks, Blender) et
  ne se règlent pas bouton par bouton.
- Aucun réglage n'est propre à une pièce : tout est global.
