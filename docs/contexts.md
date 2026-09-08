# Bounded contexts

Where the seams are, why they fall there, and what has to be translated when
something crosses one. [`ARCHITECTURE.md`](ARCHITECTURE.md) says which crate may
depend on which; this says why the lines fall where they do, and what the
refactor has left to move. The words used here are in
[`glossary.md`](glossary.md).

A context is a stretch of code where one word means one thing. Two contexts may
use the same word for different things, and that is fine as long as something
translates at the boundary. What is not fine is a boundary nobody drew.

## Today

```
cao_app  ──►  cao_part   ──►  cao_sketch
   │             └─────────►  cao_solid
   ├──────►  cao_prefs
   └──────►  cao_render
```

Six crates, one context each. `cao_app` also reaches `cao_sketch` and
`cao_solid` directly, which the arrows leave out: the route through `cao_part`
is the one that carries a part, not an exclusive one.

- **`cao_sketch` — the drawing.** Points, segments, circles, dimensions,
  constraints, the solver, closed regions. Pure mathematics: no disk, no clock,
  no GPU. Rich, precise language of its own.
- **`cao_solid` — the matter.** Polygons, meshes, prisms, revolutions, boolean
  union and difference over a BSP tree. Also pure mathematics, and it knows
  nothing of `cao_sketch`.
- **`cao_render` — the picture.** Camera, orientation cube, vertex buffers,
  wgpu. Knows no interface framework.
- **`cao_part` — the part.** `Operation`, `History`, `PartState`,
  `PartDocument`. Its language is operation, replay, step, part. Its invariant
  is that the geometry is never stored, only replayed, and that an operation's
  meaning is frozen the moment a file is written with it. It is versioned inside
  a `.caopart`.
- **`cao_prefs` — the preferences.** Themes, shortcuts, toolbar layout,
  profiles, recent files. Its language is profile, theme, chord, layout. It
  shares no invariant with the part, its lifetime is the installation rather
  than the document, and it lives under the user's config directory.
- **`cao_app` — the shell.** Window, routing between modes, gestures, wording.

## The seam that was drawn

`cao_part` and `cao_prefs` shared one crate, `cao_core`, and shared nothing
else: no
invariant, no vocabulary, no lifetime. Nothing on the preferences side ever
reached for the part. They were already decoupled; the crate boundary simply had
not been drawn where the decoupling was. Move 1 (#26) drew it. Move 2 (#28)
renamed what was left after what it holds — it never was the domain, it depends
on both domains and orchestrates them, and the old name made that
misunderstanding on every reading.

Two decisions shaped the cut, and both outlive it.

### `command.rs` went with the preferences

It is the one file where membership was a choice rather than a reading. Its
vocabulary is the part's — `NewSketch`, `Undo`, `ExtrusionCut` — but `Command`
is the vocabulary of *intent*, not of what the part records: nothing replays a
`Command`, and no `.caopart` mentions one. Its only two consumers are
`toolbar.rs` and `shortcuts.rs`, which bind it to a gesture and to a chord, and
`toolbar.rs` serialises it into the profile. Leaving it behind would have
pointed `cao_prefs` at `cao_part`, the one edge the graph forbids.

### `storage.rs` was cut three ways rather than moved

It was imported from both sides: `document.rs` took `StorageError` from it,
`settings.rs` and `recents.rs` took `StorageError` **and** `project_dirs()`. It
also held three unrelated things:

| What | Went to | Why |
| --- | --- | --- |
| `StorageError` | split in two | `cao_part` got `PartFileError`, with `Archive` and `MissingEntry`; `cao_prefs` kept `StorageError`, with `NoProjectDirs`. `Io`, `Json` and `UnsupportedVersion` sit on both. |
| `project_dirs()` | `cao_prefs` | `document.rs` never calls it — it takes the directory as a parameter. Every caller is a preference. |
| `default_projects_dir()`, `crash_log_path()`, `record_panics()` | `cao_prefs` | Planned for `cao_app`, and they stayed: all three go through `project_dirs()`, so moving them up would have exported it. `cao_app` calls them from `app.rs` and `main.rs`. |

The cut cost one variant and two `#[from]` duplicated. The three alternatives
all cost more: moving `storage.rs` whole would have made `cao_part` depend on
`cao_prefs`, leaving it behind would have made `cao_prefs` depend on `cao_part`,
and a third plumbing crate is a crate nobody asked for.

`ProjectDirs::from("dev", "cao", "cao")` was moved, never retyped. The smallest
difference in that triple relocates the user's configuration directory and loses
them their profiles, their shortcuts and their recent files.

## The folders appear where the role appears

Inside a crate, a file's role is carried by the folder that holds it —
`model/`, `ports/`, `adapters/`, `services/`, and in the shell `ui/` and
`screens/<mode>/`. [`code-layout.md`](code-layout.md) says what each one means
and what it may import.

**The first two appeared with #42.** `cao_part` holds a `ports/` — `trait Files`,
the whole of what a part asks of a filesystem — and an `adapters/` where its
in-memory double sits; the real one, `DiskFiles`, is in the shell. No other
crate holds either folder yet, and `CLAUDE.md` §Where a file goes carries the
rule that settles it: *a folder appears only where the role exists.* The layout
above is a destination, not a scaffold to erect now and fill later.

So moves 1 and 2 were flat `git mv`s, one file each, and no folder appeared.
`cao_prefs` earns its own `ports/` and `adapters/` when #43 and #44 give
`storage.rs` and `recents.rs` a trait to sit behind. Drawing the folders before
that is guessing where the seam falls, over files that have just moved once
already.

Two rules follow all the same, and they are the ones that keep a seam findable:

- **A folder never straddles two contexts.** A `model/` holding both a `Theme`
  and an `Operation` is not a folder in need of subheadings — it is the signal
  that the crate is two crates, which is what `cao_core` was until move 1 took
  the preferences out of it.
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

1. **`cao_prefs` out of `cao_core`.** *Done, #26.* A move of whole files, no
   logic touched — `storage.rs` excepted, which was cut three ways first — and
   the architecture test gained an edge.
2. **`cao_core` renamed `cao_part`.** *Done, #28.* Mechanical: seven files moved
   with `git mv`, no public type renamed, no ratchet figure changed.
3. **Wording up into `cao_app`.** Each file moved lowers a figure in the
   architecture test. Unblocks i18n without building it.
4. **A port for the disk, and the clock read above it.** The filesystem is a
   behaviour and earns a trait; the hour is a reading and is passed down as a
   value — #41 settled that, and no `trait Clock` was built. Removes the last
   entries from the other ratchet, and lets the persistence tests stop writing
   to a temporary directory.
5. **The drawing rules out of `viewport.rs` into `cao_sketch`.** The largest
   piece, and the reason for the four before it: it needs somewhere correct to
   land.
