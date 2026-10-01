# An exact kernel on planes and cylinders: the design #498 builds

#498's maquette, on a branch never merged. Four designs were written
independently on day 1 and judged against the sixteen cases, the harness and
the product; this is what was kept from them. [`exact-kernel-journal.md`](exact-kernel-journal.md)
says what was reached on which day.

## Scope

- **Surfaces:** planes and circular cylinders, nothing else.
- **Curves:** lines, circles, and the curve where two perpendicular cylinders
  meet. Two cylinders at any other angle than parallel or square, and a plane
  oblique to a cylinder's axis — an ellipse — are declined: the operation gives
  no answer, which the harness counts as such rather than as a wrong solid.
  Every leaf of the sixteen cases and of the campaign stands on a plane of the
  origin, so neither is needed to hold them.
- **Operations:** raise a profile of straight runs and arcs along its plane's
  normal, join, cut. Revolutions, cones, spheres and tori are out.

## Deciding once

Every failure truck showed, and the flats' #489 and #490, is one geometric fact
decided twice and answered two ways: coplanar at one corner and not at the
next, tangent in one routine and crossing in another. So each fact is decided
**once**, on the exact entities — a surface, a pair, a triple — and stored as a
symbol that every later step reads instead of measuring again.

There is one tolerance, `EPS = 1e-9 · reach`, `reach` being the largest
coordinate a body or its operands have reached, never less than one. It is ten
times the harness's `NEAR`, so what the kernel keeps apart the rules see apart,
and a thousand times below `ALONG_A_LINE`, so a merge never shows as a volume.
The result of an operation carries the larger reach of its operands.

The decisions, in the order taken, once per operation:

1. **Surface identity.** Each surface of the second operand is compared with
   the first's, never a body's surface with its own: planes one when parallel
   and their offsets within `EPS`; cylinders one when their axes are parallel,
   within `EPS` of each other over the box, and their radii within `EPS`. The
   first operand's copy is kept — the nearest, where two of the first's stand
   within `EPS` — and a plane standing strictly between two planes of the
   first within `EPS` of each is taken for neither: taken for one, it would
   stand, as its own operand was made, across the strip of wall between the
   two, where the boolean asks that operand which side a point is on. From
   here on, coplanar, coaxial and flush are comparisons of ids.
2. **The relation of a pair** of surfaces whose faces' boxes overlap: apart,
   one line, a tangent line, two lines, a circle, or the curve two
   perpendicular cylinders meet along with its special points. This is the
   only place a tangency is decided; once decided, it is built exactly — one
   line from a formula, never a double root.
3. **Line identity.** The same line comes out of several pairs; lines within
   `EPS` over the box are one, and their supports are joined — never across
   two surfaces decided apart, and never two curves of one operand, which
   the operation that made it kept apart. A line a pair's line was taken for is that
   pair's alone: another within `EPS` of it, on a surface apart from the
   first's, does not lie on the pair's surfaces for that.
4. **A line meeting a cylinder at a double root**, once per pair.
5. **Point identity.** Corners within `EPS` are merged, in a fixed order, and
   their supports joined — unless the merge would put one point on two
   surfaces decided apart, or on two surfaces, one from each, crossing along
   lines that all stand further than `EPS` from it, which is refused. No
   corner lies on a curve on a surface apart from one of its own.
6. **Arc identity.** Once every curve is cut into arcs at the pooled
   corners, two arcs lying on one surface between the same two corners,
   which part by no more than `EPS` anywhere along them — measured exactly
   for lines and circles, whose distance turns only where a closed form
   says, and on dense samples along the perpendicular curve — are one arc.
   One curve is kept for it: a line before a circle before a meet, then the
   first registered. The arc lies on the surfaces of both, over its own
   stretch alone, so supports are carried by arcs rather than by curves: an
   arc lies on its curve's surfaces and on those of the arcs taken for it,
   and an edge a later operation reads lies on its curve's surfaces all
   along and on the others over its stretch. Two arcs whose surfaces
   include two decided apart stay two.

   A plane decided tangent to a cylinder thus keeps its tangent line, and a
   circle that runs within `EPS` of a side between two corners becomes that
   side. The strip of the plane and the strip of the wall between the line
   they touch along and the side's edge are then bounded by the very same
   arcs: two regions of two surfaces bounded by the same arcs, a point
   inside each within `EPS` of the other surface, are one piece of surface,
   decided once from how each operand covers either and kept on the first —
   a plane before a cylinder, then the lower id. That is what keeps a skin
   thinner than the tolerance from being left between two faces back to
   back, and what gives an answer where a point of either strip stands too
   close to the other for a ray to wind it. The kernel declines, as a tie,
   where one operand covers both twins, which only a skin an earlier
   operation kept could make; the point inside each is what tells twins from
   the two caps two crossing cylinders bound with the one loop they meet
   along.

