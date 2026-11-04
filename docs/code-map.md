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
| `cao_solid` | the matter as a `Body`: extrusion, booleans, and the mesh it is made of | `glam`, `serde` |
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
| An ellipse, the stretch of it the cuts on its ends leave, and the turns at which the drawing runs through it | `sketch/src/ellipse_edges.rs` | `Oval`, `Sketch::ovals` |
| Erasing an element and what leans on it | `sketch/src/sketch.rs` | `Sketch::erase` |
| Two points made one — a drop, « Coïncidence », « Concentrique » — and everything that named the one that goes handed to the one that stays | `sketch/src/sketch/merging.rs` | `Sketch::join_points`, `merge_points` |
| Whether a rule still speaks of a drawing that has it | `sketch/src/sketch/holds_up.rs` | `Sketch::holds_up` |
| Taking a stretch out of a trait, and cutting one in two | `sketch/src/trimming.rs` | `Sketch::stretch_at`, `Sketch::trim` → `Trimmed` |
| What a cut of a **trait** carries over to a piece, and what it cannot | `sketch/src/trimming/carrying.rs` | `Sketch::carried_by`, `still_holds`, `still_measured`, `Piece` |
| Taking a stretch out of an arc, a cut on it handed to the piece left at its corner | `sketch/src/trimming/arc.rs` | `Sketch::arc_stretch_at`, `Sketch::trim_arc` → `ArcTrimmed` |
| What a cut of an **arc** carries over — nothing that names an arc names a trait, so the two have no rule in common, and the reach is read on one piece with the other held to it | `sketch/src/trimming/arc_carrying.rs` | `still_holds`, `still_measured`, `Piece` |
| Taking a stretch out of a circle, which leaves one piece and that piece is an arc | `sketch/src/trimming/circle.rs` | `Sketch::circle_stretch_at`, `Sketch::trim_circle` → `CircleTrimmed` |
| Taking a stretch out of an ellipse, which leaves the same ellipse with that stretch gone | `sketch/src/trimming/ellipse.rs` | `Sketch::ellipse_stretch_at`, `Sketch::trim_ellipse` → `EllipseTrimmed` |
| What a cut would take out of the drawing, as against what a tool would lay | `sketch/src/trimming/going.rs` | `Sketch::trim_takes`, `arc_trim_takes`, `circle_trim_takes` → `Going`, `Stretch` |
| Dropping a point where curves cross and cutting each of them in two there | `sketch/src/splitting.rs` | `Sketch::crossing_at` → `Crossing`, `Sketch::split` → `Split` |
| The ways a chamfer says what it takes off a corner: one distance, two, or a distance and an angle | `sketch/src/chamfer.rs` | `Chamfer`, `ChamferMode` |
| How a corner is named — its point, its two sides, or both — and which sides meet at a point: `sides_ending_at` is the one answer, read by `sides_of`, the tool's crowd test and `other_side_at` | `sketch/src/corner.rs` | `Corner`, `Sketch::sides_ending_at`, `Sketch::sides_of`, `Sketch::corners_between`, `Sketch::corner_point`, `Sketch::other_side_at` |
| Laying a fillet or a chamfer as a cut on the sharp corner two sides share — live cuts, groups, a corner already cut, a cut handed to the piece a trim or a division leaves at the corner, and taken away for good with its corner or a side | `sketch/src/corner_cut.rs` | `CornerCut`, `CornerCutId`, `CutShape`, `CutRefused`, `Sketch::cut_corner`, `Sketch::cut_corner_at`, `Sketch::corner_named`, `Sketch::live_corner_cuts`, `Sketch::cuts_standing_at`, `Sketch::cut_standing_between`, `Sketch::cuts_grouped_with`, `Sketch::fresh_cut_group`, `cuts_follow_pieces`, `cuts_lose_what_is_erased` |
| What a side of a cut corner is — a trait, an arc or an arc of ellipse — its ends, and the curve it names | `sketch/src/corner_cut/side.rs` | `CutSide`, `CutSide::as_trait`, `both_traits`, `side_ends`, `far_end_of`, `is_erased_side` |
| A fillet on a corner with a curve for a side — an arc or an arc of ellipse: its centre a radius off each side towards the angle under half a turn, found by Newton's method and followed out of the corner, its radius doubled from one small enough for the tangents alone to place, how much of each side it takes, in the side's own measure — drawn as not fitting when no such fillet stands — and the largest value a gesture with such a corner can lay, found by halving on a copy of the drawing | `sketch/src/corner_cut/curved.rs` | `worked_beside_a_curve`, `fillet_between`, `share_by_halving` |
| The line or the curve such a side lies on, read in plain numbers for the search and over jets for the solver; and the fillet's places with how they move, from one more step of the search taken over jets | `sketch/src/corner_cut/curved/path.rs`, `sketch/src/corner_cut/curved/places.rs` | `Path`, `Run`, `JetPath`, `places_beside_a_curve` |
| What shows of a side once the cuts on its ends have taken their part — a trait, an arc or the stretch of an ellipse — as its two ends the way it runs; a curve side split into the stretch that shows and the stretches the cuts hide, each as where it starts and how far round it runs | `sketch/src/corner_cut/showing.rs` | `what_shows`, `Stretches`, `Sketch::arc_stretches`, `Sketch::ellipse_stretches` |
| What a cut draws, worked out from its corner every time it is read: where it meets each side, whether it fits with the cuts at the other ends of its sides, what is left to draw of a side, and the largest share of a value that fits | `sketch/src/corner_cut/drawn.rs` | `DrawnCut`, `CutCurve`, `PulledEnds`, `Worked`, `Sketch::drawn_cuts`, `Sketch::pulled_ends`, `Sketch::visible_part`, `Sketch::visible_ends`, `Sketch::fitting_share_at` |
| The side of a corner a click names — a trait, an arc or an arc of ellipse; taking a cut by its curve, dragging its corner from afar (`Grip`), the places it offers to snap to, and what « Mesurer » reads of it | `sketch/src/corner_cut/picking.rs` | `Sketch::nearest_side`, `Sketch::nearest_cut`, `Sketch::point_to_drag_at`, `nearest_cut_place`, `places_on_cuts`, `Sketch::cut_before_traits`, `Sketch::read_cut`, `Sketch::cut_run`, `Sketch::cut_middle` |
| A cut changed in place: a group's new shape, a cut taken off, a chamfer's sides swapped, one cut switched to construction | `sketch/src/corner_cut/changing.rs` | `Sketch::reshape_cuts`, `Sketch::take_cut_off`, `Sketch::swap_cut_sides`, `Sketch::set_cut_construction`, `Sketch::write_cuts_as`, `Sketch::cut_written` |
| The points a cut offers to hang something on — a fillet's centre, where a cut meets each side — where they stand, the nearest to a cursor, and each worked out again with how it moves with what the cut stands on: the corner, the far ends of the sides, and what a curve side is drawn about | `sketch/src/corner_cut/points.rs` | `CutPoint`, `Sketch::cut_point`, `Sketch::cut_points`, `Sketch::nearest_cut_point`, `cut_places` → `CutPlaces` |
| What hangs on a cut through what the cut goes through: taken off — an end held on its side if it lies on it, let go if not — given another shape, its sides swapped, cut again, its side trimmed or divided, a trait or an arc, or an arc of ellipse trimmed; and what a point laid on a cut lands on | `sketch/src/corner_cut/hanging.rs` | `cut_supports_at`, `onto_cut`, `let_go_of_the_cut`, `keep_what_the_shape_offers`, `hand_hung_over`, `rules_hung_on`, `hang_again` |
| The rows a point hung on a cut's point, or held on its curve, asks of the solver | `sketch/src/solver/hold_solver/on_a_cut.rs` | `cut_hold_equations` |
| A number carrying its derivative along each coordinate it was worked out from, for a place the solver reads with its gradient | `sketch/src/jet.rs` | `Jet`, `JetPlace` |
| The traits, the cuts and the arcs as the face graph reads them: each trait and arc from where its cuts leave it, the curve of every cut that fits and is not construction, named `CurveId::Corner` | `sketch/src/edges/drawn.rs` | `sides_and_cuts`, `Sketch::name_the_sides_cuts_took` |
| Laying a second copy of part of the drawing down, under any transform, its corner cuts with it whatever their sides | `sketch/src/duplicating.rs` | `Sketch::duplicate` → `Duplicated` |
| Running a change against a copy of the drawing, to show it before it is made | `sketch/src/preview.rs` | `Sketch::preview` → `Preview`, `Laid` |
| The straight line a click names, and the direction it runs in | `sketch/src/axis.rs` | `ChosenAxis`, `Sketch::axis_line` |
| Copying a selection across an axis | `sketch/src/mirroring.rs` | `Sketch::mirror` |
| Repeating a selection round a centre, or in rows | `sketch/src/patterning.rs` | `Sketch::pattern_around`, `Sketch::pattern_along`, `Repeats` |
| How wide a held selection stands, whichever way it is measured | `sketch/src/patterning/span.rs` | `Sketch::widest_span` |
| Placing or removing a constraint | `sketch/src/sketch.rs` | `add_constraint`, `erase_constraint` |
| A rule or a value landing, the first thing clicked held while it does | `sketch/src/sketch/settling/landing.rs` | `Sketch::lay_rule`, `land_value` |
| The shape an angle typed or a rule of direction turns, turned whole before the solver sees it, and what it turns about | `sketch/src/sketch/settling/turned_whole.rs` | `turned`, `turn_whole`, `land_turned`, `Landing`, `Turned` |
| A point and a trait, or a curve, under « Coïncidence », in the order of the clicks: the second placed each way it can come, the first held | `sketch/src/sketch/settling/point_on_trait.rs` | `meet_in_order`, `Placing` |
| A point and a curve: where the point lands on what is drawn, clear of the ends, and the curve brought whole to a point clicked first, lengthened past it | `sketch/src/sketch/settling/point_on_curve.rs` | `landing_on`, `bring_curve`, `CLEAR_OF_AN_END` |
| Fixed points giving when only they stand in the way of a value or a rule | `sketch/src/sketch/settling/fixed_gives.rs` | `landing_or_giving`, `what_fixed_can_give` |
| The order a pair was clicked in, kept and compared without | `sketch/src/laid_from.rs` | `LaidFrom`, `normalised`, `Sketch::carries` |
| Setting a value on the drawing, moving where it is written, taking it away | `sketch/src/sketch/dimensions.rs` | `Sketch::set_dimension`, `dimension_of`, `offset_dimension`, `nearest_dimension`, `erase_dimension` |
| What holds a point where it was laid, and what that still lets it do | `sketch/src/holding.rs` | `Support`, `Sketch::supports_at`, `supports_for`, `holds_on`, `slide`, `let_go` |
| What a rule holding a point asks of the solver, and which of the two gives | `sketch/src/solver/hold_solver.rs` | `hold_equations`, `held_alone`, `pulled_elsewhere` |
| What being an ellipse asks of the solver: axes square and halved by the centre | `sketch/src/solver/ellipse_solver.rs` | `ellipse_equations` |
| What a circle or an ellipse brushing a line asks of the solver | `sketch/src/solver/tangent_solver.rs` | `circle_tangent_equations`, `ellipse_tangent_equations` |
| A point dropped and the drawing settled around it; a handful dropped at once | `sketch/src/sketch/settling.rs` | `Sketch::settle_around`, `settle_around_all` |
| What a drag of one point may do: stretch, follow one way, pivot, or nothing | `sketch/src/sketch/settling/pull.rs` | `PointPull`, `Sketch::pull`, `lay_pulled` |
| One frame of a drag laid down, and how far it may throw a point, the dragged one included | `sketch/src/sketch/settling/pull/throw.rs` | `Sketch::settle_pulled`, `throws_a_point`, `Throw` |
| Where a pulled point is taken at each frame, and what it lands on | `sketch/src/sketch/settling/pull/landing.rs` | `PointPull::landing`, `onto_grid`, `arrived`, `joined_to`, `joins` |
| The shape a dragged point belongs to, and the point of it that stays | `sketch/src/sketch/settling/shape.rs` | `shape_of`, `stay_point`, `centres_under`, `turned_about` |
| The pairs of points the drawing joins, and how a rule ties them | `sketch/src/sketch/settling/joins.rs` | `joined_pairs`, `tied_pairs`, `drawn_pairs`, `Tie` |
| The traits a drag keeps pointing the way they did | `sketch/src/sketch/settling/kept.rs` | `Kept`, `tied_by_direction`, `lines_kept_in` |
| Where a dragged point can go without its shape turning | `sketch/src/sketch/settling/give.rs` | `Give`, `give_of` |
| The kept lines as rows of the solver | `sketch/src/solver/kept_solver.rs` | `kept_equations`, `kept_row` |
| The way-up guard of a drawing that settles, and the turn back that breaks nothing the settle made true | `sketch/src/solver/orientation.rs`, `orientation/turning_back.rs` | `orientations`, `rotation_gauges`, `hold_orientations` |
| When a sweep of the solver has stopped getting anywhere | `sketch/src/solver/stall.rs` | `Stall::stalled` |
| A point held keeping its place along what holds it through a gesture | `sketch/src/sketch/settling/share.rs` | `Sketch::shares`, `keep_shares` |
| A side or a curve pulled: across or along, what a press takes hold of | `sketch/src/pulling.rs` | `Sketch::pulled_at`, `side_drag`, `curve_drag`, `SideDrag`, `CurveDrag` |
| A side moved across, the shape stretching after it | `sketch/src/pulling/travel.rs` | `Sketch::move_side` |
| A shape turned by hand, and the grid pulling its end | `sketch/src/turning.rs` | `Turn`, `Sketch::turn_shape`, `angle_onto_grid` |
| Kinds of constraint and dimension | `sketch/src/constraints.rs` | `Constraint`, `Dimension`, `DimensionTarget` |
| What the constraint tool is pointed at, and what it means once shown enough | `sketch/src/rule_intent.rs` | `rule_intent`, `Rule`, `RuleIntent`, `RulePick` |
| Where a rule's mark is written, and the nearest one to a cursor | `sketch/src/rule_marks.rs` | `Sketch::rule_marks`, `Sketch::nearest_rule` |
| The solver | `sketch/src/solver.rs` | `solve(millimeters_per_unit)` → `SolveOutcome` |
| Laying a tangency with the point where the two touch, and taking both away | `sketch/src/sketch/tangency.rs` | `add_tangency`, `add_ellipse_tangency`, `laid_as_a_tangency`, `erased_as_a_tangency` |
| Whether a tangency's contact has slid off its segment | `sketch/src/sketch/tangency.rs` | `Sketch::has_a_flipped_tangent` |
| One equation of the system, linearised around the drawing's current shape | `sketch/src/equation.rs` | `Equation` |
| The blocks that keep their shape while the rest of the drawing settles | `sketch/src/rigid.rs` | `Block`, `rigidify`, `ownership` |
| What a set of equations holds, and what it leaves free | `sketch/src/independence.rs` | `rank`, `null_space`, `is_dependent` |
| How much of a drawing is already decided | `sketch/src/settled.rs` | `Freedom`, `freedom`, `is_fully_constrained`, `settled_points` |
| A circle, an arc or an ellipse drawn to another size about its centre | `sketch/src/resizing.rs` | `Curved`, `Sketch::curve_at`, `reach_through`, `resize`, `resize_in_place`, `resize_circle`, `resize_arc`, `resize_ellipse` |
| The five circle constructions, and the ways of drawing one | `sketch/src/construct.rs` | `centre_through`, `centre_touching_two`, `circle_touching_three`, `CircleMode` |
| Work plane, going 2D ↔ 3D | `sketch/src/plane.rs` | `WorkPlane::to_world`, `to_local`, `ray_intersection`, `kind`, `near_side` |
| Closed areas, to extrude | `sketch/src/regions.rs` | `Sketch::regions()` |
| How deep an area lies and what it is hollow of: the areas directly inside it, or the outside several of them make where they touch (#540) | `sketch/src/regions/hollow.rs`, `sketch/src/arc_regions.rs` | `nest`, `Walked::outsides`, `apart_at_bridges` |
| How much surface an area holds and how far it is round, the curve honoured rather than the steps it was sampled into | `sketch/src/regions/measure.rs` | `Region::area`, `Region::perimeter`, `Outline::area`, `Outline::perimeter` |
| An area's outline and holes as runs — straight, round about a centre with how far it turns, or along an ellipse — which is how the exact kernel is handed an area | `sketch/src/regions/runs.rs` | `Outline::runs`, `Leg` |
| An area cut along a line into its sides, arcs cut as arcs, for a turn about an axis running through it | `sketch/src/regions/across.rs` | `Sketch::pieces_across` |
| How finely the drawing holds its rules: the distance below which it cannot tell two places apart | `sketch/src/resolution.rs` | `Sketch::resolution` |
| Which side of a line a corner lies on, read exactly, for cutting an area into triangles | `sketch/src/regions/side.rs` | `side`, `Side` |
| Naming an area by the curves that bound it, and finding it again | `sketch/src/naming.rs` | `CurveId`, `Area`, `Standing`, `Became`, `area_under` |
| Random drawings held to the rules every area keeps, a campaign over them, and what it found ([`sketch-soundness.md`](sketch-soundness.md)) | `sketch/tests/random_sketches/`, `sketch/tests/every_sketch_keeps_its_areas.rs`, `sketch/tests/what_random_sketches_found.rs` | `holds`, `drawn`, `a_campaign_of_random_drawings_keeps_every_rule` |
| Where two curves of the drawing cross | `sketch/src/crossing.rs` | `where_segments_cross`, `where_segment_crosses_arc`, `where_arcs_cross`, `where_segment_crosses_circle`, `where_arc_crosses_circle`, `where_circles_cross` |
| A circle, and the turns at which the drawing runs through it | `sketch/src/circle_edges.rs` | `Sketch::rounds`, `Round` |
| The drawing as half-edges a face walk can turn at, cut wherever two curves cross and wherever a point sits on one | `sketch/src/edges.rs` | `Sketch::crossed`, `Sketch::crossings`, `Crossed`, `ArcHalfEdge` |
| Points drawn in one place, taken by that graph as one vertex while the drawing keeps both | `sketch/src/edges/weld.rs` | `welded` |
| One end of a curved piece as the walk reads it: its departing tangent, how hard it bends, the places it draws | `sketch/src/edges/half_edge.rs` | `CurvedHalfEdge`, `Bend` |
| One curve of the drawing as that graph reads it: where it runs, how far along a place stands, the runs it is left as | `sketch/src/edges/curve.rs` | `Curve`, `between`, `pieces` |
| A stretch two curves lie along, entering that graph as the one border it is and named by both, and a circle or an ellipse drawn twice as one loop | `sketch/src/edges/border.rs` | `Borders`, `Loops`, `also_named` |
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
| The matter as every crate above sees it: raised, turned, joined, cut, drawn, pointed at — the one thing that leaves `cao_solid`. It holds the exact kernel's body or the flats, and which one is decided here alone: exact with exact stays exact, anything with the flats goes to the flats, and a turn the exact kernel cannot make is declined (#632) | `solid/src/body.rs` | `Body`, `Body::tool_raised`, `tool_turned`, `union`, `difference` → `Result<_, Declined>`, `behind`, `triangles`, `ray_hit` → `FaceHit`, `triangles_of`, `is_flat`, `plane_of` → `FacePlane`, `has_face`, `count_past`, `count_past_turned`, `bounds`, `volume` |
| An exact body behind `Body`: the brep, the face counter, a panic in the kernel caught as a decline, a face left in pieces given fresh numbers and the number each replaced kept as its root, the names of its edges and vertices worked out once as it is made, by the edge's and the vertex's rank | `solid/src/body/exact.rs` | `Exact`, `Declined::Panicked`, `roots`, `root`, `parted` |
| The triangles an exact body is drawn with, made once and kept, each knowing its face; a ray against them answering with the true surface's normal | `solid/src/body/drawn.rs` | `Drawn`, `DRAWN`, `ray_hit` |
| What a body is made of inside the crate: faces as flat pieces, ray casting | `solid/src/mesh.rs` | `Mesh`, `Polygon`, `ray_hit`, `bounds` |
| Extruding an area into a prism on the flats | `solid/src/sweep.rs` | `prism(...)`, reached as `Body::tool_raised` when the part is on the flats |
| Turning an area around an axis on the flats | `solid/src/sweep.rs` | `revolution(...)`, reached as `Body::tool_turned` when the part is on the flats or the profile comes without its runs — the harness's flats, never the application's since #632 |
| How a profile is read before it is turned, once for both kernels: whether the turn is whole, which side of the axis the area lies on, how close counts as on the axis, and the profile laid straight, its arcs laid by centre and radius, a ball end a hair off its shaft made tangent | `solid/src/turning.rs`, `turning/straight.rs`, `turning/straight/bends.rs` | `Turn::of`, `Turn::lie` → `Lie`, `Turn::on_axis`, `is_whole`, `ON_THE_AXIS`, `Straight::of`, `Straight::numbers`, `Bend`, `Corner.bend`, `Drawn` |
| A profile turned by the flats, its points within the band laid on the axis first | `solid/src/body/turned.rs` | `turned_flats`, `Snapped` |
| A profile of straight runs and arcs turned by the exact kernel: cylinders, cones, spheres, tori, discs, rings and the two ends of a partial turn ([`exact-kernel.md`](exact-kernel.md#turning)) | `solid/src/brep/turned.rs`, `brep/turned/faces.rs`, `brep/turned/faces/partial.rs`, `brep/turned/faces/round.rs`, `brep/laying.rs`, `brep/piece.rs`, `brep/piece/read.rs` | `brep::Body::turned`, `Laying`, `pieces_numbered`, `read`, `joined`, `Round`, `swept`, `meridian` |
| Adding or taking away matter | `solid/src/boolean.rs` | `Mesh::union`, `Mesh::difference` (BSP tree) |
| Keeping only what lies behind a plane, to look inside rather than to cut | `solid/src/clipping.rs` | `Mesh::behind` |
| The exact kernel of #498: a body of planes, cylinders, elliptic walls, cones, spheres and tori, raised from a profile of straight runs and arcs and turned from one of straight runs and arcs, joined and cut exactly — behind `Body` since #526, and refused above `cao_solid` by `the_exact_kernel_stays_behind_the_body` and `the_insides_of_the_matter_stay_in_cao_solid` ([`exact-kernel.md`](exact-kernel.md)) | `solid/src/brep.rs` and `brep/` | `brep::Body::raised`, `joined`, `cut_by` → `Result<_, Declined>`, `triangles`, `volume`, `listing` → `Listing`, `crossings_along`, `winding` |
| A sphere and a torus: their placement, the chart `(θ, φ)` the two share, the poles, the meridian circle read against a coaxial line or circle (#576) | `solid/src/brep/surface/sphere.rs`, `brep/surface/torus.rs`, `brep/surface/round.rs` | `Sphere`, `Sphere::through`, `Torus`, `Torus::through`, `round`, `Round`, `poles`, `meridian`, `windowed` |
| How a sphere or a torus meets a plane square to or holding its axis, and a coaxial cylinder, cone, sphere or torus: rims with a touch decided once, meridians and their poles, and the pairs it declines — a plane at a slant, a wall or a round of another axis, an elliptic wall (#576) | `solid/src/brep/relation/round.rs`, `brep/relation/revolved/arc.rs`, `brep/relation/crossing/round.rs` | `round`, `Meridian::Arc`, `Relation::Rims`, `Relation::Meridians`, `line_and_sphere`, `line_and_torus`, `circle_and_sphere`, `circle_and_torus` |
| The profile a kernel is handed: loops of straight runs and arcs, and the frame they stand in, beside the steps the flats raise | `solid/src/profile.rs` | `Profile`, `Contour`, `Run`, `Frame` |
| Where a raised profile touches itself — a whole circle touching another loop, two corners in one place — laid as one corner every loop through it shares; a stretch run both ways, the corridor a trait joining two loops leaves, and a corner whose walls leave it nearly along each other, declined | `solid/src/brep/prism/touch.rs`, `brep/prism/corridor.rs` | `parted`, `corners_met`, `run_both_ways`, `needle` |
| The numbers a face of the exact kernel answers to, carried through a raise, a boolean and a merge | `solid/src/brep/topology/numbers.rs`, `brep/selection/numbers.rs`, `brep/prism/walls.rs` | `Body::numbers`, `renumbered`, `pieces_of`, `rename` |
| The names an edge and a vertex of an exact body answer to, read off its faces' numbers, the faces at its ends and the way its loops run, and where it lies only where nothing else tells two apart; a name found as a face is, by a number its sides share and on one end kept, or reported split or gone (#582) | `solid/src/brep/names.rs`, `brep/names/found.rs`, `solid/src/body/names.rs` | `EdgeName`, `VertexName`, `Lost`, `brep::Body::edge_names`, `vertex_names`, `names_rooted`, `find`, `find_rooted`, `Body::edge_names`, `vertex_names`, `lost` |
| How two surfaces of the exact kernel meet, decided once | `solid/src/brep/relation.rs`, `brep/meet.rs`, `brep/canonical/` | `relation`, `crossings`, `Meeting` |
| A plane at a slant to a cylinder or a cone, decided into its conic, the rulings through the apex, the ruling it touches along or the apex alone | `solid/src/brep/relation/sliced.rs`, `brep/relation.rs` | `at_a_slant`, `plane_and_cylinder`, `plane_and_cone` |
| The conic a plane cuts a wall along: one formula from an anchor, built from the wall's ruled form, seen on its plane and its wall ([`exact-kernel.md`](exact-kernel.md#the-conic-a-plane-cuts-a-wall-along)) | `solid/src/brep/conic.rs`, `brep/conic/built.rs`, `brep/conic/seen.rs` | `Conic`, `Conic::cut`, `Round`, `Arm`, `parameter`, `bounds`, `turning` |
| Two parallel walls, an elliptic one among them, met along the rulings their sections cross or touch at, found from the feet of the normals they share; the side one stands on between two rulings; the move onto a touch; their triangles on rays from a point inside every wall of a group (#571) | `solid/src/brep/relation/walls.rs`, `brep/relation/walls/met.rs`, `brep/relation/walls/normals.rs`, `brep/selection/wrapped/above/walls.rs`, `brep/canonical/touches/walls.rs`, `brep/tessellation/sampling/abreast.rs`, `sampling/abreast/groups.rs` | `round_and_elliptic`, `elliptic_pair`, `Relation::Parallel`, `Ruled`, `feet_of_touches`, `standing_above`, `onto_a_touch`, `Abreast::of` |
| Where a conic crosses a third surface, by the call the other curve of the corner makes | `solid/src/brep/relation/crossing/conic.rs` | `conic_and_surface`, reached from `crossings_given` |
| An edge along a conic sampled for the triangles | `solid/src/brep/tessellation/sampling/conic.rs` | `on_conic` |
| A coaxial plane, cylinder or cone a hair from touching a sphere or a torus, or from a rim on it, moved onto it with its operand — along the axis, or grown away from it (decision 12, #626) | `solid/src/brep/canonical/tangent.rs`, `brep/canonical/carried/across.rs`, `brep/relation/round.rs` | `tangent`, `unbroken`, `standing`, `Move::Across`, `Across`, `carried_by`, `untouched`, `off_the_rim`, `Gap` |
| A plane a hair from a straight edge of the other operand it runs along, moved onto the edge with its operand (decision 13, #644) | `solid/src/brep/canonical/edged.rs` | `edged`, `carried_along` |
| A face of a sphere or a torus cut into triangles: the round's grid of angles and rows, a meridian arc sampled on the rows, the face laid out square to its chart with its poles as floors, and cut in strips between the rows on a lattice added inside it (#576) | `solid/src/brep/tessellation/sampling/round.rs`, `brep/tessellation/outline/round.rs`, `outline/round/floors.rs`, `outline/strips.rs` | `Rounded`, `on_meridian`, `rows`, `Outline::rounded`, `Floors`, `Lattice`, `Outline::stripped` |
| The floors of a round: its pole lines, a whole ring's seam, a round face closed without a loop, and a circle seen in the chart's parameters (#576) | `solid/src/brep/domain/floor/round.rs`, `brep/domain/traced/round.rs` | `Pole`, `poles_of`, `closed_round`, `seam`, `floors_round`, `holds` |
| The poles a face holds, at most two, the cone's apex among them, and the merge they refuse (#576) | `solid/src/brep/topology/poles.rs` | `poles_held`, `hold_poles`, `poles_within` |
| The exact volume of a round face: the flux of the position through it, anchored at a pole, with the far pole's floor (#576) | `solid/src/brep/volume/round.rs` | `round_anchor`, `round_floors`, `Meridian` |
| A ray through a sphere by its quadratic, through a torus by the quartic bracketed between the critical points of its level, four hits at most and a graze doubtful (#576) | `solid/src/brep/ray/hits.rs` | `Hits::At` |
| A round face's reach read off its surface, a face beyond or in the hollow of a round passed apart, and which side of a coaxial pair a face stands on read from the rims — the surface whose meridian bends more towards the other's normal along a touching rim (#576) | `solid/src/brep/combine/clear/round.rs`, `brep/selection/wrapped/above/round.rs` | `bulges`, `all_round`, `beyond`, `in_the_hollow`, `across_the_rims`, `rounded`, `bending_more` |
| A boolean keeping what it keeps of a whole ring from straddling the chart's cut: the cut moved into a band no face is kept of before the regions are read (#576) | `solid/src/brep/combine/seamed.rs` | `seamed` |
| The listing of a round held to its own meridian circle, its poles and its chart's cut (#576) | `solid/src/soundness/listed/round.rs` | `Tube`, `of_sphere`, `of_torus`, `rise`, `poles` |
| Join and cut on the exact kernel | `solid/src/brep/combine.rs` and `combine/` | `Body::joined`, `Body::cut_by` |
| Two faces at a slant — two walls at a skew angle, a wall off a cone's axis — decided clear of each other on the faces themselves before the kernel declines; a plane at a slant to a wall asked first, so its conic is registered only where faces may meet along it | `solid/src/brep/combine/clear.rs`, `combine/clear/walls.rs`, `combine/clear/hollow.rs` | `clear`, reached from `combine/related.rs`; `Wall`, `in_the_hollow`, `beyond` |
| The exact kernel's triangles, every edge sampled once | `solid/src/brep/tessellation.rs` and `tessellation/` | `Body::triangles(tolerance)` |
| A body written as a binary STL at a gap in millimetres, declined when some round is too wide for the most steps a turn is cut into (#580) | `solid/src/stl.rs`, `brep/tessellation/sampling/capped.rs` | `stl::of`, `stl::Uncovered`, `Cut::capped` |
| Which side of a line or of an edge a point stands on, and which way a loop turns, decided exactly: the triangles' sweep and folds, the laid profile of a turn (#585) | `solid/src/orientation.rs`, `orientation/expansion.rs` | `turn`, `rotation`, `beside`, `Expansion` |
| The rules every solid must keep, for the tests | `solid/src/soundness.rs` and `soundness/` | `closed`, `uncrossed`, `Lines`, `repeatable`, `campaign`, `listed` (an exact body's listing held to its geometry) |
| Solids drawn at random, shrunk and printed as Rust | `solid/tests/random_solids/` | `Case::drawn`, `Case::smaller`, `check` |
| A campaign over random solids | `solid/tests/every_solid_keeps_its_rules.rs` | `a_campaign_of_random_solids_keeps_every_rule` |
| A campaign over random parts, every gesture undone | `part/tests/an_undo_gives_back_the_part.rs` | `undoes` |
| What the campaigns found, waiting on their issues | `solid/tests/what_random_solids_found.rs` | `#[ignore = "#n"]` |
| #498's eighteen rows, the flats beside the exact kernel and OpenCascade as #447 quoted it | `solid/tests/a_bored_cylinder_on_two_kernels.rs` | `a_bored_cylinder_on_two_kernels` |
| The sixteen cases on the exact kernel, each counted by hand, and the cuts along a conic held to their arithmetic | `solid/tests/the_exact_kernel_joins_and_cuts.rs` | |
| Bores and slanted faces, cut in parts laid at a slant, along an ellipse where they meet (#552); two bores crossing at a skew angle, still declined | `solid/tests/a_slant_far_from_a_bore.rs` | `keeps_every_rule` |
| Every face of the exact kernel keeps its number through raises, joins and cuts, and when a size changes | `solid/tests/faces_keep_their_numbers.rs` | |
| Random solids on the exact kernel, held to the arithmetic along every line | `solid/tests/random_exact_solids.rs`, `solid/tests/random_solids/{along,arithmetic,kernels,outlines}.rs` | `Kernel`, `Exact`, `Flats`, `held_to_arithmetic`; the campaigns under `--ignored` |
| Turns of straight profiles on the exact kernel, slanted runs included, held to Pappus's volume and to the same profile raised in slabs | `solid/tests/the_exact_kernel_turns.rs` | its smoke campaign under `--ignored` |
| Random turns among prisms, on the exact kernel and through `Body`, held to the arithmetic; profiles holding arcs drawn in a draw of their own, a ball, a ring, a groove, a rounded shoulder and the hair cases beside them (#576) | `solid/tests/random_turned_solids.rs`, `random_solids/rounds.rs`, `random_solids/drawing/rounds.rs` | `Case::drawn_turned`, `drawn_turned_off_the_lattice`, `drawn_slanted`, `drawn_slanted_off_the_lattice`, `drawn_tilted`, `drawn_tilted_off_the_lattice`, `drawn_rounded`, `drawn_rounded_off_the_lattice`, `Bow`, `tube`, `asks_beyond_the_conics`, `asks_a_round_beyond_its_circles`; the campaigns under `--ignored`, `CAO_TRIAGE_DRAW=rounded` |
| What the exact campaigns found, in the kernel and in its triangles, and the band where a third surface crosses two that touch | `solid/tests/what_the_exact_campaigns_found_in_the_kernel.rs`, `what_the_exact_campaigns_found_in_the_triangles.rs`, `the_tangency_band.rs`; the 5 October campaigns' seeds by family: `a_crescent_s_tip_under_a_plane.rs`, `a_plane_a_hair_from_a_face.rs`, `a_surface_a_hair_from_a_line_of_touch.rs`, `a_rim_grazing_a_ruling_of_touch.rs` | `#[ignore = "…"]` for what still fails, its reason starting with the family |

What it does: [`extrusion.md`](extrusion.md), for the rules and the
campaigns [`soundness.md`](soundness.md), and for the exact kernel
[`exact-kernel.md`](exact-kernel.md), with its journal
[`exact-kernel-journal.md`](exact-kernel-journal.md) and what its campaigns
found [`exact-kernel-failures.md`](exact-kernel-failures.md).

## History and persistence — `cao_part`

| What one is after | File | Way in |
| --- | --- | --- |
| List of operations, undo, redo | `part/src/history.rs` | `History`, `Operation` |
| The number each operation is given, the next one to be given, and the one at a place in the list | `part/src/history/numbering.rs` | `History::next_number`, `number_at` |
| The major steps a design is grouped into | `part/src/history/step.rs` | `Step`, `StepKind` |
| Which sketch an operation edits, and so which step it is filed under | `part/src/history/operation/edits.rs` | `Operation::edits` |
| Where a step begins and ends in the list | `part/src/feature.rs` | `Feature::all` |
| Replaying the history for the geometry | `part/src/replay.rs`, `part/src/state.rs` | `PartState::rebuild`, `PartState::apply` → `Outcome` |
| What a replay notes: what each operation laid, what was raised, which values left the drawing | `part/src/replay.rs` | `Replay`, `Laid`, `SetBy`, `next_value_set` |
| The ways a curve of a drawing is replaced by other curves — trimmed as a trait, an arc, a circle or an ellipse, divided, chamfered or rounded — and the fillets and chamfers laid as cuts on corners, one group per gesture, their values in millimetres | `part/src/cutting.rs` | `PartState::trim`, `trim_arc`, `trim_circle`, `trim_ellipse`, `split`, `chamfer`, `fillet`, `lay_cuts`, `sides_now`, `PartState::area_rank` |
| Laying every cut still standing again after compaction, one operation per group so the corners keep changing together | `part/src/compaction/cuts.rs` | `add_cuts` |
| A cut changed in place after it was laid, as one step: the operation, and its replay | `part/src/history/operation/cut_change.rs`, `part/src/recutting.rs` | `CutChange`, `CutAsked`, `PartState::change_cut`, `PartDocument::cut_written` |
| A circle, an arc or an ellipse laid down again as the history replays it | `part/src/curves.rs` | `PartState::add_circle`, `add_arc`, `add_ellipse` |
| A side moved across or a shape turned, replayed | `part/src/pulled.rs` | `PartState::move_segment`, `turn_shape` |
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
| An area handed to the kernel as runs and as steps, an ellipse as its arcs | `part/src/profile.rs` | `profile` |
| A step of matter the exact kernel declined: broken under its own number, its faces counted all the same, its reason kept, carried by the cache | `part/src/extrusion.rs`, `part/src/document/matter.rs` | `declined_with`, `PartState::declined`, `PartDocument::is_declined`, `declined_because` |
| A revolution: the axis in the sketch, an area across it cut into its sides, each turned and joined | `part/src/extrusion.rs` | `PartState::revolve`, `axis_in_sketch`, `sides_of` |
| A part raised, joined and cut by the exact kernel, held to the arithmetic, and kept exact after a revolution of an ellipse, a broken step since #632 | `part/tests/the_exact_kernel_computes_the_part.rs`, `part/tests/an_exact_body_comes_back_from_json.rs` | |
| A part whose profile holds an arc, turned by the exact kernel into a sphere or a torus, held to Pappus's volume and to the coaxial pairs it meets, and the pairs it declines named (#576) | `part/tests/the_exact_kernel_turns_an_arc.rs` | |
| A part turned from straight runs by the exact kernel, chamfers, points and countersinks included, held to the arithmetic; an area across its axis, hairs off it, a turn declined, a cone met off its axis | `part/tests/the_exact_kernel_turns_the_part.rs` | |
| A revolution the exact kernel cannot make — an ellipse, a profile the reading cannot lay — refused as a broken step, never the flats (#632) | `part/tests/a_revolution_the_exact_kernel_cannot_make_is_refused.rs` | |
| A part cut at a slant by the exact kernel, held to the arithmetic: a hole drilled through a slanted face, a shaft cut at a slant, a flat on a chamfer, a point cut through its tip or parallel to its slope; a skew bore still declined | `part/tests/the_exact_kernel_cuts_at_a_slant.rs` | |
| Changing the variables, and what is refused | `part/src/document/variables.rs` | `PartDocument::change_variable`, `Refused`, `Use`, `uses_of`, `formula_of` |
| The variables through a compaction | `part/src/compaction/variables.rs` | `compact_variables`, `Renumbered` |
| The `.caopart` file (zip) | `part/src/document.rs` | `PartDocument`, `SCHEMA_VERSION = 6` |
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
| Canvas: the fillets and chamfers of the drawing — what is left of each side, the stretch hidden up to the sharp corner, the curve of every cut and its value | `app/src/screens/viewport/render/cuts.rs` | `CutCanvas`, `CutCanvas::push_side`, `CutCanvas::push_cuts`, `lit_corners`, `paint_cut_labels`, `cut_label_under` |
| Canvas: an arc or an ellipse beside a cut — solid where it shows, in thin dashes over the stretch a fitting cut hides up to its sharp corner | `app/src/screens/viewport/render/cuts/curve_sides.rs` | `CutCanvas::push_arc`, `CutCanvas::push_ellipse` |
| Canvas: the stretch of any side a cut hides up to its sharp corner — thin, faint, dashed, lit with its cut | `app/src/screens/viewport/render/cuts/hidden.rs` | `CutCanvas::hidden_at`, `faint` |
| Canvas: a cut changed from the selection — its values opened again on a double-click or from « Cote de rayon », taken off by `Suppr`, switched by « Construction », a chamfer's sides swapped from the button under the ribbon | `app/src/screens/viewport/input/cut_changes.rs`, `app/src/screens/ribbon/view.rs` | `open_cut_values`, `switch_held_cuts`, `chamfer_to_swap`, `swap_held_chamfer`, `chamfer_row` |
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
| Canvas: the two clicks of the chamfer and the fillet — a corner's point or its two sides — the values typed for every corner taken, and a value too big held back with the largest that fits | `app/src/screens/viewport/input/corner.rs`, `input/corner/laying.rs` | `corner`, `cut`, `corners_taken`, `on_enter`, `held_back`, `offer_the_largest`, `laying` |
| Canvas: what two sides clicked name — where they meet, or the end nearest the click of two sides sharing both — and why a chamfer refuses a corner | `app/src/screens/viewport/input/corner/naming.rs` | `named_by`, `chamfer_refused`, `said_of` |
| Canvas: the cut or the curve the corners taken would take, shown as the value is typed | `app/src/screens/viewport/input/corner/preview.rs` | `previewed` |
| Canvas: what the mirror and the two patterns take hold of, and the axis, centre or direction they lay the copy against | `app/src/screens/viewport/input/copying.rs` | `copy`, `hold_is_done`, `axis_at`, `turned`, `filled` |
| Canvas: the copies a mirror or a pattern would lay, shown before the click that names where | `app/src/screens/viewport/input/copying/preview.rs` | `previewed` |
| Canvas: the values a pattern's fields open on | `app/src/screens/viewport/input/copying/opening.rs` | `fields_open_on` |
| Canvas: which closed areas an extrusion is offered, and which one a click takes | `app/src/screens/viewport/input/areas.rs` | `pick_areas` |
| Canvas: drawing a circle, an arc or an ellipse to another size by its curve, or turning it | `app/src/screens/viewport/input/resizing.rs` | `drag_curve` |
| Canvas: a side pulled across or slid along, and the step it writes — a curve slid round's too | `app/src/screens/viewport/input/sides.rs` | `drag_side`, `side_operation`, `turn_operation`, `curve_turn_operation`, `resize_operation` |
| Canvas: what a drag takes hold of and moves | `app/src/screens/viewport/input/dragging.rs` | `drag_point`, `letting_go`, `reshaped` |
| Canvas: a selection pressed on and moved as one block | `app/src/screens/viewport/input/selection_drag.rs` | `grabbed_group`, `drag_group` |
| Canvas: moving an annotation, finding the one under the cursor | `app/src/screens/viewport/input/annotation_drag.rs` | `drag_annotation`, `nearest_annotation`, `annotation_position` |
| Canvas: what a point laid down by a tool lands on, and what a dragged one is held by once dropped | `app/src/screens/viewport/input/landing.rs` | `landed_on`, `held_at_drop`, `point_ref_at`, `born_at` |
| Canvas: the camera's own gestures — orbit, pan, wheel, trackpad | `app/src/screens/viewport/navigation.rs` | `handle_navigation`, `advance_transition`, `ScrollInput` |
| Canvas: what a box catches, and what deleting takes with it | `app/src/screens/viewport/input/selecting.rs` | `band_select`, `erase` |
| Canvas: one click of the smart dimension tool | `app/src/screens/viewport/input/measure.rs` | `measure`, `place_dimension`, `measure_preview` |
| Canvas: one click of the measure tool, which records nothing | `app/src/screens/viewport/input/reading.rs` | `read`, `showing` |
| Canvas: the dashed triangle a measure is drawn as, and a number on each side | `app/src/screens/viewport/render/reading.rs` | `push_measure`, `paint_measure` |
| Sketch tool, keyboard input | `app/src/screens/sketch.rs` | `SketchEditor`, `LiveInput` |
| The fields a shape is drawn to, and what the first length typed in them says a unit is worth on a part with no scale yet | `app/src/screens/sketch/live_input.rs` | `LiveInput::take`, `LiveInput::scale` |
| A value typed into a dimension already on the drawing, and into the first one placed | `app/src/screens/sketch/typed_dimension.rs` | `apply_dimension_value`, `take_back_the_first_placing` |
| Turning a dimension's shape into vertices, with a colour | `app/src/screens/annotations.rs` | `push(...)`, `Style` |
| Extrusion and revolution, UI side, and what it says when nothing was made or the kernel declined | `app/src/screens/extrusion.rs` | `ExtrusionState`, `apply_extrusion` |
| The tree of a part: its bodies and sketches, a step marked when an area is lost or the kernel declined it, a sketch when its face is gone | `app/src/screens/part_tree/` | `state.rs` `PartTree::of`, `view.rs` `panel` |
| The panels beside a part, and what is asked in them | `app/src/panels.rs` | `beside_the_part` |
| Variables panel: the rows, what is typed into them, what was refused | `app/src/screens/variables/` | `state.rs` `VariablesPanel`, `Named`, `view.rs` `panel`, `mod.rs` `run` |
| What a refusal names blinks, and for how long | `app/src/screens/blinking.rs` | `lit`; `ViewportState::blink`, `VariablesPanel::blink` |
| The field a formula is typed into: a name completed from a list under it | `app/src/ui/formula_field.rs`, `app/src/screens/variables/mod.rs` | `formula_field`, `keeps_escape`; `offered`, `NAMING` |
| A variable's name in a formula as a block: shown, stepped over, erased whole | `app/src/ui/formula_field/blocks.rs` | `found`, `stepped`, `erased`, `snapped`, `laid_out` |
| A variable dragged from the panel into a dimension's field | `app/src/ui/dropping.rs`, `app/src/screens/variables/view.rs`, `app/src/screens/viewport/render/dimensions.rs` | `grip`, `take_a_dropped_name`, `DraggedName` |
| Canvas: the values a shape earns, laid as typed; a click refused for a field that does not read; the scale the shape in hand reads what is typed with | `app/src/screens/viewport/values.rs` | `lay_values`, `as_typed`, `refused_for_what_is_typed`, `shape_scale` |
| History panel | `app/src/screens/history_tree.rs` | `show(...)` → `HistoryAction` |
| Files panel: what it holds and what is half-done to it | `app/src/screens/explorer/state.rs` | `Explorer` |
| Files panel: the drawing of it | `app/src/screens/explorer/view.rs` | `panel(...)` → `ExplorerAction` |
| Starting a second window on another part | `app/src/adapters/window.rs` | `open_another` |
| Exporting the part to STL: the gap each level asks for, the level suggested when one is refused, the file written where the save dialog says (#580) | `app/src/export.rs`, `app/src/adapters/save_dialog.rs`, `app/src/wording/export.rs` | `gap`, `export_request`, `stl`; `stl_destination`, `with_stl_extension`; `uncovered`, `written` |
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
scripts/verify.sh            # fmt --check, clippy -D warnings — what a push runs
cargo test --workspace       # the whole suite, minutes — what the CI runs
cargo test -p cao_sketch     # one crate alone, while iterating
cargo run -p cao_app         # launch the application
```
