---
name: architecture-rust
description: SOLID, clean architecture, ports and adapters, and bounded contexts applied to the CAO workspace. Use before adding a crate, a module or a dependency, before doing I/O (file, clock, network) in a business layer, and whenever you wonder where a responsibility should live or why a test is hard to write.
---

# Architecture

## The rules are a test

`crates/app/tests/architecture.rs` runs in the gate and enforces what follows:
the crate graph, the purity of the two geometry crates, no interface crate below
the shell, no disk or clock under a domain boundary, no user-facing wording
below `cao_app`.

Read it before arguing with this file. Where the two disagree, the test is
right — it is the one that has been run.

## The allowed graph

```
cao_app  ──►  cao_core  ──►  cao_sketch
   │             └────────►  cao_solid
   └──────────►  cao_render
```

An arrow to the left is forbidden. `cao_sketch` will never know `cao_core`,
`cao_render` will never know `cao_app`.

| Crate | Dependencies | Forbidden |
| --- | --- | --- |
| `cao_sketch` | `glam`, `serde` — nothing else | everything else |
| `cao_solid` | `glam`, `serde` — nothing else | everything else |
| `cao_render` | `wgpu`, `glam`, `bytemuck` | any interface framework |
| `cao_core` | `cao_sketch`, `cao_solid`, `serde`, `zip`, `chrono`, `directories`, `uuid`, `thiserror` | **any UI crate**, `wgpu` |
| `cao_app` | everything above, `egui`, `eframe` | — |

**`cao_core` without UI is not negotiable**: it is the condition for a future
tablet or web front-end to reuse it as it stands.

**`cao_app` is a thin shell**: window and routing between modes. As soon as a
mode carries non-trivial business logic it becomes its own crate
(`cao_assembly`, …) rather than swelling `screens/`.

## Bounded contexts

The crate graph says what may depend on what. [`docs/contexts.md`](../../../docs/contexts.md)
says where the seams are and why, and it is the target the refactor works
towards. [`docs/glossary.md`](../../../docs/glossary.md) holds the words.

Read both before deciding where something lives. The short version:

- **`cao_sketch` — the drawing**, and **`cao_solid` — the matter**, are the two
  real domains. Pure mathematics. A geometry rule belongs in one of them.
- **`cao_render` — the picture**, **`cao_app` — the shell**.
- **`cao_core` is two contexts, not one.** The part (`history`, `state`,
  `document`) and the preferences (`settings`, `theme`, `shortcuts`, `toolbar`,
  `recents`, `config`, `command`) share a manifest and nothing else: 1 796 of
  its 4 328 lines never mention the geometry. The preferences are meant to leave
  as `cao_prefs`.

## Three patterns already here, never named

Naming them matters because an agent that does not see them breaks them.

**Event sourcing.** `History` is a log of `Operation`; `PartState::rebuild` is
the projection. Undo, redo and stepping back are the same operation because
they are all replay. The consequence is hard: **an `Operation` is immutable once
written and its meaning is frozen** — changing how one replays changes what
every existing `.caopart` draws. A new behaviour is a new variant, never a new
reading of an old one.

**Aggregate root.** `Sketch` is one. Points, segments, circles, dimensions and
constraints are not valid independently — the solver resolves them together. Its
fields are private and stay private: every change goes through the root, or the
invariant is lost.

**Anti-corruption layer.** `PointRef::{Existing, New}` translates between what
the user did and what the drawing records: snapping depends on the zoom at the
time, so the interface resolves it at the click and the part stores the answer.
The `f64` → `f32` narrowing towards `cao_render` is the other one.

## The known gaps — debt, not models

Two places contradict the above. They are written down here so they are never
copied as examples.

### `cao_core` is not the domain

Its documentation says "domain types". That is false: it depends on
`cao_sketch` and `cao_solid` and orchestrates sketch, solid, history and
persistence. It is the **application** layer.

In practice: when you look for "where does this geometry rule go", the answer is
`cao_sketch` or `cao_solid`, never `cao_core`. `cao_core` takes what
**coordinates** — the history, the document, the replayed state.

### I/O is hardwired below the boundary

`document.rs`, `recents.rs`, `settings.rs` and `storage.rs` call `std::fs`,
`directories::ProjectDirs`, `zip` and `chrono::Utc::now()` directly.

