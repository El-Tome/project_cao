---
name: code-map
description: Find where to act in the CAO code. Use as soon as one is looking for which file or which function carries a behaviour — sketching, dimensioning, solver, constraints, extrusion, revolution, booleans, history, undo, viewport, camera, orientation cube, grid, wgpu rendering, settings, profiles, shortcuts, toolbar, .caopart format, start menu.
---

# Where what lives

`docs/` tells what the software does. This skill says where it is written. The
complete reference, readable by a human too: `docs/code-map.md`.

Here one looks for **where a behaviour is already written**. To know **where a
new file goes** — which folder, what it may import — that is
`docs/code-layout.md`, and the architecture test checks it.

## The six crates and the direction of the dependencies

```
cao_app  ──►  cao_part  ──►  cao_sketch
   │             └────────►  cao_solid
   ├──────────►  cao_prefs
   └──────────►  cao_render
```

`cao_sketch` and `cao_solid` depend on nothing but `glam` and `serde`.
`cao_render` knows only `wgpu`, `glam` and `bytemuck` — no interface framework.
`cao_prefs` knows neither the geometry nor the interface. `cao_app` is the only
one that sees `egui`/`eframe`.

**The real domains are `cao_sketch` and `cao_solid`** — they depend on nothing.
`cao_part` depends on both and orchestrates sketch, solid, history and
persistence: it is the **application** layer, and a geometry rule never goes
there. See the `architecture-rust` skill.

## Behaviour → file

### Drawing and dimensioning — `cao_sketch`

| What one is after | File | Way in |
| --- | --- | --- |
| The sketch model, points, traits, circles | `sketch/src/sketch.rs` | `Sketch`, `live_points`, `live_segments`, `live_circles` |
| Erasing an element and what leans on it | `sketch/src/sketch.rs` | `Sketch::erase`, `Erased` |
| Placing or removing a constraint | `sketch/src/sketch.rs` | `add_constraint`, `add_tangency`, `erase_constraint` |
| The kinds of constraint and dimension | `sketch/src/constraints.rs` | `Constraint`, `Dimension`, `DimensionTarget`, `Freedom` |
| **The solver** — making every value true together | `sketch/src/solver.rs` | `solve(millimeters_per_unit)` → `SolveOutcome` |
| The five circle constructions | `sketch/src/construct.rs` | `centre_through`, `centre_touching_two`, `circle_touching_three` |
| The work plane, 2D ↔ 3D | `sketch/src/plane.rs` | `WorkPlane::to_world`, `to_local`, `ray_intersection` |
| The closed areas, to extrude | `sketch/src/regions.rs` | `Sketch::regions()` → `Vec<Region>` |

### Volumes — `cao_solid`

| What one is after | File | Way in |
| --- | --- | --- |
| The mesh, the faces, the ray cast | `solid/src/mesh.rs` | `Mesh`, `Polygon`, `ray_hit`, `bounds` |
| Extruding an area into a prism | `solid/src/mesh.rs` | `prism(...)` |
| Turning an area around an axis | `solid/src/mesh.rs` | `revolution(...)` |
| Adding or taking away matter | `solid/src/boolean.rs` | `Mesh::union`, `Mesh::difference` (BSP tree) |

### History and persistence — `cao_part`

| What one is after | File | Way in |
| --- | --- | --- |
| The list of operations, undo, redo | `part/src/history.rs` | `History`, `Operation`, `applied_operations` — what a step is *called* is in `app/src/wording/history.rs` |
| **Replaying the history to get the geometry** | `part/src/state.rs` | `PartState::rebuild`, `PartState::apply` |
| The `.caopart` file (zip), reading and writing | `part/src/document.rs` | `PartDocument`, `SCHEMA_VERSION = 3` |
| What fails when opening a part | `part/src/errors.rs` | `PartFileError` |

## Settings, profiles and recents — `cao_prefs`

| What one is after | File | Way in |
| --- | --- | --- |
| The ten recent parts | `prefs/src/recents.rs` | `RecentList` |
| Paths, parts folder, crash log | `prefs/src/storage.rs` | `project_dirs`, `default_projects_dir`, `record_panics` |
| The commands of the interface | `prefs/src/command.rs` | `Command`, `CommandFamily`, `family` — what a command is *called* is in `app/src/wording/command.rs` |
| Settings and named profiles | `prefs/src/settings.rs` | `Settings`, `Profile`, `Profiles`, `DEFAULT_PROFILE` — what it is *called* is in `app/src/wording/settings.rs` |
| Viewport and navigation settings | `prefs/src/config.rs` | `ViewportConfig`, `Binding`, `NavigationPreset` |
| Colours, gradients | `prefs/src/theme.rs` | `Theme`, `Background`, `Rgba`, `Stop` |
| Keyboard shortcuts | `prefs/src/shortcuts.rs` | `Shortcuts`, `Chord`, `Key` — what a chord is *called* is in `app/src/wording/shortcuts.rs` |
| Arrangement of the toolbar | `prefs/src/toolbar.rs` | `ToolbarLayout`, `Item`, `Edge` — what a placement and an entry are *called* is in `app/src/wording/toolbar.rs` |

