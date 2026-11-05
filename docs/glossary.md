# Glossary

The words this project uses, and the ones it refuses. Code and test names are
English; what the user reads is French. That pairing is the reason this file
exists: the two drift apart silently, and nothing else in the repository holds
them side by side.

A term is in here when getting it wrong changes the code, not merely the
wording. Where a term carries a rule, the rule is on the line under it.

## The drawing

| Code | Interface | What it names |
| --- | --- | --- |
| `Sketch` | esquisse | A 2D drawing on one plane, with its points, segments, circles, arcs, dimensions and constraints. |

> **The aggregate.** Nothing inside a `Sketch` is valid on its own — the solver
> resolves the whole of it at once. Its fields are private and stay private:
> every change goes through the root, or the invariant the solver maintains is
> lost.

| Code | Interface | What it names |
| --- | --- | --- |
| `WorkPlane` | plan de travail | The plane a sketch is drawn on, and the frame its coordinates are given in. |
| `PointId`, `SegmentId`, `CircleId`, `ArcId` | — | A rank in the sketch's own list. Not an identity that survives anything else. |
| `Segment` | trait | A straight piece between two points. Never called "line": a `Line` in `construct` is an infinite one used for tangency. |
| `Circle` | cercle | A centre and a radius. The radius is an unknown of the solver, like any coordinate. |
| `Arc` | arc | A piece of a circle, as a centre and the two ends it runs between — three ordinary points of the drawing, shared like a segment's. No radius of its own: it is read off the end the curve starts at, which leaves the way round a consequence of the order of the two ends. |
| `CornerCut`, `CornerCutId`, `CutShape`, `DrawnCut` | congé / chanfrein | A fillet or a chamfer standing on a sharp corner, by rank: its two sides, the shape it draws and whether it is construction. Nothing is trimmed and nothing is solved — what it draws is worked out from the corner, its sides and the value every time it is read. |
| `CutPoint`, `Constraint::AtCutPoint`, `Constraint::OnCut` | — | One of the points a cut offers: a fillet's centre, or where the cut meets a side. Not a point of the drawing until something hangs on it; then a point is laid there, held by a rule the solver works out from the corner, its sides and the value. `OnCut` holds a point on the cut's curve instead. |
| `Element` | élément | Whichever of point, segment, circle or arc a rule is about. |
| `Region` | aire | A closed loop of the drawing, the thing an extrusion can be raised from. |
| `Dimension` | cote | A rule that carries a number: length, angle, radius, diameter, distance. |
| `Support` | — | What a point can be held on: a trait, a circle, an arc, or one of the plane's own axes. |
| `Constraint` | contrainte | A rule that carries no number: perpendicular, parallel, equal, tangent, on-segment, on-circle, on-arc, on-axis. |

> **A hold corrects the point, not the curve.** The step `OnSegment`,
> `OnCircle`, `OnArc` and `OnAxis` take moves the point they hold and leaves
> what holds it where it is: a point laid on a trait follows that trait and
> does not drag it about. Two points do not follow on their own, and for them
> the curve gives instead: one that cannot move at all — pinned, or under the
> cursor — and one something else is already pulling on, such as a rim point at
> the end of a trait whose length is typed. The rule's own equation says the
> whole truth either way: it is what the drawing is *read* from, and a row that
> left the curve out would answer "is this settled?" about a drawing nobody
> drew.

> **Dimension and constraint are not synonyms.** They live in separate lists on
> the sketch, and the split is what lets a value be typed without inventing a
> constraint for it. `Constraint` never holds a number.

| Code | Interface | What it names |
| --- | --- | --- |
| `driven` | cote de lecture | A dimension that reports a value rather than imposing one, because the drawing already fixes it. |
| `Freedom` | degrés de liberté | How much the drawing can still move. |

> **Counted by rank, never by tally.** `Freedom` comes from the rank of the
> constraint system. Counting rules could never see that a triangle's third
> side follows from the other two and their angle.

| Code | Interface | What it names |
| --- | --- | --- |
| settled | posé | A point no remaining freedom can move. Shown differently, and refused to a drag. |
| held | tenu | A point the cursor is holding right now. Lasts one gesture, never stored. |
| `PointPull` | — | What a drag of one point may do, read once when the press lands: what stays, what keeps its direction, where the point can go. |
| stays | reste en place | The point a drag leaves where it is: the one holding the shape, the centre of the curve pulled on, or the farthest point along the traits tied by direction. |
| kept direction | — | A trait, or an arc's ray, a drag holds to the direction it had, as a linear row swept first. Never part of the system outside a drag. |
| across, along | en travers, le long | Which way a side or a curve is pulled: across resizes, along turns the shape — about the side opposite, or the curve's centre. A trait on its own has neither, and goes wherever the hand takes it. |
| erased | effacé | Marked as gone, still in the list. |

> **Erased, not removed.** Taking a segment out of the middle would shift the
> rank of every later one, and every dimension recorded against those ranks
> would quietly start pointing at a different piece of the drawing.

