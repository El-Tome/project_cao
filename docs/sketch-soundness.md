# Soundness: the rules every area of a drawing keeps

Nobody can write down the areas of a random drawing, so they have no expected
answer to be held against. What can be written down is what the areas of
**every** drawing must satisfy, whatever is drawn — and a generator of random
drawings that throws them at the walk until one of them breaks a rule. That is
#500, the same idea as [the one for solids](soundness.md) turned on the drawing.

The hand-written scenarios in `crates/sketch/src/regions/tests.rs`,
`arc_regions/tests.rs` and `crates/sketch/tests/` stay: a named case that once
failed is worth more than a random one that passes. This adds the cases nobody
listed.

Everything lives in new files under `crates/sketch/tests/`, which reach
`cao_sketch` and nothing else — no crate is added, and nothing is borrowed from
`cao_solid`'s harness: the random numbers, the shrinking and the campaign are
written again in `random_sketches/`, after the same ideas.

## The rules

They live in `crates/sketch/tests/random_sketches/rules.rs`, and are checked
after **every** gesture of a drawing.

| Rule | Function | What breaks it |
| --- | --- | --- |
| Tint is measure | `tint_is_measure` | the triangles an area is tinted with cover more or less than its outline encloses |
| Nothing missing | `nothing_missing` | a place something closes around lies in no area |
| Nothing extra | `nothing_extra` | a place lies in two areas, or in one while nothing closes around it |
| Order does not matter | `laid_alike` | the same drawing laid again — in another order, moved, turned — gives another count of areas, or puts a place in an area of another size |
| Answers | `answer` | finding the areas panics, or does not come back within the patience given |

The rules read **plain data** — outlines, holes, triangles, places — and never
a `Sketch`, so that each is caught out by a list of areas broken on purpose in
`every_sketch_keeps_its_areas.rs`. A few decisions are worth knowing before
reading a report.

**The tint covers the holes, on purpose.** A shape drawn inside another is
tinted twice ([`sketch.md`](sketch.md), *Closed surfaces*), so an area's
triangles are held against what its **outline** encloses: `Region::area()`
with the holes added back, each hole being the outline of an area one level
deeper. A place lies *in* an area when it lies in its triangles and in none of
its holes.

**The measure is read on the curves, the tint on their steps.** A circle is
tinted as the straight steps it was sampled into, a forty-eighth of a turn
each at most, while `Region::area()` is exact on the curve. A step `c` long
across a stretch of circle turning by `θ` leaves out
`c² (θ − sin θ) / (8 sin²(θ/2))`, which grows with `θ`; across an ellipse, at
most that times how much longer the ellipse is than it is wide. The tint may
stand from the measure by that much for every curved step the outline
actually walks — read off the outline, not off the curves it names, so that a
small piece cut from a big circle is allowed only its own few steps — plus a
little for the ends a weld moved off the curve. That allowance is written in
`random_sketches/areas.rs`; a straight outline gets none.

**Whether a place is enclosed is never read from the walk under test.** A rule
checked by the code it checks would agree with every bug. It comes from a
reckoning of its own, `random_sketches/enclosing.rs`: the curves are cut
wherever they cross, touch or end on one another into a graph with no order
round a vertex and no faces, and a place is enclosed when one of the graph's
fundamental loops winds round it. That is enough, and needs no walk: a place
shut in is wound round once by the boundary of the piece of plane it sits in,
which is a sum of fundamental loops. The winding is read exactly — a straight
step turns by the angle it is seen under, a curved one by that angle plus a
whole turn when the place lies between the curve and its chord.
`what_closes_around_a_place.rs` holds the reckoning to drawings whose answer is
known.

**An area is closed, or it is no area.** Two places nearer than twice the
drawing's own `THE_SAME_PLACE` (`crates/sketch/src/edges.rs`) are one place for
the reckoning; two farther apart than five times it are two. A shape missing
its last stretch by a hair encloses nothing, and an area there is extra.