It shows in the tests: they write into `std::env::temp_dir()`, create real
directories and delete them with `remove_dir_all`. They are slow, they depend on
the environment, and two tests landing on the same directory tread on each
other.

The architecture test lists those four files and refuses a fifth.

## The rule of ports

**No `std::fs`, `directories`, `chrono::Utc::now()` or network access under a
domain boundary without a trait.**

The test is simple: if a function cannot be tested without touching the disk,
the clock or the network, a port is missing.

The port is a trait, in the layer that needs it:

```rust
pub trait PartRepository {
    fn load(&self, path: &Path) -> Result<PartDocument, StorageError>;
    fn save(&self, path: &Path, document: &PartDocument) -> Result<(), StorageError>;
}

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}
```

The real adapter lives beside it, and **it alone** may call `std::fs`:

```rust
pub struct ZipPartRepository;

impl PartRepository for ZipPartRepository {
}
```

The test adapter is a **working** implementation, not an empty stub: it stores
in a `HashMap` and really behaves like a repository.

```rust
#[derive(Default)]
struct InMemoryParts(RefCell<HashMap<PathBuf, PartDocument>>);
```

Business code takes the trait, never the implementation:

```rust
fn open_part<R: PartRepository>(repo: &R, path: &Path) -> Result<PartState, StorageError>
```

Generic rather than `dyn` while there is one implementation at a time: no
indirection at the call, and the compiler sees everything.

**None of these traits exist yet.** The workspace has zero of them today. This
section describes the target, not the state.

### When not to add a port

A port for pure computation buys nothing. `cao_sketch` and `cao_solid` do
mathematics: they are already testable as they stand and need no abstraction. An
abstraction that removes neither disk, nor clock, nor network, nor GPU is dead
weight.

## The rule of wording

**No text meant for a reader below `cao_app`.** A layer underneath returns a
named case — `ExtrusionMode::Cut`, `StorageError::MissingEntry` — and the
interface decides how it is said, and later in which language.

This is what makes i18n a wiring job rather than a rewrite. It is not true yet:
75 lines of French still sit below `cao_app`, spread over eight files, two of
them in `cao_sketch`. The architecture test holds the count per file so that it
can only fall.

## SOLID, applied here

**Single responsibility.** The repellent is `app/src/screens/viewport.rs`:
4 234 lines, 105 functions, 10 types, where the camera, hit-testing, keyboard
input, gestures and annotation drawing all live together. Add nothing to it that
could live elsewhere. A file that grows is signalling one responsibility too
many, not a need for subheadings.

**Open/closed.** `PartState::apply` is a `match` on `Operation`: every new
operation reopens the function. That is accepted for now — the exhaustive
`match` is exactly what guarantees no operation is forgotten at replay, and the
compiler checks it. But if the `match` starts holding logic rather than short
calls, extract the body of each arm.

**Liskov substitution.** One implementation of a port must behave like the
others. If the test adapter accepts a path the real one refuses, the tests lie.

**Interface segregation.** One trait per need. A single `Storage` carrying
parts, settings, recents and the crash log would force every test to implement
all of it.

**Dependency inversion.** The business layer defines the trait, the
infrastructure implements it. The trait lives with the code that uses it, never
with the implementation.

## Adding something

**A crate** — only when a mode outgrows a single screen and carries business
logic of its own, or when a bounded context in `docs/contexts.md` is being split
out. It respects the graph, and the architecture test gains an edge in the same
commit.

**A mode** — a variant of `enum Screen` (`app/src/screens/mod.rs`) and its own
module in `screens/`. Never a branch grafted onto an existing module.

**A dependency** — in `[workspace.dependencies]` at the root, with the version,
then referenced with `.workspace = true`. Check first that it does not break the
table above; the architecture test will refuse it for the two geometry crates.

**A constant** — a `const` or an `as const` object. Never mutable global state.

## Errors

`thiserror`, with variants that **name** the case:

```rust
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("part file has no entry named {0}")]
    MissingEntry(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

Never a catch-all variant carrying a free `String`: the caller can then decide
nothing, it can only display. The message names the case; the wording the user
reads is chosen in `cao_app`.

No `unwrap()`, `expect()` or `panic!()` in production code. An `expect()` in a
test is normal and wanted.
