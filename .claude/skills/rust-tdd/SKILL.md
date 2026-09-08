---
name: rust-tdd
description: Écrire du Rust en test-first sur ce dépôt CAO. À utiliser dès qu'on ajoute une fonctionnalité, corrige un bug, touche au solveur, aux contraintes, à la géométrie ou aux booléens — et dès qu'on se demande où poser un test, comment comparer des f64, ou comment tester du code sans filet.
---

# Test-first, en Rust, ici

## La boucle

Un test qui échoue, le code minimal qui le fait passer, puis le suivant.

```
JUSTE :  test1 → impl1 → test2 → impl2 → test3 → impl3
FAUX  :  test1, test2, test3 → impl1, impl2, impl3
```

Écrire tous les tests d'abord puis tout le code produit de mauvais tests : en
lot, on teste un comportement **imaginé**, et on finit par vérifier la *forme*
des choses — signatures, structures — plutôt que ce que le système fait. Un test
écrit juste après le bout de code correspondant sait, lui, ce qui compte
vraiment.

Un test écrit **après** tout le code est encore pire : il valide ce qui existe
au lieu de ce qui devrait exister.

**Ne jamais refactoriser tant qu'un test est rouge.** D'abord le vert.

## Où poser le test

**Colocalisé**, en bas du fichier, c'est la convention du dépôt :

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tangent_circle_keeps_touching_after_the_line_moves() {
        // ...
    }
}
```

**Intégration**, dans `crates/<crate>/tests/`, quand le test traverse plusieurs
modules ou martèle le système. Modèle existant :
`crates/core/tests/stress_tangent.rs`.

Les noms de tests sont des **phrases anglaises complètes** qui disent le
comportement, pas la fonction appelée. C'est l'usage établi :

```
a_crash_is_written_down_with_its_hour_and_its_stack
cutting_a_pocket_takes_matter_away
turning_backwards_still_faces_outwards
```

Si le nom ne suffit pas à comprendre le test, renomme le test — n'ajoute pas de
commentaire. Un test ne se commente jamais.

## Boucler vite

Le workspace entier met ~12 s. Pendant la boucle rouge/vert, cible :

```sh
cargo test -p cao_sketch                      # une crate
cargo test -p cao_sketch tangent              # les tests dont le nom contient « tangent »
cargo test -p cao_core --test stress_tangent  # un fichier d'intégration
```

Le workspace entier une fois, avant de commiter — le gate le refera de toute
façon.

## Les `f64`, jamais avec `==`

Le noyau calcule en flottants. Deux chemins de calcul différents donnent deux
résultats à un cheveu près, et `0.1` n'est pas représentable en binaire.

```rust
// NON
assert_eq!(circle.radius, 12.5);

// OUI — avec une tolérance choisie, et dite
const TOLERANCE: f64 = 1e-9;
assert!(
    (circle.radius - 12.5).abs() < TOLERANCE,
    "rayon attendu 12.5, obtenu {}",
    circle.radius,
);
```

Choisis la tolérance en fonction de ce qui est mesuré, ne la recopie pas :
`1e-9` pour une comparaison géométrique directe, plus large après un solveur
itératif ou une suite d'opérations booléennes. Une tolérance qu'on élargit pour
faire passer un test rouge cache un bug — c'est le moment de s'arrêter.

## Les zones sans filet

`sketch/src/solver.rs`, `sketch/src/constraints.rs` et tout `crates/app/` n'ont
**aucun test**. Le solveur concentre à lui seul
quatre correctifs récents.

Y toucher se fait en trois temps :

1. **Caractériser d'abord.** Écrire un test qui décrit ce que le code fait
   *aujourd'hui*, même si c'est bancal, et le voir passer. C'est le filet.
2. **Puis un test rouge** pour le comportement voulu.
3. **Puis le changement.** Si le test de caractérisation casse, la modification
   a un effet de bord — c'est exactement ce qu'on voulait apprendre.

Sur le solveur, préfère des **propriétés** à des valeurs en dur : après
résolution, une tangence tient toujours, une contrainte d'égalité reste vraie,
une figure entièrement contrainte ne bouge plus. Les valeurs numériques exactes
d'un solveur itératif changent au moindre réglage ; les propriétés, non.
`crates/core/tests/stress_tangent.rs` en donne le modèle : un générateur
pseudo-aléatoire martèle une configuration et vérifie qu'elle ne part jamais en
morceaux.

## Ce qui se teste, et comment

| Cible | Où | Dépendances |
| --- | --- | --- |
| Géométrie pure (`plane`, `construct`, `regions`) | colocalisé | aucune |
| Solveur, contraintes | colocalisé + propriétés | aucune |
| Maillage, booléens | colocalisé | aucune |
| `PartState::apply`, historique | colocalisé | aucune |
| Persistance (`document`, `settings`, `recents`) | colocalisé | écrit sur disque — voir ci-dessous |
| Caméra, cube, géométrie de rendu | colocalisé | calcul pur, pas de GPU |
| Pipelines wgpu | non testé unitairement | contrôle visuel via l'exemple `offscreen` |
| Présentateur d'un écran (`state.rs`) | colocalisé | aucune — c'est le but |
| Dessin d'un écran (`view.rs`), primitives `ui/` | non testé | demanderait une fenêtre |

**Sur les écrans :** un écran se découpe en un présentateur (`state.rs`) et une
vue (`view.rs`). Le présentateur ne prend jamais `&mut egui::Ui` — le test
d'architecture le refuse — et c'est précisément ce qui le rend testable sans
fenêtre ni GPU. Quand tu ajoutes un comportement dans `crates/app/`, la question
n'est pas « est-ce testable », c'est « qu'est-ce qui décide, et pourquoi est-ce
dans le fichier qui dessine ». Voir `docs/code-layout.md`.

**Sur la persistance :** ces tests écrivent aujourd'hui dans
`std::env::temp_dir()` et nettoient au `remove_dir_all`. C'est lent, et deux
tests qui choisissent le même dossier se marchent dessus. Si tu ajoutes un test
là, donne-lui un dossier au nom unique. La solution de fond — un port pour le
système de fichiers — est décrite dans le skill `architecture-rust`, et c'est un
chantier séparé.

## Avant de commiter

`scripts/verifier.sh` lance `clippy -D warnings` puis `cargo test --workspace`.
Le gate le fait tout seul au commit ; le lancer à la main avant fait gagner un
aller-retour.
