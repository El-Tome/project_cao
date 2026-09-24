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
| `cao_prefs` | theme, shortcuts, toolbar, profiles, recents | `serde`, `chrono` |
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
| Sketch model: points, traits | `sketch/src/sketch.rs` | `Sketch`, `live_points`, `live_segments` |
| A whole circle, as a centre and a size | `sketch/src/circle.rs` | `Circle`, `Sketch::add_circle`, `live_circles`, `nearest_circle` |
| A piece of a circle, and what keeps it round | `sketch/src/arc.rs` | `Arc`, `Sketch::add_arc`, `arc_sweep`, `arc_polyline`, `arc_equations` |
| Which arc the clicks gathered so far mean | `sketch/src/arc_placing.rs` | `arc_from`, `aimed`, `angle_reference`, `ArcMode` |
| The curve an arc is, and the steps it is drawn as | `sketch/src/arcing.rs` | `ArcDraft`, `sweep_of`, `places_along`, `steps_along`, `bounds_of` |
| An ellipse, laid with its two axes as construction traits, and the stretch of it a cut left | `sketch/src/ellipse.rs` | `Ellipse`, `Sketch::add_ellipse`, `ellipse_draft`, `ellipse_run`, `ellipse_ends`, `place_on_ellipse`, `erase_ellipse` |
| How far an ellipse's axes reach, laid across the curve or out from its centre | `sketch/src/ellipse/axes.rs` | `Sketch::ellipse_draft`, `axis_stands_on_the_centre` |
| The curve an ellipse is: where a turn lands, the nearest place, its box | `sketch/src/ellipsing.rs` | `EllipseDraft`, `through`, `at`, `nearest`, `bounds`, `places` |
| Which ellipse the clicks gathered so far mean, once what was typed has had its say, and which half of it a placement by two ends draws | `sketch/src/ellipse_placing.rs` | `EllipseMode`, `ellipse_aimed`, `ellipse_from`, `Rise`, `rise_of` |
| Where an ellipse crosses a trait, a circle, an arc or another ellipse | `sketch/src/crossing/ellipse.rs` | `where_segment_crosses_ellipse`, `where_circle_crosses_ellipse`, `where_arc_crosses_ellipse`, `where_ellipses_cross` |
| An ellipse, and the turns at which the drawing runs through it | `sketch/src/ellipse_edges.rs` | `Oval`, `Sketch::ovals` |
| Erasing an element and what leans on it | `sketch/src/sketch.rs` | `Sketch::erase` |
| Whether a rule still speaks of a drawing that has it | `sketch/src/sketch/holds_up.rs` | `Sketch::holds_up` |
| Taking a stretch out of a trait, and cutting one in two | `sketch/src/trimming.rs` | `Sketch::stretch_at`, `Sketch::trim` → `Trimmed` |
| What a cut of a **trait** carries over to a piece, and what it cannot | `sketch/src/trimming/carrying.rs` | `Sketch::carried_by`, `still_holds`, `still_measured`, `Piece` |
| Taking a stretch out of an arc | `sketch/src/trimming/arc.rs` | `Sketch::arc_stretch_at`, `Sketch::trim_arc` → `ArcTrimmed` |
| What a cut of an **arc** carries over — nothing that names an arc names a trait, so the two have no rule in common, and the reach is read on one piece with the other held to it | `sketch/src/trimming/arc_carrying.rs` | `still_holds`, `still_measured`, `Piece` |
| Taking a stretch out of a circle, which leaves one piece and that piece is an arc | `sketch/src/trimming/circle.rs` | `Sketch::circle_stretch_at`, `Sketch::trim_circle` → `CircleTrimmed` |
| Taking a stretch out of an ellipse, which leaves the same ellipse with that stretch gone | `sketch/src/trimming/ellipse.rs` | `Sketch::ellipse_stretch_at`, `Sketch::trim_ellipse` → `EllipseTrimmed` |
| What a cut would take out of the drawing, as against what a tool would lay | `sketch/src/trimming/going.rs` | `Sketch::trim_takes`, `arc_trim_takes`, `circle_trim_takes` → `Going`, `Stretch` |
| Dropping a point where curves cross and cutting each of them in two there | `sketch/src/splitting.rs` | `Sketch::crossing_at` → `Crossing`, `Sketch::split` → `Split` |
| Cutting the corner two traits share with a straight line | `sketch/src/chamfer.rs` | `Sketch::chamfer` → `Chamfered`, `Sketch::chamfer_fits`, `Chamfer`, `ChamferMode` |
| Rounding that same corner into a curve tangent to both sides | `sketch/src/fillet.rs` | `Sketch::fillet` → `Rounded`, `Sketch::fillet_fits` |
| Laying a second copy of part of the drawing down, under any transform | `sketch/src/duplicating.rs` | `Sketch::duplicate` → `Duplicated` |
| Running a change against a copy of the drawing, to show it before it is made | `sketch/src/preview.rs` | `Sketch::preview` → `Preview`, `Laid` |
| The straight line a click names, and the direction it runs in | `sketch/src/axis.rs` | `ChosenAxis`, `Sketch::axis_line` |
| Copying a selection across an axis | `sketch/src/mirroring.rs` | `Sketch::mirror` |
| Repeating a selection round a centre, or in rows | `sketch/src/patterning.rs` | `Sketch::pattern_around`, `Sketch::pattern_along`, `Repeats` |
| How wide a held selection stands, whichever way it is measured | `sketch/src/patterning/span.rs` | `Sketch::widest_span` |
| Placing or removing a constraint | `sketch/src/sketch.rs` | `add_constraint`, `erase_constraint` |
| Setting a value on the drawing, moving where it is written, taking it away | `sketch/src/sketch/dimensions.rs` | `Sketch::set_dimension`, `dimension_of`, `offset_dimension`, `nearest_dimension`, `erase_dimension` |
| What holds a point where it was laid, and what that still lets it do | `sketch/src/holding.rs` | `Support`, `Sketch::supports_at`, `supports_for`, `holds_on`, `slide`, `let_go` |
| What a rule holding a point asks of the solver, and which of the two gives | `sketch/src/solver/hold_solver.rs` | `hold_equations`, `held_alone`, `pulled_elsewhere` |
| What being an ellipse asks of the solver: axes square and halved by the centre | `sketch/src/solver/ellipse_solver.rs` | `ellipse_equations` |
| What a circle or an ellipse brushing a line asks of the solver | `sketch/src/solver/tangent_solver.rs` | `circle_tangent_equations`, `ellipse_tangent_equations` |
| A point dropped and the drawing settled around it; an axis end held about its ellipse's centre | `sketch/src/sketch/settling.rs` | `Sketch::settle_around`, `settle_around_all` |
| Kinds of constraint and dimension | `sketch/src/constraints.rs` | `Constraint`, `Dimension`, `DimensionTarget`, `Freedom` |
| What the constraint tool is pointed at, and what it means once shown enough | `sketch/src/rule_intent.rs` | `rule_intent`, `Rule`, `RuleIntent`, `RulePick` |
| Where a rule's mark is written, and the nearest one to a cursor | `sketch/src/rule_marks.rs` | `Sketch::rule_marks`, `Sketch::nearest_rule` |
| The solver | `sketch/src/solver.rs` | `solve(millimeters_per_unit)` → `SolveOutcome` |
| Laying a tangency with the point where the two touch, and taking both away | `sketch/src/sketch/tangency.rs` | `add_tangency`, `add_ellipse_tangency`, `laid_as_a_tangency`, `erased_as_a_tangency` |
| Whether a tangency's contact has slid off its segment | `sketch/src/sketch/tangency.rs` | `Sketch::has_a_flipped_tangent` |
| One equation of the system, linearised around the drawing's current shape | `sketch/src/equation.rs` | `Equation` |
| The blocks that keep their shape while the rest of the drawing settles | `sketch/src/rigid.rs` | `Block`, `rigidify`, `ownership` |
| What a set of equations holds, and what it leaves free | `sketch/src/independence.rs` | `rank`, `null_space`, `is_dependent` |
| How much of a drawing is already decided | `sketch/src/settled.rs` | `freedom`, `is_fully_constrained`, `settled_points` |
| A circle, an arc or an ellipse drawn to another size about its centre | `sketch/src/resizing.rs` | `Curved`, `Sketch::curve_at`, `reach_through`, `resize`, `resize_circle`, `resize_arc`, `resize_ellipse` |
| The five circle constructions, and the ways of drawing one | `sketch/src/construct.rs` | `centre_through`, `centre_touching_two`, `circle_touching_three`, `CircleMode` |
| Work plane, going 2D ↔ 3D | `sketch/src/plane.rs` | `WorkPlane::to_world`, `to_local`, `ray_intersection`, `kind`, `near_side` |
| Closed areas, to extrude | `sketch/src/regions.rs` | `Sketch::regions()` |
| How much surface an area holds and how far it is round, the curve honoured rather than the steps it was sampled into | `sketch/src/regions/measure.rs` | `Region::area`, `Region::perimeter`, `Outline::area`, `Outline::perimeter` |
| Naming an area by the curves that bound it, and finding it again | `sketch/src/naming.rs` | `CurveId`, `Area`, `Standing`, `Became`, `area_under` |
| Where two curves of the drawing cross | `sketch/src/crossing.rs` | `where_segments_cross`, `where_segment_crosses_arc`, `where_arcs_cross`, `where_segment_crosses_circle`, `where_arc_crosses_circle`, `where_circles_cross` |
| A circle, and the turns at which the drawing runs through it | `sketch/src/circle_edges.rs` | `Sketch::rounds`, `Round` |
| The drawing as half-edges a face walk can turn at, cut wherever two curves cross and wherever a point sits on one | `sketch/src/edges.rs` | `Sketch::crossed`, `Sketch::crossings`, `Crossed`, `ArcHalfEdge` |
| One end of a curved piece as the walk reads it: its departing tangent, how hard it bends, the places it draws | `sketch/src/edges/half_edge.rs` | `CurvedHalfEdge`, `Bend` |
| One curve of the drawing as that graph reads it: where it runs, how far along a place stands, the runs it is left as | `sketch/src/edges/curve.rs` | `Curve`, `between`, `pieces` |
| Which reading of a leaning trait the cursor asks for | `sketch/src/dimensioning.rs` | `Sketch::oriented`, `Sketch::is_slanted`, `Sketch::segment_touches`, `axis_under` |
| What pulls the cursor, and which magnet wins | `sketch/src/snap.rs` | `Sketch::magnetise`, `SnapSettings`, `Snap` |
| What a click takes hold of, and what a selection carries | `sketch/src/picking.rs` | `Sketch::pick`, `Sketch::points_of`, `Selection` |
| What a box dragged across the drawing catches | `sketch/src/banding.rs` | `Sketch::inside_band` |
| Where a dimension's annotation is drawn, and where its value belongs | `sketch/src/annotation.rs` | `Sketch::place`, `AnnotationMetrics`, `Placement` |
| The arc an angle is drawn as, and the arm it opens from when nothing else draws one | `sketch/src/annotation/angle.rs` | `angular`, `Arm` |
| Where a trait being drawn ends, and the four-degree square snap | `sketch/src/aim.rs` | `Sketch::aim`, `rectangle_corner`, `LockedInput`, `ChainAnchor` |
| One click of the line tool | `sketch/src/chain.rs` | `chain_click`, `ChainClick` |
| Which circle the clicks gathered so far mean | `sketch/src/circling.rs` | `circle_from`, `rim_of`, `Found` |
| What one click of the smart dimension tool measures | `sketch/src/measuring.rs` | `measure_pick`, `DimensionMode`, `DimensionPick` |
| What the measure tool reads off a target or off the area under a place, and the run it draws its triangle on | `sketch/src/reading.rs` | `Sketch::read`, `Sketch::read_inside`, `Sketch::run_of`, `Reading` |
| The dimensions a freshly-drawn rectangle or line earns on its own | `sketch/src/shape_dimensions.rs` | `rectangle_dimensions`, `line_dimensions` |
| What each tool remembers between one click and the next | `sketch/src/tool.rs` | `ToolState`, `SelectState` |

