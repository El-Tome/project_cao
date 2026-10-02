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
One pair alone is read wider, at twenty times it: two parallel walls of
one radius (decision 8). The result of an operation carries the larger reach of its operands. A pair
of surfaces one operand alone carries is decided at the tolerance it was
decided at when the later of the two came into that operand: a body keeps,
for each of its surfaces, the scale of the operation that brought it in —
its own for the first operand's surfaces the second does not carry, the
operation's for the others — and a pair is read at the larger of its two.
A later leaf growing the reach, however many operations later, does not
make one of two walls of a crescent a hair wide, crossing along two rulings
the operand's own corners stand on. The second operand's own pairs are
read at the operation's scale: it is a leaf, whose surfaces stand far
apart.

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
   line from a formula, never a double root. Two perpendicular cylinders,
   one of each operand, a hair from touching — inside, outside, or at a
   node — are made to touch: the one the second operand alone carries is
   moved onto the touch, by less than `EPS`, as a surface, before any curve
   or corner is found on it, so that the curve they meet along and every
   corner found on that cylinder stand on the one cylinder. A cylinder
   carrying the first operand's corners is never moved, or they would stay
   on its old wall: where the pair would move it, its partner moves instead,
   across it the other way, and takes its radius at a node of two of one
   radius. A plane and a cylinder, or two
   parallel cylinders, one of each operand, decided to touch along a line a
   hair apart are made to touch the same way: the surface the second operand
   alone carries is moved onto the touch — a cylinder along the plane's
   normal or towards the other's axis, a plane along its own — so that the
   line they touch along, and every corner on it, stands on both rather than
   on one and a hair off the other. A surface both operands carry is never
   moved. Only pairs whose faces' boxes meet touch: a plane whose face is far
   off touches nothing. Every touch a surface has is read before it moves,
   and a move is made only where every other touch of the surface ends as
   near exact as it was: moved onto one touch, a cylinder in two would break
   the other, unless the move runs along it — a bar touching a floor and a
   ceiling exactly moves along them onto a post it touches. A wall its
   operand drew corners on, slid along an exact touch alone, would take the
   line of touch along and leave the corners drawn on that line behind, a
   rounded corner's where its straight run starts; left where it is, it
   stays a hair off the other touch. It is slid with its operand: the
   second operand is moved by the slide as decision 8 moves it, its
   corners, its curves and the surfaces they stand on along, before
   decision 1 reads it. Where that move is not made, the wall is not slid.
   A cylinder
   touching two parallel planes on opposite sides is moved midway between
   them, its radius half their gap, and touches both exactly.
   A surface moved is the boolean's: its pairs, though one operand alone
   carries them, are decided at the boolean's tolerance, and it comes into
   the result at it — moved, it no longer stands where its operand decided
   them.
3. **Line identity.** The same line comes out of several pairs; lines within
   `EPS` over the box are one, and their supports are joined — never across
   two surfaces decided apart, and never two curves of one operand, which
   the operation that made it kept apart. A line a pair's line was taken for is that
   pair's alone: another within `EPS` of it, on a surface apart from the
   first's, does not lie on the pair's surfaces for that. An operand's edge
   beside a face whose surface was taken for another or moved onto a touch
   is laid on the curve its surfaces now share, the nearest within twice
   `EPS`: each of the two may stand up to `EPS` from where the operand drew
   it, and a bore's rim moved onto a side, its floor taken for one a hair
   off, stands √2 hairs from the circle they share. Which of those surfaces
   carry the edge all along is read off the operand as it was built.
4. **A line meeting a cylinder at a double root**, once per pair. A line
   passing within `EPS` outside a wall touches it once, where it passes
   closest. A line passing within `EPS` inside it crosses it twice, its
   roots tolerances apart however thin the hair — unless a surface it lies
   on was decided to touch the wall there (decision 2): along a line, a
   plane or a parallel wall touching it, or at a point, the node or the
   contact of two perpendicular cylinders, which the line passes within
   `EPS` of. A corner between the two roots would stand tolerances off every
   other curve through either; a ruling through a node only touches the
   other cylinder, whatever rounding leaves of the touch. A circle against a
   plane or a perpendicular wall is read the same way, through the lines
   their planes share.