**The places looked at** are a grid across the drawing, the middle of every
triangle of tint, and a few places either side of every curve — none of them
nearer a curve than its tint could stray from it, nor than the rounding a weld
works with. Every tolerance is read against how far the drawing reaches.

**Laid again** means the same points and curves added to a fresh sketch in the
reverse order, each trait drawn from its other end; or moved by a step of the
drawing's own size, chosen so that the fresh sketch's origin lands clear of
everything; or turned a quarter turn, which is exact, and by thirty degrees,
which is not. Rules and values are left behind: no area reads them.

The walk welds a place onto a curve within a distance that grows with how far
out the place stands, so laying a drawing again moves what it welds. An area
laid again may differ in size by that distance, read where its own corners
land, times how far round it is, and by the rounding of the sums it is
measured with — all of it read on that area alone, so that a small area
cannot change by a fifth unseen beside a large one, however far the drawing
reaches. And a drawing is not moved or turned a given way when one of its
near misses lies between the distance the walk welds at where the miss
stands and the distance it welds at where the miss lands: welded in one
laying and not in the other, that miss changes the drawing itself, not the
walk's answer to it. A quarter turn lands every place exactly as far out as it
stood, and is never set aside.

## The drawings

`random_sketches/drawing.rs` draws a list of gestures from a seed, with every
tool of the sketch mode:

- **drawing** — a chain of traits closed or left open, a rectangle, a circle,
  an arc, an ellipse, half an ellipse, a lone point, each of them now and then
  as construction geometry;
- **changing** — a fillet, a chamfer, a mirror, a circular and a rectangular
  pattern, a division at a crossing, a trim, an erasure.

No value, no rule and no dragged point: those make the solver move the
drawing, and they are #501.

A gesture acts on what lies **at a place** rather than on an index, the way a
click does, and `random_sketches/laying.rs` lays it the way its tool does
through `cao_part` and the application: a click landing on a point reuses it,
one landing on a curve is held there, a rectangle's two other corners are
always new points, a circle keeps the point its radius was given at, the
ellipse tool by its ends draws only the half on the side of the rise. Nothing
is solved afterwards. Undo lives in `cao_part`, out of reach here; #448's part
campaign holds it.

A drawing in general position does not break, so the generator leans towards
where it does. Corners sit on a lattice of round numbers, so that sides land
along each other on their own. Half the shapes are drawn from one already
there: from its corner, along its side, inside it against its edge, standing on
it, sharing a side, about its centre, with a radius that makes two circles
touch, inscribed in a rectangle. Some drawings close nothing. Now and then a
coincidence is **missed by a hair** — from five times the distance under which
the drawing calls two places one, read forty times the drawing's unit out from
the origin, further than the places a hair is drawn at, up to a thousand times
that, never finer. `the_generator_draws_with_every_gesture` holds the *never
finer* on every pair of places its drawings are drawn through. Some drawings
are thirty times larger, where a tolerance taken in absolute units stops
holding.

## Running a campaign

A campaign is not a unit test. It is ignored by the gate and run by hand,
bounded by a deadline rather than a count:

```sh
CAO_FUZZ_SECONDS=300 cargo test --release -p cao_sketch \
    --test every_sketch_keeps_its_areas -- --ignored --nocapture
```

`CAO_FUZZ_SEED` starts from a given seed rather than from the clock;
`CAO_FUZZ_PATIENCE` is how many seconds a single drawing may take before it
counts as no answer. They mean what they mean for the solids. `--release` is
worth it: the same minutes try several times more drawings.

The deadline is checked between drawings, so a run can overshoot it by one
patience. The seed being tried is written on the standard error as it goes:
a walk that blows its stack ends the program, which nothing can catch, and the
last seed written is the one to run again.

A campaign shrinks the first three failures of each rule and counts the rest.
One common cause can hide a rarer one behind it: several campaigns from
different seeds see more than one long one.

## Reading a finding