What it does: [`sketch.md`](sketch.md).

## Volumes — `cao_solid`

| What one is after | File | Way in |
| --- | --- | --- |
| Mesh, faces, ray casting | `solid/src/mesh.rs` | `Mesh`, `Polygon`, `ray_hit`, `bounds` |
| Extruding an area into a prism | `solid/src/mesh.rs` | `prism(...)` |
| Turning an area around an axis | `solid/src/mesh.rs` | `revolution(...)` |
| Adding or taking away matter | `solid/src/boolean.rs` | `Mesh::union`, `Mesh::difference` (BSP tree) |
| Keeping only what lies behind a plane, to look inside rather than to cut | `solid/src/clipping.rs` | `Mesh::behind` |
| Keeping only what lies behind a plane, for looking rather than for cutting | `solid/src/clipping.rs` | `Mesh::behind` |

What it does: [`extrusion.md`](extrusion.md).

## History and persistence — `cao_part`

| What one is after | File | Way in |
| --- | --- | --- |
| List of operations, undo, redo | `part/src/history.rs` | `History`, `Operation` |
| The major steps a design is grouped into | `part/src/history/step.rs` | `Step`, `StepKind` |
| Which sketch an operation edits, and so which step it is filed under | `part/src/history/operation/edits.rs` | `Operation::edits` |
| Where a step begins and ends in the list | `part/src/feature.rs` | `Feature::all` |
| Replaying the history for the geometry | `part/src/replay.rs`, `part/src/state.rs` | `PartState::rebuild`, `PartState::apply` → `Outcome` |
| What a replay notes: what each operation laid, what was raised, which values left the drawing | `part/src/replay.rs` | `Replay`, `Laid`, `SetBy`, `next_value_set` |
| The six ways a curve is replaced by other curves | `part/src/cutting.rs` | `PartState::trim`, `trim_arc`, `trim_circle`, `split`, `chamfer`, `fillet`, `PartState::area_rank` |
| A circle, an arc or an ellipse laid down again as the history replays it | `part/src/curves.rs` | `PartState::add_circle`, `add_arc`, `add_ellipse` |
| A point, a trait, a symmetric trait or a rectangle laid down again as the history replays it | `part/src/straight.rs` | `PartState::add_point`, `add_segment`, `add_symmetric_segment`, `add_rectangle` |
| What a drawing's curves became, so a name written before a cut can be read after it | `part/src/descent.rs` | `Descent::record`, `Descent::follow` |
| What a part does when a tool lays copies down | `part/src/copying.rs` | `PartState::mirror`, `PartState::pattern_around`, `PartState::pattern_along` |
| What an operation has to say for itself | `part/src/outcome.rs` | `Outcome` |
| What a typed value does to a part, and what it measures back | `part/src/dimensioning.rs` | `DimensionOutcome`, `PartState::measured`, `apply_written_dimension` |
| A size as the user wrote it: read, worked out, written back | `part/src/formula.rs`, `formula/reading.rs`, `formula/writing.rs` | `Formula::read`, `value`, `whole`, `written`, `stored` |
| The part's table of variables: names, loops, what a typed size comes to | `part/src/variables.rs` | `Variables`, `VariableChange`, `check_name`, `loop_through`, `size_of` |
| Where the changes to the variables sit in the history | `part/src/history/table.rs` | `History::variable_changes`, `table_operations` |
| The sizes a chamfer or a pattern was asked for, as written | `part/src/history/operation/sizes.rs` | `ChamferAsked`, `RepeatsAsked`, `Operation::sizes` |
| A size that does not hold once the part is rebuilt | `part/src/broken.rs` | `Broken`, `PartState::size`, `broken_since` |
| Which faces of the part a step of matter made | `part/src/extrusion.rs`, `part/src/document/matter.rs` | `PartState::raising`, `PartDocument::faces_made_by` |
| Changing the variables, and what is refused | `part/src/document/variables.rs` | `PartDocument::change_variable`, `Refused`, `Use`, `uses_of`, `formula_of` |
| The variables through a compaction | `part/src/compaction/variables.rs` | `compact_variables`, `Renumbered` |
| The `.caopart` file (zip) | `part/src/document.rs` | `PartDocument`, `SCHEMA_VERSION = 5` |
| The design folder: the index, one folder per step, and the line between them | `part/src/document/design.rs` | `laid_out`, `read`, `taken_apart`, `put_together` |
| The geometry a part is cached with | `part/src/document/geometry_cache.rs` | `write`, `read`, `GEOMETRY_ENTRY` |
| A part drawn to order, for a test or a measurement | `part/src/drawn_to_order.rs` | `Recipe`, `Recipe::drawn` |
| Writing one out, and timing what it costs | `part/examples/draw_a_part.rs` | `cargo run --release -p cao_part --features test-support --example draw_a_part -- /tmp/big.caopart sketches=6 storeys=4` |
| The picture a part carries of itself | `part/src/picture.rs` | `Picture` |
| Pulling that picture out without replaying | `part/src/document.rs` | `PartDocument::picture_in` |
| What fails when opening a part | `part/src/errors.rs` | `PartFileError` |
| What a part asks of a filesystem | `part/src/ports/files.rs` | `Files`, `FileError` |
| What browsing a folder of parts asks of it | `part/src/ports/folders.rs` | `Folders`, `Entry` |
| The tree of folders and parts under a root | `part/src/library/mod.rs` | `read`, `Folder`, `Part` |
| Making a folder, renaming, throwing away | `part/src/library/tidying.rs` | `create_folder`, `rename_folder`, `rename_part`, `discard` |
| A filesystem for tests | `part/src/adapters/in_memory_files.rs` | `InMemoryFiles`, behind `test-support` |
| The real filesystem, atomic writes | `app/src/adapters/files.rs` | `DiskFiles` |

