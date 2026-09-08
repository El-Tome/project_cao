# Where a file goes, and what it may import

[`ARCHITECTURE.md`](ARCHITECTURE.md) says which crate may depend on which.
[`contexts.md`](contexts.md) says where the seams fall. This says what a folder
means inside a crate, and it is enforced by
[`crates/app/tests/architecture.rs`](../crates/app/tests/architecture.rs), which
runs in the commit gate. Where the two disagree, the test is right — it is the
one that has been run.

## The role is the folder

A file's job is read from the folder that holds it, never from a suffix in its
name. That is not a preference: a Rust module name is an identifier, so
`button.ui.rs` and `part-repository.rs` cannot name a module at all, and
`#[path = "button.ui.rs"]` on every module would buy a convention at the price
of fighting the compiler on every file.

Carrying the role in the path is also the stronger rule. `crate::ui::button`
says what it is *at the point of use*, and a test can forbid an import; nothing
can forbid a name.

## Three topologies, not one

A folder exists where the role exists. Inventing `ports/` for a crate that does
mathematics adds an indirection and removes no disk, no clock and no network.

**A pure domain** — `cao_sketch`, `cao_solid`. Flat. No port, no adapter, no
service, no primitive. It is already testable as it stands.

```
crates/sketch/src/
├── sketch.rs
├── solver.rs
├── constraints.rs
└── regions.rs
```

**A context that coordinates** — `cao_part` and `cao_prefs`. This is where hexagonal has something to say, because this is where
the disk, the clock and the file format are.

```
crates/part/src/
├── model/       what the context is about: Operation, History, PartState
├── ports/       the traits it needs from outside — one need per trait
├── adapters/    the implementations. The only place std::fs, chrono,
│                directories and zip are allowed to appear
└── services/    what coordinates a model and its ports
```

**The shell** — `cao_app`. Window and routing, and the interface vocabulary.

```
crates/app/src/
├── ui/                  primitives: egui, and nothing of the workspace
├── wording/             what the user reads, one file per source below
└── screens/<mode>/
    ├── state.rs         the presenter — holds, decides, never draws
    └── view.rs          the drawing, through ui/ primitives
```

`wording/` is where a named case from a lower crate becomes a sentence:
`wording/shortcuts.rs` says how a `Key` and a `Chord` read, `wording/toolbar.rs`
how an `Edge` and an `Item` do, `wording/history.rs` how a step of the part's
history reads, `wording/dimension.rs` what a dimension measures,
`wording/settings.rs` what a profile is called. One file per source so that no
single one gathers the whole application, and so that the eventual translation
system has one directory to pass under.

A name a lower crate stores and compares against is a **key** — `default`,
`sketch`, `drawing` — never the sentence the user reads. Translating a
sentence then moves nothing: the comparison keeps matching.

## What may import what

Each line is a test, not a wish.

| A file under | may reach for | never |
| --- | --- | --- |
| `ui/**` | `egui`, `std` | any `cao_*` crate, `crate::screens` |
| `wording/**` | the `cao_*` case it names | `egui` — it says, it never draws |
| `screens/**/view.rs` | `crate::ui`, `egui`, its presenter | dressing a widget by hand |
| `screens/**/state.rs` | the business crates | `egui::Ui` — **it does not draw** |
| `model/**` | `std`, the pure domains | `crate::ports`, `crate::adapters` |
| `ports/**` | `crate::model` | `crate::adapters`, `crate::services` |
| `adapters/**` | its port, and the technology | being named by `services/` |
| `services/**` | `model`, `ports` | `crate::adapters`, `crate::screens` |

The last one is the whole of dependency inversion in a line: a service names the
trait it needs, and something above it — `main`, or a test — decides which
implementation goes in. That is what lets a test hand it a `HashMap` where
production hands it a zip file.

`adapters/**` is also the one place exempt from the rule that nothing below
`cao_app` touches the disk, the clock or the environment. Reaching outside is
what an adapter is for; the rule is that nothing else does.

## The presenter, which is what a hook would have been

`egui` is immediate mode. There is no hook here and pretending otherwise would
produce a dead abstraction. But what a hook *does* — hold the state, work out
what the view should show, take back what the user did — is a struct, and it is
worth separating for exactly the reason hooks exist.

