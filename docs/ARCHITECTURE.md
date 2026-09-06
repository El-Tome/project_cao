# Architecture

## Vision

Un outil de CAO 3D (type SolidWorks / Fusion 360), 100 % Rust, open source,
conçu pour être **modulaire** : chaque grande fonctionnalité (menu de
démarrage, croquis/extrusion, assemblage, futurs modes...) est un module
indépendant, et la plupart des comportements doivent rester configurables
plutôt que codés en dur.

Plateformes visées, dans l'ordre :

1. Desktop : Windows, Linux, macOS.
2. Tablette / iPad, avec support du stylet pour esquisser rapidement à la
   main puis passer en 3D.
3. Téléphone, en bonus, sans garantie d'usage réel.

## Pourquoi egui/eframe

`egui` est du Rust pur, basé sur `wgpu` — le même socle graphique qui portera
le futur viewport 3D, pas de pont vers un autre langage. Il compile
nativement sur les trois OS desktop et vers WASM, ce qui ouvre la voie à un
portage tablette/web sans réécrire l'interface. C'est un choix pragmatique
pour la V1 ; il pourra être remis en question si les besoins tactile/stylet
(mode Apple Pencil notamment) s'avèrent trop limités par ce framework.

## Découpage en crates

Un crate = une responsabilité, sans dépendance dans le mauvais sens :

- `cao_core` : types de domaine (document de pièce, échelle, liste des
  récents, chemins de stockage), les réglages — thème, raccourcis, barre
  d'outils, profils — et la persistance de tout cela. **Aucune dépendance UI.**
  Doit rester réutilisable tel quel par n'importe quel futur front-end (desktop,
  web, tablette). Voir [configuration.md](configuration.md).
- `cao_sketch` : modèle d'esquisse (plan de travail, points, traits, cotes) et
  la règle qui applique une longueur. Ni rendu ni interface.
  Voir [esquisse.md](esquisse.md).
- `cao_solid` : les volumes — maillage de polygones, extrusion d'une aire en
  prisme, opérations booléennes (ajout et enlèvement de matière). Ni rendu ni
  interface. Voir [extrusion.md](extrusion.md).
- `cao_render` : rendu GPU du viewport (`wgpu`), sans dépendance interface.
  Voir [rendu.md](rendu.md).
- `cao_app` : shell applicatif desktop (`eframe`). Contient l'état de
  l'application et le routage entre écrans/modes.

Au fur et à mesure que les modes (croquis, extrusion, assemblage...)
grossiront, ils doivent devenir leurs propres crates (`cao_sketch`,
`cao_assembly`, ...) plutôt que de s'accumuler dans `cao_app`, qui ne doit
rester qu'un shell fin : fenêtre, routage entre modes, rien de plus.

## Système de modes

L'application est un menu de démarrage qui bascule vers différents modes :

- **Croquis → Extrusion** : cycle esquisse 2D puis extrusion, répétable en
  boucle pour construire une pièce. Les deux existent :
  [esquisse.md](esquisse.md), [extrusion.md](extrusion.md).
- **Assemblage** : assembler plusieurs pièces entre elles (pas encore
  implémenté).
- D'autres modes viendront s'ajouter au menu au fil du temps.

Aujourd'hui, `crates/app/src/screens/mod.rs` définit un enum `Screen` avec
deux variantes : le menu de démarrage et la pièce ouverte, qui affiche le
viewport 3D (axes, grille, cube d'orientation — voir [viewport.md](viewport.md)).
Chaque nouveau mode doit ajouter une variante à cet enum et son propre module
dans `screens/`, jamais une branche ajoutée à un module existant.

## Documentation par sujet

- [esquisse.md](esquisse.md) — dessiner, coter, et la règle d'échelle
- [historique.md](historique.md) — opérations, annulation, format de fichier
- [interface.md](interface.md) — barre d'outils détachable, panneaux
- [viewport.md](viewport.md) — les deux modes du canvas, la grille, le cube
- [rendu.md](rendu.md) — le crate `cao_render`, pipelines wgpu, lignes épaisses
- [navigation.md](navigation.md) — gestes souris, comportement de la caméra
- [configuration.md](configuration.md) — ce qui est réglable, et ce qui ne l'est pas encore
- [build.md](build.md) — compiler, exécutable Windows

## Format de fichier

Une pièce est une **archive zip** (`.caopart`) contenant ses métadonnées et son
historique d'opérations. La géométrie n'est pas enregistrée : elle est
reconstruite en rejouant l'historique, ce qui fait de l'annulation, du
rétablissement et du retour à une étape la même opération. Les fichiers écrits
au format précédent (un JSON unique) sont convertis à l'ouverture. Voir
[historique.md](historique.md).

## Pistes non prioritaires (à débattre plus tard)

- **Travail collaboratif** : verrouillage d'une pièce par un seul
  utilisateur à la fois, vs édition simultanée à plusieurs. Choix à faire
  quand le besoin deviendra concret ; ne pas anticiper l'architecture
  réseau/sync avant ça.
- **Licence pro** : une offre commerciale en plus de la double licence
  MIT/Apache-2.0, modalités non définies.

## Les nombres

Le noyau — esquisse, solveur, solide, booléens — calcule en **`f64`**. La
caméra, le rendu et l'interface restent en `f32`, qui est ce que le GPU et egui
prennent, et la conversion se fait au dernier moment, à chaque passage de
frontière.

Le `f32` garde environ sept chiffres : une pièce d'un mètre décrite en
millimètres n'a déjà plus qu'un pas de 6·10⁻⁵ mm, et l'erreur s'accumule dans
les booléens — c'est ce qui avait fait boucler la partition de l'espace
([extrusion.md](extrusion.md)). Le `f64` en garde seize.

Ce que cela ne donne pas : l'**exactitude**. 0,1 mm reste un nombre que le
binaire ne sait pas écrire, et deux chemins de calcul différents peuvent
toujours donner deux résultats à un cheveu près. Y répondre demanderait de
ranger les valeurs saisies en entiers (le picomètre comme unité, le micro-degré
pour les angles) au moment où elles entrent dans l'historique, en continuant de
calculer en `f64`. Ce n'est pas fait.
