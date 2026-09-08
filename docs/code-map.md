# Code map

The other pages of `docs/` tell what the software does. This one says where it
is written. It serves whoever arrives on the project and looks for a way in —
human or agent.

It answers "where is this already written". For "where does a new file go" —
which folder, what it may import, what size it does not exceed — that is
[`code-layout.md`](code-layout.md), and the architecture test checks it.

## The six crates

```
cao_app  ──►  cao_part  ──►  cao_sketch
   │             └────────►  cao_solid
   ├──────────►  cao_prefs
   └──────────►  cao_render
```

| Crate | Role | Depends on |
| --- | --- | --- |
| `cao_sketch` | sketch model, constraints, solver | `glam`, `serde` |
| `cao_solid` | mesh, extrusion, booleans | `glam`, `serde` |
| `cao_render` | GPU rendering of the viewport | `wgpu`, `glam`, `bytemuck` |
| `cao_part` | document, history, persistence of a part | both domains |
| `cao_prefs` | theme, shortcuts, toolbar, profiles, recents | `serde`, `directories` |
| `cao_app` | desktop shell, routing between modes | everything |

An arrow to the left is forbidden: `cao_sketch` will never know `cao_part`,
`cao_render` will never know `cao_app`.

No length is written down here: a figure in prose is exact at the commit that
writes it and false at the next. The only lengths that carry a rule — the files
over the budget of 400 lines — are held by `crates/app/tests/architecture.rs`,
which fails when they move.

**The real domains are `cao_sketch` and `cao_solid`** — they are the two that
depend on nothing. `cao_part` depends on both and orchestrates sketch, solid,
history and persistence: it is the **application** layer. A geometry rule goes
in one of the two domains, never there.

## Drawing and dimensioning — `cao_sketch`

| What one is after | File | Way in |
| --- | --- | --- |
| Sketch model: points, traits, circles | `sketch/src/sketch.rs` | `Sketch`, `live_points`, `live_segments`, `live_circles` |
| Erasing an element and what leans on it | `sketch/src/sketch.rs` | `Sketch::erase` |
| Placing or removing a constraint | `sketch/src/sketch.rs` | `add_constraint`, `add_tangency`, `erase_constraint` |
| Kinds of constraint and dimension | `sketch/src/constraints.rs` | `Constraint`, `Dimension`, `DimensionTarget`, `Freedom` |
| The solver | `sketch/src/solver.rs` | `solve(millimeters_per_unit)` → `SolveOutcome` |
| The five circle constructions | `sketch/src/construct.rs` | `centre_through`, `centre_touching_two`, `circle_touching_three` |
| Work plane, going 2D ↔ 3D | `sketch/src/plane.rs` | `WorkPlane::to_world`, `to_local`, `ray_intersection` |
| Closed areas, to extrude | `sketch/src/regions.rs` | `Sketch::regions()` |

What it does: [`sketch.md`](sketch.md).

## Volumes — `cao_solid`

| What one is after | File | Way in |
| --- | --- | --- |
| Mesh, faces, ray casting | `solid/src/mesh.rs` | `Mesh`, `Polygon`, `ray_hit`, `bounds` |
| Extruding an area into a prism | `solid/src/mesh.rs` | `prism(...)` |
| Turning an area around an axis | `solid/src/mesh.rs` | `revolution(...)` |
| Adding or taking away matter | `solid/src/boolean.rs` | `Mesh::union`, `Mesh::difference` (BSP tree) |

What it does: [`extrusion.md`](extrusion.md).

## History and persistence — `cao_part`

| What one is after | File | Way in |
| --- | --- | --- |
| List of operations, undo, redo | `part/src/history.rs` | `History`, `Operation` |
| Replaying the history for the geometry | `part/src/state.rs` | `PartState::rebuild`, `PartState::apply` |
| The `.caopart` file (zip) | `part/src/document.rs` | `PartDocument`, `SCHEMA_VERSION = 3` |
| What fails when opening a part | `part/src/errors.rs` | `PartFileError` |
| What a part asks of a filesystem | `part/src/ports/files.rs` | `Files`, `FileError` |
| A filesystem for tests | `part/src/adapters/in_memory_files.rs` | `InMemoryFiles`, behind `test-support` |
| The real filesystem, atomic writes | `app/src/adapters/files.rs` | `DiskFiles` |

## Settings, profiles and recents — `cao_prefs`

| What one is after | File | Way in |
| --- | --- | --- |
| The ten recent parts | `prefs/src/recents.rs` | `RecentList` |
| Paths, crash log | `prefs/src/storage.rs` | `project_dirs`, `default_projects_dir`, `record_panics` |
| Commands of the interface | `prefs/src/command.rs` | `Command`, `CommandFamily` |
| Settings and named profiles | `prefs/src/settings.rs` | `Settings`, `Profile`, `Profiles` |
| Viewport and navigation settings | `prefs/src/config.rs` | `ViewportConfig`, `Binding`, `NavigationPreset` |
| Colours and gradients | `prefs/src/theme.rs` | `Theme`, `Background`, `Rgba`, `Stop` |
| Keyboard shortcuts | `prefs/src/shortcuts.rs` | `Shortcuts`, `Chord`, `Key` |
| Toolbar | `prefs/src/toolbar.rs` | `ToolbarLayout`, `Item`, `Edge` |