## Settings, profiles and recents — `cao_prefs`

| What one is after | File | Way in |
| --- | --- | --- |
| The ten recent parts | `prefs/src/recents.rs` | `RecentList` |
| Where the platform keeps things | `prefs/src/locations.rs` | `Locations`, `default_projects_dir` |
| Asking the platform where that is | `app/src/adapters/locations.rs` | `discover` |
| Which clock a moment is read on | `app/src/adapters/clock.rs` | `reader_zone` |
| What the installation remembers, and where | `app/src/remembered.rs` | `Remembered` |
| The crash log | `app/src/crash.rs` | `record_panics` |
| What can go wrong with the settings | `prefs/src/storage.rs` | `StorageError` |
| What the preferences ask of a filesystem | `prefs/src/ports/files.rs` | `Files`, `FileError` |
| A filesystem for tests | `prefs/src/adapters/in_memory_files.rs` | `InMemoryFiles`, behind `test-support` |
| Commands of the interface | `prefs/src/command.rs` | `Command`, `CommandFamily` |
| What one profile is | `prefs/src/settings.rs` | `Settings`, `Profile`, `DEFAULT_PROFILE` — what it is *called* is in `app/src/wording/settings.rs` |
| The set of profiles, read and written | `prefs/src/profiles.rs` | `Profiles` |
| Viewport and navigation settings | `prefs/src/config.rs` | `ViewportConfig`, `Binding`, `NavigationPreset` |
| Colours and gradients | `prefs/src/theme.rs` | `Theme`, `Background`, `Rgba`, `Stop` |
| Keyboard shortcuts | `prefs/src/shortcuts.rs` | `Shortcuts`, `Chord`, `Key`, `Modifier`, `adopt_new_bindings` |
| Toolbar | `prefs/src/toolbar.rs` | `ToolbarLayout`, `Item`, `Edge`, `adopt_new_commands` |
| Which buttons a fresh installation shows | `prefs/src/toolbar/standard.rs` | `impl Default for ToolbarLayout` |

