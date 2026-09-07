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
| `Sketch` | esquisse | A 2D drawing on one plane, with its points, segments, circles, dimensions and constraints. |

> **The aggregate.** Nothing inside a `Sketch` is valid on its own — the solver
> resolves the whole of it at once. Its fields are private and stay private:
> every change goes through the root, or the invariant the solver maintains is
> lost.

| Code | Interface | What it names |
| --- | --- | --- |
| `WorkPlane` | plan de travail | The plane a sketch is drawn on, and the frame its coordinates are given in. |
| `PointId`, `SegmentId`, `CircleId` | — | A rank in the sketch's own list. Not an identity that survives anything else. |
| `Segment` | trait | A straight piece between two points. Never called "line": a `Line` in `construct` is an infinite one used for tangency. |
| `Circle` | cercle | A centre and a radius. The radius is an unknown of the solver, like any coordinate. |
| `Element` | élément | Whichever of point, segment or circle a rule is about. |
| `Region` | aire | A closed loop of the drawing, the thing an extrusion can be raised from. |
| `Dimension` | cote | A rule that carries a number: length, angle, radius, diameter, distance. |
| `Constraint` | contrainte | A rule that carries no number: perpendicular, parallel, equal, tangent, on-segment. |

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
| `millimeters_per_unit` | échelle | Millimetres one world unit is worth. Undefined until the first dimension is typed, which is what sets it. |

## The matter

| Code | Interface | What it names |
| --- | --- | --- |
| `Mesh` | maillage | Triangles. What is drawn, never what is saved. |
| `body` | matière | The whole part as a single surface, not a pile of separate lumps — so a pocket cut in a block really is a hole in the block. |
| `Polygon` | — | One planar face, its points in order. |
| `prism`, `revolution` | prisme, révolution | Raising a region into matter, straight or turned about an axis. |
| `ExtrusionMode::Add` / `Cut` | ajout / enlèvement de matière | Whether the prism joins the body or is taken out of it. |

## The part

| Code | Interface | What it names |
| --- | --- | --- |
| `Operation` | opération | One thing the user did, recorded. |
| `History` | historique | The list of them, in order. |
| `PartState` | — | The geometry, rebuilt from the history. |
| `rebuild` | — | Replaying the history from nothing. |
| `PartDocument` | pièce | The history and its metadata. What a `.caopart` holds. |
| `PointRef` | — | Which point an operation meant: an existing one, or a position. |

> **The geometry is never saved.** A `.caopart` is a history; `PartState` is a
> projection of it. Undo, redo and stepping back are the same operation, which
> is why they cannot disagree.

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