Everything else is derived. A vertex lies on a curve exactly when the curve's
support is among the surfaces the vertex lies on — but for the line two
surfaces decided tangent touch along: they stand within `EPS` of each other
over a band far wider than it, so a corner on both lies on that line only
within `EPS`. A line or a circle lying on a surface that does not carry it,
only because an arc of it was taken for an arc of the surface (decision 6), is
seen there as the segment between where its ends stand — a circle square to a
cylinder's axis or leaning on a plane only where its stretch stands within
`EPS` of that chord. A corner whose support holds only planes, three of
them spanning space, stands where they meet — each plane taken for another
within `EPS`, the place they fix moves by more. A triple of surfaces is
solved from the most degenerate of its three pairs: a tangent line first, then
any line against the third surface, then a circle, then the perpendicular
curve — which is crossed with a surface through the lines that surface makes
with one of its two cylinders wherever it makes any, rather than by scanning
it, since two cylinders crossing at a grazing angle stand within `EPS` of
each other over millimetres of it. What remains are signs of exact evaluations with no tolerance at all —
point in a face, order along a curve, order around a vertex — and the
decisions above are what keep every point they are asked about away from a
boundary.

No `HashMap` is iterated, no thread is used, sums run in a fixed order: the
same input gives the same bits.

## The body

- **Geometry is canonical.** A plane keeps the point nearest the origin and a
  normal with a fixed sign; a cylinder the point of its axis nearest the origin,
  an axis with a fixed sign, and a reference direction `u` read off the axis
  alone, so that parallel cylinders share where their angles start. The face
  carries which side the matter is on, not the surface.
- **Parameters.** A plane is read in `(s, t)` along its `u` and `v`; a cylinder
  in `(θ, h)`, the angle from `u` and the height along the axis. Every curve on
  a cylinder is either a ruling or a graph over `θ`, which is what point
  location, tessellation and the volume stand on.
- **Topology** is arenas of plain structs with `u32` ids: vertices with their
  point and the surfaces they lie on, edges with their curve and parameter
  range, faces with their surface, their side, and loops of oriented uses of
  edges. A loop has its face on its left, seen from outside the matter.
- **No seams.** A cylinder's face may go all the way round. A whole circle is an
  edge with no vertex — which is how the application will want to pick it, and
  how Parasolid holds it.
- **Non-manifold edges** have four uses, or six: two blocks touching along an
  edge, a hole tangent to a wall.

## The boolean

A classic boolean splits faces against faces. This one splits **surfaces**: all
the faces of both operands lying on one surface, and every curve where another
surface crosses them, go into a single overlay in that surface's parameters.
Flush, touching and coincident faces are then the ordinary path rather than a
special one: a coincident face is a region covered twice, a shared wall a region
covered once from each side.

1. **Identity** of surfaces (decision 1).
2. **Relations** of pairs whose faces' boxes overlap (2, 3).
3. **Corners.** Every edge curve of one operand against every surface of the
   other, and the special points of the relations (4, 5). A curve can only
   leave a face through the other operand's edge, and two crossings can only
   meet where the other operand's faces do, so these are all the corners.
4. **Arcs.** Every curve is cut at the corners lying on it, by support. An arc
   is kept if it is part of an input edge, or lies inside a face of each
   operand on two of its surfaces; kept arcs that are one are one (6).
5. **Overlay** per surface. At each vertex the arcs are ordered by tangent
   angle, ties broken by signed curvature — a second tie declines. Cycles are
   traced, then grouped into regions by a ray up the second parameter from
   each cycle's top. Each region gets an interior point.
6. **Winding.** For each operand, a region is covered with the surface's
   normal, against it, or not at all. Covered gives the winding on each side
   of it; not covered gives the same winding on both, from an exact ray cast
   through that operand's faces, retried along another direction when it
   grazes or lands near an edge. Twin regions of two surfaces are wound
   once, together (6).
7. **Selection.** A region is kept when the operation — or, or and-not — says
   something different on its two sides; its outside is the side where the
   operation is false. One rule gives coincident faces once, drops a shared
   wall, and leaves a non-manifold edge its four uses.
8. **Assembly.** Kept regions become faces and their arcs edges. Faces on one
   surface with one side are merged across an arc nothing else uses; two edges
   on one curve meeting at a vertex nothing else uses become one; a circle that
   loses its last vertex becomes a ring.
9. **Verification.** The listing is checked before the body is handed back.
   What the kernel cannot verify it does not answer.

## The curve two perpendicular cylinders meet

In a frame whose third axis is the first cylinder's and whose first axis is the
second's, the first is `x² + y² = a²` and the second `(y − d)² + (z − e)² = b²`.
For `y` in `[max(−a, d − b), min(a, d + b)]` the curve is four arcs,

    x = σ₁ √(a² − y²),   z = e + σ₂ √(b² − (y − d)²),