What they do: [`history.md`](history.md),
[`configuration.md`](configuration.md).

## GPU rendering — `cao_render`

| What one is after | File | Way in |
| --- | --- | --- |
| wgpu pipelines, render pass | `render/src/renderer.rs` | `SceneRenderer::prepare`, `paint` |
| Orbit camera, transitions | `render/src/camera.rs` | `OrbitCamera`, `ViewTransition` |
| Orientation cube | `render/src/cube.rs` | `push_faces`, `zone_at`, `is_visible` |
| Axes, grid, background, solids | `render/src/geometry.rs` | `push_axes`, `push_grid`, `push_background`, `push_solid` |
| Drawing into an image, with no window | `render/src/offscreen.rs` | `draw`, `Size` |
| Visual check without a window | `render/examples/offscreen.rs` | `cargo run -p cao_render --example offscreen -- /tmp` |

What it does: [`render.md`](render.md), [`viewport.md`](viewport.md).

## Interface — `cao_app`

| What one is after | File | Way in |
| --- | --- | --- |
| Application state, frame loop | `app/src/app.rs` | `CaoApp`, `impl eframe::App` |
| Routing between modes | `app/src/screens/mod.rs` | `enum Screen`, `struct OpenPart` |
| Canvas: what it knows between frames, and who a gesture is for | `app/src/screens/viewport/state.rs` | `ViewportState`, `ViewMode`, `ViewScale`, `gesture_goes_to` |
| Canvas: the frame loop and the entry point | `app/src/screens/viewport/view.rs` | `show(...)` |
| Canvas: gestures turned into calls on `cao_sketch` | `app/src/screens/viewport/input/mod.rs` | `pick`, `drag_point`, `constrain`, `aim`, `measure` |
| Canvas: gathering one frame of everything drawn | `app/src/screens/viewport/render.rs` | `build_frame` |
| Canvas: the grid and the drawing's axes, on the plane a sketch is open on | `app/src/screens/viewport/render/grid.rs` | `push` |
| Canvas: the drawing itself — contour, areas, points | `app/src/screens/viewport/render/drawing.rs` | `push_sketch`, `push_regions`, `what_would_be_laid` |
| Canvas: a point's square, a midpoint's triangle, a right angle's corner | `app/src/screens/viewport/render/marks.rs` | `push_point_markers`, `push_midpoint_mark`, `push_square_mark` |
| Canvas: what a click right now would lay down | `app/src/screens/viewport/render/preview.rs` | `push_preview`, `push_preview_line`, `pending_annotation` |
| Canvas: the planes and the face a sketch can be started on | `app/src/screens/viewport/render/planes.rs` | `push_choosable_planes`, `push_hovered_face` |
| Canvas: the areas an extrusion would turn into matter, and its axis | `app/src/screens/viewport/render/extrusion.rs` | `push_chosen_areas` |
| Canvas: what egui draws over the scene — cube labels, rule marks, band, scale bar | `app/src/screens/viewport/render/overlays.rs` | `paint_face_labels`, `paint_rule_marks`, `paint_band`, `paint_ruler` |
| Canvas: the value a dimension carries, and the field that edits it | `app/src/screens/viewport/render/dimensions.rs` | `paint_dimension_labels`, `paint_dimension_field` |
| Canvas: the values a shape is drawn to, typed beside the cursor | `app/src/screens/viewport/render/live_fields.rs` | `paint_live_input`, `live_field` |
| Canvas: a circle, an arc or a dashed line as straight steps | `app/src/screens/viewport/render/curves.rs` | `push_line`, `push_circle_at`, `push_arc_at` |
| Canvas: what the selection, the cursor or a rule already holds, drawn apart | `app/src/screens/viewport/render/emphasis.rs` | `mark`, `push_picked_axes` |
| Canvas: one click of the arc tool, and what it shows in between | `app/src/screens/viewport/input/arcs.rs` | `draw_arc`, `arc_preview` |
| Canvas: the arc tool's fields, its preview and the leg its angle opens from | `app/src/screens/viewport/render/arc.rs` | `live_fields`, `push_preview` |
| Canvas: one click of the ellipse tool, and the dimensions what was typed leaves on its axes | `app/src/screens/viewport/input/ellipses.rs` | `draw_ellipse`, `ellipse_preview` |
| Canvas: the ellipse tool's fields and its preview | `app/src/screens/viewport/render/ellipse.rs` | `live_fields`, `push_preview` |
| Canvas: one click of the circle tool, and the circle the picks so far make | `app/src/screens/viewport/input/circles.rs` | `draw_circle`, `circle_from` |
| Canvas: one click of the trim tool, and what it would take | `app/src/screens/viewport/input/trim.rs` | `trim`, `previewed` |
| Canvas: drawing the stretch a cut would take | `app/src/screens/viewport/render/trim.rs` | `what_would_go`, `push_going` |
| Canvas: one click of the division tool | `app/src/screens/viewport/input/split.rs` | `split` |
| Canvas: the two clicks of the chamfer and the fillet, and the values typed between them | `app/src/screens/viewport/input/corner.rs` | `corner`, `cut`, `corner_held` |
| Canvas: the cut or the curve a corner would take, shown as the value is typed | `app/src/screens/viewport/input/corner/preview.rs` | `previewed` |
| Canvas: what the mirror and the two patterns take hold of, and the axis, centre or direction they lay the copy against | `app/src/screens/viewport/input/copying.rs` | `copy`, `hold_is_done`, `axis_at`, `turned`, `filled` |
| Canvas: the copies a mirror or a pattern would lay, shown before the click that names where | `app/src/screens/viewport/input/copying/preview.rs` | `previewed` |
| Canvas: the values a pattern's fields open on | `app/src/screens/viewport/input/copying/opening.rs` | `fields_open_on` |
| Canvas: which closed areas an extrusion is offered, and which one a click takes | `app/src/screens/viewport/input/areas.rs` | `pick_areas` |
| Canvas: drawing a circle, an arc or an ellipse to another size by its curve | `app/src/screens/viewport/input/resizing.rs` | `grabbed_curve`, `drag_curve` |
| Canvas: what a drag takes hold of and moves | `app/src/screens/viewport/input/dragging.rs` | `drag_point`, `drag_group`, `drag_annotation`, `letting_go` |
| Canvas: what a point laid down by a tool lands on | `app/src/screens/viewport/input/landing.rs` | `landed_on`, `dropped_on`, `point_ref_at`, `born_at` |
| Canvas: the camera's own gestures — orbit, pan, wheel, trackpad | `app/src/screens/viewport/navigation.rs` | `handle_navigation`, `advance_transition`, `ScrollInput` |
| Canvas: what a box catches, and what deleting takes with it | `app/src/screens/viewport/input/selecting.rs` | `band_select`, `erase` |
| Canvas: one click of the smart dimension tool | `app/src/screens/viewport/input/measure.rs` | `measure`, `place_dimension`, `measure_preview` |
| Canvas: one click of the measure tool, which records nothing | `app/src/screens/viewport/input/reading.rs` | `read`, `showing` |
| Canvas: the dashed triangle a measure is drawn as, and a number on each side | `app/src/screens/viewport/render/reading.rs` | `push_measure`, `paint_measure` |
| Sketch tool, keyboard input | `app/src/screens/sketch.rs` | `SketchEditor`, `LiveInput` |
| A value typed into a dimension already on the drawing | `app/src/screens/sketch/typed_dimension.rs` | `apply_dimension_value` |
| Turning a dimension's shape into vertices, with a colour | `app/src/screens/annotations.rs` | `push(...)`, `Style` |
| Extrusion and revolution, UI side | `app/src/screens/extrusion.rs` | `ExtrusionState` |
| The panels beside a part, and what is asked in them | `app/src/panels.rs` | `beside_the_part` |
| Variables panel: the rows, what is typed into them, what was refused | `app/src/screens/variables/` | `state.rs` `VariablesPanel`, `Named`, `view.rs` `panel`, `mod.rs` `run` |
| What a refusal names blinks, and for how long | `app/src/screens/blinking.rs` | `lit`; `ViewportState::blink`, `VariablesPanel::blink` |
| Canvas: the values a shape earns, laid as typed; a click refused for a field that does not read | `app/src/screens/viewport/values.rs` | `lay_values`, `as_typed`, `refused_for_what_is_typed` |
| History panel | `app/src/screens/history_tree.rs` | `show(...)` → `HistoryAction` |
| Files panel: what it holds and what is half-done to it | `app/src/screens/explorer/state.rs` | `Explorer` |
| Files panel: the drawing of it | `app/src/screens/explorer/view.rs` | `panel(...)` → `ExplorerAction` |
| Starting a second window on another part | `app/src/adapters/window.rs` | `open_another` |
| What a part's picture is made of | `app/src/picture/scene.rs` | `of` |
| Taking that picture | `app/src/picture/mod.rs` | `Painter::take`, `SIDE` |
| Toolbar | `app/src/screens/ribbon/` | `view.rs` draws, `state.rs` says what a command is in, `Drawn` is what it is told about the part |
| Settings screen | `app/src/screens/settings/` | `show(...)`, one file per section |
| Start menu | `app/src/screens/start_menu.rs` | `show(...)` → `StartMenuAction` |
| What a key actually says, in French and in any language dropped in | `app/src/lang/` | `Catalogue::french`, `load`, `t`, `t_with`, `t_moment`, `fr.json` |
| What a command, its help and its family are called | `app/src/wording/command.rs` | `label`, `hint`, `family_heading` |
| What a history step and its unfolded line say | `app/src/wording/history/` | `label` in `mod.rs`, `detail` in `detail.rs` |
| What a dimension measures and spans | `app/src/wording/dimension.rs` | `label`, `spans` |
| A size written from variables, and what is wrong with a formula | `app/src/wording/formula.rs` | `sized`, `unreadable`, `unusable` |
| What the part says when it refuses a change to its variables | `app/src/wording/variables.rs` | `refused`, `name` |
| What a measure says, one number per side of its triangle | `app/src/wording/measure.rs` | `says`, `Said` |
| What an operation just did, said to the user | `app/src/wording/outcome.rs` | `message` |
| What a rule of the drawing is called and marked | `app/src/wording/constraints.rs` | `label`, `mark`, `axis` |
| What a way of drawing a circle asks for | `app/src/wording/circle.rs` | `asks_for` |
| What a way of drawing an arc asks for | `app/src/wording/arc.rs` | `asks_for` |
| What a work plane is called | `app/src/wording/plane.rs` | `label` |
| What a face of the orientation cube is called | `app/src/wording/cube.rs` | `face` |
| What a profile is called | `app/src/wording/settings.rs` | `profile` |
| What a key and a chord are called | `app/src/wording/shortcuts.rs` | `chord` |
| What a toolbar placement and a tree entry are called | `app/src/wording/toolbar.rs` | `edge`, `item` |
| What went wrong with a part file | `app/src/wording/part_file.rs` | `say` |
| What went wrong with the settings | `app/src/wording/storage.rs` | `say` |
| The three ways the disk can refuse | `app/src/wording/file.rs` | `absent`, `refused`, `interrupted` |

