---
name: revue-rust
description: Relire du Rust avant de le commiter sur ce dépôt CAO. À utiliser avant tout commit, quand on relit un diff, quand on veut savoir si un fichier est devenu trop gros, ou pour vérifier qu'un changement respecte les règles du projet.
---

# Relire avant de commiter

Relis le **diff**, pas le fichier. Pour chaque point ci-dessous, la question est
« est-ce que mon changement introduit ça », pas « est-ce que le dépôt en
contient ».

## Taille et responsabilité

Les repoussoirs du dépôt, pour calibrer : `viewport.rs` fait 4 179 lignes et
105 fonctions, `sketch.rs` 2 372, `solver.rs` 1 470, `state.rs` 1 280.

- [ ] Le fichier touché grossit-il encore ? Si oui, ce que j'ajoute pourrait-il
      vivre ailleurs ?
- [ ] Une fonction dépasse-t-elle l'écran ? Fait-elle plus d'une chose ?
- [ ] Ai-je ajouté un paramètre à une fonction qui en avait déjà cinq ? C'est
      souvent une structure qui manque.
- [ ] Une nouvelle structure porte-t-elle des champs qui ne servent que dans la
      moitié des cas ? C'est deux types, pas un.

## Ce qui doit faire lever un sourcil

- [ ] `unwrap()`, `expect()` ou `panic!()` hors code de test. Dans un test,
      `expect("le fichier existe")` est normal et souhaitable.
- [ ] `pub` posé par réflexe. Un champ ou une fonction qui n'est pas utilisé
      hors du module reste privé — c'est ce qui permet de le changer plus tard.
- [ ] Un `==` entre deux `f64`. Toujours une tolérance, choisie et justifiée.
- [ ] Un `as f32` ou `as f64` loin d'une frontière. La conversion se fait au
      dernier moment, au passage vers le GPU ou `egui`.
- [ ] Une allocation (`Vec::new`, `to_string`, `collect`) dans une boucle de
      rendu ou de hit-test, qui tourne à chaque trame.
- [ ] `clone()` pour faire taire l'emprunteur. Souvent une référence suffit ;
      sinon le découpage est à revoir.
- [ ] Une variante d'erreur portant une `String` libre — l'appelant ne peut
      alors rien décider. Voir `architecture-rust`.
- [ ] Un `enum` de constantes remplacé par des entiers ou des chaînes nues.

## Les règles du dépôt

- [ ] **Aucun commentaire**, sauf pour ce que le code ne peut pas dire :
      une contrainte invisible depuis le fichier, une alternative écartée qui
      serait retentée sans la note, une règle venue de l'extérieur. Une
      paraphrase de la ligne d'en dessous se supprime, elle ne se réécrit pas.
      Le récit de ta propre modification va dans le message de commit.
- [ ] **Jamais de commentaire sur un test.** Le nom du test est la phrase.
- [ ] Le code et les noms de tests sont en **anglais**, les textes vus par
      l'utilisateur en **français**.
- [ ] `cao_core` n'importe aucune crate d'interface.
- [ ] Un nouveau mode est une variante de `Screen`, pas une branche greffée
      ailleurs.
- [ ] Aucune fonctionnalité, crate ou abstraction non demandée. Ce projet
      grossit par petites étapes explicitement demandées.

## Les tests

- [ ] Le comportement ajouté a-t-il un test ? S'il touche `solver.rs`,
      `constraints.rs` ou `crates/app/`, y a-t-il d'abord un test qui
      caractérise l'existant ?
- [ ] Le test décrit-il un **comportement** observable par l'interface
      publique, ou l'implémentation ? Un test qui casse au renommage d'une
      fonction privée testait la mauvaise chose.
- [ ] Le nom du test est-il une phrase qui dit ce que le système fait ?
- [ ] Une tolérance a-t-elle été élargie pour faire passer un test rouge ? Si
      oui, il y a un bug dessous — arrête-toi là.

## Avant de valider

```sh
scripts/verifier.sh
```

`clippy -D warnings` puis `cargo test --workspace`, ~12 s. Le gate le refera au
commit, mais le lancer avant évite un aller-retour.

Pour une relecture indépendante, le sous-agent `revue-archi-rust` applique ce
skill et `architecture-rust` à un diff, dans un contexte séparé — celui qui vient
d'écrire le code est mal placé pour le juger.