5. **Point identity.** Corners within `EPS` are merged, in a fixed order —
   the operands' own corners, the relations' special points, the crossings
   three planes fix, then the others, so that the crossings a curve makes
   near a place three planes fix are merged into it rather than into each
   other — and their supports joined — unless the merge would put one point on two
   surfaces decided apart, or on two surfaces, one from each, crossing along
   lines that all stand further than `EPS` from it, which is refused. No
   corner lies on a curve on a surface apart from one of its own.
6. **Arc identity.** Once every curve is cut into arcs at the pooled
   corners, two arcs lying on one surface between the same two corners,
   which part by no more than `EPS` anywhere along them — measured exactly
   for lines and circles, whose distance turns only where a closed form
   says, and on dense samples along the perpendicular curve, each against
   the other's arc rather than its whole curve: two halves of two circles a
   hair apart, one each side, run between the same corners within a hair
   of each other's circle all along — are one arc.
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
   close to the other for a ray to wind it. Where one operand covers both
   twins it holds a skin or a crack an earlier operation kept, thinner than
   this one's tolerance: its two faces turning their matter towards each
   other, a skin, it wraps neither side once the sheet is gone; turning it
   away, a crack, it wraps both. Which way is towards is read off the pair
   as that operand decided it, at the tolerance it decided it at — not at
   this one's, which may take a slice for a touch. Decided to touch, it is
   not measured: a cylinder touching a plane stands on its axis's side of
   it. Decided to cross — a plane slicing a hair off a wall, two parallel
   cylinders a hair apart crossing at a grazing angle — the two stand one
   above the other all along the stretch between the lines they cross
   along, the wall's slice beyond the plane from its axis, read at the
   middle of that stretch, where they stand furthest apart. Other twins
   are still a tie. The
   point inside each is what tells twins from the two caps two crossing
   cylinders bound with the one loop they meet along.
7. **The band.** A plane and a cylinder decided to touch, or two cylinders
   touching or of one radius crossing at a grazing angle, stand within
   `EPS` of each other over a band about `√(2·r·EPS)` wide — a micron or
   more, a thousand times the tolerance. Each pair is decided apart, so a
   third surface or line within that band leaves two or three lines a hair
   apart, and strips of face between them thinner than `EPS`, bounded by no
   arcs a strip of the other surface shares: decision 6 takes none of them,
   and a strip's point stands femtometres off the other surface, where no
   ray tells which side it is on. The band is decided once, topologically —
   never by moving a surface across it, which would move it by a volume
   the rules see. A region of a surface standing within `EPS` of another
   surface all across, the two lying along each other, is decided with
   that surface: an operand none of whose faces on the region's surface
   covers it, but one of whose faces on the other lies over the region's
   point — its foot on that surface located in the face — wraps the region
   once where it stands on that face's matter side, and not at all on the
   other. Two such faces on one side, the walls of a cusp both lying under
   a plane they touch, are read by the nearer. Which side of a face the
   region stands on, and which of two is nearer, is read off the pairs as
   the arena decided them, not measured: a cylinder touching a plane stands
   on its axis's side of it, the inner of two touching cylinders inside the
   outer, and two surfaces crossing stand one above the other all along
   each stretch between the lines they cross along, read at its middle —
   of two walls, a point of one inside the other has the other's wall
   beyond it where the two face one way there, a bore crossing a bore it
   all but touches inside, and behind it where they face each other, a pin
   dipping into a stock. All across is read on the region's point and
   along every arc bounding it: both surfaces are smooth and the region
   thin, so a region whose boundary stands within `EPS` of a surface stands
   within it inside too. Two planes are never read this way — decision 1
   takes them for one or keeps them apart — nor two surfaces crossing
   steeper than a band can turn, which a sliver stands within `EPS` of only
   for being a sliver.
   Where the foot lands on the face's boundary, where the pairs do not
   order two faces on one side, or where the faces either side, or the
   twins of one region, disagree, a ray is cast as before.

   The region is wound beside the face, not covered by it. Taken for the
   face's twin and covered as the face covers it, as two regions bounded
   by the same arcs are, it was kept where the face's own region, decided
   at its own point beyond the band, is kept too — a skin of two faces —
   or dropped where the exact geometry keeps it, and the regions around
   it, decided exactly, lost the partner along their arcs. A region one
   operand covers on both sides is left to its own face for the same
   reason: taken for a skin covered by neither, region by region, it left
   the skin's other regions, bounded by other arcs, without theirs. Read
   off the decided pairs, the winding agrees with a ray wherever the
   region's point stands clear of the faces by more than rounding, follows
   the pairs where rounding alone puts the point on one side — a bore
   decided to touch inside another, its strip 4e-15 outside the other by
   rounding — and answers where no ray can.