exact to rounding and monotone in `y` — a graph over the angle of either
cylinder. Each end of the interval is where one root vanishes and two arcs
join, or, within `EPS`, both: a singular point where four arcs meet. That gives
the configurations: two loops, one loop, a figure of eight through a node
(internal tangency), two ellipses crossing at two nodes (equal radii, axes
meeting), a single point of contact. Nodes are vertices. The parameter is
`y = m + ℓ sin ψ`, which keeps the roots free of cancellation at the ends.

## Triangles

- **Watertight by construction.** Every edge is sampled once, and every face
  using it takes those samples, bit for bit, with the vertices' own points at
  the ends. No sample between stands within `EPS` of an end, nor within
  rounding of any vertex: it would be that vertex a second time — a corner
  the kernel left a hair off the curve, which a ray through it lands on. A
  sample a hair from a vertex off the edge stays: on the ray through the end
  of a curve beside it, it keeps the two in order.
- **Sampling.** A line at its ends. A curve on a cylinder at the cylinder's
  grid `θ_k = 2πk/N`, `N` a multiple of four chosen so a chord stands within
  the tolerance asked of the surface, anchored at the cylinder's `u`: the
  points where planes of the origin touch a cylinder are always samples. A
  circle is also sampled at the angle of every vertex on its cylinder, so that
  a ruling from a vertex off the grid meets a sample on every rim of its wall:
  otherwise a strip a hair high — a skin left under a cap — is cut by a
  triangle reaching from the vertex to the next step of the other rim, lying
  flat over the face beside it. It keeps that ray even where it stands
  within the room of another wall that `Contact` speaks of below, where the
  rays passed between walls are dropped. That sample stands where the vertex nearest
  it in height stands, moved along the axis: a tangency decided within `EPS`
  leaves the exact surfaces overlapping by up to `EPS`, ten times what the
  rules tell apart, and the vertices of the line they touch along are where
  both walls' samples meet. A vertex standing inside a wall closer than
  twice a chord's sag — a pocket's corner a hair inside it — gives the wall's
  circles a sample at its angle too, so that no chord passes inside it: within
  the room of another wall and beside an arc's end alike, where a third wall
  crossing two that touch, a hair from the line they touch along, leaves a
  vertex of the inner one just inside the outer, whose arc ends where the
  third crosses it. An
  arc ending where a plane touches its wall takes no sample so near that end
  that it would stand on the plane's edge, and no circle of the wall takes a
  ray so near the line the plane touches along that it would stand on the
  plane: a strip of the wall a hair wide would lie on the plane's face. A
  plane touches a wall only where the body holds the line, a vertex lying on
  both: the plane of a face far off along it touches nothing, and the rays it
  would take from the wall leave the rim of a disc a bore hollowed a hair
  off its axis without a sample for two steps. Two cylinders touching are both
  sampled on the line they touch along, vertex or not, and a circle whose
  wall is gone is sampled as its own cylinder's. An arc ending on that line,
  though, is sampled there at its end alone: the kernel may lay the line
  leaning a hair, its two ends within `EPS` of both walls, and a ray through
  the other end, or through the exact line, would put a sample a hair from
  the vertex, as good as on the other wall's arc ending at the same vertex —
  a cap holding both arcs would fold back on itself there.
- **Faces** are cut into vertical strips in their parameters, at every vertex,
  at every place a curve turns back, and on a cylinder at every grid angle.
  Inside a strip the arcs are graphs that do not cross, so each piece of face
  is a band between two of them, zipped into triangles. No triangle spans more
  than one grid step, so none leaves the cylinder by more than the tolerance.