What they do: [`interface.md`](interface.md),
[`navigation.md`](navigation.md).

`app/src/wording/` decides which key a case from a lower crate earns, one file
per source; what that key says lives in `app/src/lang/fr.json`. A sentence a
screen writes itself — a heading, a prompt, a button — names its key on the
spot instead, since it translates no case of anything. No source below
`cao_app` says its own sentences, and none inside it writes one out either:
`crates/app/tests/architecture.rs` holds both counts at zero, and the first
file to write a sentence back in fails the gate.

A key names a sentence but does not carry it, and `Catalogue::t` answers with
the key itself when `lang/fr.json` has no entry for it — so a typo would show
on screen rather than at build time. `crates/app/tests/language_keys.rs` closes
that: a key named in Rust with no entry fails the gate, and so does an entry no
Rust file names, which is what a rename leaves behind.

A sentence still has to be drawable. The fonts `egui` ships with carry no glyph
for `⌂`, `✕` or the four plain arrows, and what they draw instead is an empty
box — five buttons in a row once read as the same square.
`crates/app/tests/glyphs.rs` asks those fonts, for every entry of the language
file and every literal of `cao_app`, so a mark nobody can draw cannot reach a
screen.

## The invariants

**The numbers.** The core computes in `f64`, the camera and the rendering in
`f32`, and the conversion happens at each crossing of the boundary. The
reasoning is in [`ARCHITECTURE.md`](ARCHITECTURE.md), in one copy.

