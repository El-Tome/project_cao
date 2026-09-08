# Bounded contexts

Where the seams are, where they should be, and what has to be translated when
something crosses one. [`ARCHITECTURE.md`](ARCHITECTURE.md) says which crate may
depend on which; this says why the lines fall where they do, and it is the
target the refactor works towards. The words used here are in
[`glossary.md`](glossary.md).

A context is a stretch of code where one word means one thing. Two contexts may
use the same word for different things, and that is fine as long as something
translates at the boundary. What is not fine is a boundary nobody drew.

## Today

```
cao_app  ──►  cao_core  ──►  cao_sketch
   │             └────────►  cao_solid
   └──────────►  cao_render
```

Four of these are one context each and are in the right place:

- **`cao_sketch` — the drawing.** Points, segments, circles, dimensions,
  constraints, the solver, closed regions. Pure mathematics: no disk, no clock,
  no GPU. Rich, precise language of its own.
- **`cao_solid` — the matter.** Polygons, meshes, prisms, revolutions, boolean
  union and difference over a BSP tree. Also pure mathematics, and it knows
  nothing of `cao_sketch`.
- **`cao_render` — the picture.** Camera, orientation cube, vertex buffers,
  wgpu. Knows no interface framework.
- **`cao_app` — the shell.** Window, routing between modes, gestures, wording.

## The seam that is missing

`cao_core` is not one context. It is 4 328 lines that share a manifest and
nothing else:

| File | Lines | Mentions of `cao_sketch` / `cao_solid` |
| --- | --- | --- |
| `state.rs` | 1 297 | 33 |
| `history.rs` | 539 | 3 |
| `document.rs` | 339 | 4 |
| `storage.rs` | 95 | 0 |
| `toolbar.rs` | 475 | **0** |
| `shortcuts.rs` | 342 | **0** |
| `config.rs` | 315 | **0** |
| `settings.rs` | 307 | **0** |
| `theme.rs` | 283 | **0** |
| `command.rs` | 235 | **0** |
| `recents.rs` | 74 | **0** |

1 796 lines — 41 % of the crate — never mention the geometry. Two contexts, held
together by the accident of being neither interface nor mathematics:

**The part.** `Operation`, `History`, `PartState`, `PartDocument`. Its language
is operation, replay, step, part. Its invariant is that the geometry is never
stored, only replayed, and that an operation's meaning is frozen the moment a
file is written with it. It is versioned inside a `.caopart`.

**The preferences.** Themes, shortcuts, toolbar layout, profiles, recent files.
Its language is profile, theme, chord, layout. It shares no invariant with the
part, its lifetime is the installation rather than the document, and it lives
under the user's config directory.

Nothing in the second reaches for the first. They are already decoupled; the
crate boundary simply has not been drawn where the decoupling is.

### The one file that straddles the seam

`storage.rs` is the exception, and the only one. It is imported from both sides:
`document.rs` takes `StorageError` from it, `settings.rs` and `recents.rs` take
`StorageError` **and** `project_dirs()`. It also holds three unrelated things,
so it is cut three ways rather than moved:

| What | Goes to | Why |
| --- | --- | --- |
| `StorageError` | split in two | The part keeps `Archive` and `MissingEntry`, the preferences keep `NoProjectDirs`; `Io`, `Json` and `UnsupportedVersion` are duplicated. |
| `project_dirs()` | `cao_prefs` | `document.rs` never calls it — it takes the directory as a parameter. Every caller is a preference. |
| `default_projects_dir()` | `cao_app` | Where parts land is a choice of the shell, made once in `app.rs`. |
| `crash_log_path()`, `record_panics()` | `cao_app` | The crash log belongs to neither context. It is called from `main.rs` and nowhere else. |

The cost of the cut is one variant and two `#[from]` duplicated. The three
alternatives all cost more: moving `storage.rs` whole would make `cao_part`
depend on `cao_prefs`, leaving it behind would make `cao_prefs` depend on
`cao_part`, and a third plumbing crate is a crate nobody asked for.