- **Contact.** Chords sag towards their own cylinder's axis, so against a plane
  or a convex neighbour they retreat. Where two curved faces face each other on
  their hollow side closer than a chord's sag — an internal tangency, a near
  miss — both are sampled on common abscissae along the common tangent, or on
  common rays from the smaller centre further out, so that their polylines
  stay ordered. So are two walls crossing each other by less than twice a
  chord's sag — cylinders of one radius a hair off one axis — and every ray
  either takes on its own, through a vertex, a curve it meets a third wall
  along or a line it touches or crosses a third parallel wall along, the
  other takes too, and so on down a chain of walls each touching the next
  inside it: a circle printed on a floor, its wall touching the stock inside,
  takes the line its wall crosses a bore along, and the stock's rim, not
  taking it, would chord inside the printed circle. A ray along which the two stand closer than a
  fifth of `EPS` — twice what the rules tell apart — neither takes: the whole
  of `EPS` would leave walls barely more than it apart with hardly a sample
  round the turn. Two walls decided to touch take no ray either where one
  stands on the wrong side of the other, however deep: the kernel may leave
  them overlapping by up to `EPS`, one poking through the other round the
  line they touch along, and a ray passed on there puts a sample of a cap's
  arc on the wrong side of the other. On the side they were decided to
  stand on, a fifth of `EPS` is room enough, and the whole of it would
  withhold a wide arc of two walls nearly concentric. The gap is measured
  along the common ray from the smaller wall's axis, the same sum on
  either wall, so that a ray is crowded for both or for neither. A circle belongs to
  the nearest wall it lies on: two walls decided apart may both hold it once
  a later leaf grows the tolerance, and an end lies on a surface other than
  its circle's own only as far as rounding allows. Nor does a wall take such a
  ray from a third it shares rays with: a bore touching the stock inside passes its grid on to the stock,
  and the stock would pass back the very steps the bore withholds beside a
  boss crossing it. Such a ray is withheld only where both walls hold a face,
  at a height they share: a union keeps each wall where the other is gone,
  and two cylinders stacked keep each wall at its own heights, so there the
  place is one wall's alone and the step stays, or the chord across would sag
  past the tolerance. And it is withheld only from the circles bounding a
  stretch of height both walls hold a face over there: the rim of a disc
  left whole above a crescent two walls bound below it faces nothing, and
  keeps its whole grid. Its triangles reach down the wall to the circles
  that do withhold the step, and sag less than the other wall's chords
  beside the line the two meet along: that keeps them clear of it where it
  stands on the rim's hollow side further than the rules tell apart, and
  would have them pass through it anywhere else — a bore's rim above a
  boss it touches inside — so there every circle of the wall withholds the
  step. A wall holding no face, its circles printed on a cap, faces the
  other at the heights of its circles alone: a circle printed round a hole
  it touches inside is sampled in common with the hole's rim on that cap,
  and the hole's other rims are not. Two walls of radii apart
  decided to touch face each other too where a circle of each stands at one
  height at that angle, though the walls stand at heights apart: a pocket's
  floor round a hole touching the pocket's wall inside is bounded by both.
  Not two walls crossing or all but one: a union keeps both circles on its
  caps all round. A place two rays put at one vertex is taken once. A
  curve two perpendicular cylinders meet along is sampled on every ray the
  circles of either take: a wall a hair inside one of them takes the rays of
  that one's other curves, and its chords, shallower between two samples of
  the curve than the curve's own, would pass under the face beyond it. It is
  not sampled where either wall all but lies on one facing it, as a circle
  withholds its steps there, nor where it all but lies on a plane touching
  either wall off the line they touch along: a bar a hair proud of a side a
  cylinder touches meets the cylinder a hair from that line, and a sample
  there lays a strip of the wall on the side. Nor does an arc take a ray a step or less from
  its end where it all but lies on a surface that end lies on and it does
  not: a wall grazing the plane of the arc there stands on the plane's face
  over a band far wider than the rules tell apart. Nor a place of its grid
  where it stands within a thousandth of `EPS` of such a surface: two circles
  of one radius crossing a hair apart stand within the rounding of each other
  a long way round from their vertex, and a place of each there, on no ray
  they share, has the two arcs of a cap cross back and forth. What
  still crosses is refined locally, and what survives
  that is reported as a failure of the triangles, apart from the kernel's.

## How it is held

The harness's line rule compares a result's triangles with its inputs'. That
works for the flats, whose result is made of its inputs' own facets, and not
for a kernel that facets each body afresh: a line grazing a wall sees two
different chords.

- **The table**, per case: the rules on the triangles (closed, uncrossed,
  repeatable); the volume of the triangles against the arithmetic within
  truck's `MESHED`, so the columns read alike; the exact volume of the body,
  by the divergence theorem over its exact faces, against the arithmetic; the
  listing verified; the time of the boolean.
- **The campaign**, where there is no arithmetic for a whole case but there is
  for every line: the promise along each of the harness's lines is computed
  from the leaves by arithmetic written in the test — a slab against a
  rectangle or a disc — and combined step by step with `Spans::union` and
  `without`. Against it:
  - the body's exact spans along the same line, at `ALONG_A_LINE`;
  - the triangles' spans, with a slack of the tolerance over the cosine at each
    crossing, a line grazing a cylinder left out and counted;
  - the rules on the triangles, `within_reach`, and the listing.
- **The listing** is checked by `soundness::listed`, written apart from the
  kernel with its own formulas: every edge on the surfaces of the faces beside
  it, every end on its vertex, every vertex on the surfaces around it, loops
  closed, uses balanced.

## Where it lives

`crates/solid/src/brep.rs` and `brep/`, beside truck's `exact.rs`. The profile
vocabulary both share — `Contour`, `Run`, `Frame` — is `profile.rs`.