**A presenter never takes `&mut egui::Ui`.** That single rule is what makes it
testable with no window and no GPU:

```rust
// screens/sketch/state.rs — no egui in sight
pub struct SketchEditor { /* ... */ }

impl SketchEditor {
    pub fn tool(&self) -> Tool { self.tool }
    pub fn clicked_at(&mut self, point: Vec2, snap: Snap) -> Option<Operation> { /* ... */ }
}
```

```rust
// screens/sketch/view.rs — draws, and decides nothing
pub fn show(ui: &mut egui::Ui, editor: &mut SketchEditor) -> Vec<Command> { /* ... */ }
```

`SketchEditor`, `ViewportState` and `Ribbon` are already presenters. They have
simply not been separated from their views yet: `viewport.rs`, the largest file
in the repository, is where the camera, hit-testing, the keyboard, gestures and
annotation drawing share one file, and none of it can be exercised without
opening a window.

## Primitives

A primitive takes plain values — a `&str`, an `f32`, a `bool` — and hands back
what the user did. It knows no part, no sketch, no history. That is the same
inversion as a port, applied to the interface: `ui/` is a vocabulary, and the
screen is what has something to say.

The measure of what this is for: `settings.rs` builds **22 `egui::Slider` and
4 `egui::TextEdit` by hand**. Every one of them re-decides the same width, the
same step, the same way of showing a unit. One `ui/slider.rs` ends that, and the
architecture test holds the count at 31 across `cao_app` so it can only fall.

Layout containers — `Frame`, `Area`, `ScrollArea`, the panels — are not
primitives. Arranging a screen is the screen's own business.

## Names

Files and folders are **snake_case**, because a module name is an identifier.
Kebab-case is impossible here whatever any other convention says.

| Role | Folder | File | What is in it |
| --- | --- | --- | --- |
| Model | `model/` | the thing | `operation.rs` → `Operation` |
| Port | `ports/` | the need | `part_repository.rs` → `trait PartRepository` |
| Real adapter | `adapters/` | technology and need | `zip_part_repository.rs` → `ZipPartRepository` |
| Test adapter | `adapters/` | `in_memory_…` | behind `feature = "test-support"`, so `tests/` can see it |
| Service | `services/` | the verb | `rebuild_part.rs` |
| Primitive | `ui/` | the widget | `button.rs` → `fn button(…) -> Response` |
| Presenter | `screens/<mode>/` | `state.rs` | `SketchEditor` |
| View | `screens/<mode>/` | `view.rs` | `fn show(&mut Ui, …)` |

**No `utils`, `helpers`, `common`, `misc`, `shared`, `manager`, `handler`.** A
name that does not say what is inside is where responsibilities come to hide,
and it fills faster than anything else in a repository. A function belongs to
the thing it serves; if it genuinely serves several, it gets a name of its own —
`geometry_conversion`, not `utils`. The test refuses these names outright.

## Per context, not per crate

The folders above belong to the **context**, not to the crate that happens to
hold it. Every crate in the workspace is one context today: #26 took the
preferences out as `cao_prefs`, and #28 renamed what was left `cao_part`. So the
rule reads as a way of staying there, not as a split still to make.

Three consequences:

- **A folder appears only where the role exists.** `cao_part` earned a `ports/`
  and an `adapters/` with #42, and no other crate holds either. A context moving
  is a flat `git mv` per file, and earns its folders when a port gives it a role
  to name.
- **A folder never straddles two contexts.** A `model/` holding both a `Theme`
  and an `Operation` is not a folder that needs subheadings; it is two crates
  that have not been separated. That is what `cao_core` was, and
  [`contexts.md`](contexts.md) records how it came apart.
- **A context never reaches into another one's `model/`.** It goes through a
  port, or the translation at the seam is named and lives at the seam. The three
  that already exist — `PointRef::{Existing, New}`, the `f64` → `f32` narrowing,
  and a named case becoming a sentence — are described in
  [`contexts.md`](contexts.md).

## The budget

**400 lines.** Past that, a file is holding more than one responsibility.

Seventeen files are already over it. They are listed by name in the architecture
test with the length they had the day the rule landed, and none of them may
grow. Once one falls back under 400, its entry has to go — the test says so,
because a list of exceptions nobody prunes stops being a debt and becomes a
second standard.