**The geometry is never the truth in the file.** A `.caopart` holds its
metadata and its history of operations; the geometry beside them is a cache
that names the design it was rebuilt from, and is dropped for a replay as soon
as it stops answering to it. The geometry is rebuilt by `PartState::rebuild`,
which replays the operations — that is what makes undo, redo and going back to
a step one and the same operation. There is **one single** place where geometry
is produced: `PartState::apply`. A copy that could be read without saying what
it came from would end up diverging.

**One mode = one variant of `Screen`.** A new mode adds a variant to
`enum Screen` and its own module in `screens/`, never a branch grafted onto an
existing module.

**`cao_part` depends on no UI crate.** That is the condition for a future
tablet or web front-end to reuse it as it is.

**`cao_app` is to stay a thin shell.** That is an aim, not an observation: it is
the largest crate in the repository, and `viewport.rs` alone holds the camera,
the hit test, the keyboard, the gestures and the drawing of the annotations. As
soon as a mode carries non-trivial business logic, it becomes its own crate.

## What has no net

These places carry no test of their own. **Carrying none is not the same as
being unreachable**: since #377 the application is driven with no window at all
— `crates/app/tests/driver/` opens `CaoApp` through `egui_kittest`, clicks a
tool by the label the user reads, draws in the canvas by coordinate, and reads
the answer back out of the accessibility tree, with no GPU and in a fifth of a
second. `crates/app/tests/drawing.rs` is what it looks like. A file leaves the
list below by earning a test of its own, not by being walked through from above
— but nothing here is out of reach any more.