8. **Two walls of one radius a hair apart.** Two parallel cylinders, one
   of each operand, their radii within `EPS` and their axes within
   `G = 20 · EPS` of each other — two hundred-millionths of the reach —
   are one surface, the second operand's taken for the first's as decision
   1 takes
   one within `EPS`. This is the kernel's one tolerance wider than `EPS`,
   and it is kept to the one pair whose crossing is ill conditioned. Two
   parallel planes a hair apart never cross: the skin between them is well
   defined and kept, and case 8 keeps its floor of a ten-millionth. Two
   cylinders of one radius `d` apart cross along two rulings at an angle
   `d / r`, and a band a thousand tolerances wide surrounds the crossing,
   where every corner a third surface makes is ill conditioned. The
   crescent merged away is at most `G` thick and is almost always what was
   meant: a slot's cap, a rounded corner or a bore drawn on a hole of its
   radius, a hair off.

   Twenty, bounded rather than measured alone. A line crossing the moved
   wall at a slant sees it moved by the offset over the cosine, at each of
   its two crossings, and the harness holds lines down to a cosine of a
   twentieth: a move of `G` is seen as forty `G` at worst, under the
   thousand tolerances a line resolves only while `G` stays under
   twenty-five, and twenty keeps a fifth of that room for the curve of the
   wall. The hundred first set was seen by a campaign of profiles, a disc
   moved onto a slot's cap at a slant of a sixth (80 502 201); the fifty
   round 5's kernel lane chose next went unseen over some forty-eight
   thousand random cases, but a block bored twice forty-eight tolerances
   apart is seen at a slant of a nineteenth, a line holding half again its
   room (the review of round 5). A merge the rules can see is a wrong
   answer; two walls left two decline, at worst, in the band of 1-9.

   What the lower hair costs, on five-minute campaigns of profiles, counted
   over the seeds both runs reached: from seed 80 500 000, against the 85
   failures of the kernel before decision 8, `G` of ten tolerances fixed 47,
   twenty 57, twenty-five 61, fifty 65, a hundred 65 while making one, a
   thousand 65 while making 104, the merge seen along lines wherever it was
   made; from 82 500 000, against 104, twenty fixed 71, twenty-five 74 and
   fifty 78. What twenty leaves of what fifty fixed is two walls of one
   radius between twenty and fifty tolerances apart, crossing at a grazing
   angle. At fifty, the crescents merged were at most `5e-8` of the reach
   thick and `1.3e-7` of its cube in volume, half of them under `1e-9`.

   What the second operand built on the wall goes with it, as it does
   after decision 1: the second operand is moved square to the axis, by
   the offset, over every surface a corner or an edge of a moving surface
   lies on — a slot's sides with its cap, its other cap with its sides —
   and over the corners and the curves on them, so that its own decisions
   hold exactly as it took them and nothing of it is left a hair off the
   merged wall. A surface the move slides along itself, a cap square to the
   axis or a side along the offset, stays. The move is made on the second
   operand before decision 1 reads it, so that identity, the moves onto a
   touch, the lines and the corners are all decided on the moved operand.
   It is not made where it would part a surface it carries from one of the
   first operand's it was one with or touched — a slot's side flush with
   the body's, the body's wall a hair across from its cap — nor where it
   would move the curve two perpendicular cylinders meet along: the two
   walls are then left two, as before. The wall itself parts from what it
   touched freely: it is the first's now, and stands with every surface as
   the first operand decided.