Every drawing that breaks a rule is shrunk: gestures are dropped, curves made
straight, corners cut rather than rounded, copies made fewer times,
construction drawn plain and numbers rounded, for as long as the smaller
drawing still breaks **the same rule**. A drawing shrunk into a different
failure is a different bug, and would describe neither. The report prints how
many drawings broke each rule, and for the first few of each, the flaw, after
which gesture it broke, and the shrunk drawing as a test ready to paste:

```text
── TintIsMeasure, seed 11 ──
as drawn: Tint { near: …, tinted: 22.77…, measured: 20.27… }
shrunk:   Tint { near: DVec2(0.0, -10.0), tinted: 525.0, measured: 475.0 }, after gesture 2 of 2

#[test]
fn seed_11_keeps_the_rules_of_its_areas() {
    random_sketches::holds(&[
        Gesture::Chain { through: vec![[10.0, 5.0], [20.0, 5.0], [25.0, 10.0]], closed: true, construction: false },
        Gesture::Rectangle { corner: [30.0, 5.0], opposite: [0.0, -10.0], construction: false },
    ]);
}
```

Numbers are printed the way `{:?}` prints an `f64` — the shortest text that
reads back as the same bits — so the drawing pasted is the drawing that failed
and not a neighbour of it. A number that is one of the constants Clippy refuses
to see written out comes out as that constant. `cargo fmt` lays the pasted
test out.

A flaw names a place to go and look at: `Missing { at }` and `Extra { at }` a
place that lies in the wrong number of areas, `Tint { near }` the lowest corner
of the outline whose tint is off, `Relaid { how, areas, at }` how the drawing
was laid again, the two counts of areas, and a place whose area changed size.

## What the rules cannot see

- **One area where two are drawn**, when the one is tinted and measured right:
  every place still lies in exactly one area. A diagonal drawn before the
  square on its ends was that until #494 welded points standing in one place.
- **The matter.** The rules hold the tint, `Region::triangles`. What an
  extrusion raises is `Region::face_triangles`, the outline with the holes cut
  out of it, and is not checked here.
- **What only the mirror image shows.** Every way of laying a drawing again
  keeps which way round it is, while the walk turns as tightly as it can and
  so depends on it. A drawing whose mirror image breaks a rule passes, until a
  campaign happens to draw the mirror image itself. Laying it again mirrored
  takes every arc and every stretch of ellipse drawn back to front, and was
  set aside for now.
- **Dimensions, rules, dragging**, and whether a piece is entirely
  constrained: #501.
- **The solid** and undo: #448.

## Where a finding goes

Each distinct failure goes to **the issue that names its cause**, or to an
issue of its own when none does, with the shrunk drawing in it. Fixing it is
not the harness's job.

A drawing whose issue is still open is kept as a named test under
`#[ignore = "#n"]` in `crates/sketch/tests/what_random_sketches_found.rs`, so
that it stays in the repository and runs with `--ignored`. The commit that
fixes it takes the `ignore` off, and from then on the gate holds it. Whether
campaigns run on a timer follows #495's answer for the solids.

## What the first campaigns found

Seven campaigns of 300 s each, in `--release`, from seeds 1,000, 50,000,
900,000, 2,000,000, 3,000,000, 4,000,000 and 5,000,000: some 190,000 drawings,
as they were run, the last two with the order rule as it stands. About a third
of them break a rule, nearly all for one of five causes. No drawing panicked or
went without an answer.

| Cause | Issue |
| --- | --- |
| Two curves lying along each other between the same two places: a side along another, a trait or a circle drawn twice | #493 |
| Corners standing in one place without being one point: a copy whose corners land on the original's or on an earlier copy's, a rectangle drawn from another's corner | #494 |
| A touch read one way or the other depending on rounding: an ellipse or a circle touching a side or another curve, and a crescent tinted across the place it is pinched at | #419 |
| A circle touching the one around it at their lowest point, read as inside it and around it at once | #503 |
| An outline with a corner lying on one of its own diagonals, the side it lies on read by rounding, and tinted with a triangle wound backwards | #504 |

Each is kept, shrunk, in `what_random_sketches_found.rs`. Where two causes
meet, the case sits under the one whose fix it waits on last.