**That last sentence was put as a question, and answered in #387.** Counting a
driver run as a net was weighed and refused: no text proves which files a run
touched, so the link would be asserted by hand, and a ratchet asserted by hand
drifts. Splitting the list in two — no net at all against reached only from
above — was refused at the same price: a second list to keep, a second constant
in `crates/app/tests/architecture.rs`, and a rule with two tiers is one people
misremember. The list is not a coverage report and was never meant to be one:
it says a change here is caught by nothing **local**, and that stays true of
`start_menu.rs` with the driver merged. A driver test notices that the part
list lost a name; it does not notice that the recents came back in the wrong
order.

The way off the list is open, and it is the only one. A test beside the module
it covers, in its own file as #362 asks, reaches the dev-dependencies like any
other test in the crate — so a screen that wants a net can have one where it
lives.

- `crates/sketch/src/solver.rs` — the algorithmic heart, most of whose history
  is made of successive fixes (`git log -- crates/sketch/src/solver.rs`);
- the canvas and the modes drawn on it — gesture dispatch, pixel ↔ world
  conversion, and pushing the result to egui and the GPU. The drawing rules
  they call into — hit test, magnetism, dimensioning — moved to `cao_sketch`,
  where each is tested without opening a window; what is left is glue. The
  toolbar came out of this list when it was split into a presenter and a view,
  which is the move each of these is waiting for. Since #386 the canvas has
  made it: `state.rs` holds what it knows between frames and `view.rs` the
  frame loop, and the rule saying who a gesture is for — the cube, picking an
  area, or the tool in hand — is read by four tests with no window. What is
  left in `view.rs` is the loop itself.
  `crates/app/src/screens/viewport/cube_labels.rs` came out of it already:
  fitting a face's label to its own projected shape is pure geometry, once the
  projecting and the measuring are done, and that part is tested without a
  window.
  What a painter under `render/` pushes is a `Vec<cao_render::Vertex>`, two
  vertices to a straight step, and those steps read back onto the sketch's
  plane say what was drawn with no window and no GPU. `arc.rs` was the first to
  earn a test that way; since #385 so have `grid.rs`, `marks.rs`, `preview.rs`,
  `planes.rs`, `extrusion.rs`, `drawing.rs` and `overlays.rs` — the last of
  which draws with `egui` rather than pushing vertices, and is read by asking
  `egui` for one pass with no window and looking at the shapes it hands back.
  `circle.rs`, `curves.rs`, `dimensions.rs`, `live_fields.rs` and
  `symmetric_line.rs` still carry none and go the same way whenever someone
  writes them. `render.rs`, which now does nothing but gather the frame, left
  this list the same way: three tests read a built `SceneFrame` back, and one
  of them is the rule that no grid is drawn while the view is still swinging
  onto a plane. `sketch.rs` left it for the same reason: `Tool` now answers
  whether pointing it at something lights the whole of it, and that answer is
  asserted.
  - `crates/app/src/screens/viewport/view.rs`;
  - `crates/app/src/screens/viewport/navigation.rs`, which came out of it and
    carries the same glue: a gesture read off `egui` and handed to the camera;
  - `crates/app/src/screens/viewport/input/mod.rs`;
  - `crates/app/src/screens/viewport/input/arcs.rs`;
  - `crates/app/src/screens/viewport/input/circles.rs`;
  - `crates/app/src/screens/viewport/input/constrain.rs`;
  - `crates/app/src/screens/viewport/input/rectangle.rs`;
  - `crates/app/src/screens/viewport/input/resizing.rs`;
  - `crates/app/src/screens/viewport/input/symmetric_line.rs`;
  - `crates/app/src/screens/settings/`;
  - `crates/app/src/screens/extrusion_row.rs`;
  - `crates/app/src/screens/history_tree.rs`;
  - `crates/app/src/screens/annotations.rs`;
  - `crates/app/src/screens/start_menu.rs`;
  - `crates/app/src/screens/mod.rs`.