### GPU rendering — `cao_render`

| What one is after | File | Way in |
| --- | --- | --- |
| The wgpu pipelines, the render pass | `render/src/renderer.rs` | `SceneRenderer::prepare`, `paint`, `SceneFrame` |
| Orbit camera, view transitions | `render/src/camera.rs` | `OrbitCamera`, `ViewTransition`, `view_angles_towards` |
| The orientation cube, faces/edges/corners | `render/src/cube.rs` | `push_faces`, `zone_at`, `is_visible` |
| Axes, adaptive grid, background, solids | `render/src/geometry.rs` | `push_axes`, `push_grid`, `push_background`, `push_solid`, `adaptive_step` |
| Offscreen rendering, visual check | `render/examples/offscreen.rs` | `cargo run -p cao_render --example offscreen -- /tmp` |

### Interface — `cao_app`

| What one is after | File | Way in |
| --- | --- | --- |
| The application state, the frame loop | `app/src/app.rs` | `CaoApp`, `impl eframe::App` |
| The routing between modes | `app/src/screens/mod.rs` | `enum Screen`, `struct OpenPart` |
| **The canvas: gestures, hit test, drawing** | `app/src/screens/viewport.rs` — the largest file in the repository | `show(ui, state, sketch)`, `ViewportState`, `ViewMode` |
| The sketch tool, keyboard input | `app/src/screens/sketch.rs` | `SketchEditor`, `LiveInput`, `CircleMode`, `Selection` |
| The placing of dimensions on screen | `app/src/screens/annotations.rs` | `push(...)`, `Placement`, `Style` |
| Extrusion and revolution, interface side | `app/src/screens/extrusion.rs` | `ExtrusionState` |
| The History panel | `app/src/screens/history_tree.rs` | `show(...)` → `HistoryAction` |
| The toolbar | `app/src/screens/ribbon.rs` | `Ribbon::show`, `is_enabled` |
| The settings screen | `app/src/screens/settings.rs` | `show(ui, profiles, editor)` |
| The start menu | `app/src/screens/start_menu.rs` | `show(...)` → `StartMenuAction` |
| What a command, its help and its family are called | `app/src/wording/command.rs` | `label`, `hint`, `family_heading` |
| What a history step and its unfolded line say | `app/src/wording/history.rs` | `label`, `detail` |
| What a dimension measures and spans | `app/src/wording/dimension.rs` | `label`, `spans` |
| What a profile is called | `app/src/wording/settings.rs` | `profile` |
| What a key and a chord are called | `app/src/wording/shortcuts.rs` | `chord` |
| What a toolbar placement and a tree entry are called | `app/src/wording/toolbar.rs` | `edge`, `item` |

## The invariants — do not break them

**The numbers.** The core — sketch, solver, solid, booleans — computes in
`f64`. The camera, the rendering and the interface are in `f32`, because that
is what the GPU and `egui` take. The conversion happens **at the last moment**,
at each crossing of the boundary. `f32` keeps only seven digits: a part one
metre long described in millimetres has a step of no better than 6·10⁻⁵ mm, and
the error accumulates in the booleans — that is what once sent the partition of
space into a loop.

**The geometry is never saved.** A `.caopart` holds the metadata and the
history of operations, nothing else. The geometry is rebuilt by
`PartState::rebuild`, which replays the operations. That is what makes undo,
redo and going back to a step one and the same operation. Any geometry kept
alongside would end up diverging from the history: there is **one single** place
where geometry is produced, `PartState::apply`.

**One mode = one variant of `Screen`.** A new mode (assembly, …) adds a variant
to `enum Screen` and its own module in `screens/`. Never a branch grafted onto
an existing module.

**`cao_part` depends on no UI crate.** That is the condition for a future
tablet or web front-end to reuse it as it is. No `egui`, no `eframe`, no
`winit`, no `wgpu`.

**`cao_app` is to stay a thin shell** — window and routing between modes. That
is an aim, not an observation: it is the largest crate in the repository. As
soon as a mode carries non-trivial business logic, it becomes its own crate.

## The places with no net

Three places have **no test at all**:

- `sketch/src/solver.rs` — the algorithmic heart, whose history is made of
  successive fixes (`git log -- crates/sketch/src/solver.rs`);
- `sketch/src/constraints.rs`;
- `crates/app/src/` — the files in `crates/app/tests/` test the repository (its
  shape, the agreement between the local gate and the CI, and the language it
  is written in), not the interface.

Working in them means first writing a test that characterises what is there.
See the `rust-tdd` skill.