| Code | Interface | What it names |
| --- | --- | --- |
| block, `rigidify` | bloc rigide | A part of the drawing a correction has no reason to reshape, carried and turned whole rather than bent. |
| `SolveOutcome` | — | `Solved`, `Residual` (constraints contradict each other), `Nothing`. |
| `LengthOutcome` | — | `Exact`, `BestEffort` (the far end could not move freely), `Degenerate`. |
| world unit | unité | What coordinates are stored in. |
| `millimeters_per_unit` | échelle | Millimetres one world unit is worth. Undefined until the part's first value is given, which is what sets it: a length typed while drawing or into a dimension is read against what the drawing already measures, and a size with nothing drawn to read it against — an extrusion's depth, a rectangular pattern's step — fixes a unit at a millimetre. An angle sets nothing. |

## The matter

| Code | Interface | What it names |
| --- | --- | --- |
| `Body` | matière | The whole part as a single surface, not a pile of separate lumps — so a pocket cut in a block really is a hole in the block. The only shape of the matter that leaves `cao_solid`. It holds either an exact body or the flats; the application reaches only the first since #632. |
| exact kernel, `brep` | noyau exact | What computes a part on planes, cylinders, elliptic walls, cones, spheres and tori, on their true surfaces: a Ø40 raised is 40 across, not a polygon inscribed in it. Never named above `cao_solid`. |
| the flats, `Mesh` | maillage | Flat pieces, each carrying the number of the face it belongs to: what the view behind a sketch plane is cut from, and the harness's second kernel. A part went to the flats from its first revolution of an ellipse, or of a profile the reading could not lay, until #632 made those broken steps: a revolution of straight runs or of an arc is the exact kernel's — those parallel or square to its axis since #533, slanted ones since #536, an arc since #576 — and an ellipse raised since #571. Never named above `cao_solid`. |
| face number | — | The number a face of the part answers to, given when the step that made it is replayed: 0 for the floor of a raise, 1 for its top, then one per run of its outline and its holes. A drawing laid on a face records it (`FaceAnchor`), so it must not move when a size changes. A face a cut leaves in pieces keeps it on the first piece; each other piece answers to a number of its own, in a range far above any step's and counted apart, so a later step's numbers do not move when an earlier cut starts or stops parting a face (#682). A later piece's own number still does: it counts every piece parted before it. |
| edge name, `EdgeName` | — | What an edge of the part answers to (#582): the sides of the two faces it lies between — a face's numbers, each taken back to its root — its `ends`, the sides of the other faces met where it starts and where it ends along its first side's loop, both empty for a whole closed edge, and `among`, how many edges lay between those two sides when it was named. Never a surface's parameters, which a turn of the part would reorder; what nothing topological tells apart is ranked by one point of it — halfway between its vertices, or a whole closed edge's at its curve's origin — read in the world within the body's tolerance: two edges share no point but at a vertex, so one inside each tells them apart. A move keeps that order and a turn may swap it, a cross-bore's two rims. Found as a face is, by a number its sides share, and on one end kept when a cut moved the other; split when more edges lie between its sides than did and none is it, gone when fewer do or both its ends moved (`Lost`). Two edges a join bridges into one are both found on it. The flats name none. |
| vertex name, `VertexName` | — | What a vertex of the part answers to (#582): the sides of every face at it and, where another vertex shares them, the names of the edges ending there. A ball's two poles share their face and have no edge, and are ranked by where they lie, which a turn may swap. No lookup yet, and holding its edges' names, a vertex is renamed by a change at an edge's far end. |
| root | — | The number of the face a piece was parted from, followed back to the first: what an edge or a vertex of that piece is named by, so that an edge a cut parts along with its face answers to the face it lay on. The piece's own face number stays its own, `PIECES` past the count of the pieces parted before it. |
| `Declined` | non construite | The exact kernel's refusal to answer — a radial hole in a cone, two walls nearly parallel within a chord's sag of each other, a result it cannot verify, a panic inside it. Never a wrong solid, and never a silent fall back to the flats: the step is broken, the part stands as it did before it, and the step still counts the face numbers it would have given. |
| `Polygon` | — | One flat piece of a face, its corners in order. Never named above `cao_solid`. |
| elliptic wall, `EllipticCylinder` | — | The wall an ellipse drawn in a profile is raised into, whole or in arcs: read by its eccentric angle from its major half-axis, beside the circular cylinder rather than a cylinder of two radii (#571). An ellipse a hair from round is raised as a circle. A wall at a skew angle meets it along their curve (#577); against a cone it is declined where the faces may cross. Never named above `cao_solid`. |
| conic, `Conic` | — | The curve a plane cuts a cylinder or a cone along at a slant: an ellipse, a parabola or a branch of a hyperbola, held by one formula anchored at a point of it (#552). A hole drilled through a slanted face leaves one in the floor. Never named above `cao_solid`. |
| `prism`, `revolution` | prisme, révolution | Raising a region into matter, straight or turned about an axis. |
| on the axis, `ON_THE_AXIS` | — | A point of a profile within a thousandth of the area's furthest point from the axis it turns about, on either side. It is laid on the axis before either kernel turns it: a corner the solver left a hair off is what the drawing meant (#487, #488). An area with points past that band on both sides lies across its axis, and is cut along it and turned side by side. |
| laid straight, `Straight` | — | A profile read against its axis: every run parallel or square to it within the drawing's resolution laid exactly so, every other slanted as drawn between the levels of its corners, every arc laid by its centre and radius with its turn read again between its corners, levels a hair apart made one, and a wall or a gap between them gone. What the exact kernel turns, arcs since #576; a profile that cannot be laid straight is a broken step since #632. |
| `Sphere` | — | The surface an arc turns into when its centre lies on the turn's axis (#576): the height of its centre on the axis and its radius, read in the round's chart. A whole ball is one face with two poles and no loop. Never named above `cao_solid`. |
| `Torus` | — | The surface an arc turns into when its centre stands off the turn's axis (#576): the centre `(height, ring radius)`, the tube's radius, and the seam its chart is cut at. Ring, horn or spindle by whether the ring radius is larger than, equal to, or smaller than the tube's. Never named above `cao_solid`. |
| round, `Round` | — | A sphere or a torus, as one surface family, and the chart `(θ, φ)` the two share (`brep/surface/round.rs`): `θ` about the axis, `φ` along the meridian circle, so that every latitude circle and every meridian is a line of the chart. Never named above `cao_solid`. |
| pole, `Face.poles` | — | The end of a round's axis, where every meridian meets: two for a sphere or a spindle, one for a horn, none for a ring. A pole is a vertex of its face and closes it, with a face holding one within it taking its floor as a loop. |
| rim, `Relation::Rims` | — | A circle where a round crosses a coaxial plane, cylinder, cone, sphere or torus, carrying whether the two surfaces only touch along it. Up to two, and a touch is built as that one circle. |
| meridian, `Relation::Meridians` | — | A circle a plane holding the round's axis cuts it along: one great circle for a sphere, two tube circles for a torus. The half of it in the `(h, ρ)` half-plane is the round's own meridian line, read as a cone's. |
| `Bend`, `Corner.bend` | — | An arc of a profile as laid against the axis: its centre in `(h, r)` and how far round it turns. A ball end a hair off its shaft is laid tangent, its pole slid along the axis onto the circle. |
| `ExtrusionMode::Add` / `Cut` | ajout / enlèvement de matière | Whether the prism joins the body or is taken out of it. |

## The part

| Code | Interface | What it names |
| --- | --- | --- |
| `Operation` | opération | One thing the user did, recorded. |
| `History` | historique | The list of them, in order, grouped into the steps of the design. |
| `Step` | étape | One major step — a sketch, an extrusion — and the operations under it. |
| `PartState` | — | The geometry, rebuilt from the history. |
| `rebuild` | — | Replaying the history from nothing. |
| `PartDocument` | pièce | The history and its metadata. What a `.caopart` holds. |
| `PointRef` | — | Which point an operation meant: an existing one, or a position. |
| `Variables`, `Variable` | variables | The part's table of named values, one for the whole part. |
| `Formula` | formule | A size as written: a number, or a calculation over the variables. |
| `VariableChange` | — | One change to the table, recorded in the history under no step. |

> **A formula names a variable by its rank, not its name.** Renaming a
> variable renames it in every formula, and a variable taken away keeps its
> rank and its last formula for whatever was written from it before. A size
> keeps its formula and is worked out again at every rebuild — never frozen to
> the number it came to.

> **The geometry is never the truth.** A `.caopart` is a design; `PartState` is
> a projection of it, cached in the file and dropped for a replay the moment it
> stops answering to that design. Undo, redo and stepping back are the same
> operation, which is why they cannot disagree.

> **An `Operation` is immutable once written.** Its meaning is frozen: changing
> how one replays changes what every existing file draws. A new behaviour is a
> new variant, never a new reading of an old one.

## The preferences

| Code | Interface | What it names |
| --- | --- | --- |
| `Profile`, `Profiles` | profil | A named, exportable set of preferences. |
| `Theme`, `Background` | thème, dégradé | Colours. |
| `Shortcuts`, `Chord`, `Key` | raccourci | A key combination bound to a `Command`. |
| `ToolbarLayout`, `Edge` | disposition | Where the toolbar sits and what is on it. |
| `RecentList` | récents | Parts opened lately. |

## Words this project does not use

| Not this | This | Why |
| --- | --- | --- |
| line (for a drawn one) | segment / trait | `Line` is the infinite one, in `construct`. Two different things. |
| entity, feature | element, operation | Borrowed from other CAD tools and meaning something else there. |
| delete | erase | Deletion marks; it does not remove. The word has to say so. |
| service, manager, helper | the thing it does | A name that says nothing hides a responsibility nobody chose. |
| enum of constants as integers | a named variant | The compiler checks variants at replay. Integers do not. |

## Keeping it true

The line between `cao_app` and everything under it is where French stops. A
layer below the interface returns a named case; the interface decides how it is
said, and later in which language. `crates/app/tests/architecture.rs` enforces
that, and counts what is left to move.