9. **The band laid out.** Decision 7 winds a region of the band lying
   along the other surface all across; but a third surface's line or
   corner inside the band parts one of the two surfaces into strips and
   not the other, or ends inside the band, and then no region is a strip:
   decision 6 takes none for twins, and two faces back to back are drawn
   with the very same triangles. So the band of two surfaces decided to
   touch — a plane and a cylinder, or two parallel cylinders — is laid out
   once, the same on both, wherever the operation brings something into
   it:

   - a corner standing within `EPS` of both surfaces, on a face of each,
     and off the line of touch where faces of both operands meet it, is a
     corner of the band;
   - the line through each corner of the band along the line of touch is
     drawn on both surfaces;
   - a surface through a corner of the band, or through a corner on the
     line of touch, standing square enough to the line of touch to part
     the band across — a cap, a side, not a wall turning off a plane it
     continues, which crosses a line of the band a hair either side of
     where it stands — puts a corner on both surfaces and on itself
     wherever it crosses one of the band's lines, between the band's
     farthest corners along it;
   - along that stretch, a corner within `EPS` of the other surface and on
     a face of it lies on it too, and so does an arc whose two corners do,
     standing within `EPS` of it all along and seen by it as a curve it
     can carry;
   - an arc on two surfaces decided to touch is kept between faces of
     either operand: where one operand alone carries both, it holds a skin
     or a crack there — the cusp a bore leaves under a top it touches
     inside — which this operation's band parts, and whose parts decision
     6 decides with the rest.

   Every strip of the band is then parted on both surfaces where the
   others are, by arcs both carry, and the strips of the two are twins,
   decided once. The skin between a plane and a wall resting on it is
   closed up to the band's line farthest from the line of touch, rather
   than left a hair thick between two faces drawn alike; a cusp a third
   surface's line ends inside is closed across at its end.

   A band is laid out only where it is the operation's. A face of each
   surface stands at its corners: laid wherever a face reached within the
   band's width, a band put a slot's run plane on a bar's line past the
   run's end, where the plane holds no face, and crossed the end's line
   there with no corner (90509248). Faces of both operands meet at its
   corners off the line of touch: an operand alone holds its own bands as
   it decided them, and a rounded rectangle joined a hair beside a block
   tied when a block cut far off, sharing the block's side, made its bands
   the boolean's (90504120). But once laid, a band is laid along its whole
   stretch, an operand's own skin included: laid only along the other
   operand's corners, the corner where the other's line ends stood alone on
   both surfaces beyond, and the bar and the bore touching its top inside
   were drawn with the same triangle there (3192987). And no band is laid
   at a corner on a third surface lying along the band, touching neither
   of its surfaces: a pin dipping a hair into a post and the block resting
   on it stands within `EPS` of both over strips of its own, its matter
   above them, which twins of the two leave out (90016363).

