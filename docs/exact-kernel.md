# An exact kernel on planes and cylinders: the design #498 builds

#498's maquette, put behind `Body` by #526: it computes the part the
application shows wherever it can, and nothing above `cao_solid` may name it.
Four designs were written
independently on day 1 and judged against the sixteen cases, the harness and
the product; this is what was kept from them. [`exact-kernel-journal.md`](exact-kernel-journal.md)
says what was reached on which day.

## What the kernel can do, and what is left

Kept current, so that whoever takes the next step knows where to start.

It can:

- **compute the part.** `Body` holds an exact body or the flats. A profile of
  straight runs and arcs, holes included, is raised square to its plane, then
  joined or cut, by the exact kernel; the arcs reach it as arcs, centre and
  turn, through `Outline::runs`. What is drawn, lit under the cursor and
  picked are its own triangles, each knowing its face, and a click reads the
  true surface's normal. The section view is read off those triangles. An
  exact body is written to the geometry cache whole and read back as the
  replay leaves it.
- **raise a profile whose loops touch at one point**: a whole circle touching
  another loop of its profile — a hole touching its outline, a ring inside
  the circle around it, two holes side by side — within the raise's
  tolerance. The touch is decided once, on the pieces (`prism/touch.rs`),
  and laid as the boolean lays it when it cuts the same hole out: a corner
  both loops share on each cap, the line the two walls touch along between
  them, and each wall parted there. Two corners standing in one place, within
  the tolerance, are laid as one corner the same way: a loop passing twice
  through it, as the opening two windows meeting at a corner leave (#540), a
  shape pinched against its outline, or a hole in a corner of it. A circle
  touching another loop at a corner of that loop is not laid this way. Still
  declined: a stretch run once each way, the corridor a trait joining a loop
  to another leaves, which would stand two walls back to back
  (`prism/corridor.rs`); a corner passed through twice whose walls leave it
  less than a thousandth of a radian apart, the fin a side the solver left
  leaning on its neighbour by a hair would stand; a whole circle sharing its
  loop with other runs; and, by a turn, any profile passing twice through one
  corner.
- **turn a profile of straight runs** about an axis lying in its plane: by
  any angle up to a whole turn, either way, adding or taking away matter. A
  run parallel to the axis turns into a cylinder, a run square to it into a
  disc or a ring, a slanted run into a cone — a chamfer, a point, a
  countersink, a taper (#536) — and a partial turn ends on two planes holding
  the axis (`brep/turned.rs`). The section [Turning](#turning) says how.
- surfaces: planes, circular cylinders parallel or square to each other, and
  circular cones against cylinders and cones of their axis; a plane against
  a cylinder or a cone at any angle (#552); curves: lines, circles, the
  conics a plane cuts a wall along at a slant — an ellipse, a parabola, a
  branch of a hyperbola — and the curve two perpendicular cylinders meet
  along. The section [The conic a plane cuts a wall
  along](#the-conic-a-plane-cuts-a-wall-along) says how a conic is held.
- **number its faces as the flats do**, so a drawing laid on a face keeps it
  when a size changes: 0 for the floor of a raise, 1 for its top, one per run
  for the walls — two runs on one line, or two arcs of one circle, make one
  wall answering to both; a straight run whose two ends are one corner,
  within the tolerance, is left out and names none, as a turn leaves out a
  run laid to no length (`brep/piece.rs`). A face kept from a boolean
  answers to the numbers of every operand face it lies on; a face merged across a seam to all of its
  members'. A face a cut leaves in pieces that no longer touch keeps its
  number on the first piece, and the others get fresh numbers.
- **decline**, with a reason (`Declined`), and never hand back a solid it could
  not verify. A declined step is a broken step: the part stands as before it,
  the screen says so and the tree marks it, and the step still moves the face
  counter by what its profile would have numbered, so later drawings keep
  their faces. A panic inside the kernel is caught at `Body` and is a decline
  too (`Declined::Panicked`), not a lost part, and is kept from the
  application's crash log, since it is no crash. A body is drawn as it is
  built: one with a face the triangles cannot cover is declined too
  (`Declined::Undrawn`). The face counter only goes up, on the flats as well
  once a part has gone to them, so a number is never given twice.
- **tell faces at a slant that never touch from faces that cross**: two skew
  cylinders, a cylinder or a cone off a cone's axis, are decided apart on the
  faces themselves before the kernel declines (`combine/clear.rs`); a plane
  at a slant to a wall is asked the same first, so that the conic it cuts
  is registered only where two faces may meet along it.
- hand a revolution of a profile with an arc, or an area bounded by an
  ellipse, **to the flats**, from that step on: it has no sphere, torus or
  ellipse. So does a profile of straight runs the reading cannot lay against
  its axis: pinched at a corner on the axis, or a corner of it within the
  tolerance of a slant it does not end.

Left, in the order #497 set:

- revolutions of arcs: spheres, tori. Those of straight runs are done:
  parallel or square to the axis (#533), slanted (#536);
- an ellipse drawn in a profile, raised: the elliptic cylinder. A plane
  crossing a cylinder or a cone at a slant is done (#552);
- a cone against a cylinder or a cone of another axis, such as a radial hole
  through it, and two cylinders at a skew angle;
- an exact STEP;
- chamfers and fillets on a straight edge, then on a circular one, then where
  fillets meet at a corner;
- draft, shell, loft.

Known and still open:

- the tangency band, where a third surface crosses two that touch a hair from
  the touch: a few failures in ten thousand random parts, all of them declines
  or triangles drawing a right body badly. The crescent's tip, two walls of
  one radius crossing a hair apart under a plane, has its band since #528;
  what it leaves is listed in `exact-kernel-failures.md`;
- a cone's own (#536): partial turns about a cone's axis, coaxial turns
  with a cone and a cone beside prisms, under one failure in a thousand
  random parts; `exact-kernel-failures.md` counts them by family;
- a face bounded by the curve two perpendicular cylinders meet is never
  decided clear of a face at a slant, so such a pair still declines;
- the conics (#552): a plane a hair off square to, or along, a wall's
  axis is laid exactly so where that moves the tool by no more than `G`
  (decision 11); past that, up to a sine of a thousandth, it is built as
  drawn and the user is told the step is unstable (step 10 of the
  boolean). A lean in that band, a lean of a hair between two planes or
  two walls, which decision 11 leaves, and a plane or a conic a hair from
  tangent by an offset, held by no band, decline or draw crossing about
  twice in a thousand random parts of the tilted draws, a draw weighted
  towards such hairs; a handful with no hair are left too, a turn of
  359.9° whose slit opens at a line of touch and cone bands of a wedge of
  0.01° among them. `exact-kernel-failures.md` names them, with the rates
  of every round. Two planes at a few times the angle read as parallel
  graze, and hold, where their corners allow it. The slanted and turned
  draws are under one in a thousand;
- a face standing inside a cylinder's hollow is not decided clear of a
  cylinder at a skew angle to it: a boss raised, or a hole cut, square to
  the elliptic face a slanted cut leaves on a shaft declines, though the
  tool never reaches the shaft's wall.

## Scope

- **Surfaces:** planes, circular cylinders and circular cones, nothing else.
- **Curves:** lines, circles, the conics a plane cuts a cylinder or a cone
  along at a slant (#552), and the curve where two perpendicular cylinders
  meet. A cone meets a plane square to its axis, a cylinder or a cone of its
  axis along circles or at its apex, a plane holding its axis along two
  rulings, and any other plane along an ellipse, a parabola, a branch of a
  hyperbola, two rulings through its apex, the ruling it touches along, or
  its apex alone. Two cylinders at any other angle than parallel or square,
  a cone against a cylinder or a cone of another axis, and a cone within the
  tolerance of a plane or a cylinder of its axis all across the box are
  declined where their faces may cross: the operation gives no answer, which
  the harness counts as such rather than as a wrong solid. Where the faces stand clear of each other — a
  bore square into one side of a hexagonal prism, beside the next side, or
  drilled right through it past the far side's slanted neighbours — the pair
  is passed over, decided on the faces themselves rather than on the boxes
  round them (`combine/clear.rs`), along the directions the two surfaces and
  their straight edges offer; beside a cone, also where the other face
  stands inside the hollow the cone's face leaves about its axis, or beyond
  a plane touching the cone along a ruling — a hex socket in a chamfered
  head, a pocket beside a countersink, a block beside a point's slope.
  Every leaf of the sixteen cases and
  of the campaign stands on a plane of the origin, so neither is needed to
  hold them.
- **Operations:** raise a profile of straight runs and arcs along its plane's
  normal; turn a profile of straight runs, each parallel, square or slanted
  to the axis; join, cut. Spheres and tori, what an arc turns into, are
  out. A turn about a line that is no axis of the planes the rest
  of the part stands on brings cylinders at a skew angle to those, and
  declines where their faces cross: a broken step, as #533 decided.

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
Four configurations alone are read wider, at twenty times it, each as ill
conditioned as the others: two parallel walls of one radius (decision 8), a
wall touching one of two parallel planes (decision 10), a wall touching
one of two planes it stands between (decision 2), and a plane a hair off
square to, or along, a wall's axis (decision 11), which turns the second
operand before decision 1 reads it. The result of an operation carries the larger reach of its operands. A pair
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
   two, where the boolean asks that operand which side a point is on — unless
   it stands on one of them, within rounding: that one it is. Taken for
   neither, a floor drawn on the middle one of three floors a hair apart
   stood as a fourth plane on it, its corners within the tolerance of
   corners no merge may join, and a region between them found no column to
   be read in (8537287, 8087943). A wall taken for the first's a hair off,
   further than rounding, moves its operand onto that wall with what it
   built on it, as decision 8 moves it further off: left where it is, a
   slot's cap taken at just under the tolerance kept the side its operand
   made touch it just over the tolerance from the wall, and the side crossed
   the wall it should have touched (533613392). From here on, coplanar,
   coaxial and flush are comparisons of ids.
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
   radius. A cylinder grown onto such a node that would lift off a plane
   square to its partner's axis, which it touched, is slid along that axis
   by what it grew: the node stands wherever along the axis the two meet,
   and a post resting on a shaft's end, crossing the shaft at a node but
   for a hair of radius, rests on it still (533609727). A plane and a
   cylinder, or two
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
   them, its radius half their gap, and touches both exactly. So is one
   touching one of the two and standing within decision 8's hair `G` of the
   other: a bore whose centre stood a hair off the middle of a block's
   height touched its floor within `EPS` and missed its top by a hair past
   it, and the sliver of side between the top and the wall's crown stood
   within `EPS` of both, where no ray tells which side a point is on
   (8554448). Its radius moves by half that hair at most, under what
   decision 8 moves a wall by.
   A surface moved is the boolean's: its pairs, though one operand alone
   carries them, are decided at the boolean's tolerance, and it comes into
   the result at it — moved, it no longer stands where its operand decided
   them.
   Two parallel walls of one radius decided to cross along two lines at an
   angle whose sine is a hundredth or less — their axes a hair apart,
   further than decision 8 merges — are decided to **graze**, once: like
   two surfaces touching, they stand within `EPS` of each other far from
   the lines, `EPS` over the angle on either side of each, and decisions 5,
   6 and 9 read the pair as they read a touch.
   A wall standing within `EPS` of two parallel planes an operand decided
   apart, a hair apart on one side of it — at a finer tolerance, before a
   later leaf grew the reach — is
   decided against each at the tolerance that operand told them apart at.
   At the boolean's, it touched both along two lines a hair apart, on two
   planes no corner may lie on together, and no move settled both: a disc
   touching a block's side and a second side a tenth of a micron off it
   declined (8581852), and so did a slot whose cap touched both faces of a
   skin a cut had left (8588829). Decided finer, it touches the plane it
   touches and crosses or misses the other, as it does. Two planes either
   side of the wall, a diameter apart, are not read so: a slot's cap taken
   for a bore's wall, its sides a hair off the faces the bore all but
   touches, missed one side and crossed the other, and the slot's lines of
   touch stood on surfaces decided apart (8575631).
   A cone (#536) and a surface of its axis — a plane square to it, a
   cylinder or a cone about it — are two lines of the meridian half-plane
   `(h, ρ)`, each kept at `ρ ≥ 0`, decided once where they cross
   (`relation/revolved.rs`): a circle where they cross off the axis, the
   apex alone where they meet on it (`Relation::Apex`), apart where they
   cross on the other nappe or not at all. A plane holding the axis cuts the
   two rulings (`Relation::Rulings`), each built through its meridian line's
   foot, never through the apex, which a cone a hair from a cylinder holds a
   hundred million reaches away; the apex goes with them where it stands in
   the box. A cylinder or a cone off its axis is unsupported, and declines
   unless `clear` stands the two faces apart. No pair with a cone grazes and
   none is moved onto a touch: a cone a hair from a cylinder of its axis meets it
   along one exact circle, whatever the band the two stand within `EPS` of
   each other over; nor is a cylinder about a cone's axis moved off it onto
   a touch with a third surface. Two surfaces meeting at the apex alone
   carry no line together: two cones of one apex, each the other's mirror,
   share their rulings as whole lines, one half on each, and a ruling one of
   them shares with a plane holding the axis is never laid on the other's
   (decision 3). A circle lies on a cone's own nappe or nowhere, and one
   within `EPS` of two such cones lies on both: two cones of one apex a hair
   apart in slope stand within it of each other all along their faces.
   A cone whose axis stands a hair off the other's is taken about one axis
   by decision 8.
   A plane at a slant to a wall (#552) — neither square to a cylinder's
   axis nor along it, neither square to a cone's axis nor holding it — is
   decided by `relation/sliced.rs` alone, once the rows above have passed
   it over (`at_a_slant`). Against a cylinder it cuts one ellipse, never
   apart and never touching: a plane along a ruling is square to the axis,
   and reads as before. Against a cone the rows are read in order, space
   before angle, as a cylinder is decided by its gap first: with the apex in
   the box and within `EPS` of the plane, the plane holds the two rulings
   where it runs along the cone (`Relation::Rulings`) when it is steeper
   than the slope by more than the angle `parallel` reads, touches the cone
   along one ruling (`Relation::Ruling`, its apex with it) when it is within
   that angle of the slope, and meets it at its apex alone when flatter;
   with the apex beyond the box, within that angle of the slope, and no
   conic cut, the ruling of touch, with no apex, where it stands within
   `EPS` of the plane at its foot and runs along it across the box — cut as
   a conic, a doubled ruling is nothing, and a face touching a cone along a
   ruling at exactly its slope was refused (5543214149, 5543216778). Where
   a conic is cut, or the plane is steeper, the conic is kept: the rulings
   held there instead broke three bodies the hyperbola held (5543114315,
   5543213205, 5543216548); a
   cut within `EPS` of the axis all round is the point of the axis on the
   plane, as a circle within `EPS` of it is; otherwise the conic on the
   cone's own nappe — an ellipse, a parabola, a branch of a hyperbola, the
   flat on a chamfer among them — or nothing where the plane meets only the
   other nappe (`Relation::Conic`). The conic is built from the wall's own
   ruled form and never forms the apex. Where its vertices stand further
   than ten thousand reaches from the origin it is split at them into the
   arms that cross the box, decided once at the pair's scale. A plane at a
   slant whose faces `clear` stands apart is passed over before any of this,
   so the bodies built before #552 register no new curve. A ruling of touch
   lays no band (decisions 7 and 9 are not extended to it), and no surface
   is moved onto a touch with a cone or onto a conic.
3. **Line identity.** The same line comes out of several pairs; lines within
   `EPS` over the box are one, and their supports are joined — never across
   two surfaces decided apart, nor, for lines, decided to meet at a cone's
   apex alone, and
   never two curves of one operand, which
   the operation that made it kept apart. A line a pair's line was taken for is that
   pair's alone: another within `EPS` of it, on a surface apart from the
   first's, does not lie on the pair's surfaces for that. An operand's edge
   beside a face whose surface was taken for another or moved onto a touch
   is laid on the curve its surfaces now share, the nearest within twice
   `EPS`: each of the two may stand up to `EPS` from where the operand drew
   it, and a bore's rim moved onto a side, its floor taken for one a hair
   off, stands √2 hairs from the circle they share. Which of those surfaces
   carry the edge all along is read off the operand as it was built. The
   corner such an edge ends at goes with it: it is laid where a line of the
   edges ending there now crosses a plane it lies on, square enough to it
   to fix the place. Left where its operand drew it, a slot's corner on a
   top taken for the block's a hair above stood that hair under the top,
   and the next operation found the lines it ends crossing the top that far
   from it (8511092, 8505208).
   Two conics (#552) are one when their planes are one, their walls are
   one — alike cylinders, or cones of parallel axes whose meridian lines
   are one across the box — and they are the same stretch of the cut: the
   whole of it, or the arm on the same side of its plane of symmetry. A
   conic is never one with a line, a circle or a meet. A conic is carried
   by a plane alike to its plane and a wall alike to its wall, nothing else;
   moved with its operand, it is cut again on the moved surfaces.
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
   their planes share. One more touch (`relation/crossing/section.rs`): the
   line a plane square to a wall's axis shares with a plane crossing that
   axis steeply, at a cosine of a tenth or more, standing inside the wall's
   section by rounding alone, a millionth of `EPS`, only touches the wall,
   where it passes nearest the axis — the line, the circle the square plane
   cuts and the conic the steep one cuts each reach that touch, the circle
   only as an edge on the square plane: a rim where a wall gives way to a
   cone, crossed by a plane through the cone's apex a hair off its slope,
   keeps crossing twice, where that plane's hairpin on the cone does
   (5545108465). A cut at
   60° through a bore's lowest line at a tube's end shares a line with the
   end found 3e-14 inside the bore, which crossed it twice 3.2e-6 apart
   where the cut's ellipse touches the end once, a sliver between them no
   face agreed on (5544100660). An elliptic wall's section is held the same
   way (#571), its rim, a conic, reaching the touch through the line its
   plane shares with the steep one: two cuts at 45° and 75° through one
   point of an elliptic post's rim (5571628398). A plane leaning along the
   axis is left to
   cross twice: its hairpin's vertex stands where rounding over its lean
   puts it, and its arms pass the square plane where the line's two roots
   do (5549116996). A line found on its own keeps crossing twice however
   thin the hair, as real walls rely on. A line meets a cone (#536) along
   a ruling; square to the axis, where it crosses the cylinder of the
   cone's section at its height, the corner the circle of that plane gives
   — a line inside that cylinder by rounding alone touches it, where it
   passes nearest the axis: the line a plane square to the axis shares with
   a plane touching the cone exactly where it crosses it, found a femtometre
   inside, crossed it twice four tolerances apart (5543231406); through the
   apex, once, or touching there where it stays outside on both sides; and
   otherwise at the roots of the cone's quadric kept on its own nappe,
   touching it once where it passes within `EPS` of it at its closest, as
   a wall. A circle in a plane holding the axis crosses the two rulings
   that plane cuts; any other circle off the axis is unsupported, as the
   pair it lies on is (`relation/crossing/cone.rs`). A conic, its plane `P` cut by its wall
   `Q`, meets a third surface `S` where the other curve of that corner does,
   by the very same call (`relation/crossing/conic.rs`): against a plane,
   the line `P` and `S` share crossing `Q`; against a wall, the lines, the
   circle or the curve `Q` and `S` meet along crossing `P`. Each corner is
   computed once, with the same double-root decision and the same bits
   whichever curve reaches it, and no quartic is solved. Where `Q` and `S`
   meet in a way the kernel does not build, so does the conic; and a
   crossing left unsolved on a pair the kernel builds declines rather than
   being passed over. A plane leaning off a wall's axis by less than the
   angle read as along it, by more than rounding, is cut along lines drawn
   through the foot of the wall's origin, and stands a fraction of the
   tolerance off them where a corner is: a hair inside the wall's ruling,
   the line it shares with an end crossed the wall microns aside of them,
   and every corner of the sliver it cuts off was found twice (5520135751).
   So a line or a circle lying on two surfaces, one of them so decided to
   cut or touch the other along lines — a plane and a wall, two walls read
   parallel — crosses it where those lines cross the curve's other plane,
   by the call those lines make themselves (`relation/crossing/lying.rs`),
   and a conic crosses such a plane the same way, on an elliptic wall too
   (#571): an elliptic hole reaching a hair past a cut laid along it
   (5571636439). Only where the lines
   cross that plane steeply, at a cosine of a tenth or more: shallower, a
   hair moves the place along them by far more than the other pairs of the
   triple do (5524010446). A curve lying on a third surface, taken for one
   that surface carries, is left to its own crossings: two planes crossing
   at a grazing angle near a wall's axis put the rulings and the conic a
   fraction of the tolerance apart, and the corner found through the
   rulings stood that far off the conic's (5523010597).
5. **Point identity.** Corners within `EPS` are merged, in a fixed order —
   the operands' own corners, the relations' special points, the crossings
   three planes fix, then the others, so that the crossings a curve makes
   near a place three planes fix are merged into it rather than into each
   other — and their supports joined — unless the merge would put one point on two
   surfaces decided apart, or on two surfaces, one from each, crossing along
   lines that all stand further than `EPS` from it, which is refused —
   unless the two touch or graze (decision 2): within `EPS` of both far
   from their lines, a corner one of two grazing walls carries near their
   crossing and the node a band puts on the other there are one corner.
   A corner found along a ruling through a cone's apex is on that cone
   only where it stands on its own nappe: past the apex, the ruling the
   cone shares with a plane holding its axis is that plane's line alone,
   and a plane apart from the cone, square to its axis over the other
   nappe, may cross it there. Counted on the cone, the corner was kept
   apart from the one the other curves through that place found, and the
   step was refused (5543226572: a slant through the apex of a cone
   joined to two quarter turns, touching it along the ruling one of their
   end planes holds).
   A corner standing within `EPS` of a point where two walls were decided
   to touch (decision 2) — the node or the contact of two perpendicular
   cylinders — lies on both, and so on every curve of either it stands
   on. Two walls of one operand touching at a
   point leave no edge there, and the curves a third surface cuts each of
   them along meet there with no edge to find the corner through: a rod
   joined to a post it touches, cut by a pin of the rod's radius whose
   axis meets the rod's at the touch, put the node of the pin and the rod
   on neither the post nor the line the pin touches it along, and that
   line was not cut there (5544320693). Not at a cone's apex: a plane at
   the cone's slope a hair off it, taken onto every surface through the
   apex, had corners within `EPS` of it cut curves that pass it by
   (5545235515). Nor onto a wall decided apart from one the corner is on:
   a post across two rods of one axis a hair apart in radius has a node
   with each, and the two stay two (5545618519).
   Where two walls of one operand touch at a point that a surface of the
   other passes through, reaching it with a face, that point is a corner
   candidate on the three, merged after the special points: the curves
   the surface cuts the two walls along each touch the other wall there,
   and with no edge of the operand there, no crossing finds it — a rod and
   a post side by side joined to a disc whose floor passes through the
   touch (5550316656), a slot's round end touching a turn cut by a block
   whose face holds the touch (5550230611).
   No corner lies on a curve on a surface apart from one of its own. Each
   corner is measured where it stands (below, where planes fix it): one
   whose three planes fix a place stands there, though an operand put it
   up to three hairs off, each plane taken for another, and a curve
   crossing a fourth surface at that place finds that corner rather than a
   second one beside it (8520500). An operand's own corner, on the
   surfaces its operand built it on, none taken for another nor moved onto
   a touch, and on no plane of the other operand, stands where its operand
   built it, on the line two of its planes share: that operand decided it.
   A cut a hair from a block's edge leaves the edge's corners on the cut's
   plane within the tolerance, though that plane, the floor and a side
   meet twice the tolerance beyond, and the end face they were decided on
   is gone; moved there by the next operation, they left the edge they end
   that far off, and the floor lost its last edge (5520107844). A plane of
   the other operand through the corner fixes its place anew, where the
   boolean finds its other crossings with that plane (5523006542). A plane
   and a wall crossing along a conic at a grazing angle stand within `EPS`
   of each other far from it, so the refusal reads their conic as it reads
   their lines (#552). Two planes
   too near parallel to fix the line they share (below) are decided, once,
   to graze where they stand within `EPS` of each other across every box
   faces of both may meet in, every corner candidate on either in such a
   box stands within `EPS` of both, and every curve crossing both
   there crosses them within `EPS` of one place: they stand within `EPS` of
   each other far from that line too, and the refusal does not hold
   between them — the corner a rim, a side and a top meet at and the one an
   ellipse, that side and a floor leaning a hair off the top meet at are
   one, and the rim and the ellipse between them one arc (5520121880).
   Where one candidate stands further, the two cross, and no corner is one
   on both: one pair of corners merged and the next a hair too far apart
   would make one arc of two at one end and leave them two at the other
   (5520267036), and a cone's ruling crossing the two 1.2e-8 apart would
   leave one of its corners two (5520167649). Nor do two whose faces may
   meet further out, past every corner: a disc's rim has none at its far
   side, three hundred units from the line, where the two part 5.2e-7 at a
   tolerance of 5.1e-7 (5527004090).
6. **Arc identity.** Once every curve is cut into arcs at the pooled
   corners, two arcs lying on one surface between the same two corners,
   which part by no more than `EPS` anywhere along them — measured exactly
   for lines and circles, whose distance turns only where a closed form
   says, and on dense samples along the perpendicular curve, each against
   the other's arc rather than its whole curve: two halves of two circles a
   hair apart, one each side, run between the same corners within a hair
   of each other's circle all along — are one arc.
   One curve is kept for it: a line before a circle before a conic before a
   meet, then the first registered — a conic before a meet because a plane
   cannot see a meet. The arc lies on the surfaces of both, over its own
   stretch alone, so supports are carried by arcs rather than by curves: an
   arc lies on its curve's surfaces and on those of the arcs taken for it,
   and an edge a later operation reads lies on its curve's surfaces all
   along and on the others over its stretch, and so does a corner a
   surface of the other operand crosses it at there: a circle a face
   leaning a hair off its plane is left bounded by, crossed by a cut, is
   crossed on that face too, and the line the cut leaves on the face ends
   there (5520140378). Two arcs whose surfaces
   include two decided apart stay two. The curve two perpendicular walls
   meet along is carried all along by its own two walls only, not by a wall
   of one radius a hair beside one of them: there it lies over its stretch
   alone, and is seen there as below, under "Everything else is derived".
   Two arcs between the same corners that both lie on two surfaces decided
   to touch are one when they part by no more than the hair, `HAIR`
   tolerances: they are where a third surface crosses the band, each the
   curve it cuts on one of the two, standing within `EPS` of both, and they
   part by `EPS` over the sine of the angle the third crosses at — a cone's
   wall crossing a wedge's wall a hair inside a block's side at 45°, 1.08e-8
   apart at a tolerance of 9.5e-9 (5550223366).
   Nor is a line carried all along by a plane it crosses at a grazing
   angle: a cut's corner line beside a block's side leaning 1e-3° off the
   cut's face lies on that side over the 1.5e-4 of its edge alone, and laid
   on it all along it put a corner at the cut's floor, twenty tolerances
   off the side, on the side (5541239472).
   A conic on a plane or a wall that is not its own, and a circle leaning
   on a plane by less than the tolerance across its rim, are seen there as
   the conic that surface meets the curve's other own surface along,
   between where the curve's ends stand on it (`domain/traced.rs`).

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
   middle of that stretch, where they stand furthest apart. Two planes
   are read at the region's point, where they stand further apart than
   rounding: the two ends of a turn a tenth of a degree short of whole,
   near its axis, a crack (533708173). Other twins are still a tie. The
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
   for being a sliver. No band is laid along a conic, nor along the ruling a
   plane touches a cone along (#552): a plane a hair from tangent to a wall
   at a slant, a conic a hair from a ruling or from an apex, are built with
   none.
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
   and it is kept to the configurations whose crossing is ill conditioned.
   Two parallel planes a hair apart never cross: the skin between them is
   well defined and kept, and case 8 keeps its floor of a ten-millionth —
   unless a wall touches one of them (decision 10). Two
   cylinders of one radius `d` apart cross along two rulings at an angle
   `d / r`, and a band a thousand tolerances wide surrounds the crossing,
   where every corner a third surface makes is ill conditioned. The
   crescent merged away is at most `G` thick and is almost always what was
   meant: a slot's cap, a rounded corner or a bore drawn on a hole of its
   radius, a hair off.

   A cone (#536) is decided against a cylinder or a cone about one axis
   only: a cone and a wall or a cone, one of each operand, whose axes are
   parallel and stand within `G` of each other, whatever their radii, are
   taken about the first's axis, the second operand moved square to it with
   what it built, as two walls of one radius are. Left two, the pair met
   along a curve the kernel does not build, or, within `EPS`, along a circle
   read about the cone's axis while the lines and circles of the wall were
   read about its own: a ring cut by a quarter turn of a cone `3e-7` off its
   axis found the corner where the cone leaves the ring's wall twice, a hair
   apart along the slope (5361108529), and a point on a disc a hair off a
   ring's axis declined (5361110937). Nor is any move made, by this
   decision or decision 2, that would take a cone off a surface of the
   first it was decided with about one axis: a ring's bore moved onto a
   pocket of its radius a hair off the wall its cone widens onto took the
   cone off that wall (5361127255).

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
   would move the curve two perpendicular cylinders meet along, nor where
   it would lay a surface it carries on one of the second operand's own it
   leaves behind, which that operand built apart — a rounded rectangle a
   hair taller than its two corners, its top laid on a face by decision 10,
   took its upper corners' walls onto its lower ones (8528992): the two
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
   it — and so is the band of two walls decided to graze, about each of
   the two lines they cross along as about a line of touch, a corner nearer
   the other line standing in that line's band: the crescent's tip:

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
   - along that stretch, a corner within `EPS` of the other surface, where
     faces of both surfaces stand, lies on it too, and so does an arc whose
     two corners do, standing within `EPS` of it all along, seen by it as
     a curve it can carry, and where faces of both stand at its middle —
     a slot cut by a second starting a hair past its end leaves the plane
     of their runs with no face between them, and the run's line laid
     across that gap onto the two ends' walls passed the corner where
     those walls cross without being cut there (92530408);
   - an arc on two surfaces decided to touch, or to graze, is kept between
     faces of either operand: where one operand alone carries both, it
     holds a skin or a crack there — the cusp a bore leaves under a top it
     touches inside, the crescent a disc cut by its twin a hair aside leaves
     — which this operation's band parts, and whose parts decision 6
     decides with the rest.

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

   A band laid on a plane touching both walls of a crescent's tip makes
   twins of three: the plane's strip first, then the tip's two walls, one
   operand's skin. Decision 6 reads which way that skin turns its matter
   off the pair as the operand decided it, by the normal of the first of
   the two, so their coverings are read in that wall's frame, not in the
   plane's, which is turned from both: read in the plane's, the skin was
   taken for a crack, the strip dropped, and the plane's face left open
   along it (92510427).

10. **A wall touching one of two planes a hair apart.** Two parallel
    planes, one of each operand, further apart than `EPS` and within `G`,
    are one surface where a wall of either operand runs along them touching
    one within `EPS`: the second operand's taken for the first's, the
    second operand moved along the normal by the hair with what it built
    on it, as decision 8 moves it, before decision 1 reads it. A wall
    touching a plane crosses a parallel one a hair across at a grazing
    angle, along two rulings a band apart, or misses it by the hair it was
    meant to touch: a rounded rectangle drawn a hair and a half beside a
    block touched its own sides with its corners' walls and crossed the
    block's, and declined (8540801, 8510444). Without the wall the skin is
    well defined, and the planes are left two. Where the move is not made,
    as decision 8's, they are left two too.

11. **A plane a hair off square to, or along, a wall's axis** (#552). A
    plane of one operand and a cylinder, a cone or an elliptic wall (#571)
    of the other, their faces in boxes that meet, the plane leaning off square to the wall's axis or
    off along it: the second operand is turned so that the plane lies
    exactly square or along, about where the two operands meet and set down
    on what it held against the first (below), where that turn moves none
    of its points further than `G`. The conic such a plane cuts the
    wall along divides by the lean: a cut a ten-millionth of a degree off
    square to a post cuts an ellipse within the tolerance of the rim beside
    it, a side as far off along a bore cuts a hyperbola within it of the
    two lines, and the kernel answered both ways, in different places — an
    ellipse here and a rim there, crossings found once and twice. Turned,
    the pair is the circle or the lines the kernel already holds exactly.

    The measure is a move, not an angle. The lean is the sine `s` of the
    plane's normal off the axis, `|n × a|`, or off square to it, `|n · a|`,
    whichever is smaller; the lever `L` is half the diagonal of the second
    operand's box, the furthest any of its points moves under a turn about
    its middle. A pair is turned where `s·L` stands between rounding — a
    millionth of the tolerance, under which it is exact already — and `G`.
    The turn as made must move no point further than `G` either: its angle
    times the furthest corner of the box from the point it is made about,
    with the shift that sets it down.
    For a tool as large as the part, `G` turns up to some 1e-8 radians,
    about 6e-7 degrees; for a tool a tenth of the reach, ten times that.

    `G` rather than the tolerance, though #552 decided on "less than the
    tolerance across the part": it is the furthest the kernel already moves
    an operand (decisions 8 and 10), a move of `G` is seen by a line at a
    cosine of a twentieth as at most eight hundred tolerances, under the
    thousand a line resolves, and it was measured. On the 538 shrunk
    failures of #552's third round, a prototype turning at one tolerance
    held 159 of the 236 within 1e-7 degrees and none of the 129 between
    2e-7 and 1e-3 degrees; at `G` it held 216 and 60. Moving it to the
    tolerance is one constant, and leaves the band #552 warns of wider.

    The turn is decided once, before decision 8, and made rigidly with the
    operand's surfaces, curves and corners, so that its own decisions hold
    as it took them; a direction it turns to within rounding of one of the
    first operand's is laid on that one's bits, so that a circle and its
    wall share their axis, and identity, decisions 1 and 8 then compare
    exactly. A tool turned 1e-7 degrees off a bore of its radius is then
    merged onto it by decision 8. An elliptic wall (#571) turns its axis
    and its half-axes together, their lengths kept to the bit, and the
    section a plane cuts it along, its rim, is cut again on the turned
    plane and wall; its major half-axis turned within rounding of an
    elliptic wall's of the first is laid on that one's bits too, so that
    decision 1 compares the two ellipses exactly.

    Where the turn is made about decides where the operand ends up, a hair
    either way, and a hair parts what the second held against the first.
    Turned about the middle of its box, a wedge's side drawn on a disc's
    end stood 1.2e-8 off it, its corners at the wedge's axis four times
    further (5544600162); a post's end stood a hair off a line of a turn it
    held (5544510380); a stepped turn's axis stood 2e-8 off a disc's end it
    lay in (5544600864); a face laid square to a cone stood 0.98 of the
    tolerance short of the circle where the cone meets a shaft
    (5544232904): each declined as unverified, each held as drawn on
    `85a2d2e`. The operand is turned about the middle of where the boxes of
    the two operands overlap — or about the middle of its own box where,
    turned about where they meet, a part of it reaching far past the other would
    move past `G`: a cut reaching past a turn, drawn on its slit's face 1.4e-6
    degrees off, was then left as drawn and refused (5547227649) — then
    shifted square to the turn's axis — the
    shift a turn about another point makes — so that each pair the turn
    lays parallel lies on the other exactly wherever the two stood within
    `EPS` of each other, taken for one place already: two flats, a plane or
    the plane of a circle of each; a line of either, a straight edge or a
    wall's axis, in a flat of the other; a straight edge of either
    touching a cylinder of the other across its axis. Turned about where
    the operands meet, a slot's side drawn on a disc's wall and a rounded
    block's seam drawn on a ring's stood a fraction of `EPS` off, and were
    refused (5546322326, 5546607658); their edges lay them. An elliptic
    wall is touched across its axis where an edge stands as far from it as
    its support reaches across the edge. A flat is not
    laid on a wall it touches: a face drawn on a turn's wall, a hair inside
    a second wall of the turn a hair larger, was laid on that one, further
    off the first, and refused (5546111832). A flat passing
    within `EPS` of several lines of the other that part by more than a
    thousandth of it lies on none of them: a cut drawn through a point a
    hair from a turn's axis, its slit's line and an earlier cut's corner
    was laid on the nearest, the axis, and refused, where as turned it held
    (5546116488). Laying pairs from as far as `G` moved a cut's face three
    tolerances onto what it only passed near, and it was refused
    (5545209574). The pairs nearest to lying come first; a pair the shift
    cannot lay without taking an earlier one off, or past `G`, is left as
    it stands. A turn about a point has two such degrees of freedom, and
    they are spent on what was drawn together (`canonical/placed.rs`).

    The turn is solved one degree of freedom at a time: a pair of
    directions exactly parallel locks the axis it turns about, the pairs off
    square come first, and each next one is laid by a turn about the axis
    locked. It is not made where no one turn lays
    every pair exactly; where it would take a pair of directions of the two
    operands, plane or wall, off exactly square or along; where it would
    change the kind of an exact relation the operand had with the first —
    one surface, a touch, a ruling, an apex, a plane holding a cone's axis,
    which turned a hair off it would become a hyperbola a hair from the
    apex; where it would lay a plane a hair beside a cone's apex, read
    through it — a face through a point's apex leaning 1e-7 degrees off its
    axis, turned about the middle of its box, stood a hair beside the apex
    and was cut along two rulings it does not hold (5543232861); where a
    canonical direction would
    turn round, the way its curves run with it; nor where the operand holds
    a conic other than an elliptic wall's whole section, or the curve two
    perpendicular cylinders meet along, which a leaf never does. The pair is then built as drawn.

    Only a plane against a wall's axis, as #552 decided. Turning for any
    pair of directions — two planes, two walls — held fourteen more of the
    shrunk failures and four more fresh seeds, but broke one that held, and
    is set aside: the pairs are found by one predicate, and widening it is
    a one-line change.

    What it held, on round 3's seeds against `85a2d2e`, fifteen minutes a
    draw: tilted on the exact kernel, 43 failures in 21 823 cases where the
    base had 118 (5.4 to 2.0 in a thousand); tilted through the
    application's body, 135 in 48 658 where it had 311 (6.4 to 2.8);
    slanted, 11 in 15 078 where it had 10; turned through the body, 15 in
    17 085 where it had 17. Of round 3's 538 shrunk failures, 282 hold.
    Nine seeds the base held fail, three of them only by the path their
    draw took — their shrunk cores fail on the base too. The other six are
    a lean laid exactly beside an offset hair the lean had kept apart, or
    beside a turn's walls: a plane laid along a wall beside a slope a hair
    off its corner, two floors 2e-8 apart laid along a wall, a face laid
    parallel to another a few tolerances off, an end laid on a shaft's end
    it crossed, a turn's end planes laid along walls, two turns' walls laid
    square. They are named in
    `what_the_exact_campaigns_found_in_the_kernel.rs`, ignored with their
    reason. Refusing those turns does not hold them: a turn refused at one
    step leaves its tool at its lean beside a tool drawn at that lean and
    turned the step before, and refusing two skew walls laid square held
    one seed and broke two.

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
`EPS` of that chord. A circle square to a cylinder's axis that bulges off
its chord there, a rim passing a hair from a ruling it crosses twice, is
seen instead as the curve its own wall meets that cylinder along, between
where its ends stand on it, where its middle stands within `EPS` of it: it
was taken for an arc of that curve, and the cylinder carries the curve.
So is the curve two perpendicular cylinders meet along, on a wall parallel
to one of them and further than `EPS` from it — two walls of one radius a
hair apart, a crescent: it is seen as the curve that wall meets the other
cylinder along, where its middle stands within `EPS` of it, and in that
wall's own angles otherwise — never in its own wall's, which part from them
by the angle the two axes part by. Which wall is beside is read once, off
the curve's own two cylinders (`Meet::own`). A corner whose support holds three planes spanning space, and no
cylinder but one decided to touch one of its surfaces, stands where the
planes meet — each plane taken for another within `EPS`, the place they fix
moves by more, and the line a cylinder touches a plane along, taken for
the line two of the planes share, is fixed by them: corners found along it,
some on the planes and some on the touch, would lean its edge a hair
across both. A corner on two planes across each other, and on no third
spanning space with them, stands on the line they share, nearest where it
was found, whatever cylinders it lies on: a wall touching one plane a hair
from the other's line puts its corners on that line where it touches, and
the strip of the wall beside an edge leaning between them was drawn
through the plane (8010303); an operand's corner on its own plane, taken
for the body's a hair off by decision 1, stayed a hair off the plane it
lies on, and three samples of that plane's face in a row but for rounding
made a triangle standing upright on the hair (8531226, 80511824). A triple
of surfaces is
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
  a cylinder or a cone is either a ruling or a graph over `θ`, a conic a plane
  cuts it along included, which is what point location, tessellation and the
  volume stand on.
- **Cones** (#536, `brep/surface/cone.rs`). A cone keeps the origin, axis,
  `u` and `v` every cylinder of its axis has, and its meridian line as a
  foot — the `(h, ρ)` of the line's point nearest `(0, 0)` — and a ruling of
  unit length growing away from the apex: the foot stays within the box and
  the ruling of unit length whether the cone is a hair from a cylinder or a
  hair from a disc, where an apex and a slope would not, and nothing reads
  the apex but the apex itself. It is read in `(θ, l)`, `θ` as on a cylinder
  of the same axis and `l` along the ruling from the foot, which stays well
  conditioned at both limits. Its normal is the chart's own,
  `R(θ)·w_h − A·w_ρ`, pointing away from the axis only where the cone opens
  along it; the face carries the matter's side, as on every surface. The
  apex is the chart's pole, the line `l = l_a`: a face reaching it is closed
  along it for parity and inside points alone, never measured as a boundary
  (`domain/floor.rs`), and only where the apex stands in the box. A point's
  tip on the axis is a vertex of its cone's face, a whole turn's included
  (`Face::apex`): the listing lists it, and soundness holds it to the
  geometry as any other vertex.
- **Topology** is arenas of plain structs with `u32` ids: vertices with their
  point and the surfaces they lie on, edges with their curve and parameter
  range, faces with their surface, their side, and loops of oriented uses of
  edges. A loop has its face on its left, seen from outside the matter.
- **No seams.** A cylinder's face may go all the way round. A whole circle is an
  edge with no vertex — which is how the application will want to pick it, and
  how Parasolid holds it.
- **Non-manifold edges** have four uses, or six: two blocks touching along an
  edge, a hole tangent to a wall.

## Turning

#533 brought the kernel its first turn, of the profiles whose surfaces it
already had: every run parallel or square to the axis. #536 added the cone,
and with it every slanted run. The body is laid directly, as a raise lays its
walls, rather than raised in slabs and joined or swept through the raise's
code: one surface per piece of the profile, one circle per corner, which is
the construction spheres and tori will grow from.

- **The reading** is decided once, before either kernel, in `turning.rs`:
  whether a turn is whole (within a thousandth of a radian, the flats' own
  rule), which side of its axis the area lies on, and how close to the axis
  counts as on it — a thousandth of the area's furthest point from the axis,
  on either side, the band the flats always used (#487, #488). An area
  across its axis is cut along it before the body is asked, and each side
  turned apart. `Straight` is the profile laid against its axis, its corners
  read `(h, r)`, along the axis and away from it: every run within the
  tolerance of parallel or square to it laid so, exactly, and every other
  slanted as drawn, joining no level; one that cannot be laid so goes to the
  flats. Levels closer than the tolerance are one across the whole profile,
  its holes' included, and laying holds to that: a wall or a gap between two
  of them is not there. A slant's two corners stand on the levels the runs
  beside them give them, so a chamfer's ends are, to the bit, the radius of
  the cylinder and the height of the shoulder it joins, and its angle is
  never snapped; a slant reaching within the band of the axis ends on it, at
  an apex. There is no angle below which a run is taken for parallel or
  square but the tolerance itself, so the kernel turns cones a hair from a
  cylinder or a disc. A hole a hair inside its outline opens onto it as a notch, a
  slot a hair wide closes, and the outline's run along the wall that went is
  left in pieces, each answering to its number. Where laying makes the
  profile touch itself, it is read again as the matter it bounds, column by
  column between its levels along the axis, a slant crossing a column whole
  (`turning/straight/bounded.rs`); it is declined only where that matter is
  not one piece, or touches itself at a corner. A profile laid to no area —
  a disc a hair thick turned about its edge — is nothing, and turns into an
  empty body counted past its numbers; a hole laid so is not there. A fin
  a hair thick at a cone's end is not there, and a hole a hair under a rim
  past a chamfer opens onto it. The flats, handed such a wall, kept it a
  hair thick, and their boolean left it open (533626745, 5361217405,
  5365201759). A slant shares no level with the corners beside it, so a
  corner within the tolerance of a slant it does not end, read one
  coordinate at a time as levels are told apart, cannot be laid onto it,
  with one exception: a run bringing the slant's own end back to within the
  tolerance of it stays that close the whole way, so the sliver between is
  thinner than the tolerance, and the slant ends where the run back does
  (`turning/straight/slivers.rs`, 5365208537). Any other such profile goes
  to the flats — a hole under a chamfer — and so does one with a run across
  a slant, or a corner on the axis between two runs that both leave it, a
  pinch the turn would close on a point (`turning/straight/contacts.rs`). A
  corner on a level of its own a run past the tolerance from a slant's end
  is no contact (5361126605).
- **The placement** (`brep/turned.rs`). A corner `(h, r)` turned by `φ` is
  `O + A h + r (R cos φ + S sin φ)`, `S = A × R`: the turn the flats make
  with `DQuat::from_axis_angle`. A turn backwards is the turn forwards about
  the axis turned round, its corners read along it the other way, so the
  sketch's plane is always the end the turn opens on. A turn within
  `Scale::HAIR` tolerances of a quarter, a half or three quarters of a turn,
  at the profile's furthest corner, is taken as that turn exactly, its
  cosine and sine from a table: the ends then stand on the planes a raise from
  the planes of the origin stands on, and a half turn ends on one plane.
- **The construction** (`brep/turned/faces.rs`, on the raise's own
  `brep/laying.rs` and `brep/piece.rs`). Runs on one line are one piece. A
  piece parallel to the axis lies on `Cylinder::about(O, A, r)`, the very
  cylinder a circle raised along that axis lies on, so a coaxial bore is
  decision 1's identity and needs no tolerance; a piece square to it on the
  plane through `O + A h` square to `A`; a slanted piece on
  `Cone::through(O, A, [from, to])`, whose rims are then the very circles of
  the cylinder and the plane beside it; a piece on the axis on nothing. A
  partial turn has a corner where each corner off the axis starts and ends,
  and the arc between them; one corner where a corner on the axis stays; a
  straight edge where each piece starts and ends; and the two ends, holding
  the profile's holes as holes. A whole turn is its limit, the two ends
  closing on each other: a whole circle for each corner off the axis, no
  corner but at a cone's tip — a square turned about its side is two discs
  and a cylinder between two circles, and a point turned whole a disc and a
  cone with one vertex, at its tip. A half turn lays its closing line on its
  opening one only where the piece is square: the rulings of a cone at
  nought and at a half turn are two lines. A hole of the profile, or a notch on the
  axis, turned whole leaves a closed hollow inside.
- **Tidied and checked** as a boolean's result is (`assembly::tidied`):
  faces on one surface with one side merged across what nothing else uses,
  and the body verified. On a turn this only changes a half turn of a profile
  touching its axis: the two ends are one face answering to both numbers,
  and each pair of radial lines, laid on one curve, one diameter, the corner
  on the axis gone.
- **Declined:** a corner turned, or the slit a partial turn leaves, narrower
  than the tolerance (`Declined::Travel`) — past the reading's own rules,
  which make such a turn nothing or whole first; a profile whose corners
  stand on each other, or with a corner on the axis between two pieces that
  both leave it (`Declined::Profile`), which the reading hands the flats
  before the kernel sees it.
- **The numbers** are the flats': run `k`, the outline's first and then each
  hole's, names the face it turns into, a cone included; a run on the axis,
  or one laid to no length, names none; a partial turn's opening end is
  `runs` and its closing end `runs + 1`. The count a turn moves the part's counter by copies the
  flats' `faces_end`, quirk included: a whole turn counts up to its last run
  whose flats were thick enough to keep, read on the profile as drawn
  (`body/turned.rs`, `numbers_turned_whole`), so the steps after a shaft
  drawn the usual way, its last side on the axis, keep the numbers a part
  saved before turns were exact gave them — and so do the steps after a
  shaft whose side was drawn within the band, laid on the axis since but
  counted then. `Straight::numbers` counts the profile as laid, for the
  pieces of an area across its axis, which `main` never turned. A face a
  wall a hair thin left in pieces is told apart by the first boolean, as a
  cut's pieces are: the second piece takes the next fresh number, one past
  the count the flats gave the profile with its wall.

## The boolean

A classic boolean splits faces against faces. This one splits **surfaces**: all
the faces of both operands lying on one surface, and every curve where another
surface crosses them, go into a single overlay in that surface's parameters.
Flush, touching and coincident faces are then the ordinary path rather than a
special one: a coincident face is a region covered twice, a shared wall a region
covered once from each side.

1. **Identity** of surfaces (decision 1), once the second operand is turned
   square to, or along, a wall's axis it leans a hair off (decision 11) and
   moved onto a wall of its radius a hair off (decision 8).
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
   angle, ties broken by signed curvature, then by where each stands
   halfway along the shorter, which neither crosses the other before, then
   by the chord a tenth along: a curve two cylinders meet along leaves a
   rim it touches to the fourth power of the way run, and its chord a
   tenth along stood 8e-10 off the rim's (5543224272); two such curves of
   different lengths, a tenth along each, were read at places at different
   distances, the shorter one's below the longer though it runs above
   (5544515442) — a last tie declines.
   Cycles are traced, then grouped into regions by a ray up the second
   parameter from each cycle's top. Each region gets an interior point. On
   a cone each arc ending at the apex ends at a vertex of its own, at the
   angle it arrives at, since the apex's own angle is rounding; the floor
   joins them, and closes below a face holding the apex within it, so that
   the region at the tip is bounded. No region keeps a stretch of the
   floor.
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
   wall, and leaves a non-manifold edge its four uses. A plane holding a
   cone's axis is read against the cone on either lobe the two rulings part
   it into, the axis side all round; a place at the very angle of a ruling
   is on neither.
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
10. **A lean read unstably** (#552, `combine/unstable.rs`). Once decision 11
    has turned the second operand, a plane of one operand and a cylinder, a
    cone or an elliptic wall (#571) of the other, their faces near enough to meet and not standing
    clear, may still lean off square to the wall's axis, or off along it,
    by more than decision 11 lays exactly and by a sine under a thousandth.
    The conic it cuts divides by that lean, so the step's rounding is
    magnified up to a thousand times: built, it may change for a change too
    small to see. The step is built as drawn and the body carries the least
    such lean, `Unstable` with its `Lean` and its degrees, which the
    interface says to the user (Tom's decision of 2026-10-10: never
    blocked). A decline stays a decline, with its own reason. Only pairs
    across the two operands are read, so a later step never notes an
    earlier one's lean, and a body handed back whole, where nothing was
    combined, carries no note.

    Why a thousandth of a radian, some 0.057°: it is where the kernel already
    stops trusting a near-parallel pair — two planes stop fixing the line
    they share (`canonical/planes.rs`), a surface stops parting a band across
    (`combine/band.rs`) — and past it rounding is magnified less than a
    thousand times, a ten-thousandth of the tolerance. Every hair the tilted
    draw makes, 1e-3° at most, is inside. On 20 000 fresh tilted seeds with
    decision 11, one case in eleven kept a lean in the band, and those
    failed at 1.4 %, against 0.18 % for the others; a bound of 1e-4 would
    miss the wedges of a hundredth of a degree, which are among the
    failures. A draft drawn under 0.057° gets the notice, and nothing else.

## The conic a plane cuts a wall along

#552, `brep/conic.rs` and `conic/`. One curve holds every conic a plane cuts
a cylinder or a cone along, anchored at a point `A` of it:

    p(τ) = A + T·τ·s(kτ²) + N·τ²·c(kτ²)

`s` and `c` the Stumpff functions, `T` the unit tangent at `A` and `N` the
second derivative there. The curve solves `p''' = −k·p'`: the sign of `k`
tells the ellipse from the hyperbola and the formula runs through the
parabola with no case of its own. A point stands to rounding times its
distance from the anchor along the curve, wherever the centre is: a plane a
hair off parallel to a bore's axis cuts an ellipse whose centre stands a
billion reaches away.

It is built from the wall's ruled form, `P(θ) = F(θ) + l(θ)·d(θ)`, `F` the
foot of the ruling at the wall's angle, `d` its direction, `l` how far along
it the plane stands; a cylinder is the cone whose rulings run along its axis.
Which nappe, which kind and where the vertices stand are read off that form,
and `T`, `N` and `k` at the anchor are closed forms of the section's
quadratic part and the wall's normal there — never of its constant term,
which squares the reach a hair from an apex. The anchor is the own vertex
nearest the origin when it stands within ten thousand reaches of it: one
parameter then holds the whole stretch in the box to rounding. Past that,
the cut is split at its far vertices into arms, each anchored where it
crosses the wall's foot circle; an arm crossing it nowhere never meets the
box and is not built.

On its plane a conic is read through the plane's parameters, on its wall in
`(θ, h)` or `(θ, l)`, a graph over the wall's angle — on a cylinder the
wall's angle grows as `√k·τ`. Its point at a grid angle of the wall is the
point of the cut on the ruling at that angle, which is where its triangles
take their samples. It is listed by its fields, and soundness evaluates it
by a formula of its own, apart from the kernel's.

A plane at 45° through where the axes of two walls of one radius meet
square holds one of the two ellipses they meet along, and cuts each wall
along it: the conic is that component of the meet. It is read so where it
is asked — one curve within the other when the conic is cut whole on one
of the meet's walls and the component lies in its plane all round, and,
crossing the conic with the other wall, a curve of that wall and its own
lying on the conic's plane is the conic, along the surface (9501873,
5550232928). It was declined as unsupported before round 6 of #552.

### What #552 ends with

- **Built.** Every section of a cylinder or a cone by a plane at any
  angle: an ellipse, a parabola, a branch of a hyperbola, two rulings
  through a cone's apex, the ruling a plane touches a wall along, the apex
  alone — beside the circles and the rulings the kernel held before. A
  conic whose far vertices stand past ten thousand reaches is built as
  arms, each on its own anchor.
- **Rounded.** A plane of one operand leaning off square to, or along, a
  wall's axis of the other by a lean whose move across the second operand,
  `s·L`, stands under `G`, twenty tolerances, is laid exactly square or
  along, and the kernel builds the circle or the lines it already holds
  (decision 11, `canonical/placed.rs`). It is decided once, for the
  boolean, before decision 1 reads the pair, on a plane against
  a wall's axis only. Tom's decision of 2026-10-10 asked for less than one
  tolerance across the part; `G` was measured and is one constant to move.
- **Warned of.** Past that, up to a sine of a thousandth, some 0.057°, the
  step is built as drawn, the body carries the least lean (`Unstable`,
  `combine/unstable.rs`, step 10 of the boolean), and the application
  says it: the tree marks the step « cas instable », and the step's panel
  names the plane nearly square, or nearly parallel, to the wall's axis,
  with the angle (`extrusion.unstable_square`, `extrusion.unstable_along`).
  It is never refused for it; a decline in the band is a broken step with
  its own reason.
- **Still declined.** Curves that are no conic — two cylinders at a skew
  angle, a cone against a cylinder or a cone of another axis — where their
  faces may cross; a face bounded by the curve two perpendicular cylinders
  meet along against a face at a slant; a face inside a cylinder's hollow
  against a cylinder at a skew angle to it (see the start of this
  document). Beside those, what the kernel cannot verify it declines as
  unverified or as a tie: about two tilted random parts in a thousand,
  hairs nearly all of them, named in `exact-kernel-failures.md`, "#552:
  conics".
- **Merged short of the bar.** Tom had it merged on 2026-10-12 knowing
  it is not perfect: the tilted draws above one in a thousand, one seed
  broken, a frustum's line of touch read as two arcs, a slower gate. The
  list is in `exact-kernel-failures.md`, "Merged as it stands".

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
  of a curve beside it, it keeps the two in order. Two edges touching
  between their vertices share the place they touch at: a line takes every
  sample standing on it within rounding, vertex or not, in its order along
  it, and a sample within rounding of another edge's is that one. The kernel
  puts no vertex where two edges touch without crossing — a rim touching the
  line a floor through a wall's axis cuts on it, at the rim's vertex or at a
  step of its grid; two curves grazing at a step of both — and with the line
  sampled at its ends alone, or a sample on each edge a rounding apart, the
  face holding both met its own loop in the middle of a segment, which no
  sweep cuts: the body was declined as undrawn (533647113, 533643649,
  533786336, 533600448).
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
  So does the point where a line across the wall's axis passes through the
  plane of one of its rims, standing inside it as close: a pocket's side
  leaving a step's plane 6e-4 inside its inner rim passed under the step's
  triangles, which reach as far inside as the rim's chords sag
  (5550106720). Not a line along the axis, whose ends are vertices.
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
  Each face is cut alone, and a triangle of one lying on a triangle of
  another across an edge they share, or passing through one beside a
  corner they share, is cut the other way with a neighbour where the flip
  makes neither: at a corner where a face turns into the matter, a wall's
  triangle cut across the corner, true to the region in the wall's
  parameters, sags by its chord through the face beyond — a post's wall
  where a hole cut into its foot leaves its ceiling (5550629622).
- **Cones.** A cone has one grid, cut for the furthest from its axis any
  circle or vertex on it stands, and every circle lying on it is sampled on
  that grid as well as on its own: the narrow rim and the wide one then
  stand at the same angles, and the strip between two of them is a flat
  trapezoid whose sag is the wide rim's chord's. A circle on no cone is
  sampled as before, bit for bit. A face is laid out in `(θ, l)`, `l` along
  the ruling; at the apex every angle is one point, so a loop passing
  through it — a point turned part way — is laid along the line of the apex
  from the angle it comes in at to the one it leaves at, and a face holding
  its apex within it — a point turned whole — takes that line as a loop of
  its own, once round the other way. A triangle with two corners on that
  line is the apex twice and is dropped: what is left is a fan from the tip.
- **Conics** (#552, `tessellation/sampling/conic.rs`). An edge along a conic
  is sampled the way the curve two perpendicular cylinders meet is, with one
  wall: at every grid angle of its wall, each sample the point of the cut on
  the ruling at that angle, so that the wall's outline finds one at every
  grid angle as it does on a circle; at the angle of every vertex on its
  wall; on every ray the wall's circles take; at its ends. Then wherever a
  chord would stand further from it than the tolerance, halved: seen on its
  plane the conic bends more than its wall's grid allows for, by as much as
  the plane leans from square to the axis. A cone's grid is cut for the
  widest any conic on it reaches, a ring with no vertex included. Two
  conics of one plane cut on walls whose axes cross, leaving a vertex they
  share together — a plane through a node of two walls of one radius cuts
  both along curves tangent there — each take the other's samples as rays
  of their own wall: sampled each on its own wall's angles, one chord of
  one crossed the other's samples by the node, and the horn of face
  between them could not be cut (5550117749). Not on two parallel walls,
  sampled on the rays they share already (5551609285). A circle and a
  conic ending at one vertex on the circle's own wall are the two rims of
  a strip of that wall pinching there, and drop the same grid angles beside
  it: within a fifth of `EPS` of the conic's plane, as the conic drops its
  own, where the circle drops one only within a thousandth of `EPS` of any
  other surface. Kept by the circle and dropped by the conic, the angle
  left the strip's one triangle across the step lying on the leaning face
  beside it — a half turn cut by a face decision 11 laid square and one
  leaning 9.4e-6° past the hair (5550133356). Not every plane an end lies
  on: a circle a hair off a block's face all along lost a grid angle by
  each end, and the drawing lost matter (5561366633).
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
  ruling. The point stands too for a vertex lying on both walls that stands
  off one of them by more than a thousandth of `EPS`: that one's samples
  are anchored at the point, not at the vertex, whose ruling would lean a
  hair over the other wall — a pin touching a 0.01° wedge's wall, one of
  the wedge's edges crossing the pin's ruling 7.6e-9 outside it
  (5550629812). A vertex standing on a wall anchors its samples there
  still: a floor 1e-7 under the touch leaves a line on the other wall
  there (5551116747, 5551409953). So does a vertex on a plane holding the
  wall's ruling through it, however far it stands off the wall: a face of
  that plane ends at the vertex, and the ruling anchored at the point runs
  through it — a cylinder 1e-7 too wide to touch a post, ending square to
  its axis on a plane through the post's, whose rim stands inside the post
  at the corner and whose end face the post's ruling crossed 5e-8 short of
  it (5564306306). A
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
  that vertex. All but lying on it is read along the stretch from the end,
  at the ray and halfway back: a circle crossing the surface again a step
  from where it crossed it at the end — a disc dipping a hair under a plane
  its wall crosses twice — stands on the line of that second crossing there,
  off the plane's face, and keeps the ray, which a wall a hair inside it,
  sampled on common rays, takes too: left to the partner alone, the
  partner's sample stood above the circle's chord across the step, and the
  cap of the sliver between them crossed itself (8538738, 8585618). Nor does
  the curve two
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
  `MESHED`, the slack the maquette gave truck's column (#449), which did not
  come to `main`; the exact volume of the body,
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
  - the rules on the triangles, `within_reach` — the result inside the
    leaves' box grown by `G` (`Scale::HAIR` tolerances), the furthest
    decision 8 moves an operand — and the listing.
- **The listing** is checked by `soundness::listed`, written apart from the
  kernel with its own formulas: every edge on the surfaces of the faces beside
  it, every end on its vertex, every vertex on the surfaces around it, loops
  closed, uses balanced. A cone is held to its own meridian, measured to the
  half-line from its apex rather than to the whole line; a loop through the
  apex is closed there, in the listing's own reading, by the turn that
  leaves it going round the axis nought times, and a tip is held to the
  apex as any vertex to its surfaces.

## Where it lives

`crates/solid/src/brep.rs` and `brep/`, beside the flats. The profile
vocabulary it reads — `Contour`, `Run`, `Frame` — is `profile.rs`.

It is public inside `cao_solid` so that the crate's own tests can reach it,
and refused above: `the_exact_kernel_stays_behind_the_body`, in
`crates/app/tests/architecture.rs`, fails on any production file of
`cao_part` or `cao_app` that names `cao_solid::brep`, and
`the_insides_of_the_matter_stay_in_cao_solid` on any of their tests. What the
rest of the workspace computes with is `Body` (`body.rs`, `body/exact.rs`,
`body/drawn.rs`), which holds the exact body or the flats and decides alone
which one an operation goes to.

The tests that hold it:

- `crates/solid/tests/a_bored_cylinder_on_two_kernels.rs`, the table: the
  flats and the exact kernel on #498's cases, OpenCascade quoted from #447;
- `the_exact_kernel_joins_and_cuts.rs`, the same cases counted by hand, and
  the cuts along a conic of #552 held to their arithmetic: a hole square to
  a slanted face out through the floor, a shaft cut at a slant, a point cut
  across, through its tip and parallel to its slope;
- `the_exact_kernel_turns.rs`, turns held to Pappus's volume and to the same
  profile cut into annular slabs, each raised and joined, along a grid of
  lines; chamfers, points and countersinks held to the arithmetic, and what
  a cone cannot meet declined or passed over; its smoke campaign of turned leaves and prisms under `--ignored`,
  compiled only with `--features campaigns`;
- `random_turned_solids.rs`, #448's harness drawing turns among prisms,
  slanted ones in a draw of their own (`Case::drawn_slanted`), tools on
  leaning planes in another (`Case::drawn_tilted`), on the exact
  kernel and through `Body`, its fast cases in the gate and its
  campaigns under `--ignored`, compiled only with `--features campaigns`;
- `random_exact_solids.rs`, the harness on the exact kernel, and on it again
  through `Body` as the application computes with it (`Application`), its
  campaigns under `--ignored` and compiled only with `--features campaigns`;
- `what_the_exact_campaigns_found_in_the_kernel.rs`,
  `what_the_exact_campaigns_found_in_the_triangles.rs` and
  `the_tangency_band.rs`, what the campaigns found, still failing ones under
  `#[ignore]` with their reason;
- `a_crescent_s_tip_under_a_plane.rs`, `a_plane_a_hair_from_a_face.rs`,
  `a_surface_a_hair_from_a_line_of_touch.rs` and
  `a_rim_grazing_a_ruling_of_touch.rs`, the seeds the campaigns of 5 October
  2026 failed, shrunk and gathered by what breaks them, ignored with their
  reason; those the triangles draw badly are with the triangles' findings;
- `a_plane_a_hair_off_square_to_a_wall.rs`, decision 11 held to the
  arithmetic: a post cut a ten-millionth of a degree off square, a bore cut
  a hair off along its axis, a tool a hair off a bore of its radius merged
  onto it.
