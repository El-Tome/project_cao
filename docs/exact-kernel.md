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
   first operand's copy is kept. From here on, coplanar, coaxial and flush are
   comparisons of ids.
2. **The relation of a pair** of surfaces whose faces' boxes overlap: apart,
   one line, a tangent line, two lines, a circle, or the curve two
   perpendicular cylinders meet along with its special points. This is the
   only place a tangency is decided; once decided, it is built exactly — one
   line from a formula, never a double root.
3. **Line identity.** The same line comes out of several pairs; lines within
   `EPS` over the box are one, and their supports are joined.
4. **A line meeting a cylinder at a double root**, once per pair.
5. **Point identity.** Corners within `EPS` are merged, in a fixed order, and
   their supports joined — unless the merge would put one point on two
   surfaces decided apart, which is refused.

Everything else is derived. A vertex lies on a curve exactly when the curve's
support is among the surfaces the vertex lies on. A triple of surfaces is
solved from the most degenerate of its three pairs: a tangent line first, then
any line against the third surface, then a circle, then the perpendicular
curve. What remains are signs of exact evaluations with no tolerance at all —
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
   operand on two of its surfaces.
5. **Overlay** per surface. At each vertex the arcs are ordered by tangent
   angle, ties broken by signed curvature — a second tie declines. Cycles are
   traced, then grouped into regions by a ray up the second parameter from
   each cycle's top. Each region gets an interior point.
6. **Winding.** For each operand, a region is covered with the surface's
   normal, against it, or not at all. Covered gives the winding on each side
   of it; not covered gives the same winding on both, from an exact ray cast
   through that operand's faces, retried along another direction when it
   grazes or lands near an edge.
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
  the ends.
- **Sampling.** A line at its ends. A curve on a cylinder at the cylinder's
  grid `θ_k = 2πk/N`, `N` a multiple of four chosen so a chord stands within
  the tolerance asked of the surface, anchored at the cylinder's `u`: the
  points where planes of the origin touch a cylinder are always samples. A
  circle is also sampled at the angle of every vertex on its cylinder, so that
  a ruling from a vertex off the grid meets a sample on every rim of its wall:
  otherwise a strip a hair high — a skin left under a cap — is cut by a
  triangle reaching from the vertex to the next step of the other rim, lying
  flat over the face beside it. That sample stands where the vertex nearest
  it in height stands, moved along the axis: a tangency decided within `EPS`
  leaves the exact surfaces overlapping by up to `EPS`, ten times what the
  rules tell apart, and the vertices of the line they touch along are where
  both walls' samples meet. Two cylinders touching are both sampled on the
  line they touch along, vertex or not, and a circle whose wall is gone is
  sampled as its own cylinder's.
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
  either takes on its own, through a vertex or a curve it meets a third wall
  along, the other takes too, and so on down a chain of walls each touching
  the next inside it. A ray along which the two stand closer than a
  fifth of `EPS` — twice what the rules tell apart — neither takes: the whole
  of `EPS` would leave walls barely more than it apart with hardly a sample
  round the turn. What still crosses is refined locally, and what survives
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