Everything else is derived. A vertex lies on a curve when the curve's
support is among the surfaces the vertex lies on and it stands within `EPS`
of the curve: two surfaces crossing at a grazing angle stand within `EPS` of
each other microns from the curve they meet along, and so may a corner on
both, or a corner merged from a chain of places each within `EPS` of the
next; cut there, the curve would end microns from its vertex. Two surfaces
decided tangent are the extreme case: they stand within `EPS` of each other
over a band far wider than it, so a corner on both lies on the line they
touch along only within `EPS` of it and of both. A line or a circle lying on a surface that does not carry it,
only because an arc of it was taken for an arc of the surface (decision 6), is
seen there as the segment between where its ends stand — a circle square to a
cylinder's axis or leaning on a plane only where its stretch stands within
`EPS` of that chord. A corner whose support holds three planes spanning space, and no
cylinder but one decided to touch one of its surfaces, stands where the
planes meet — each plane taken for another within `EPS`, the place they fix
moves by more, and the line a cylinder touches a plane along, taken for
the line two of the planes share, is fixed by them: corners found along it,
some on the planes and some on the touch, would lean its edge a hair
across both. A triple of surfaces is
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

1. **Identity** of surfaces (decision 1), once the second operand is moved
   onto a wall of its radius a hair off (decision 8).
2. **Relations** of pairs whose faces' boxes overlap (2, 3).
3. **Corners.** Every edge curve of one operand against every surface of the
   other, and the special points of the relations (4, 5). A curve can only
   leave a face through the other operand's edge, and two crossings can only
   meet where the other operand's faces do, so these are all the corners —
   but for those each band laid out adds, with its lines (9).
4. **Arcs.** Every curve is cut at the corners lying on it, by support. An arc
   is kept if it is part of an input edge, or lies inside a face of each
   operand on two of its surfaces, or inside faces of either on two surfaces
   decided to touch (9); kept arcs that are one are one (6).
5. **Overlay** per surface. At each vertex the arcs are ordered by tangent
   angle, ties broken by signed curvature — a second tie declines. Cycles are
   traced, then grouped into regions by a ray up the second parameter from
   each cycle's top. Each region gets an interior point.
6. **Winding.** For each operand, a region is covered with the surface's
   normal, against it, or not at all. Covered gives the winding on each side
   of it; not covered gives the same winding on both, from an exact ray cast
   through that operand's faces, retried along another direction when it
   grazes or lands near an edge. Twin regions of two surfaces are wound
   once, together (6). A region standing within `EPS` of a face of that
   operand all across is wound as the pair was decided, not by a ray (7).
   A region is read at another point of its chord where
   its own stands on a corner of its surface: a corner inside a region is
   where another surface touches it at a point, and a ray from there is
   taken on whichever side of that surface rounding leaves it.
7. **Selection.** A region is kept when the operation — or, or and-not — says
   something different on its two sides; its outside is the side where the
   operation is false. One rule gives coincident faces once, drops a shared
   wall, and leaves a non-manifold edge its four uses.
8. **Assembly.** Kept regions become faces and their arcs edges. Faces on one
   surface with one side are merged across an arc nothing else uses; two edges
   on one curve meeting at a vertex nothing else uses become one; a circle that
   loses its last vertex becomes a ring. A vertex lying on the surface of
   another face is used by it: there the curve touches that face's wall, a
   pocket's rim a lying wall it reaches at one point, and the wall's
   triangles take a sample. No seam is left for a later operation to read
   again: an operand's edge inside a face of the other, a hair off whatever
   a later, larger tolerance takes the face's surface for.
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
  the ends. No sample between stands within `EPS` of an end. A sample
  within rounding of another vertex — a corner the kernel left on the curve
  within rounding, not at its end, which a ray through it lands on — is that
  vertex's own point: a second point there would be the vertex twice, and
  with none the edge's chord passes a hair beside the corner, on the wrong
  side of the curve ending there, crossing it. A face beside the edge
  pinches there, or runs out to the vertex and back along the edge as a
  hair bounding nothing, which the sweep follows. A
  sample a hair from a vertex off the edge stays: on the ray through the end
  of a curve beside it, it keeps the two in order.
