---
name: architecture-rust
description: SOLID, clean architecture et ports & adapters appliqués au workspace CAO. À utiliser avant d'ajouter une crate, un module ou une dépendance, avant de faire de l'I/O (fichier, horloge, réseau) dans une couche métier, et quand on se demande où une responsabilité doit vivre ou pourquoi un test est difficile à écrire.
---

# Architecture

## Le graphe autorisé

```
cao_app  ──►  cao_core  ──►  cao_sketch
   │             └────────►  cao_solid
   └──────────►  cao_render
```

Une flèche vers la gauche est interdite. `cao_sketch` ne connaîtra jamais
`cao_core`, `cao_render` ne connaîtra jamais `cao_app`.

Ce que chaque crate a le droit de voir :

| Crate | Dépendances | Interdit |
| --- | --- | --- |
| `cao_sketch` | `glam`, `serde` | tout le reste |
| `cao_solid` | `glam`, `serde` | tout le reste |
| `cao_render` | `wgpu`, `glam`, `bytemuck` | tout framework d'interface |
| `cao_core` | `cao_sketch`, `cao_solid`, `serde`, `zip`, `chrono`, `directories` | **toute crate UI** — `egui`, `eframe`, `winit` |
| `cao_app` | tout ce qui précède, `egui`, `eframe` | — |

**`cao_core` sans UI n'est pas négociable** : c'est la condition pour qu'un
futur front-end tablette ou web le réutilise tel quel.

**`cao_app` est un shell fin** : fenêtre et routage entre modes. Dès qu'un mode
porte une logique métier non triviale, il devient son propre crate
(`cao_assembly`, ...) plutôt que de grossir `screens/`.

## Les écarts actuels — dette connue, pas des modèles

Deux endroits du dépôt contredisent ce qui précède. Ils sont documentés ici pour
qu'on ne les recopie **jamais** comme exemple.

### `cao_core` n'est pas le domaine

Sa documentation dit « types de domaine ». C'est faux : il dépend de
`cao_sketch` et `cao_solid` et orchestre esquisse, solide, historique et
persistance. C'est la couche **application**. Les vrais domaines sont
`cao_sketch` et `cao_solid`, qui ne dépendent que de `glam` et `serde`.

Conséquence pratique : quand tu cherches « où mettre une règle métier de
géométrie », la réponse est `cao_sketch` ou `cao_solid`, pas `cao_core`.
`cao_core` reçoit ce qui **coordonne** — l'historique, le document, l'état
rejoué.

### L'I/O est en dur sous la frontière

`document.rs`, `recents.rs`, `settings.rs` et `storage.rs` appellent
directement `std::fs`, `directories::ProjectDirs`, `zip` et `chrono::Utc::now()`.

Ça se voit dans les tests : ils écrivent dans `std::env::temp_dir()`, créent de
vrais dossiers et les effacent au `remove_dir_all`. Ils sont lents, dépendants
de l'environnement, et deux tests qui tombent sur le même dossier se marchent
dessus.

## La règle des ports

**Aucun `std::fs`, `directories`, `chrono::Utc::now()`, ni accès réseau sous une
frontière de domaine sans passer par un trait.**

Le critère est simple : si une fonction ne peut pas être testée sans toucher le
disque, l'horloge ou le réseau, c'est qu'il manque un port.

### Le patron, en Rust

Le port est un trait, dans la couche qui en a besoin :

```rust
pub trait PartRepository {
    fn load(&self, path: &Path) -> Result<PartDocument, StorageError>;
    fn save(&self, path: &Path, document: &PartDocument) -> Result<(), StorageError>;
}

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}
```

L'adaptateur réel vit à côté, et c'est **lui seul** qui a le droit d'appeler
`std::fs` :

```rust
pub struct ZipPartRepository;

impl PartRepository for ZipPartRepository {
    fn load(&self, path: &Path) -> Result<PartDocument, StorageError> { /* zip + serde */ }
    fn save(&self, path: &Path, document: &PartDocument) -> Result<(), StorageError> { /* ... */ }
}
```

L'adaptateur de test est une implémentation **fonctionnelle**, pas un bouchon
vide : il stocke dans une `HashMap` et se comporte réellement comme un dépôt.

```rust
#[derive(Default)]
struct InMemoryParts(RefCell<HashMap<PathBuf, PartDocument>>);
```

Le code métier prend le trait, jamais l'implémentation :

```rust
fn open_part<R: PartRepository>(repo: &R, path: &Path) -> Result<PartState, StorageError>
```

Générique plutôt que `dyn` quand il n'y a qu'une implémentation à la fois : pas
d'indirection au moment de l'appel, et le compilateur voit tout.

### Quand ne pas poser de port

Un port pour du calcul pur ne sert à rien. `cao_sketch` et `cao_solid` ne font
que des mathématiques : ils sont déjà testables tels quels et n'ont besoin
d'aucune abstraction. Une abstraction qui n'enlève ni disque, ni horloge, ni
réseau, ni GPU est du poids mort.

## SOLID, appliqué ici

**Responsabilité unique.** Le repoussoir est `app/src/screens/viewport.rs` :
4 179 lignes, 105 fonctions, 7 structures, où cohabitent la caméra, le
hit-test, la saisie clavier, les gestes et le dessin des annotations. N'y ajoute
rien qui puisse vivre ailleurs. Un fichier qui grossit signale une
responsabilité de trop, pas un besoin de sous-titres.

**Ouvert/fermé.** `PartState::apply` est un `match` sur `Operation` : chaque
nouvelle opération oblige à rouvrir la fonction. C'est assumé pour l'instant —
l'exhaustivité du `match` est justement ce qui garantit qu'aucune opération
n'est oubliée au rejeu, et le compilateur le vérifie. Mais si le `match` se met
à contenir de la logique plutôt que des appels courts, extrais le corps de
chaque bras.

**Substitution de Liskov.** Une implémentation d'un port doit se comporter comme
les autres. Si l'adaptateur de test accepte un chemin que l'adaptateur réel
refuse, les tests mentent.

**Ségrégation des interfaces.** Un trait par besoin. Un `Storage` unique portant
pièces, réglages, récents et journal de plantage obligerait chaque test à tout
implémenter.

**Inversion des dépendances.** Le métier définit le trait, l'infrastructure
l'implémente. Le trait vit avec le code qui l'utilise, jamais avec
l'implémentation.

## Ajouter quelque chose

**Une crate** — seulement quand un mode dépasse un simple écran et porte une
logique métier propre. Elle respecte le graphe : elle ne remonte jamais vers
`cao_app`.

**Un mode** — une variante de `enum Screen` (`app/src/screens/mod.rs`) et son
propre module dans `screens/`. Jamais une branche greffée sur un module
existant.

**Une dépendance** — dans `[workspace.dependencies]` de la racine, avec la
version, puis référencée par `.workspace = true`. Vérifie d'abord qu'elle ne
casse pas le tableau ci-dessus.

**Une constante** — un `const` ou un objet `as const`. Jamais un état global
mutable.

## Les erreurs

`thiserror`, avec des variantes qui **nomment** le cas :

```rust
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("le fichier de pièce ne contient pas « {0} »")]
    MissingEntry(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

Jamais de variante fourre-tout portant une `String` libre : l'appelant ne peut
alors rien décider, il ne peut qu'afficher. Les messages destinés à
l'utilisateur sont en français, comme le reste de ce qu'il lit.

Pas de `unwrap()`, `expect()` ni `panic!()` dans du code de production. Un
`expect()` dans un test est normal et souhaitable.