`ProjectDirs::from("dev", "cao", "cao")` is moved, never retyped. The smallest
difference in that triple relocates the user's configuration directory and loses
them their profiles, their shortcuts and their recent files.

## The target

```
cao_app  ──►  cao_part   ──►  cao_sketch
   │             └─────────►  cao_solid
   ├──────►  cao_prefs
   └──────►  cao_render
```

`cao_core` becomes `cao_part`, and the preferences leave as `cao_prefs`, which
depends on nothing of the geometry. A crate rather than a module, because a
crate boundary is the one the compiler checks — and the whole point of this
work is rules that are executed rather than promised.

`cao_core` also stops being called core. It never was the domain: it depends on
both domains and orchestrates them. Naming it after what it holds ends a
misunderstanding the current name creates on every reading.

## The folders travel with the context

Inside a crate, a file's role is carried by the folder that holds it —
`model/`, `ports/`, `adapters/`, `services/`, and in the shell `ui/` and
`screens/<mode>/`. [`code-layout.md`](code-layout.md) says what each one means
and what it may import.

Those folders belong to the **context**, not to the crate that currently holds
it, and that is what makes the split above cheap: when `cao_prefs` leaves, it
takes its own `model/`, `ports/` and `adapters/` with it. A `git mv` of whole
folders rather than a file-by-file sort of eleven files nobody has looked at in
months.

Two rules follow, and they are the ones that keep a seam findable:

- **A folder never straddles two contexts.** A `model/` holding both a `Theme`
  and an `Operation` is not a folder in need of subheadings — it is the signal
  that the crate is two crates, exactly as `cao_core` is today.
- **A context never reaches into another one's `model/`.** It goes through a
  port, or the translation is named and lives at the seam. The three that exist
  are described just below.

## What crosses, and what translates

Three boundaries carry a real translation. They are the places where a change
on one side must not be allowed to leak to the other.

**Intent → reference.** `PointRef::{Existing, New}` sits between what the user
did and what the drawing records. Snapping depends on the zoom at the time, so
the interface resolves it *at the moment of the click*; the part stores the
answer, never the question. Re-deriving it on replay would rebuild a different
drawing.

**Exactness → pixels.** `cao_sketch` and `cao_solid` compute in `f64`;
`cao_render` and `egui` take `f32`. The narrowing happens at that boundary and
nowhere earlier — a conversion further in is a loss of precision taken for no
reason.

**Named case → wording.** A layer below `cao_app` returns
`ExtrusionMode::Cut`, not "Enlèvement de matière". The interface decides how a
case is said, and later in which language. This one is not yet true: 75 lines
of French still sit below `cao_app`, and
`crates/app/tests/architecture.rs` holds the count so it can only fall.

## What is deliberately not its own context

- **A mode** (sketching, assembly) is a variant of `Screen`, not a context of
  its own — until it carries real rules, at which point it earns a crate.
- **The solver** is a domain service inside the drawing, not a context. It has
  no vocabulary of its own that the sketch does not already have.
- **Undo and redo** are not a context. They are `rebuild` at another index.
  Giving them their own machinery is how they start disagreeing with a replay.

## Order of the moves

1. **`cao_prefs` out of `cao_core`.** A move of whole files, no logic touched —
   `storage.rs` excepted, which is cut three ways first — and the architecture
   test gains an edge.
2. **`cao_core` renamed `cao_part`.** Mechanical, and best done while the crate
   is already being handled.
3. **Wording up into `cao_app`.** Each file moved lowers a figure in the
   architecture test. Unblocks i18n without building it.
4. **Ports for disk and clock.** Removes the last entries from the other
   ratchet, and lets the persistence tests stop writing to a temporary
   directory.
5. **The drawing rules out of `viewport.rs` into `cao_sketch`.** The largest
   piece, and the reason for the four before it: it needs somewhere correct to
   land.
