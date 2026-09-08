# Architecture

## Vision

A 3D CAD tool (of the SolidWorks / Fusion 360 kind), 100 % Rust, open source,
designed to be **modular**: every large feature (start menu, sketch/extrusion,
assembly, future modes…) is an independent module, and most behaviours should
stay configurable rather than hard-coded.

Platforms aimed at, in order:

1. Desktop: Windows, Linux, macOS.
2. Tablet / iPad, with stylus support to sketch quickly by hand and then move
   to 3D.
3. Phone, as a bonus, with no promise of real use.

## Why egui/eframe

`egui` is pure Rust, built on `wgpu` — the same graphics base that carries the
3D viewport, with no bridge to another language. It compiles natively on the
three desktop OSes and to WASM, which opens the way to a tablet or web port
without rewriting the interface. It is a pragmatic choice for the V1; it may be
questioned again if the touch and stylus needs (the Apple Pencil mode in
particular) turn out to be too constrained by this framework.

## The split into crates

The dependencies go one way only. Not every crate is one context, though — the
seam that is still missing is named in [contexts.md](contexts.md).

- `cao_core`: the application layer. Depends on `cao_sketch` and `cao_solid`,
  and orchestrates sketch, solid, history and persistence — it is not the
  domain, whatever its name suggests. **No UI dependency at all**, so as to stay
  reusable as it is by any future front-end (desktop, web, tablet). It is to
  become `cao_part`: see [contexts.md](contexts.md).
- `cao_prefs`: theme, shortcuts, toolbar, profiles, recent files, and the
  persistence of all of it. Knows neither the geometry nor the interface. See
  [configuration.md](configuration.md).
- `cao_sketch`: the sketch model (work plane, points, traits, dimensions) and
  the rule that applies a length. No rendering, no interface. See
  [sketch.md](sketch.md).
- `cao_solid`: the volumes — a polygon mesh, the extrusion of an area into a
  prism, boolean operations (adding and taking away matter). No rendering, no
  interface. See [extrusion.md](extrusion.md).
- `cao_render`: GPU rendering of the viewport (`wgpu`), with no interface
  dependency. See [render.md](render.md).
- `cao_app`: the desktop application shell (`eframe`). Holds the state of the
  application and the routing between screens and modes.

As the modes (sketch, extrusion, assembly…) grow, they are to become their own
crates (`cao_sketch`, `cao_assembly`, …) rather than piling up in `cao_app`,
which must stay a thin shell: window, routing between modes, nothing more.

Inside a crate, the role of a file is carried by its folder — `model/`,
`ports/`, `adapters/`, `services/`, and in the shell `ui/` and
`screens/<mode>/`. What each of them means, what it may import, and the budget
of 400 lines per file: [code-layout.md](code-layout.md).
`crates/app/tests/architecture.rs` checks it, and `scripts/verify.sh` — format,
clippy then `cargo test --workspace`, called by both local hooks before every
commit — refuses the commit that breaks it.

## The mode system

The application is a start menu that switches to different modes:

- **Sketch → Extrusion**: the 2D sketch then extrusion cycle, repeatable in a
  loop to build a part. Both exist: [sketch.md](sketch.md),
  [extrusion.md](extrusion.md).
- **Assembly**: assembling several parts together (not implemented yet).
- Other modes will join the menu over time.

Today `crates/app/src/screens/mod.rs` defines a `Screen` enum with two
variants: the start menu, and the open part, which shows the 3D viewport (axes,
grid, orientation cube — see [viewport.md](viewport.md)). Every new mode is to
add a variant to that enum and its own module in `screens/`, never a branch
added to an existing module.

## Documentation by subject

- [contexts.md](contexts.md) — where the seams are, and where they are going
- [code-layout.md](code-layout.md) — where a new file goes, what it may import
- [code-map.md](code-map.md) — which file carries which behaviour
- [glossary.md](glossary.md) — the words, and what they mean here
- [sketch.md](sketch.md) — drawing, dimensioning, and the rule of scale
- [history.md](history.md) — operations, undo, the file format
- [interface.md](interface.md) — the toolbar and the panels
- [viewport.md](viewport.md) — the two modes of the canvas, the grid, the cube
- [render.md](render.md) — the `cao_render` crate, wgpu pipelines, thick lines
- [navigation.md](navigation.md) — mouse gestures, how the camera behaves
- [configuration.md](configuration.md) — what can be set, and what cannot yet
- [build.md](build.md) — building, the Windows executable

## The file format

A part is a **zip archive** (`.caopart`) holding its metadata and its history
of operations. The geometry is not saved: it is rebuilt by replaying the
history, which makes undo, redo and going back to a step one and the same
operation. Files written in the previous format (a single JSON) are not read:
the tool has changed too much for a conversion to be trustworthy, and nothing
precious was drawn with those versions. See [history.md](history.md).

## Not a priority (to be argued later)

- **Collaborative work**: locking a part to one user at a time, versus several
  editing at once. A choice to be made when the need becomes concrete; the
  network and sync architecture is not to be anticipated before that.
- **A professional licence**: a commercial offering on top of the dual
  MIT/Apache-2.0 licence, terms undefined.

## The numbers

The core — sketch, solver, solid, booleans — computes in **`f64`**. The camera,
the rendering and the interface stay in `f32`, which is what the GPU and egui
take, and the conversion happens at the last moment, at each crossing of the
boundary.

`f32` keeps about seven digits: a part one metre long described in millimetres
already has a step of no better than 6·10⁻⁵ mm, and the error accumulates in
the booleans — that is what once sent the partition of space into a loop
([extrusion.md](extrusion.md)). `f64` keeps sixteen.

What that does not give is **exactness**. 0.1 mm is still a number binary
cannot write, and two different paths of computation can still give two results
a hair apart. Answering that would mean storing the values typed as integers
(the picometre as the unit, the micro-degree for angles) at the moment they
enter the history, while going on computing in `f64`. It is not done.