- **Sampling.** A line at its ends. A curve on a cylinder at the cylinder's
  grid `θ_k = 2πk/N`, `N` a multiple of four chosen so a chord stands within
  the tolerance asked of the surface, anchored at the cylinder's `u`: the
  points where planes of the origin touch a cylinder are always samples. A
  ring takes each step and each ray once, wherever the range a merge left
  it starts: a range starting on a step or a ray, or a rounding past it,
  would lose that place at both ends — a ray at the very angle it starts
  at too, past half a turn, its angle given on the other half. A
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
  third crosses it. So does a sample of a curve not parallel to the wall
  standing there, where the wall holds a face: the rim of a bore's cap
  ending inside a lying post a hair under its wall, with no vertex there —
  however little inside, as for a vertex: the kernel puts no vertex where
  the rim passes the line the cap's plane leaves on the wall by `EPS`, and
  seen from the wall's axis the rim stands inside by less than that.
  The edges are then sampled a second time, on those rays. An
  arc ending where a plane touches its wall takes no sample so near that end
  that it would stand on the plane's edge, and no circle of the wall takes a
  ray so near the line the plane touches along that it would stand on the
  plane: a strip of the wall a hair wide would lie on the plane's face. A
  plane touches a wall only where the body holds the line, a vertex lying on
  both and on a face of the plane: the plane of a face far off along it
  touches nothing, and the rays it would take from the wall leave the rim of
  a disc a bore hollowed a hair off its axis without a sample for two steps —
  nor does a block's top far off across the wall, though a side crossing the
  wall a hair from where that top would touch it leaves a vertex taken to
  lie on it: the line two walls of one radius cross along there, a ray of
  both, would be dropped beside the steps they withhold. Two cylinders touching are both
  sampled on the line they touch along, vertex or not, and a circle whose
  wall is gone is sampled as its own cylinder's. An arc ending on that line,
  though, is sampled there at its end alone: the kernel may lay the line
  leaning a hair, its two ends within `EPS` of both walls, and a ray through
  the other end, or through the exact line, would put a sample a hair from
  the vertex, as good as on the other wall's arc ending at the same vertex —
  a cap holding both arcs would fold back on itself there. Only there,
  though: where no arc of the other wall ends at that vertex, a ray through
  a vertex a third wall puts where it crosses the other is taken, or the
  arc chords past the third wall's sample on the same ray.
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
  taking it, would chord inside the printed circle. Walls close one through
  the next all draw their rays from one centre, the smallest's, where it
  stands inside every one of them: three walls touching along one line are
  three pairs, and a ray passed round them from each pair's own smaller
  centre comes back a hair round from where it left, so that every round of
  passing adds rays and the last round's never reach the other walls. Where a step of the
  larger's grid and one of the smaller's fall within `EPS` of each other
  along the circle — walls of one radius, or all but, a hair off one axis —
  they are one place, the smaller's step, and the larger withholds its own
  over the heights the two face each other: kept each on its own grid, a
  hair round, the two would stand off one ray, and the larger's chord from
  the line they touch along, long where the steps beside it are withheld,
  crosses the smaller's next. A ray along which the two stand closer than a
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
  caps all round. A place two rays put at one vertex is taken once. Two
  perpendicular walls decided to touch at a point, side by side, leave no
  curve and no vertex, and may overlap there by up to `EPS`; the point
  stands on a step of both grids, and each wall's ruling through it would
  pass through the other's triangles. Where both hold a face there, the
  point halfway between the two walls along their common perpendicular is
  taken as a vertex of both: their samples at its angle stand square to
  their axes from it, so that both rulings lie in one plane and meet there,
  as two walls touching exactly do. Withholding the step instead would
  leave a coarse wall's chord across two steps the whole length of the
  ruling. A
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
  over a band far wider than the rules tell apart — a plane, or a wall
  square to the arc's own, or a wall parallel to it whose own arc ends at
  that vertex. Nor does the curve two
  perpendicular cylinders meet along, a step of the finer grid or less from
  its end: a ray a cylinder takes from a wall a hair off its own would put a
  sample of the curve on the cap's arc ending at the same vertex. Nor a place of its grid
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