**This list is the only copy.** The skills that warn about these places name
this section rather than restating it, and `crates/app/tests/architecture.rs`
holds it against the code: the day one of them grows a `#[test]`, the test
fails and the line comes out. Keeping the same sentence in four places is how
the previous version of it went on claiming the whole of `crates/app/` was
uncovered, for a day after it had stopped being true.

The rest of `crates/app/src/` is covered: the French the interface says is
held word by word in `wording/`, and `adapters/`, `autosave.rs` and `crash.rs`
carry their own tests — `adapters/clock.rs` apart, which asks the machine which
zone it is set to and could assert nothing but the machine's own answer handed
back to it. What that zone then does to an hour is held in `lang/`, against a
zone the test names itself. The files in `crates/app/tests/` are about the
repository rather than the interface — `architecture.rs` its shape,
`gate.rs` the agreement between `scripts/verify.sh` and
`.github/workflows/ci.yml`, `language.rs` that nothing a developer reads is
written in French, `language_keys.rs` that a key and an entry name each other,
and `glyphs.rs` that every mark shown is one the fonts can draw.

Working in them means first writing a test that characterises what is there —
the `rust-tdd` skill says how. The linear algebra the solver rests on is the
exception: `independence.rs` is covered, so a claim about how much of a drawing
is held down can be checked without opening a window.

Elsewhere the repository is tested, and each test lives in the file it covers.
`cargo test --workspace` gives the count of the day.

## Checking

```sh
scripts/verify.sh            # fmt --check, clippy -D warnings, cargo test --workspace, ~12 s
cargo test -p cao_sketch     # one crate alone, while iterating
cargo run -p cao_app         # launch the application
```