What they do: [`history.md`](history.md),
[`configuration.md`](configuration.md).

## GPU rendering — `cao_render`

| What one is after | File | Way in |
| --- | --- | --- |
| wgpu pipelines, render pass | `render/src/renderer.rs` | `SceneRenderer::prepare`, `paint` |
| Orbit camera, transitions | `render/src/camera.rs` | `OrbitCamera`, `ViewTransition` |
| Orientation cube | `render/src/cube.rs` | `push_faces`, `zone_at`, `is_visible` |
| Axes, grid, background, solids | `render/src/geometry.rs` | `push_axes`, `push_grid`, `push_background`, `push_solid` |
| Visual check without a window | `render/examples/offscreen.rs` | `cargo run -p cao_render --example offscreen -- /tmp` |

What it does: [`render.md`](render.md), [`viewport.md`](viewport.md).

## Interface — `cao_app`

| What one is after | File | Way in |
| --- | --- | --- |
| Application state, frame loop | `app/src/app.rs` | `CaoApp`, `impl eframe::App` |
| Routing between modes | `app/src/screens/mod.rs` | `enum Screen`, `struct OpenPart` |
| Canvas: gestures, hit test, drawing | `app/src/screens/viewport.rs` — the largest file in the repository | `show(...)`, `ViewportState`, `ViewMode` |
| Sketch tool, keyboard input | `app/src/screens/sketch.rs` | `SketchEditor`, `LiveInput`, `CircleMode` |
| Placing dimensions on screen | `app/src/screens/annotations.rs` | `push(...)`, `Placement`, `Style` |
| Extrusion and revolution, UI side | `app/src/screens/extrusion.rs` | `ExtrusionState` |
| History panel | `app/src/screens/history_tree.rs` | `show(...)` → `HistoryAction` |
| Toolbar | `app/src/screens/ribbon.rs` | `Ribbon::show`, `is_enabled` |
| Settings screen | `app/src/screens/settings.rs` | `show(...)` |
| Start menu | `app/src/screens/start_menu.rs` | `show(...)` → `StartMenuAction` |
| What a command, its help and its family are called | `app/src/wording/command.rs` | `label`, `hint`, `family_heading` |
| What a key and a chord are called | `app/src/wording/shortcuts.rs` | `chord` |
| What a toolbar placement and a tree entry are called | `app/src/wording/toolbar.rs` | `edge`, `item` |

What they do: [`interface.md`](interface.md),
[`navigation.md`](navigation.md).

`app/src/wording/` holds the sentences the user reads, one file per source.
A layer below `cao_app` returns a named case and this is where it is decided
how that case is said, which is what will make translation a wiring job. The
sources still saying their own sentences are counted by
`crates/app/tests/architecture.rs`, and that count only falls.

## The invariants

**The numbers.** The core computes in `f64`, the camera and the rendering in
`f32`, and the conversion happens at each crossing of the boundary. The
reasoning is in [`ARCHITECTURE.md`](ARCHITECTURE.md), in one copy.

**The geometry is never saved.** A `.caopart` holds its metadata and its
history of operations, nothing else. The geometry is rebuilt by
`PartState::rebuild`, which replays the operations — that is what makes undo,
redo and going back to a step one and the same operation. There is **one single**
place where geometry is produced: `PartState::apply`. Any copy kept alongside
would end up diverging.

**One mode = one variant of `Screen`.** A new mode adds a variant to
`enum Screen` and its own module in `screens/`, never a branch grafted onto an
existing module.

**`cao_part` depends on no UI crate.** That is the condition for a future
tablet or web front-end to reuse it as it is.

**`cao_app` is to stay a thin shell.** That is an aim, not an observation: it is
the largest crate in the repository, and `viewport.rs` alone holds the camera,
the hit test, the keyboard, the gestures and the drawing of the annotations. As
soon as a mode carries non-trivial business logic, it becomes its own crate.

## What has no tests

| Area | Tests |
| --- | ---: |
| `sketch/src/solver.rs` | 0 |
| `sketch/src/constraints.rs` | 0 |
| `crates/app/src/` | 0 |

The test files of that crate are about the repository, not about the
interface: `crates/app/tests/architecture.rs` tests its shape — crate graph,
folders, line budget, French below the interface — `crates/app/tests/gate.rs`
checks that `scripts/verify.sh` and `.github/workflows/ci.yml` check the same
things, and `crates/app/tests/language.rs` that nothing a developer reads is
written in French.

The solver is the algorithmic heart and most of its history is made of
successive fixes (`git log -- crates/sketch/src/solver.rs`), with no net at
all. Working in it means first writing a test that characterises what is there.

Elsewhere the repository is tested, and each test lives in the file it covers.
`cargo test --workspace` gives the count of the day.

## Checking

```sh
scripts/verify.sh            # fmt --check, clippy -D warnings, cargo test --workspace, ~12 s
cargo test -p cao_sketch     # one crate alone, while iterating
cargo run -p cao_app         # launch the application
```
