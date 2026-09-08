# CAO

A 3D design program (in the manner of SolidWorks / Fusion 360) written entirely
in Rust, meant to be modular and cross-platform from the start.

## Where it stands

- **Start menu**: create a new part, or reopen one of the last 10 parts opened.
- **3D viewport**: a 3D space with the X/Y/Z axes, an orientation cube whose
  faces, edges and corners are clickable, an adaptive grid when settled on a
  plane, and a scale bar in mm. Mouse and trackpad navigation. See
  [`docs/viewport.md`](docs/viewport.md).

- **Sketch**: pick a plane, then draw lines, rectangles, circles and points,
  and dimension lengths, radii and angles. A trait is drawn to the length and
  angle wanted, typed beside the cursor, and dimensions itself; right angles
  place themselves; anything erases with `Suppr`. The drawing is coloured by
  how much freedom it has left. The first dimension defines the scale, the
  following ones deform the geometry. See [`docs/sketch.md`](docs/sketch.md).

- **History**: every gesture is a recorded operation. Undo (`Ctrl+Z`), redo,
  and a direct return to any step from the History panel. See
  [`docs/history.md`](docs/history.md).

- **Extrusion**: once a sketch is finished, choose one or several closed areas
  and give them a height — or an angle and an axis, for a revolution — adding
  or taking away matter. Two circles one inside the other give a tube, not a
  rod. The flat faces of the part then serve as sketch planes. See
  [`docs/extrusion.md`](docs/extrusion.md).

- **Settings**: a preferences screen for the viewport, navigation, the colours
  and the background (gradients included), the keyboard shortcuts and the
  arrangement of the toolbar. All of it in named profiles, kept between
  sessions, reset at the press of a button and shareable as a file. See
  [`docs/configuration.md`](docs/configuration.md).

Assembly is still to come — see
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the whole picture and the
road ahead.

## Running the application

```sh
cargo run -p cao_app
```

## The shape of the workspace

- `crates/part` (`cao_part`) — the application layer: it orchestrates sketch,
  solid, history and persistence. It is not the domain, whatever its name
  suggests. With no UI dependency at all, so reusable as it is by a future
  web or tablet front-end.
- `crates/prefs` (`cao_prefs`) — theme, shortcuts, toolbar, profiles, recent
  files, with no geometry and no UI.
- `crates/sketch` (`cao_sketch`) — the sketch model and the application of
  dimensions, with no rendering and no UI.
- `crates/solid` (`cao_solid`) — volumes, extrusion and boolean operations,
  with no rendering and no UI.
- `crates/render` (`cao_render`) — GPU rendering of the viewport (wgpu), with
  no UI dependency either.
- `crates/app` (`cao_app`) — the desktop interface (egui/eframe): start menu
  and viewport.

## A Windows executable

```sh
./scripts/build-windows.sh
```

Produces a standalone `.exe` from macOS or Linux — see
[`docs/build.md`](docs/build.md).

## Documentation

- [Architecture](docs/ARCHITECTURE.md) — vision, the split, the road ahead
- [Contexts](docs/contexts.md) — where the seams are, and where they are going
- [Where a file goes](docs/code-layout.md) — the folders, and what they import
- [Code map](docs/code-map.md) — which file carries which behaviour
- [Glossary](docs/glossary.md) — the words, and what they mean here
- [Sketch](docs/sketch.md) — drawing, dimensioning, the rule of scale
- [History](docs/history.md) — operations, undo, the `.caopart` format
- [Interface](docs/interface.md) — the toolbar and the panels
- [Viewport](docs/viewport.md) — the two modes of the canvas, the grid, the cube
- [Rendering](docs/render.md) — wgpu pipelines, thick lines, offscreen rendering
- [Navigation](docs/navigation.md) — mouse gestures, camera
- [Configuration](docs/configuration.md) — what can be set
- [Building](docs/build.md) — Windows executable, other platforms

## Tests

```sh
cargo test --workspace
cargo run -p cao_render --example offscreen -- /tmp   # writes 3 PNGs to check
```

`scripts/verify.sh` chains `cargo fmt --all --check`, `clippy -D warnings` then
`cargo test --workspace`; both local hooks call it before every commit.
`crates/app/tests/architecture.rs` checks the architecture rules there — the
crate graph, the folders, the budget of 400 lines per file — and
`crates/app/tests/gate.rs` that the CI checks the same things it does.

## Licence

Dual-licensed MIT / Apache-2.0, see [`LICENSE-MIT`](LICENSE-MIT) and
[`LICENSE-APACHE`](LICENSE-APACHE). An additional professional offering is
envisaged in time (undefined for now).
