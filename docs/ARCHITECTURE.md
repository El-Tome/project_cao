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

- `cao_core` : types de domaine (document de pièce, liste des récents,
  chemins de stockage) et persistance. **Aucune dépendance UI.** Doit rester
  réutilisable tel quel par n'importe quel futur front-end (desktop, web,
  tablette).
- `cao_app` : shell applicatif desktop (`eframe`). Contient l'état de
  l'application et le routage entre écrans/modes.

Au fur et à mesure que les modes (croquis, extrusion, assemblage...)
grossiront, ils doivent devenir leurs propres crates (`cao_sketch`,
`cao_assembly`, ...) plutôt que de s'accumuler dans `cao_app`, qui ne doit
rester qu'un shell fin : fenêtre, routage entre modes, rien de plus.

## Système de modes

L'application est un menu de démarrage qui bascule vers différents modes :

- **Croquis → Extrusion** : cycle esquisse 2D puis extrusion, répétable en
  boucle pour construire une pièce (pas encore implémenté).
- **Assemblage** : assembler plusieurs pièces entre elles (pas encore
  implémenté).
- D'autres modes viendront s'ajouter au menu au fil du temps.

Aujourd'hui, `crates/app/src/screens/mod.rs` définit un enum `Screen` avec
une seule alternance réelle : le menu de démarrage et un écran vide affiché
après création/ouverture d'une pièce. Chaque nouveau mode doit ajouter une
variante à cet enum et son propre module dans `screens/`, jamais une branche
ajoutée à un module existant.

## Format de fichier

Une pièce est aujourd'hui un simple JSON (`.caopart`) ne contenant que des
métadonnées (id, nom, dates). Il portera l'arbre de fonctions (croquis,
extrusions, ...) une fois le mode croquis implémenté — ce sera un changement
de schéma à versionner, pas une réécriture du format.

## Pistes non prioritaires (à débattre plus tard)

- **Travail collaboratif** : verrouillage d'une pièce par un seul
  utilisateur à la fois, vs édition simultanée à plusieurs. Choix à faire
  quand le besoin deviendra concret ; ne pas anticiper l'architecture
  réseau/sync avant ça.
- **Licence pro** : une offre commerciale en plus de la double licence
  MIT/Apache-2.0, modalités non définies.
