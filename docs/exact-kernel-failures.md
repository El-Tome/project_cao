# What the campaigns found in the exact kernel

Every distinct failure a campaign of #448's harness found in #498's exact
kernel, named, with its cause and whether it is fixed. [`exact-kernel-journal.md`](exact-kernel-journal.md)
says when each campaign ran; [`soundness.md`](soundness.md) says what the
rules are.

A failure is one cause: the step of the kernel or of its triangles that goes
wrong, and the geometry that triggers it. It is the kernel's when the body is
wrong or declined, the triangles' when a right body is drawn badly, the
harness's when a rule misjudges a right answer. The share is what the failure
explains of its campaign, estimated from a shrunk sample of each rule weighted
by that rule's count.

## Campaign 1

One hour from seed 1 000 000, on the kernel of 30 September: 51 298 cases,
980 failures. A sample of 138 was shrunk, 30 per rule — all 18 for Listed —
and every seed of the sample fell into exactly one of nineteen failures. Ten
are the kernel's, about 46 % of the failures; eight the triangles', about
54 %; one the harness's. No failure broke the Spans rule: whenever the kernel
answered, its exact body was right along every line.

| # | failure | owner | rules | share | status |
| --- | --- | --- | --- | --- | --- |
| 1-1 | Two parallel cylinders touching along a line: the grid step at the contact is withheld from every circle of both, far from the contact too | triangles | Volume, Closed | 27 % | fixed in round 1 |
| 1-2 | A plane tangent to a cylinder a hair from a line crossing that plane: corners are put on the circle by distance, leaving a two-sided face thinner than the tolerance | kernel | Answers, Listed, Closed | 14 % | 7 of 21 seeds fixed in round 1 |
| 1-3 | Two parallel cylinders crossing, on a grid angle or at a grazing angle: the step at the crossing is withheld from every circle of both | triangles | Volume, Closed | 13 % | 11 of 15 fixed in round 1 |
| 1-4 | A vertex partway up a cylinder wall no circle of the wall is sampled at: the triangle from it sags through, or lies on, the plane face meeting the wall there | triangles | Uncrossed | 11 % | fixed in round 1 |
| 1-5 | Two perpendicular cylinders tangent inside, the curve a figure of eight: its node is cut into the curve at its first passage only | kernel | Answers | 8 % | fixed in round 1 |
| 1-6 | The tolerance grows with a later leaf's reach: a gap an operand already holds apart falls under it, and its own lines and corners are merged across it | kernel | Uncrossed, Answers, Closed, Listed | 6 % | 10 of 12 fixed in round 1 |
| 1-7 | Winding rays cast for a region neither operand covers, from a point that lands on a face of the other | kernel | Answers | 5 % | fixed in round 1 |
| 1-8 | Identity by distance chains through a third surface or line: three stand pairwise within the tolerance but not all together | kernel | Answers, Listed | 4 % | 1 of 4 fixed in round 1 |
| 1-9 | Two parallel cylinders of one radius a few tolerances apart, crossing at a grazing angle near a third surface | kernel | Answers, Listed | 3 % | open |
| 1-10 | Perpendicular cylinders tangent inside with a radius not exact in binary: the snap moves the curve's cylinder by a rounding, and the trace no longer knows its surface | kernel | Answers | 1.3 % | fixed in round 1 |
| 1-11 | Two operands flush on two walls, each offset by a hair under the tolerance: their common edge and corner are not derived again | kernel | Answers | 1.3 % | fixed in round 1 |
| 1-12 | A perpendicular cylinder whose cap is tangent to the other: the curve touches the cap's rim to the fourth order, and angle and curvature both tie | kernel | Answers | 1.3 % | fixed in round 1 |
| 1-13 | Two holes tangent at a point of a wall through both axes: the relations of a support are completed once rather than to the end | kernel | Answers | 1.3 % | fixed in round 1 |
| 1-14 | A circle prism's wall holding the seam and a second loop: the outer loop is laid a whole turn away from its holes | triangles | Closed | 1.1 % | 5 of 7 fixed in round 1, a second failure behind it |
| 1-15 | A circle printed on a face, internally tangent to another, its tool's cylinder gone from the body: no common sampling | triangles | Closed | 0.8 % | fixed in round 1 |
| 1-16 | A corner of a face within a chord's sag of a curve of that face, but off it: the chord passes it on the wrong side | triangles | Closed | 0.6 % | 3 of 4 fixed in round 1 |
| 1-17 | Two cylinders sampled in common whose rays from vertices and from perpendicular curves go to one of them only | triangles | Closed | 0.6 % | fixed in round 1 |
| 1-18 | Two circles of one radius a hair apart on one face, never sampled in common | triangles | Closed | 0.3 % | fixed in round 1 |
| 1-19 | The listing's check misjudges the turn of a true face only a few tolerances across | harness | Listed | 0.3 % | fixed in round 1 |

Round 1 is how the lanes fixing the kernel and its triangles stood when
merged, at 23:57 on 30 September: the 138 shrunk seeds were run again, and
107 held. What each fix was is in the named tests of
`crates/solid/tests/what_the_exact_campaigns_found_in_the_kernel.rs` and
`..._in_the_triangles.rs`; since round 6, every case of the tangency band is
in `crates/solid/tests/the_tangency_band.rs` instead.

Three families stand out. Near-tangent pairs with a third surface near them
(1-2, 1-9) are one pattern: a tangency decided once as a line, then measured
again by distance near it. Identity by distance that never checks the support
it builds (1-6, 1-8, 1-11) is decision 5's refusal, written in the design and
nowhere in the code. And the triangles' contact rule withholds steps from
whole cylinders rather than near the contact (1-1, 1-3), which is two fifths
of the campaign by itself.

## Campaign 2

One hour from seed 2 000 000, on the kernel after round 1: 49 659 cases, 188
failures — five times fewer than campaign 1. A sample of 67 was shrunk, 40 of
the 161 Answers and every failure of the other rules, and sorted the same way:
**fifteen distinct failures, seven of them campaign 1's still open, eight
new**. Every new one is a configuration a fix of round 1 made hold, moved by a
hair: a node a hair from a wall, a cap a hair inside tangency, three touch
lines a hair apart. Status is after round 2, the 67 seeds run again on its
kernel: 39 hold.

| # | failure | owner | rules | share | status |
| --- | --- | --- | --- | --- | --- |
| 1-2 | A plane tangent to a cylinder a hair from a line crossing that plane — still the largest | kernel | Answers, Uncrossed, Listed | 65 % | 32 of 40 seeds fixed in round 2 |
| 1-9 | Two cylinders of one radius a hair apart crossing at a grazing angle near a third surface | kernel | Answers, Uncrossed | 9 % | 1 of 5 fixed in round 2 |
| 2-1 | A plane and two cylinders touching one another along three lines a hair apart, each pair decided apart | kernel | Answers | 6 % | 2 of 3 fixed in round 2 |
| 1-6 | A gap an operand holds apart falls under a later, larger tolerance | kernel | Answers | 4 % | 1 of 2 fixed in round 2 |
| 2-2 | A cylinder crossing two planes a hair from their common corner line, merged into the corner on one side only | kernel | Answers | 4 % | fixed in round 2 |
| 2-5 | A touch decided within the tolerance whose vertices stay where each was computed, up to a tolerance off another of their surfaces | kernel | Uncrossed | 3 % | open |
| 2-3 | Two perpendicular cylinders tangent inside, the node of their curve a hair from a wall cutting one of them | kernel | Answers | 2 % | open |
| 2-4 | A perpendicular cylinder's cap a hair inside tangency with the other: the cap decided tangent, the curve solved exactly | kernel | Answers | 2 % | open |
| 2-6 | A perpendicular cap's rim a hair inside a wall's chord, with no vertex there to keep the chord out | triangles | Uncrossed | 0.5 % | open |
| 2-7 | A cap's rim a hair from tangent to a perpendicular wall at the node: two crossings decided as one double root | kernel | Listed | 0.5 % | open |
| 1-8 | A vertex on a curve by its support, within the tolerance of each surface but not of the curve | kernel | Listed | 0.5 % | open |
| 1-10 | The snap moves a cylinder in the perpendicular curve while corners are found on the cylinder as drawn | kernel | Listed | 0.5 % | open |
| 1-17 | Two cylinders of one radius a hair apart given a shared sample by a third surface: a lune on a cap cannot be swept | triangles | Closed | 0.5 % | open |
| 1-18 | Two circles of one radius a hair apart left on a face no merge joins: the lune between them cannot be swept | triangles | Closed | 0.5 % | open |
| 2-8 | A wall holding a node and another curve's turning point on one grid ruling, which the sweep cannot cut at the campaign's tolerance | triangles | Closed | 0.5 % | fixed in round 2 |

Round 2 is the two lanes of 1 October, the kernel's taking a sixth decision —
two arcs of one surface between the same corners and within the tolerance of
each other are one arc — and the refusals of decisions 3 and 5 widened; the
triangles' taking common sampling to the cases round 1 left. Two cases the
triangles lane had made hold went back to the kernel declining when the two
lanes were merged: a grazing pair of cylinders whose tolerance a later leaf
grows (1-9 with 1-6). The kernel declines rather than answers wrong there; the
two cases are kept, ignored, with that diagnosis.

## Campaign 3

One hour from seed 3 000 000, on the kernel after round 2: 298 949 cases, 370
failures, 1.24 per thousand. A sample of 125 was shrunk — 30 per rule, every
Closed and Volume — and sorted: **twenty-one distinct failures, fifteen of
them known, six new**. Two of the new ones are the harness's: the listing's
check, written before decision 6, read some loops wrong once arcs could be
identified. 1-1 came back, reopened by round 2's room for walls decided to
touch. Status is after round 3, the 125 seeds run again on its kernel: 84
hold.

| # | failure | owner | rules | share | status |
| --- | --- | --- | --- | --- | --- |
| 1-2 | A plane tangent to a cylinder a hair from a line crossing that plane | kernel | Answers, Uncrossed, Closed | 22 % | 5 of 21 seeds fixed in round 3 |
| 1-6 | A gap an operand holds apart falls under a later, larger tolerance — mostly the seams a join leaves inside a face | kernel | Answers, Listed, Closed | 18 % | 13 of 14 fixed in round 3 |
| 1-9 | Two cylinders of one radius a few tolerances apart crossing at a grazing angle where a plane touches both | kernel | Answers, Listed, Uncrossed | 14 % | 4 of 11 fixed in round 3 |
| 1-8 | A corner within the tolerance of three surfaces laid by its support on a line further than the tolerance from it | kernel | Answers, Listed | 9 % | fixed in round 3 |
| 2-1 | A plane and two cylinders touching one another along three lines in one band | kernel | Answers, Uncrossed, Closed | 7 % | open |
| 2-5 | A touch decided within the tolerance whose vertices stay where each was computed | kernel | Answers, Uncrossed | 5 % | fixed in round 3 |
| 1-10 | The snap moves a cylinder in the perpendicular curve while corners are found on the cylinder as drawn | kernel | Listed | 4 % | 10 of 11 fixed in round 3 |
| 2-2 | A cylinder crossing two planes a hair from their common corner line | kernel | Answers, Uncrossed | 3 % | fixed in round 3 |
| 2-7 | A line a hair inside tangency with a cylinder decided a double root, though its crossings stand apart | kernel | Listed | 2.5 % | 6 of 7 fixed in round 3 |
| 1-3 | Two walls of one radius 1 to 1.4 tolerances off one axis: the window withheld round each crossing is wider than a grid step | triangles | Volume | 2.4 % | 8 of 9 fixed in round 3 |
| 3-1 | Three walls about one line: two touch inside, the third crosses both a hair from it, and the cusp between them | triangles | Closed | 2.2 % | fixed in round 3 |
| 3-2 | A plane tangent to one of two perpendicular cylinders along a ruling that crosses the other | kernel | Answers | 1.7 % | open |
| 1-17 | Two cylinders of one radius a hair apart given a shared sample by a third surface | triangles | Closed | 1.6 % | 3 of 6 fixed in round 3 |
| 3-3 | The listing's check sweeps every circle on a cylinder's face as a cross-section, which identified arcs no longer are | harness | Listed | 1.4 % | fixed in round 3 |
| 1-4 | A vertex partway up a wall whose ray the touching window drops | triangles | Uncrossed | 0.9 % | fixed in round 3 |
| 3-4 | Two perpendicular cylinders decided to touch at one point, left up to a tolerance into each other: their grid rulings cross | triangles | Uncrossed | 0.9 % | fixed in round 3 |
| 1-1 | Walls decided to touch take the whole tolerance as room: steps withheld over a wide arc — reopened by round 2 | triangles | Volume | 0.8 % | 2 of 3 fixed in round 3 |
| 1-11 | Two operands flush on two walls, each a hair off: one wall's line kept from each | kernel | Listed | 0.7 % | fixed in round 3 |
| 1-18 | Two circles of one radius a hair apart left on a face no merge joins | triangles | Closed | 0.5 % | fixed in round 3 |
| 3-5 | The listing's sweep of an arc a few billionths of a radian long reads its angle to one rounding | harness | Listed | 0.4 % | open |
| 3-6 | The sampling where a plane touches a wall strips a sample beside the contact, and the chord spans two steps | triangles | Volume | 0.3 % | fixed in round 3 |

Round 3, the lanes of 1 October afternoon: the kernel now merges faces of one
surface and one side across an arc nothing else uses — the seams behind most
of 1-6 — moves two surfaces decided to touch onto the touch once, as
surfaces, and checks that a corner its support lays on a curve stands within
the tolerance of it. What stays is one family: a plane and cylinders, or
cylinders of one radius, all within a hair of one another along one band, each
pair decided apart from the others. The analysts of campaign 3 agree on its
cure — the band decided once, as one decision, rather than pair by pair —
which round 4 writes as decision 7.

## Campaign 4

One hour from seed 4 000 000, on the kernel after round 3: 279 739 cases, 169
failures, 0.60 per thousand. A sample of 72 was shrunk — 30 Answers, 30
Uncrossed, every Closed, Volume and Listed — and sorted: **fifteen distinct
failures, nine of them known, six new**. Seven tenths of the failures are one
family, the tangency band. The heaviest new one was made by round 3 itself:
moving a cylinder onto a touch settles that touch and breaks another the same
cylinder holds.

| # | failure | owner | rules | share | status |
| --- | --- | --- | --- | --- | --- |
| 1-2 | A plane tangent to a cylinder a hair from a line crossing that plane | kernel | Uncrossed, Answers | 31 % | open: the band |
| 1-9 | Two cylinders of one radius a few tolerances apart, with a plane tangent to both or crossing at the crescent's tip | kernel | Answers, Uncrossed, Listed | 25 % | open: the band |
| 4-1 | A cylinder in two touches at once: the single move onto one touch breaks the other | kernel | Answers | 15 % | open |
| 2-1 | A plane and two cylinders touching along three lines a hair apart | kernel | Answers, Uncrossed | 8 % | open: the band |
| 3-2 | A plane tangent to one of two perpendicular cylinders along a ruling that crosses or touches the other | kernel | Answers | 5 % | open: the band |
| 2-2 | A curve crossing two planes a hair from their common corner line | kernel | Uncrossed | 3 % | open |
| 1-3 | A wall facing another over part of its height only: steps withheld or rays shared where nothing faces | triangles | Uncrossed, Volume | 3 % | open |
| 1-17 | Two cylinders of one radius a hair apart given shared samples by a third surface | triangles | Closed | 3 % | open |
| 2-6 | A perpendicular cap's rim a hair inside a wall's chord | triangles | Uncrossed | 1.6 % | open |
| 4-2 | A perpendicular node touch moves the first operand's cylinder, whose own corners stay where they were | kernel | Answers | 1.5 % | open |
| 4-3 | Two cylinders of one radius exactly a tolerance apart: not one surface, yet touching inside and moved onto each other | kernel | Answers | 1.5 % | open |
| 2-3 | Two perpendicular cylinders tangent inside, their node a few tolerances outside a cap that crosses the other | kernel | Uncrossed | 0.6 % | open |
| 4-4 | Two pieces of one floor joined through a slit and a connector shorter than the tolerance, listed as one face with two outer loops | kernel | Listed | 0.6 % | open |
| 4-5 | Two perpendicular cylinders touching at a point that lies on a plane holding a face | kernel | Uncrossed | 0.6 % | open |
| 4-6 | A ring whose parameter range starts on a grid step loses that step | triangles | Uncrossed | 0.6 % | open |

## Round 4: the band decided once

The lane of 1 October evening, on the kernel after round 3. Decision 7 of
[`exact-kernel.md`](exact-kernel.md): a region standing within the tolerance
of another surface all across, the two lying along each other, is wound by
the faces lying over it there as the arena decided the pairs, never by a ray
from a point femtometres off them. Taken first, as written, for the face's
twin and covered by it, the region broke the closure of 23 named findings:
the face's own region, decided at its point beyond the band, is kept or
dropped on its own, and only twins bounded by the same arcs can be decided
together. A region one operand covers on both sides is left to its own face
for the same reason. Two walls crossing are now ordered at the middle of the
stretch between their lines, which way each faces read at the place.

Of the twenty-nine seeds of 1-2, 1-9 and 2-1 campaign 3 named that still
failed after round 3, **two hold as drawn** (3024043, 3089284), and the
shrunk cases of two more hold (3192987, 3239627) while their drawn cases
break on other shapes of the band. Seed 50002029, ignored since a
120-second campaign as a crescent straddled by a block's side, holds too.
The 122 seeds of campaign 2's and campaign 3's samples that held after
their rounds hold still. A 120-second campaign from seed 70 000 000: 9 334
cases and 5 failures before, 9 147 cases and the same 5 seeds after.

What decision 7 does not reach, by the rule each breaks:

- **Uncrossed, nineteen seeds.** A cusp a tangency leaves, cut inside the
  band by a third surface's line — a block's side, the line two walls of one
  radius cross along. Its strips between the line of touch and that line are
  a skin thinner than what the rules tell apart, kept as the exact geometry
  has it. They are no twins and mostly no regions: the third line ends inside
  the band (3156716, 3130833, 3251776), so each strip is part of a region
  decided beyond it, or it lies on the walls and not on the plane (3268226,
  3123010, 1032742), so the plane's strip runs past the walls'. Dropping the
  skin wants the regions parted along the band, which no rule on regions
  does.
- **Answers, across the band on a cap.** A line and two circles touching
  within the tolerance on one plane leave slivers thinner than rounding: no
  point of them can be located (3003014, 3172512), two arcs leave a corner
  at one angle and one curvature (3239627 as drawn), or the overlay loses
  one and the listing does not close (3218425, 3061408, 3192987 as drawn).
- **Answers, a chain.** 3163577: the strip of a side between two lines of
  touch holds the line two walls of one radius cross along, which parts
  neither, and the crescent covers half of it.
- **Listed.** 3140167, unchanged.

Tried and set aside: laying the band's corners and lines on every surface of
the band — a corner within the tolerance of a surface touching one of its
own lies on it, and an arc whose two corners do, its middle within the
tolerance, too — so that the plane's strip is parted where the walls' end
and the strips become twins. Behind a switch, five of the shrunk failures
then hold, the slivers on caps among them, but seven named findings and a
unit test break, and one Answers turns Uncrossed: it wants its cases gathered
before its rule.

Reviewed the same evening. Each guard of decision 7 was taken out in turn
and the tests run. Without the band, without reading which way two crossing
walls face, or without reading their order at the middle of the stretch
between their lines, named findings fail. Without the boundary read,
without the bound on the angle, letting two planes in, taking the first
face or the last rather than the nearer, or skipping a face whose foot
lands on its boundary, every test stayed green.

Logged against a ray beside the kernel, the regions a guard turned away
read off the pairs what the ray says, in the tests and in a 300-second
campaign from seed 71 000 000 — but for one, in a 120-second campaign from
seed 70 000 000. Seed 70002066 shrinks to a block whose side passes
through the axis of a post cut from it, the wall crossing the side square
3e-7 from the block's corner: the sliver of the side between them has its
point within the tolerance of the wall. Read beside the wall it is taken
for outside the post, for the angle round a wall tells nothing of which
side a point stands on where the other surface crosses it square, and the
kernel declines. The bound on the angle turns it away, and so does the
boundary read, the sliver being wider than the tolerance: either alone
keeps the case, and the two overlap. The nearer face, the boundary read,
and the two together now each have a test that fails without them.

Both campaigns gave the kernel before round 4 and after it the same
failures — five from seed 70 000 000, three Answers and two Uncrossed; nine
from seed 71 000 000, four Answers and five Uncrossed, on some twenty
thousand cases each — and decision 7 answered for a handful of regions in
all of them: in random cases, the band is still rarely where a region
stands.

## Round 5: the kernel lane

The kernel's lane of 1 October night, on the kernel after round 4, beside
the profile draw. Two changes, both in [`exact-kernel.md`](exact-kernel.md):

- **Decision 8**: two parallel walls of one radius, one of each operand,
  their axes within fifty tolerances, are one surface; the second operand
  is moved onto the first's wall with what it built on it. Its hair was
  measured before it was set, on five-minute campaigns of profiles: the
  numbers are in the decision. It fixes 4-3. The review brought the hair
  down to twenty tolerances, the most no line of measure can see.
- **Decision 2's moves onto a touch**, rewritten: every touch of a surface
  read before it moves, a cylinder between two parallel planes moved
  midway, only pairs whose faces' boxes meet, the second operand's
  cylinder moved rather than the first's at a perpendicular node. It
  fixes 4-1 and 4-2: the eleven shrunk cases of campaign 4 hold.

Five-minute campaigns from fixed seeds, before and after, on the same
machine under a similar load:

| draw | from seed | before | after | fixed | made |
| --- | --- | --- | --- | --- | --- |
| square | 80 000 000 | 21 151 cases, 10 failures, 0.47 per thousand | 22 751 cases, 4 failures, 0.18 per thousand | 6 | 0 |
| profiles | 80 500 000 | 12 427 cases, 85 failures, 6.8 per thousand | 13 534 cases, 22 failures, 1.6 per thousand | 65 | 0 |

Fixed and made are counted over the seeds both runs reached. Seven
findings ignored until now hold, two of the kernel's file and five of the
triangles': their walls of one radius a hair apart are one wall now.

What stays of the twenty failures of the profile draw over that range:

- **Two walls of one radius further apart than the hair**, eleven: a hair
  of 1e-5 on a body of a reach of ten to a hundred, a hundred to a
  thousand tolerances. Taken for one, the wall would move by more than a
  line at a slant can miss; kept two, the walls cross at a grazing angle
  and the band of 1-9 is back. 80505830 and 80509593 are named, ignored.
- **A rounded rectangle's straight runs a hair off a block's sides**,
  planes three tenths of a tolerance to a few tolerances apart with the
  corner's wall touching both: the band of 1-2 on the profile draw
  (80500532, 80505325, 80507291, 80507349).
- The rest, one each: a Closed of the triangles (80510549), and band
  shapes of a plane, a wall and a third surface a hair from each other.

Reviewed the same night. Four things were found, each with a test:

- **The hair was seen.** At fifty tolerances, a line crossing the moved
  wall by a tangent, at a slant of a nineteenth, saw the move nineteen
  times over at each of its two crossings: a block bored twice
  forty-eight tolerances apart came out bored as the first bore alone,
  the line holding half again its room. No line the rules hold can see
  twenty, which decision 8 now takes. 80506598, 80509744 and the
  triangles' 3202168, thirty to thirty-seven tolerances apart, are
  ignored again: two walls crossing in the band.
- **Not sliding broke what sliding had held.** Kept from sliding along a
  touch it holds exactly, a wall its operand drew corners on stays a hair
  off its other touch, and seed 82504133 of a campaign of profiles, which
  round 3 held, declined. The slide is now made with the operand, as
  decision 8 moves it: 80501596 holds through it, and so do three
  failures of the profile draw from seed 80 500 000 (80505325, one of the
  band 1-2 above, 80506660 and 80513180) and five from 82 500 000.
- **Decision 8 makes failures of its own**: two in some twelve thousand
  profiles from seed 82 500 000 (82508905, 82512408). It moves the line
  the second operand touched a side along by a few tolerances, and a
  later leaf drawn touching the side on the old line stands in the band
  of 2-1. Named, ignored.
- **Three guards no test needed.** The faces' boxes read in
  `Operands::of`, the opposite sides the move midway asks for, and the
  radius the cylinder moved instead takes at a node: each was taken out
  in turn and the whole suite stayed green. Each has a unit test now. The
  named findings of 4-1 hold through several guards at once: seven of
  the twelve shrunk cases of campaign 4 fail only with the move midway,
  the reading of every touch and the faces' boxes all three taken out.

Five-minute campaigns from fixed seeds on the same machine, the kernel
before the review (round 5's kernel lane as it stood) and after it, fixed
and made counted over the seeds both runs reached:

| draw | from seed | before the review | after | fixed | made |
| --- | --- | --- | --- | --- | --- |
| square | 82 000 000 | 21 378 cases, 4 failures, 0.19 per thousand | 22 832 cases, 4 failures, 0.18 per thousand | 0 | 0 |
| profiles | 82 500 000 | 13 141 cases, 29 failures, 2.2 per thousand | 14 102 cases, 34 failures, 2.4 per thousand | 5 | 7 |
| profiles | 80 500 000 | 13 534 cases, 22 failures, 1.6 per thousand | 14 012 cases, 27 failures, 1.9 per thousand | 3 | 8 |

Every failure made is the hair's: two walls of one radius between twenty
and fifty tolerances apart, which fifty merged where a line could see it,
left two, crossing at a grazing angle, and declined. Against the kernel
before round 5, the profile draw falls from 104 failures to 30 over the
seeds both reached from 82 500 000, and from 85 to 26 from 80 500 000.

## Round 5: the triangles

The triangles' lane of 1 October night, on the kernel after round 4 and
the profile draw, beside the kernel's lane taking two walls of one radius
a hair off one axis for one wall (decision 8). It took campaign 4's open
failures of the triangles and the profile draw's failures of the rules on
triangles.

| # | failure | owner | rules | status |
| --- | --- | --- | --- | --- |
| 4-6 | A ring whose parameter range starts on a grid step, a rounding past it, loses that step at both ends | triangles | Volume | fixed in round 5 |
| 2-6 | A curve passing just inside a wall between two of its samples, with no vertex there — a perpendicular cap's rim under a lying post — is left outside the wall's chords | triangles | Uncrossed | fixed in round 5 |
| 1-3 | A wall facing another over part of its height only | triangles | Volume, Uncrossed | open: each seed of campaign 4 holds with the two walls on one axis, so decision 8 reaches them; two walls touching inside, nearly concentric, want a seam inside the face |
| 1-17 | Two walls of one radius a hair apart given shared samples by a third surface | triangles | Closed, Uncrossed | open: each seed holds without the third surface, and decision 8 reaches them |
| 5-1 | A corner of an operand whose plane was taken for another's stays a hair off that plane: a triangle of the plane's face no wider than rounding in its parameters stands up by the hair, on the wall beside it | kernel | Uncrossed | open |

A ring now takes each step of its grid once, wherever its range starts.
The samples of a curve not parallel to a wall — a circle about another
axis, the curve two other cylinders meet along — standing inside the wall
closer than twice a chord's sag, where the wall holds a face, now give
the wall's circles rays as a vertex there does; the edges are sampled a
second time, on those rays, when there are any. Both have a named test.
Campaign 4's six other seeds of 1-3 and 1-17 are named and ignored, as
the kernel's: each holds once its two walls stand on one axis or the third
surface is gone.

The profile campaign before this round, 300 s from seed 80 500 000:
12 265 cases, 84 failures, 54 of them Answers. Its 30 failures of the
rules on triangles, shrunk, are all the kernel's: 26 hold two walls of one
radius a hair off one axis — a slot's or a rounded rectangle's arc a hair
off a circle of its radius, a rounded rectangle whose corners make it a
disc a hair beside its twin — mostly beside a plane one of them touches;
three are the band without them (1-2, 2-1); one is 5-1. A square
campaign of the same length from seed 80 000 000 — 20 849 cases, ten
failures — gave seven of those rules, five the band and two the walls of
one radius. Two campaigns of 600 s on the round's triangles, from seeds
80 600 000 and 81 000 000, gave 64 and 14 failures of those rules: tagged
by the shapes they hold and the ones without walls of one radius shrunk,
the same families, 4-1, and one more of the kernel's: a wall bounded by a
line a block's side leaves 0.6 tolerances off it, the strip of the side
between that line and the one the wall crosses it along drawn on the
wall's triangles (81 019 107). Every one of them fails on the triangles
before the round too. Six of the profile campaign's are named and ignored,
one per family, as the kernel's.

After the round, the same two campaigns of 300 s: from seed 80 000 000,
22 228 cases and the same ten failures, 0.45 per thousand against 0.48;
from seed 80 500 000, 12 919 cases and 86 failures, 6.7 per thousand
against 6.8 — the 84 of before and two past its last seed. Neither drew a
ring or a cap's rim of the kind the round fixed, which campaign 4 met at
two seeds in an hour: the rules on triangles are now broken in these
draws by what the kernel leaves them, and decision 8 is where most of
that goes.

### Round 5's review

A second lane read the round the same night: each guard of the sampling
it added taken out or widened in turn, the tests run, and the campaign's
cases replayed on the seeds where the guard decides anything — from seed
83 000 000, fifteen thousand square cases, and from seed 83 500 000, eight
thousand profiles.

- **A ring's rays.** A ring now kept its steps wherever its range starts,
  but its rays were still placed by adding whole turns to an angle `atan2`
  gives within half a turn of nought: for a ring starting past half a
  turn, a ray at the very angle it starts at came out a rounding short of
  the range and was lost at both ends. Rays are placed on the turn as the
  steps are; a unit test holds every half step round it.
- **How far inside.** A sample of a curve gave the wall a ray only beyond
  the kernel's tolerance inside it, where a vertex gives one however
  little inside it stands. The kernel puts no vertex where a cap's rim
  passes the line its plane leaves on the wall by its tolerance, and seen
  from the wall's axis the rim stands inside by less, by the cosine of the
  angle between cap and wall: a bore ending inside a post with its rim so
  placed poked its cap through the post's chords. The two bands are one,
  and a case built for it is named.
- **Seed 3 243 921**, ignored as the kernel's, holds since fix round 3,
  and is held by the gate.

Left as they are, and what was measured of them. The curves two other
cylinders meet along give rays on 19 of those seeds, the face test turns
samples away on 408, samples within the kernel's tolerance stood inside a
wall on 8, and on 54 the second pass leaves new samples inside a wall with
no ray: dropping the meets, dropping the face test, or sampling again
until nothing new stands inside changed no outcome on its seeds. With
every cylinder on an axis of the origin, a meet of two cylinders square to
a wall is the farthest of a hollow from it only where the boss bounding
the hollow covers the wall beyond, which then holds no face there; with
one of them parallel to the wall, the wall takes its rays from the two
standing close. No case found needs the meets, and none the face test,
which keeps the second pass rarer. Replayed seed by seed on the round's
two 300-second windows, the code after the review breaks the rules on the
same seeds as the round's own.

Campaigns of 300 s before the review and after it, on the round's code
and on the review's: from seed 83 000 000, 22 442 square cases and then
22 669, the same ten failures, 0.45 per thousand and 0.44; from seed
83 500 000, 13 075 profiles and then 13 229, the same 80 failures — 44
Answers, 32 Uncrossed, 3 Closed, 1 Volume — 6.1 per thousand. Every one of
those breaking a rule on triangles also breaks it on the code before the
round. Neither window drew a ring starting on a ray nor a rim within the
kernel's tolerance of a wall.

## Campaign 6

An hour on each draw, on the kernel after round 5: the square draw from seed
6 000 000, 282 219 cases and 90 failures (0.32 per thousand); the profile
draw from 6 500 000, 174 256 cases and 310 failures (1.78 per thousand). A
sample of 44 and one of 58 were shrunk and sorted. **Nine failures in ten
are now the tangency band**, on both draws: a plane and one or two
cylinders, or two cylinders of one radius further apart than decision 8's
hair, standing within the tolerance of one another along one band, each pair
decided apart. What is new is small: the parallel form of 4-5 (two walls
decided to touch, their line of touch on a plane crossing both square), a
touch held to a rounding, a ruling crossing exactly a line of inside touch,
decision 8's move seen by the rule that bounds the box, and the listing's
check reading an arc merged exactly at the tolerance.

| family | square draw | profile draw | owner |
| --- | --- | --- | --- |
| 1-2 a plane tangent to a cylinder a hair from a line crossing it | 43 % | 23 % | kernel |
| 1-9 two walls of one radius beyond decision 8's hair, with a third surface | 10 % | 32 % | kernel |
| 2-1 a plane and two cylinders touching along three lines a hair apart | 13 % | 23 % | kernel |
| 3-2 a plane tangent to one of two perpendicular cylinders along a ruling crossing the other | 10 % | 7 % | kernel |
| other band forms (4-1 and 4-5 one-sided, 1-6 in a crescent, 1-12 on a touch line) | 14 % | 8 % | kernel |
| outside the band (1-8, 5-1, the harness's three, the triangles' few) | 9 % | 8 % | kernel, harness, triangles |

## Round 6: the band laid out

The lane of 2 October, on the kernel after round 5, in two steps: the cases
first, then the rule.

**The corpus.** `the_tangency_band.rs` gathers every case of the band
known: the 83 band seeds of campaign 6's two samples, shrunk, by family;
the band's findings the two findings files kept ignored, moved there; the
five findings that hold near the band and that round 4's switch — the
band's lines and corners laid on every surface of it — broke when run on
the kernel after round 5 (3150523, 3192987, 28000951, 3157560, 3175702);
seven findings of two walls of one radius a hair off one axis, still
ignored though decision 8 had made them hold; and nine shapes of the band
built at hairs of half a tolerance, two, twenty, two hundred and two
thousand. Of its 157 cases, 36 held before the rule.

**The rule** is decision 9 of [`exact-kernel.md`](exact-kernel.md): the
band of two surfaces decided to touch laid out once, the same on both —
its corners on both surfaces, a line through each along the line of touch,
and a corner wherever a surface square to the band crosses one of its
lines — so that every strip is parted where the others are and decision 6
decides the twins. Round 4's switch did half of it, laying corners and arcs
found within the tolerance; on the kernel after round 5 it made 33 of the
corpus hold and broke the five findings. What was missing was the line
through a corner of the band: a corner laid on both surfaces with no line
to close the strip beside it gave both faces the same triangle, which is
what broke 3175702. With the lines, the five hold. Three things were
learned on the way, each a guard with its case: the band is laid only where
faces of both surfaces stand (90509248), only where faces of both operands
meet it off the line of touch (90504120), and not where a third surface
lying along the band touches neither of its surfaces (90016363). The
listing's check was found reading a circle about another axis as a
cross-section of the wall it bounds a face of (90005701), which the
harness now sweeps along chords.

| | the corpus | square draw, 300 s from 90 000 000 | profile draw, 300 s from 90 500 000 |
| --- | --- | --- | --- |
| before | 36 of 157 hold | 22 381 cases, 4 failures: Answers 2, Uncrossed 2 | 13 894 cases, 31 failures: Answers 16, Uncrossed 12, Volume 2, Closed 1 |
| after | 113 of 163 hold | 22 292 cases, 1 failure: Uncrossed 1 | 13 522 cases, 15 failures: Answers 8, Uncrossed 4, Volume 2, Closed 1 |

The campaigns ran side by side on the same machine. Over the seeds both
reached, the square draw falls from 3 failures to 1 and the profile draw
from 30 to 15, none made: 0.18 per thousand to 0.04, and 2.2 to 1.1. The
six cases the corpus grew by are what the first campaigns on the rule found
it breaking; four hold since the guards, and the other two, shrunk from
seeds that hold now, fail before round 6 too.

What the rule leaves of the corpus, each case ignored with its reason:

- **3-2, a plane a hair inside a wall** (6136557, 6199123, 6072532,
  6143816, 6550239). The plane cuts the wall along two lines a hair apart;
  the pair is not decided to touch, and decision 9 lays out only bands of
  pairs that are.
- **3-2, a ruling of touch crossing a square wall where it grazes it**
  (6153255, 6611564, the pin at half a tolerance). The band is laid, but the
  only surface crossing it there is the square wall, nearly along the line
  of touch where it grazes it, and a line of the band crosses it a hair
  either side: no surface parts the band across, and its strips stay open.
- **3-2, a circle kept for an arc of the meet** (6185693, 6123681, the pin
  at twenty, two hundred and two thousand tolerances). Decision 6 keeps the
  rim's circle for an arc the curve two perpendicular walls meet along runs
  within the tolerance of; the other wall sees that circle only as a
  chord, and the chord stands further off than the tolerance. Not the
  band's: the circle is to be traced on that wall with the meet's own trace.
  Fixed by #528: the wall sees the circle as the curve the rim's own wall
  meets it along, and the five hold, with the bar thirty tolerances across
  its line and the ten seeds of `a_rim_grazing_a_ruling_of_touch.rs`. The
  same reading holds the curve a wall meets a square one along where it
  lies on a second wall parallel to the first, a hair off it, which saw it
  in the first's angles: a crescent's tip (94501738, and 8541588, 8544067
  and 8568600 of `a_crescent_s_tip_under_a_plane.rs`).
- **1-9, two walls of one radius crossing at a grazing angle** (6517915,
  6546656, 6587058, 6598801, 6650833, 6657327, 6074790, 6563127, 6621877,
  6637572, 6500611, 4165661, 80510549). Further apart than decision 8's
  hair, the walls cross along two rulings and are not decided to touch: no
  band of theirs is laid out. Where a plane touches both, its two bands
  carry the crossing line, and ten of the family hold so.
- **2-1, three lines of touch in one band** (6086149, 6576153, 6529277),
  not traced since the rule — 6529277 is drawn closed since a line takes
  the samples standing on it (#533, below); and 6155665, whose bore dips a hair into both
  the wall and the plane resting on it, a band the guard of 90016363 leaves
  as it was.
- **1-2, a plane touching a wall a hair from a line crossing it** (6168409,
  6504965, 6509947, 6643772, 6638088, 80507291), not traced since the rule:
  eighteen of campaign 6's 23 seeds of the family hold, and every shape of
  it built.
- **4-5, two walls touching on a plane through both axes** (6063426,
  6122353): the plane's lines with the two walls and the line of touch are
  one line, computed apart.
- **1-6** (6091545), **4-1** (6080810, 6074582, 6528115, 6521564) and
  **1-12** (6128869): a crescent under a later leaf's tolerance, a wall
  touching two parallel planes from one side, two curves touching to the
  fourth order — campaign 6's diagnoses stand; none is a band's layout.
- **The triangles'** (6513598, 6518903, 6628386, 3202168): a step withheld
  where a wall faces another over part of its height.
- **Shrunk from seeds the first rule broke** (90501389, 90504257): the
  seeds hold now; shrunk, they fail on the kernel before round 6 too.

Each guard of decision 9 was taken out in turn and the tests run. Four
fail named cases without them: the band laid only on faces of both its
surfaces (90509248), only where faces of both operands meet
(90504120), not where a third surface grazes it (90016363), and arcs kept
between faces of one operand on two surfaces decided to touch (eleven
cases, 3192987 and 3130833 among them). Four others changed nothing on the
corpus, on a scan of a bar ending a hair either side of a slot's run, or on
300-second campaigns of both draws run without each: a surface parting the
band across only where it stands square enough to the line of touch,
found with 90509248 before the faces' guard reached it too; an arc laid
on a surface only where it stands within the tolerance all along, and only
where the surface can trace it; and a corner or an arc laid only along the
stretch of a band laid out. They say what laying a band means, and are
kept untested.

### Round 6's review

A second lane read the round the same morning, looking for a case of the
band the corpus lacked, a strip taken for a twin thicker than the
tolerance, a line laid on a surface further than it, a decision taken
twice, a named finding broken, and a test that does not fail without its
fix.

What held. The findings moved into `the_tangency_band.rs` are the same
tests word for word, and none that held was ignored on the way. Each of
the four guards of decision 9 that a case holds fails that case taken
out; the four kept untested changed nothing on the review's sweeps either.
Geometry bounds the strips: a line of the band runs along the line of
touch through a corner within the tolerance of both surfaces, and so
stays as near them all along; the gap between a plane and a wall, or two
walls, grows away from the line of touch, so a strip whose boundary
stands within the tolerance stands within it inside.

**Sweeps of built shapes.** The round's nine shapes at thirty-four hairs,
from a tenth of a tolerance to fifty thousand, either way, in the three
orientations the axes can be turned to: none made by the rule, nine in
ten of the failing ones fixed, and what fails is the pin of 3-2. Some
thirty shapes of the review's own, plane and wall or two walls with a
cap, a side or a second band a hair from the line of touch, in the same
hairs and orientations: none made. Twenty shapes of profiles, joined and
cut: two slots whose ends
touch on the plane of their runs (2-1) were declined by the rule at every
hair from three tolerances to seven thousand, and held before it. Two
families not in the corpus fail before the rule and after it alike, and
are gathered: a bar resting on a top, bored by a hole whose circle grazes
the line of touch (3-2); a slot cut by a bar touching its run, the bar's
cap a hair past the run's end, where the bar and the slot's end, square to
each other, touch the run's plane at one point.

**The harness.** The listing's turning check, changed by the round for
90005701, no longer met that seed on the finished rule: the grazing guard
leaves the band of the stock and the bore as it was, and the seed holds
with the check as it was. A wall's face bounded over a hair by the rim of
a wall touching it outside, built by hand, now holds the change.

**Campaigns.** 900 seconds each, side by side, on the kernel before round
6, on the round's head, and on the review's: the square draw from 92 000
000 and the profile draw from 92 500 000, 93 500 000 and 94 500 000. Round
6's 300-second windows had seen nothing made; these found the profile draw
made by the rule on seventeen seeds while fixing ninety-two, every one two
walls of one radius further apart than decision 8's hair with a plane
touching both — a rounded corner, a slot's end, a disc and its twin a
hair aside. Gathered, shrunk, and checked to hold before round 6, they
gave two fixes:

- **A band lays a place on its other surface only where faces of both
  stand**, as it does a corner of the band. A slot cut by a second
  starting ten microns past its end (92530408) leaves the plane of their
  runs with no face between the two ends; the run's line across that gap
  was laid on both ends' walls, while the corner where those walls cross,
  in that gap, was not laid on the plane — and the piece of line and each
  wall's circle left the run's corner together and tied. The two slots
  whose ends touch hold with it.
- **A skin held on two of three twins is read in the frame of its own
  first wall.** A band on a plane touching both walls of a crescent's tip
  makes three twins, the plane's strip first; the crescent's two walls
  were handed to decision 6's reading of a skin seen from the plane,
  turned from both, and the skin was taken for a crack (92510427). The
  frames were one while twins were two.

A third fix, a node of the band taken for the band's corner it falls on
where decision 5 keeps the two apart, held three of the shrunk cases and
broke four that held on every kernel before it; it was taken back, and the
four are kept.

| | square draw from 92 000 000 | profile draw, three windows |
| --- | --- | --- |
| seeds all three runs reached | 61 124 | 113 182 |
| before round 6 | 18 failures | 187 failures, 1.65 per thousand |
| round 6's head | 9, none made | 112, 0.99 per thousand: 17 made, 92 fixed |
| the review's head | 9, none made | 107, 0.95 per thousand: 13 made, 93 fixed |

Nothing the review's head fails held at round 6's head. What it leaves:

- **The crescent's tip a plane touches both walls of** (92502574,
  92502982, 92503994, 92519519, 92524625, 92533762, 92537758, 93530253,
  94501738, 94511411, 94523199, 94531594, and as drawn 92530408 and
  93535310, whose shrunk forms fail before round 6 too). The band lays a
  corner of one wall near the walls' crossing onto the other, within the
  tolerance of it; the crescent's two real faces then share vertices
  there and are
  drawn with the same triangles, or decision 5 keeps a node the band puts
  on the other wall apart from that corner and two vertices stand at one
  place. Refusing such a corner as decision 5 does holds seven of them and
  undoes four of the 1-9 cases the rule fixed (6533713, 6547043, 6579931,
  6582528). The family wants its own rule, from the cases gathered.
- The two new families of the sweeps, failing before round 6 too.

## #528: the crescent's tip

The lane of 5 October, on #526's branch, over the 59 seeds tonight's
campaigns sorted into the family (`a_crescent_s_tip_under_a_plane.rs`) and
the cases of `the_tangency_band.rs` round 6's review left it: two walls of
one radius whose axes stand a hair apart, beyond decision 8's, crossing
along two rulings at a grazing angle under a plane that touches both or
passes through the crescent's tip.

**The rule.** Decision 2 decides such a pair to *graze*, once: two parallel
walls of one radius crossing at an angle whose sine is a hundredth or less
stand within the tolerance of each other the tolerance over the angle from
either line, as two walls touching do. Three readings of that symbol:

- decision 5 merges a corner one wall carries near the crossing with the
  node a band puts on the other there, as it does for two walls touching:
  a refusal measured on their crossing lines kept two vertices at one place
  (the ties ordering arcs round a corner);
- decision 9 lays the band of each crossing line out as it lays a line of
  touch, a corner nearer the other line standing in that line's band, and
  keeps an arc between faces of one operand on the two walls — the
  crescent an operand holds, which the band parts and decision 6 decides:
  the strip of the crescent's tip, left on both walls on one side of a
  third surface and taken away on the other, gave both walls the same
  triangle there;
- the curve a square wall meets one of the two along, taken for an arc of
  the other over the stretch the band holds, lies on the other over that
  stretch alone — it was taken for a curve the other carries all along —
  and is seen there in that wall's own angles: read in its own wall's, it
  stood the angle the two axes part by off its corners, and a loop it
  closed could not be placed.

| | before | after |
| --- | --- | --- |
| `a_crescent_s_tip_under_a_plane.rs` | 0 of 59 | 46 of 59, and two seeds the first band made, which hold |
| `the_tangency_band.rs` | 132 of 197 | 147 of 197 |
| `what_the_exact_campaigns_found_in_the_triangles.rs` | 95 of 113 | 96 of 113 (8018367) |
| `what_the_exact_campaigns_found_in_the_kernel.rs`, the 18 cases | all | all |

Ten-minute campaigns of each draw, base and lane side by side on the
machine, from tonight's first seeds, over the seeds both reached: the square
draw from 8 000 000, 14 221 seeds, 3 failures on each and the same three;
the profile draw from 8 500 000, 9 996 seeds, 9 failures before and 5 after,
0.90 per thousand to 0.50, none made; the profile draw through the body from
8 500 000, 15 062 seeds, 11 before and 7 after, 0.73 to 0.46, none made.

What the first band made, and the guard each gave:

- **8002613**, a crescent's tip dipped into by a wall a hair off both:
  the band of the lower line took the corners of the upper tip, within the
  tolerance of both walls too. A corner nearer the other line is that
  line's.
- **8501866**, two walls a hair apart crossed by a wall square to both:
  the curve the square wall meets one along, laid on the other over its
  stretch, was registered as a curve the other carries all along
  (`held.rs` read any parallel wall as carrying it), and bounded a sliver
  with the other's own curve. A wall carries it all along only when it is
  one of its own two.

A merge tried on the way and taken back: two corners within twice the
tolerance on a plane and on two surfaces touching along a line it crosses,
taken for one at the place the three fix. It held six of the family's
cases, five of `a_plane_a_hair_from_a_face.rs`, four of
`a_surface_a_hair_from_a_line_of_touch.rs` and seven of the triangles', and
broke eight that held, 1034172 among them, whose corner a bore touching a
block leaves stands 1.9 tolerances off the line of touch and must stay off
it. The √2 hairs between two such corners are decision 1's, and the lane of
a plane a hair from a face has them.

What the rule leaves of the family, each case ignored with its reason:

- **Not the tip's** (8532939, 8565492, 8569734, 8572596, 8555088): decision
  1 took a slot's cap and floor for a disc's wall and floor, and two
  corners stand √2 hairs apart on one plane and one line of touch, which
  decision 5 keeps two. 8567503 and 8500450, a plane a hair from a face;
  8579668, two faces of a block's top drawn over each other beside a disc
  and a slot a hair off its side.
- **The triangles'** (8585454, 8587307): walls just past decision 8's hair
  stand within a fifth of a tolerance of each other over a tenth of a turn
  about their crossing, where the triangles withhold every step, and a
  wall's triangle spanning it crosses a cap's.
- **The tip's, still** (8555390): the crescent's tip, ended on a cap a
  hair from the line a box's side touches it along, stands past the end of
  that side's face, where no band is laid; 8586384, a ring's wall square to
  the two crossing each tens of tolerances from the other inside their
  band; 8575631, declined as unverified, not diagnosed. In
  `the_tangency_band.rs`, 6074790 and 93530253 are still drawn crossing and
  6500611 declined, the band laid out; 6091545 is 1-6.

## #528: the six lanes together

The lanes of 5 October — the crescent's tip, a plane a hair from a face,
the line of touch, the triangles, a rim grazing a ruling of touch, a hole
touching its outline — merged one at a time onto #526's branch, each held
after its merge to every case that held at base or on any lane alone.

| | before | after |
| --- | --- | --- |
| `a_crescent_s_tip_under_a_plane.rs` | 0 of 59 | 57 of 61 |
| `a_plane_a_hair_from_a_face.rs` | 0 of 19 | 16 of 19 |
| `a_surface_a_hair_from_a_line_of_touch.rs` | 0 of 23 | 8 of 23 |
| `a_rim_grazing_a_ruling_of_touch.rs` | 0 of 10 | 10 of 10 |
| `the_tangency_band.rs` | 132 of 197 | 170 of 197 |
| `what_the_exact_campaigns_found_in_the_triangles.rs` | 95 of 113 | 111 of 116 |
| `what_the_exact_campaigns_found_in_the_kernel.rs`, the 18 cases | all | all |

What two lanes did to each other:

- **8575631** held on the line of touch's lane alone and declined once a
  plane a hair from a face's joined it: a slot's cap taken for a bore's
  wall, its sides a hair off the two faces the bore all but touches.
  Decision 2's finer reading took those faces, a diameter apart either
  side of the wall, for two planes a hair apart, and decided the wall
  against each at the stock's tolerance: it missed one and crossed the
  other, and the slot's lines of touch stood on surfaces decided apart.
  The finer reading is for two planes on one side of the wall alone.
- **The corner on two planes across each other** was decided by two lanes,
  the line of touch's in `standing` and the triangles' in `Planes::place`:
  one rule is kept, the line of touch's, which the pool and the cut both
  read, and the triangles' three seeds hold under it.
- **A meet on a wall a hair beside its own** was seen by two lanes two
  ways: as that wall's own meet with the other cylinder (the rim's), and in
  that wall's angles (the crescent's). One rule: the wall's own meet where
  the curve's middle stands within the tolerance of it, its angles
  otherwise, the wall beside read once off the meet's own two cylinders.

Thirteen seeds that no lane held alone, or that one held but left ignored
in another lane's file, hold together and are no longer ignored: 8500450,
8532939, 8555088, 8565492, 8567503, 8569734, 8572596, 8579668 of the
crescent's file; 8530908 of the line of touch's; 6143816 and 6638088 of
the band's; 8517827 and 8534264 of the triangles'.

## #528: measured like for like

The merged lanes against the night's campaigns, on the same seeds and the
same number of cases per draw (`exact-kernel-journal.md`, 5 October): the
square draw 14 failures in 88 833 cases before, 4 after; the profile draw
55 in 54 104 before, 13 after; through the body 111 in 89 821 before, 26
after.

| | before | after |
| --- | --- | --- |
| `a_plane_a_hair_from_a_face.rs` | 16 of 19 | 17 of 20 (8528992) |
| `a_surface_a_hair_from_a_line_of_touch.rs` | 8 of 23 | 8 of 25 |
| `the_tangency_band.rs` | 170 of 197 | 171 of 197 (6500611) |
| `what_the_exact_campaigns_found_in_the_kernel.rs`, the 18 cases | all | all |

What the merge made, three seeds held the night before:

- **8528992**, a rounded rectangle a hair taller than its two corners, its
  top a hair off a block's face. Decision 10 laid the top on the face and
  carried the operand along, the upper corners' walls onto the lower
  ones: two walls the operand built apart became one, and the cut
  declined. Fixed: a carried move is not made where a surface it carries
  would become one with a surface of its own operand left behind. The
  same fix holds 6500611, the band's rounded rectangle a hair taller than
  its corners.
- **8515425** and **8542629**, the line-of-touch lane's corner on two
  planes across each other, laid on the line they share at its foot, a
  hair off a wall it lies on. In 8515425 the wall is a disc's, crossing
  the band of a bore touching a plane, and two faces are drawn crossing;
  in 8542629 the wall touches a block's side along a ruling across that
  line, every rim of the wall is sampled square to the corner, on the
  side, and the block's top, whose rim it is too, has its rim sample on
  its own edge along the side and is left uncut — drawn closed since the
  edge takes the samples standing on it (#533, below). Not fixed then. A corner kept
  where it was found, or laid where a wall crossing the line crosses it,
  fixed one or both and broke up to eight seeds the merged lanes hold
  (8504113, 8511276, 8540216, 8544675, 8562569, 8573267, 8586557, 8586895);
  the one rule that broke none of them, and fixed 8550377, chose its wall
  by the order the support lists them in. Both cases are in
  `a_surface_a_hair_from_a_line_of_touch.rs`, ignored with the reason.
  Undoing the lane's rule fixes both and fails 8010303, 8010399, 8055558,
  8586895 and three cases of the band.

What the campaigns leave, by the family each seed's shrunk case was filed
in:

| family | square | profiles | through the body |
| --- | --- | --- | --- |
| a third surface a hair from a line of touch | 4 | 6 | 9 |
| a plane a hair from a face | | 3 | 3 |
| the crescent's tip | | | 4 |
| drawn wrong by the triangles, or filed with their findings | | 2 | 4 |
| shrunk case holds, the seed as drawn does not | | 2 | 6 |

The last row is 8544067 and 8552226 on both profile draws, and 8512856,
8540477, 8581445 and 8588829 through the body: shrunk on the night's
kernel, each case holds now, and the seed wants shrinking again on this
one.

## #533: turns

Two campaigns of `random_turned_solids.rs`, two hours each, side by side on
the branch's tip (`01c30e2`), on 6 October: the off-lattice turned draw
(`Case::drawn_turned_off_the_lattice`) on the exact kernel from seed
533300000, and through the application's body from seed 533400000. Every
seed they named was shrunk by
`the_seeds_a_turning_campaign_named_are_shrunk_one_by_one`, and each shrunk
case run again on both paths to tell which kernel computed each step. Two
hours is not the night the issue asked for; the rates below are what they
are worth.

| | cases | broke a rule | asking for an ellipse, declined | a wall a hair thin, declined as no profile |
| --- | --- | --- | --- | --- |
| on the exact kernel | 71 475 | 53 (0.07 %) | 7 791 | 573 |
| through the application's body | 13 520 | 72 seeds (0.53 %) | 1 478 | 0 |

No turned leaf breaks a rule alone on the exact kernel: every failure there
is a boolean meeting a turned surface. By family, the seeds' last six
digits after 533:

- **The flats turning a wall a hair thin**, 64 seeds, every one through the
  application's body: a hole a hair inside its outline's side, turned
  whole or part way, alone or joined to the part. The exact kernel declines the section as no profile, the application
  turns it on the flats, which are main's sweep unchanged
  (`a_profile_beyond_the_band_of_its_axis_is_turned_by_the_flats_as_main_turned_it`),
  and they leave an end facing the wrong way along that wall: the flats' own
  defect, out of #533's scope. Named by 400107. The others: 400076 400215
  400360 400461 400495 400628 401117 401440 401642 401891 401925 402109
  402344 402464 402529 402653 402908 403425 403793 403860 404076 404112
  404171 404350 404371 404649 404673 404842 404964 404998 405140 405152
  406130 406289 406550 406756 407246 407550 407625 407671 407688 408150
  408174 408443 408737 408789 408984 409095 409409 409550 409913 409933
  410141 410511 410814 410835 411056 412073 412077 412334 412364 412411
  413237.
- **A surface or a plane a hair from another**, 40 seeds on the exact
  kernel and 4 through the body: a coordinate a hair, from 1e-8 to 1e-5,
  off the one it would meet; the families prisms already fail by
  (plane-a-hair, the band, the line of touch), reached by turned cylinders
  and discs as by raised ones. Named by 300839, 301022 and 341432. The
  others: 300796 304139 304813 305000 305495 311777 315421 319050 319710
  320588 320774 321257 325448 327154 327166 330490 331748 332674 338751
  338790 339621 340695 342139 343652 347353 348114 348459 349284 351190
  351969 353686 358886 360327 365286 365909 369492 371154; through the
  body 404973 405215 406729 410520.
- **An axis leaning a hair**, 6 on the exact kernel and 2 through the body:
  a turn about an axis leaning 1e-7 to 1e-3, which the reading lays square,
  meeting a surface square to it. Named by 335255. The others: 307526
  317654 328870 337056 337834; through the body 401606 412197.
- **Surfaces touching, no hair**, 7 on the exact kernel and 2 through the
  body: a cylinder whose axis lies on another's tangent plane, two
  cylinders square to each other touching at a point, and a partial turn
  whose ends, planes holding an axis, meet a cylinder turned about that
  axis along its rulings, declined as a tie. With the turned cylinder
  raised as a prism instead, 330824 and 413234 fail alike: the kernel's
  line of touch, not the turn. Named by 330824, 413234 and 328889. The
  others: 306876 308079 316268 346954 365319; through the body 412761.

Each named seed is in `what_the_exact_campaigns_found_in_the_kernel.rs`,
ignored with its reason. None was fixed: every family but the last is one
the kernel already fails on prisms, or the flats' own, and the partial
turn's tie is the one finding that belongs to turning; it is left for the
cones' issue, which builds the same ends.

### The night's campaigns

Run on 6 October in release, side by side, on the branch at `2d7623e` and
on a copy of `main` at `dc5bfe5`, each tree with its own target directory.
The turned campaigns ran four hours each, the square and profile draws
the same seeds and the same number of cases on both trees
(`CAO_FUZZ_CASES`), and the part campaign an hour on both, compared over
the seeds both reached. A case that never answered ends a campaign; it was
started again from the next seed, so every seed in the range was tried.

| campaign | first seed | cases | main | branch | per thousand |
| --- | --- | --- | --- | --- | --- |
| turned, on the exact kernel | 533500000 | 124 732 | | 68 | 0.55 |
| turned, through the application's body | 533600000 | 187 902 | | 1 071 | 5.70 |
| square solids, on the exact kernel | 533700000 | 150 000 | 6 | 6 | 0.04 |
| profiles, on the exact kernel | 533800000 | 95 000 | 42 | 42 | 0.44 |
| parts, every gesture undone | 533900000 | 1 014 | 92 | 75 | 90.7 / 74.0 |

The turned campaigns also declined 13 710 and 21 011 cases asking for an
ellipse, as the scope rule has them, and the exact kernel 915 turned walls
a hair thin as no profile.

**Square and profiles: no regression.** The same 6 and 42 seeds fail on
both trees, by the same rules, and shrink into the same cases.

**Parts: sixteen seeds failed on the branch alone**, every one a circle
drawn across a sketch axis, an area `main` dropped and the branch cuts
along the axis and turns on both sides. Both sides of a circle go to the
flats, and their join took 43 s in release for a circle turned whole alone,
93 s after a raise: fourteen seeds answered too late, two came out open.

- **Fixed: a whole turn needs only the sides no other side holds.** Turned
  whole, a side whose mirror lies within the other sweeps nothing the
  other does not, and `Turn::sides_needed` leaves it out (`b55c4a8`). The
  circle alone now turns in 4 ms, after a raise in 12 s, where a circle on
  one side of the axis takes 29 s on `main`. Over the first 570 seeds,
  run again: `main` 54 failures, the branch 44 before the fix, 37 after;
  seven of the ten the branch failed alone hold. Held in the gate by
  `a_circle_across_the_axis_turned_whole_is_turned_from_its_larger_side_alone`
  and `a_circle_across_its_axis_turned_whole_undoes_every_gesture_within_the_patience`.
- **Open: a circle across its axis turned part way.** Both sides are still
  turned on the flats and joined: at 180 degrees, back to back, it takes
  longer than the patience (533900361, 533900651, 533900890). The flats'
  join, #418; each named in `an_undo_gives_back_the_part.rs`, ignored with
  its reason. At -45 degrees the seam came out open (533900197, 533900883):
  not the join but the ends, which faced the way a forwards turn would;
  fixed with the flats' ends below, and held in the gate.
- **Open: a part on the flats after a round turn.** A circle across its
  axis turned whole now makes matter, and the part goes to the flats from
  that step on, where every later join is as slow as `main` makes it for a
  circle turned on one side (533900569, 533900776, 533900903): #492.

**Turned, by family**, each seed shrunk by
`the_seeds_a_turning_campaign_named_are_shrunk_one_by_one` and the shrunk
case run again on both paths:

| family | on the exact kernel | through the body |
| --- | --- | --- |
| the flats turning a wall a hair thin | | 961 |
| a surface or a plane a hair from another | 51 | 74 |
| surfaces touching, no hair | 11 | 25 |
| an axis leaning a hair | 6 | 7 |
| the flats answering too late | | 4 |

- The wall a hair thin is the family the two-hour runs named by 533400107:
  the exact kernel declines the section, the flats turn it as `main` did
  and leave an end facing the wrong way.
- The hairs, the touches and the leaning axes are the kernel's families on
  prisms, reached by turned cylinders and discs. Nine of them were run
  again with the turned cylinder raised as a prism instead, and fail alike:
  the rules `Listed`, `Volume` and `Closed` this campaign met on the
  kernel's own triangles (533620562, 533515182, 533565540, 533544801), the
  touches declined as undrawn (533520746, 533545056, 533549652), and two
  ties (533512790, 533571909). Through the body, 533666445, a cylinder
  turned about V and cut by a ring and a post, is drawn crossing by the
  body's finer triangles, alike with the cylinder raised: named in
  `what_the_exact_campaigns_found_in_the_triangles.rs`.
- **Turning's own: a sliver about a cylinder's axis.** A sector turned a
  hundredth of a degree about the axis of a cylinder lies with its two
  ends, planes holding that axis, on the cylinder's rulings, and two faces
  are drawn crossing (533603007); at a degree it holds. The same family as
  533328889's tie, the ends of a partial turn meeting a coaxial cylinder;
  named in `what_the_exact_campaigns_found_in_the_kernel.rs` and left, as
  that one is, for the cones' issue.
- **Fixed: two edges touching between their vertices**, 15 of the 17
  bodies of the kernel-drawing family the application declined as
  undrawn. A rim touching the line a floor through a wall's axis cuts on
  it, at the rim's vertex or at a step of its grid, or two curves grazing
  at a step of both: the kernel puts no vertex there, the line was sampled
  at its ends alone, or each curve took a sample a rounding apart, and the
  face holding both met its own loop in the middle of a segment, which the
  sweep refuses. A line now takes every sample standing on it within
  rounding, and samples of two edges within rounding of each other are one
  (`tessellation/sampling/touches.rs`). Held by 533647113, 533643649,
  533786336 and 533600448; the same rule holds 533330824 and 533413234 of
  the touches above, 8542629 and the band's 6529277. The two left,
  533729464 and 533693925, are two circles a hair apart on one cap, their
  chords crossing.

### The flats' ends

`flats-ends`, the 753 seeds of the night's turns through the application's
body whose turned leaf came out open alone: a wall a hair thin, which the
exact kernel declines as no profile, turned part way on the flats. Three
faults in `sweep::revolution`, each a fact the sweep decided apart from
the rest:

- **A thin end refused.** The ends were the drawing's own triangles passed
  through `Polygon::new`, which refuses a splinter a cut leaves, while the
  strip's two walls were kept: they are now `Polygon::drawn`, kept however
  thin while they are thicker than the rules' `NEAR`. The caps of a prism
  are the same pieces and are kept the same way.
- **An end facing the way read at its first corner.** Every piece fanned
  from a corner on the axis read no way at all; and the way ignored the
  sign of the turn, so every partial turn backwards on the flats was open
  (#486). An end now faces the way a point off the axis travels, back
  where the turn opens and ahead where it closes.
- **A whole turn a hair short.** A turn the reading takes as whole, within
  a thousandth of a radian, was swept by its own angle, its last step that
  hair from its first and no end across the slit; it is swept whole.

Each seed run again alone, on `1492eed` and on the fix: 238 of the 753
hold. The 515 left break where the flats join two solids, the family
`flats-join` names: 438 at a later step whose tool holds alone, 32 a leaf
across its axis whose two sides are joined, 31 a later tool across its
axis, and 14 answer past the patience, every one a join on the flats. A
fifteen-minute campaign from 533600000 on both trees, side by side: 5 569
seeds reached by both, 24 failures before and 18 after, none failing after
alone; six of the fifteen `flats-ends` seeds in that range hold. Held by
`a_part_turn_is_closed_either_way_on_its_axis_and_off_it`,
`a_wall_a_hair_thin_keeps_its_two_ends` and
`a_turn_a_hair_short_of_whole_closes_on_itself`, #486's
`a_quarter_turn_backwards_keeps_every_rule`, and the parts 533900197 and
533900883, a circle across its axis turned -45 degrees.

The `flats-join` fix, merged beside this one, settles the rest upstream:
`Straight::of` now reads the hair-thin wall away and the exact kernel turns
these sections, so the named seeds of both families no longer reach the
flats. The flats' ends stay fixed for what still turns there: a section
the exact kernel declines, and #486's turns backwards.

### Round 1: the kernel's declines

The night's failures through the application's body were sorted by the
place at fault (`1492eed`); 32 seeds were the exact boolean declining a
step, as unverified or as a tie, a plane or a wall a hair from another.
Every shrunk case was traced to the first fact decided twice:

- **Fixed: a wall taken for another a hair off moves its operand.**
  Decision 1 took a slot's cap for a disc's wall at just under the
  tolerance and left the slot's side, which touched the cap, just over it
  from the wall: the side crossed the wall it should have touched. The
  operand now moves onto the wall it is taken for, as decision 8 moves it
  further off (533613392, 533704316, and 8515425 from the line of touch).
- **Fixed: a cylinder grown onto a node slides along its partner's axis.**
  The curve a post and a shaft meet along at a node but for a hair of
  radius was laid on the post given the shaft's radius, while the post,
  resting on the shaft's end, was left as it stood; its corners stood
  three tolerances off the curve. Grown, it is slid along the shaft's axis
  by what it grew, and rests on the end still (533609727's shrunk case).
- **Fixed: two planes as twins are read at the place.** The two ends of a
  turn a tenth of a degree short of whole stand closer than the tolerance
  near its axis, and a block's face crossing them there made twins of the
  two, a tie. They are read one above the other where they stand further
  apart than rounding (533708173's shrunk case).
- **Open, the band again:** the curve a corner's wall meets a turned
  cylinder along passing 4e-16 from the cylinder's end circle, inside the
  band of the end plane and the wall, with no corner (533648596,
  533620725); a slot's corner taken onto a cylinder its cap touches inside
  1e-7 under it, off the curve the two meet along (533631663, holding
  since #536's round 2); a meet laid
  on a moved cylinder neither operand may move (533602391's shrunk case,
  533775454); walls a hair apart beyond decision 8's reach under a
  perpendicular one (533603710); a third surface crossing a line of touch a
  hair inside (533614588). Each is the tangency band's or decision 2's,
  reached by turned surfaces as by raised ones.

Measured on the 32 seeds as drawn, through the body: 3 now hold
(533602391, 533613392, 533704316). 533609727 as drawn holds a third band
that still declines; 533708173 as drawn now answers, and its turn's own
crack near the axis is drawn crossing under the block, as it was on
`1492eed` with the block above: the triangles' family. A 15-minute
campaign from 533600000 on both trees, 3 448 seeds in common: the same
failures, less 533602391; no seed that held fails.

### Round 1: a rounded outline a hair wider than its corners

`rounded-a-hair`, four seeds of the night's turns through the
application's body: a rounded square or rectangle a hair wider than its
corner radii, raised. Its straight run between two arcs had its ends
within the tolerance, and the raise declined such a run as no profile
(`brep/piece.rs`). It is now left out and names no wall, as a turn leaves
out a run laid to no length, and the arcs on either side, their centres a
hair apart, are one: a rounded square a hair wider than its corners is
raised as a disc, a rectangle a hair wider than a slot as the slot. The
four seeds hold, run one at a time; 533656030, 533729250 and 533684645 are
un-ignored.

## #536: cones

Two campaigns of `random_turned_solids.rs`'s slanted draw
(`Case::drawn_slanted_off_the_lattice`) on 7 October, in release, each on
its own copy of the branch: first at `611b616`, on the exact kernel from
seed 536100000 and through the application's body from 536200000; then,
after the fixes they led to, at `ba3224d` from fresh seeds. Every seed the
first two named was shrunk by
`the_seeds_a_turning_campaign_named_are_shrunk_one_by_one`
(`CAO_TRIAGE_DRAW=slanted`), and each shrunk case run again on the fixed
tree.

| campaign | tree | first seed | cases | broke a rule | per thousand | asking for a conic, declined |
| --- | --- | --- | --- | --- | --- | --- |
| slanted, on the exact kernel | `611b616` | 536100000 | 42 477 | 206 | 4.85 | 11 992 |
| slanted, through the application's body | `611b616` | 536200000 | 16 481 | 80 | 4.85 | 4 813 |
| slanted, on the exact kernel | `ba3224d` | 536800000 | 30 000 | 93 | 3.10 | 8 598 |
| slanted, through the application's body | `ba3224d` | 536900000 | 15 000 | 59 | 3.93 | 4 189 |

**The bar was not met on these two campaigns.** One case in a thousand is
the issue's; what was left was, case for case, the hairs below. On the
fixed tree no ray lost its way out of a cone (`Spans`) any more. Four
rounds of lanes followed, and every draw is now under the bar: see
[The final count](#the-final-count) below.

**Fixed:**

- **A line crossing a cone a hair from a disc**, 70 of the 206 seeds on
  the exact kernel, every one a leaf alone: a shaft whose top slants in
  over a height of 1e-7 to 1e-5. The quadratic's two roots stand a hair
  apart there, one on each nappe; the discriminant left both a hair off
  and the nappe test read them on the wrong one, or rounding took it for
  nought and both were dropped, and the ray found no way out of the
  matter (`Spans`). A root is now taken onto the nappe by Newton's steps
  across its meridian line (`Cone::roots`). The 70 hold; 536101211 and
  536104721 are in the gate.
- **A point whose tip a partial turn cuts away but for a sector**
  (536211029): the loop through the apex closed along the floor the short
  way, and the face was left open to the triangles and read as pointed
  without a tip by the soundness checks. It closes the long way round.
  In the gate.
- Before the campaigns, from the review: a face standing inside the
  hollow a cone's face leaves about its axis, or beyond a plane touching
  the cone along a ruling, is clear of it — a hex socket in a chamfered
  head, a pocket in a chamfered end or beside a countersink, a block
  beside a point, which were declined as unsupported where `main`
  computed them.

**Open, hairs.** Of the 136 seeds left on the exact kernel, all but six
draw a coordinate a hair, 1e-8 to 1e-4, from another: 62 with no cone at
all, the families #533 named on prisms and turned cylinders; the
others a cone a hair from a coaxial cylinder, plane or cone, or a slant a
hair from square to its axis. Through the body, 74 of the 79 cases shrunk
carry a hair. This is the family the issue's third default names: it is
measured here, and no fall-back or decline is added for it without Tom's
word. Named by 536202582, a partial turn about an axis 1e-8 off a disc's,
declined as unverified, and 536203669, a ring a hair wide on a step,
undrawn.

**Open, no hair:**

- **A partial turn about a cone's axis**, its ends meeting the cone, or a
  cylinder of its axis, along their rulings: refused as unverified
  (536210335, 536135909), or open along a rim (536112193, 536128400). The
  family of 533328889, which #533 left for the cones; 533328889 itself
  holds since round 2.
- **A ring resting on a cone along its rim** (536212654): the kernel keeps
  the arc of touch on the cone's face as a slit, an edge run both ways,
  which the cone's chart cannot lay out. Undrawn.
- **A half turn joined to a cone of its axis** (536215029), undrawn, and a
  partial point cut by a block (536101248), refused: not traced.

Each named seed is in `what_the_exact_campaigns_found_in_the_kernel.rs`
or `…_triangles.rs`, ignored with its reason where it still fails.

**No seed that held before fails.** The earlier draws, run on `main`
(`f1d99a0`) and on the branch (`611b616`) over the same seeds, side by
side: the same seeds fail on both, by the same rules.

| campaign | first seed | cases | `main` | branch |
| --- | --- | --- | --- | --- |
| turned, on the exact kernel | 536400000 | 10 000 | 3 | 3 |
| turned, through the application's body | 536300000 | 10 000 | 3 | 3 |
| square solids, on the exact kernel | 536500000 | 20 000 | 2 | 2 |
| profiles, on the exact kernel | 536600000 | 10 000 | 2 | 2 |
| profiles, through the application's body | 536700000 | 5 000 | 1 | 1 |

**The gate's time.** `cargo test --workspace`, built, on an idle machine,
the second run of each: 31.0 s on `main` (`f1d99a0`), 30.9 s on the
branch's tip.

### Round 2: the kernel's declines

The seven cases round 2's triage sorted under `kernel-declines`, each run
step by step on the exact kernel to the decision at fault, and fixed there.
No fall-back and no decline is added.

- **A move took a surface off one it was one with** (5365230254, a turn of
  a hundredth of a degree lying on a block's side). Its other end grazed
  the block's corner round and was moved onto it with the whole turn, a
  hair off the side its first end was read on, and the line the two ends
  meet along went ninety microns along the side. A move carries nothing
  off a surface of the first operand it is one with, and a plane its
  operand drew corners on is not moved further than the tolerance off a
  line it crosses another plane along.
- **A line rounded onto a wall it crosses** (5365239969, and 533631663 of
  #533). A plane a hair off a round's axis cuts it along a line a hair
  inside the wall the round touches inside, by less than a coordinate
  holds; rounded, the line was taken to touch the wall, though it crosses
  it many tolerances either side. Such a line is measured from the radii.
- **A wall moved off a cone's axis** (5365205368, 5365208585 of the
  grazing coaxial family). A cylinder about a cone's axis was moved onto a
  touch with a third surface up to a tolerance off that axis; the circle
  it meets the cone along, built on the cone's axis, stood off it. A wall
  about a cone's axis is not moved off it.
- **Two cones of one apex a hair apart in slope** (5365211513, a
  regression of eba1fa2): within the tolerance of each other all along
  their faces, they carried no curve together, and their rims stood side
  by side. Only a line, a ruling of a mirror, is kept off such a pair.
- **A region thinner than the tolerance with an operand's edge down its
  middle** (5365202020, and 533328889 of #533): every read along its chord
  stood on that edge, a tie. It is read either side of the chord last.
- **The harness's judgement** (5365216226): a block's side parallel to a
  cone's axis 5e-8 past its widest rim, a tolerance and a tenth, is
  declined as unsupported since the kernel tells a face clear only four
  tolerances off it. `asks_for_a_conic` counts such a plane as asking.

5365202020 as drawn, shrunk again, is a round post in place of the block:
the post's rim and the cylinder's line cross on the joined block's side
8.5e-4 apart and run as one in its parameters between, two arcs leaving a
corner along one another. The band's family; named and ignored.

The slanted draw through the application's body from 5365200000, on the
branch before the lane (`ae20fa9`) and after its fixes (`833daa9`), over the
same seeds:

| tree | cases | broke a rule | per thousand |
| --- | --- | --- | --- |
| `ae20fa9` | 14 577 | 15 | 1.03 |
| `833daa9` | 14 577 | 12 | 0.82 |

Every seed failing on `833daa9` fails on `ae20fa9`, by the same rule;
5365205368, 5365208585 and 5365211513 hold.

### The final count

Each round was measured on fresh seeds, in release, the seven draws side
by side; the journal (`exact-kernel-journal.md`, 8 and 9 October) has each
round's table, the seeds that held on the round's base and failed on its
tip, and the gate's time. Failures in a thousand:

| Draw | round 1 | round 2 | round 3 | round 4 |
| --- | --- | --- | --- | --- |
| slanted, exact kernel | 0.88 | 0.55 | 0.35 | 0.50 |
| slanted, application's body | 1.05 | 0.73 | 0.40 | 0.20 |
| turned, exact kernel | 0.70 | 0.33 | 0.30 | 0.47 |
| turned, application's body | 0.50 | 0.70 | 0.43 | 0.50 |
| square | 0.07 | 0.07 | 0 | 0.13 |
| profiles, exact kernel | 0.45 | 0.50 | 0.20 | 0.10 |
| profiles, application's body | 0.35 | 0.45 | 0.15 | 0.15 |

Round 4, at `594dec4`, 40 000 slanted cases a draw, 30 000 turned and
square, 20 000 profiles: every draw is under one failure in a thousand,
and every failure fails on the round's base (`dcce010`) too, by the same
rule. Over round 1's first 15 000 slanted seeds, base and tip fail on the
same seeds, five on the exact kernel and two through the body.

**What is left, by family**, the 66 failures of round 4, shrunk:

| family | failures |
| --- | --- |
| coaxial turns with a cone, five of them partial | 9 |
| no cone at all, eight of them straight turns | 11 |
| a cone with prisms | 4 |
| a single cone alone, two of them refused | 3 |
| a cone beside a turn about another axis | 1 |
| turned, square and profile draws: straight turns (#533's families) | 26 |
| turned, square and profile draws: prisms alone (#533's families) | 12 |

By the rule broken: refused (`Answers`) 37, drawn crossing (`Uncrossed`)
23, left open (`Closed`) 2, a volume off 2, a span off along a line 1, a
face listed backwards 1. The seeds named in the rounds before and still
failing, among them 5365104153 (a stepped post ending in a point joined to
a sixth of a turn of a coaxial post), stay ignored with their reason in
`what_the_exact_campaigns_found_in_the_kernel.rs` and `…_triangles.rs`.

## #552: conics

The review of #552 ran the tilted draw on fresh seeds at `62cd56b`, the
branch as the lanes left it: 1 120 of 39 061 cases broke a rule, 28.7 in a
thousand — 842 declines, 215 meshes left open, 59 crossing, 3 listings
off, 1 volume. On the slanted draw, every failure the branch added to the
base's was a seed the base declined as asking for a conic. The rounds below
are measured with `triage_552`, a scratch twin of the campaigns that names
the kernel's reason beside each failure, in release, the seeds of a draw
dealt over every core; seeds the round named were shrunk by
`the_seeds_a_turning_campaign_named_are_shrunk_one_by_one`.

### What the review's round fixed

- **A corner of one arm on the other** (9312094, 51 of the first 153
  failures over seeds 9300000–9304999): a plane a hair off parallel to a
  shaft's axis cuts an ellipse whose far vertices stand a billion reaches
  away, built as two arms. Read through one arm's parameter, a point of the
  other came back a whole sweep away, within the tolerance by rounding, and
  the wall held the same stretch twice. A point across an arm's plane of
  symmetry is on neither (`Conic::holds`, `Conic::distance`).
- **A ray at an end's own angle** (9312005): an arm all but along the
  rulings turns by a hair across the box, and a sample read back off the
  angle of its end landed beyond it; such a ray stands at the end.
- **A conic and its neighbours on other angles** (9314220, 9512340,
  9311810): a conic halved between its wall's rays, beside a circle, a meet
  or another conic of the same axis sampled on other angles, had chords
  crossing theirs where they touch or meet at a sharp rim, and a face was
  left uncut. Every sample of a conic is a ray of every wall about its
  axis. These took the meshes left open from 31 to 2 in the first 5 000
  tilted seeds.
- **A line past a cone's apex** (9304992, 9302688, 9303477, 9301298): a
  plane at a cone's slope through its apex touches it along a ruling a
  turned section's flat face holds, and the line the plane cuts that face
  along runs on past the apex over the other nappe. It was read as lying on
  the cone there: a corner on a surface apart from the cone was refused on
  it, an arc past the apex was laid on the cone's face, and a corner past
  it took the cone among its surfaces. A cone a place stands off the own
  nappe of is left out of what that place lies on. A first cut of this left
  a corner's support growing for ever on seven seeds in 20 000; the
  support now takes, and asks for, the surfaces the corner stands on
  (9501559).

### The count

At `a74a27b`, 20 000 seeds a draw from 9500000, on the exact kernel; the
base `9e54a71` over the same seeds for the draws it has:

| draw | broke a rule | per thousand | base broke a rule | base declined as asking for a conic |
| --- | --- | --- | --- | --- |
| tilted, off the lattice | 204 | 10.2 | — | — |
| tilted, on the lattice | 71 | 3.55 | — | — |
| slanted, off the lattice | 13 | 0.65 | 10 | 5 692 |
| slanted, on the lattice | 15 | 0.75 | 10 | 5 009 |
| turned, off the lattice | 14 | 0.70 | 6 | 2 263 |

Through the application's body, 10 000 seeds from 9600000: tilted off the
lattice 115 (11.5 in a thousand), on it 33 (3.3); no mesh is left open
there — a face the triangles cannot cover is declined as undrawn — and 20
and 5 cases draw crossing triangles.

**No seed that held on the base fails.** On the slanted and turned draws,
every seed failing on the branch and not on the base was declined on the
base as asking for a conic: 3 slanted off the lattice, 5 on it, 8 turned —
the family the issue's first default names, *asked for a conic before,
built now, wrong*, counted in the table. Of those 16, 13 are declines, 2
list a vertex off its face, 1 draws crossing.

**Bit for bit.** The fingerprints of every body and its triangles over
seeds 0..1000 of the square, profile, turned and slanted draws, on and off
the lattice, are those of `9e54a71` up to the first step the base
declined; #498's eighteen cases are identical.

**The gate.** `cargo test --workspace`, warm, three runs each: 33.0 to
33.4 s on `9e54a71`, 34.7 to 34.8 s at `a74a27b` (the issue's fourth
default allows three seconds).

**The judge** (the issue's third default): read only where the leaves'
boxes meet, the cases asking beyond the conics over seeds 0..1000 go from
115 to 82 turned off the lattice, 389 to 360 slanted off it, 227 to 210
tilted and 282 to 244 tilted off it (`c2c47f3`).

### What is left, by family

The slanted and turned draws are under one failure in a thousand. The
tilted ones are not, and the bar is not met there. Every failure of the
tilted draws but four carries a hair, by design of the draw:

| family | off the lattice | on the lattice |
| --- | --- | --- |
| a plane within 1e-7° of square or parallel to a wall's axis | 148 | 3 |
| a plane or a wall a hair from tangent, by an offset (`nudge`) | 22 | 45 |
| a plane 2e-7° to 1e-3° off, drawn or computed from a nudged place | 33 | 19 |
| no hair | 1 | 4 |

- **A plane a hair off square or parallel to a wall's axis**, 7.4 in a
  thousand alone off the lattice: within four times the angle the kernel
  reads as square or parallel (5e-10 rad, 2.9e-8°), the conic stands within
  the tolerance of the rim or of the rulings, and the plane within it of the
  wall or of the end, across the box. No band is laid along a conic (the
  issue's second default), and these are declined as unverified or as a
  tie, or drawn crossing. Named by 9300728 (a shaft cut at its end by a
  plane 1e-7° off square) and 9412210 (half a shaft leaning 1e-7° joined to
  a rounded block, the review's crossing reproducer), both ignored. This
  family alone keeps the tilted draw above the bar: by the second default
  it ships named, and waits on a band on conics, an issue of its own.
- **A hair from tangent by an offset** — a hole's wall a hair from a
  block's side, a plane a hair from a shaft's ruling, a conic a hair from a
  rim — the tangency band's family of #528 and #533, now met at a slant.
  The same second default.
- **Without a hair.** Off the lattice, one: a tube cut by a shaft square
  to it and by a block, with no conic at all, listed with a loop
  backwards — on the base too (9505549). On the lattice, three, each a
  coincidence of the lattice: a plane holding one of the two ellipses two
  perpendicular shafts of one radius meet along (9501873: a conic is never
  read as one with a meet, declined as unsupported); a slanted cut touching
  a shaft's outermost ruling exactly at the next cut's floor (9519300); two
  corners a rounding apart leaving an arc of no length (9507403); the
  fourth the table counts there, 9503428, has a hair of 1e-8 in its plane's
  place the reading missed. And from the first 5 000 seeds: an ellipse
  touching a shaft's far rim at its vertex, the sliver between the chord
  rounding leaves and the rim one unit in the last place wide (9304992).
  The four of #552 are ignored with their reason.

The review's reproducers: 9400405, 9300117 and 9300193 hold; 9412210 is
the crossing above.

### Two planes at a grazing angle

The tilted draw at `0d74713`, its failures shrunk (424 on the exact kernel,
391 through the application's body), counted 244 where two planes cross at
a few times the angle the kernel reads as parallel — a block's face
leaning 4e-8° to 1e-7° off a post's end or a block's side — and stand
within the tolerance of each other far from the line they cross along.
Two corners within the tolerance of each other and of both, one on each,
were kept apart since that line passes far from them, and the arcs
between them never made one.

- **The planes graze, decided once.** Two planes too near parallel to fix
  their line graze where every corner candidate on either, in a box faces
  of both may meet in, stands within the tolerance of both, and every
  curve crossing both crosses them within it of one place; corners within
  the tolerance of each other on two grazing planes are then one
  (5520121880, 5520206994, 5520122256). A first cut that let any corner
  within the tolerance of both through, corner by corner, held 256 of the
  815, and failed three seeds the base held: one pair of corners merged
  and the next a hair too far apart (5520267036), a cone's ruling
  crossing the two planes 1.2e-8 apart (5520167649), planes at 1e-3°
  (5520271162).
- **Their faces part past every corner.** Measured again once merged, a
  ten-minute draw against `34a1b64` found two seeds the base held that the
  rule above failed: a disc's bottom and a slab's floor leaning 1e-7° off
  it, every corner within the tolerance of both, but the disc's rim, which
  has no corner at its far side, three hundred units from the line the two
  cross along and 5.2e-7 off the floor at a tolerance of 5.1e-7
  (5527004090, and 5527006753 at the scale of a post). Two planes now
  graze only where they stand within the tolerance of each other across
  every box faces of both may meet in too. A box is wider than the faces
  in it, and the rule gave back 10 of the 241 of the 815: 231 hold, none
  that held on `34a1b64` fails. Ten minutes a draw against `34a1b64` over
  the same seeds: on the exact kernel from 5527000000, 129 of 16 627 (7.8
  in a thousand) against 169 (10.2); through the application's body from
  5528000000, 121 of 15 702 (7.7) against 168 (10.7). No seed that held on
  `34a1b64` fails.
- **A corner on an edge over its stretch.** A face leaning a hair off a
  post's end, left bounded by the post's circle, lies on that circle over
  its stretch alone; a cut crossing the circle there now finds the corner
  on the face too, and the line it leaves on the face ends there
  (5520140378, failed once the corners were one).

Three seeds named above hold now and are no longer ignored: 9300728 and
9412210, a shaft's end or flat side and a plane leaning 1e-7° off it,
since the two graze; 9519300, the ellipse touching a ruling at the next
cut's floor, since the corner the floor crosses the edge at lies on the
face that edge bounds over its stretch.

Of the 815, 241 hold (130 on the exact kernel, 111 through the body),
none that held fails. On fresh seeds, the tilted draw off the lattice
against `34a1b64` over the same seeds: on the exact kernel, 224 of 31 396
(7.1 in a thousand) against 313 (10.0) from 5520140000; through the
application's body, 346 of 44 524 (7.8) against 463 (10.4) from 5520240000.
No seed that held on `34a1b64` fails. 15 of the 815 the first cut held
fail still: a corner of one plane where their faces meet stands a hair
past the tolerance from the other (5 % past it in 5520108939 and
5520102585), and the two cross.

### A plane a hair from tangent by an offset

The family a plane a hair from tangent by an offset, triaged at `34a1b64`,
counts 313 of the 815 shrunk tilted failures (156 on the kernel, 157
through the application's body). Two of its causes were facts decided
twice, and are decided once now:

- **A plane read along a wall's axis though it leans** (5520135751,
  5520203207): its rulings are drawn through the foot of the wall's
  origin, and a line it shares with an end crossed the wall microns aside
  of them. A line or a circle on two surfaces crosses the wall where those
  rulings cross its other plane (decision 4), and a conic crosses such a
  plane the same way.
- **An earlier operation's corner moved by the next one** (5520107844,
  5520210646): a cut a hair from a block's edge leaves the edge's corners
  on the cut's plane within the tolerance; the next operation moved them to
  where that plane, the floor and a side meet, off the edge they end. An
  operand's own corner on its own planes stands where its operand built it
  (decision 5).

Of the 313, 103 hold (47 and 56), and none of the 815 that held fails.
On fresh seeds, fifteen minutes a draw against `34a1b64`: the tilted
draw on the kernel from 5525000000, 131 failures in 14 704 cases against
149 in 14 766 (10.2 to 8.9 in a thousand); through the application's body
from 5526000000, 145 in 14 022 against 166 in 14 044 (11.8 to 10.3); no
seed the base held fails. What is left of the family is the band on
conics the issue defers: a plane at a slant a hair from an edge or a rim,
where a conic's vertex stands a few tolerances from a corner every plane
holds within the tolerance (5520109275, ignored with its reason).

Merged on the grazing planes above, the two hold what each held alone:
335 of the 815 (170 on the kernel, 165 through the body; 231 with the
grazing planes alone), none that held on `34a1b64` failing. Ten minutes
a draw against `34a1b64` over the same seeds: on the exact kernel from
5527000000, 115 of 16 460 (7.0 in a thousand) against 167 (10.1); through
the application's body from 5528000000, 106 of 15 738 (6.7) against 168
(10.7). No seed that held on `34a1b64`, nor with the grazing planes alone,
fails.

### Round 4: the pieces with no hair

Decided with Tom on 10 October, after round 3: the pieces with no hair
come before any hair family. Round 3's shrunk failures held twelve with no
hair in their inputs; all twelve fail at `85a2d2e`. Run at `9e54a71`, the
base of #536, eight held there only because #536 declined their conic, and
four fail there too, inherited from #533 and #536 under their draws' bar.
One of the twelve is a hair after all: 5543220895 leans 36.86989754584402°,
atan ¾ less 1e-7°, a cone's slope less a hair, which sorting by whole
degrees could not see — the plane through the rim's lowest point a hair off
the ruling there, the rim bulging 7.3e-8 off its chord at a tolerance of
6.6e-8. It belongs with the hair from tangent.

Fixed, each with its shrunk case kept as a test:

- **A face touching a cone along a ruling beyond its apex** (5543214149,
  5543216778): a plane at exactly the slope through an apex outside the box
  was cut as a conic, which is nothing for a doubled ruling; the face was
  decided apart from the cone and the ruling's corners kept apart. Such a
  plane now holds the ruling, read at its foot, where no conic is cut
  (decision 2). Holding the two rulings of a steeper plane too, and the
  ruling wherever the angle reads as touching, broke three seeds past the
  12 000 measured, found once merged (5543114315, 5543213205, 5543216548,
  each a plane a hair off an apex beyond the box whose hyperbola held):
  narrowed to the case it was for, and named.
- **A line inside a cone's section by rounding** (5543231406): the line a
  floor square to a cone's axis shares with a cut touching the cone exactly
  there, found 6e-14 inside the section, crossed it twice four tolerances
  apart, a sliver between no face agreed on. Inside by a millionth of the
  tolerance or less, it only touches the cone (decision 4). Put in the
  shared `touches`, the same rule broke three fresh seeds whose walls rely
  on a line inside crossing twice (5549110872, 5549116996, 5549119078).
- **A corner past a cone's apex** (5543226572): a slant through a cone's
  apex on the end planes of two quarter turns, touching the cone along the
  ruling one of them holds. A corner found along that ruling past the apex
  was counted on the cone and kept apart from the same corner on a plane
  apart from it. The cone counts only where the corner stands on its own
  nappe (decision 5). It also holds 9507403, ignored since round 1.
- **Two arcs leaving a corner straight together** (5543224272): a wedge of
  a hundredth of a degree cut by a post touching its end plane; the curve
  the post meets the wedge's wall along leaves the post's rim to the fourth
  power of the way run, its chord a tenth along 8e-10 off the rim's, and
  the corner was declined as a tie. Where the chords cannot tell, the two
  are read halfway along the shorter (the overlay, step 5).

On all 538 shrunk cases of round 3, these hold nine, four of them hair
cases besides (5543107477, 5543132523, 5543206708, 5543529231), and none
that failed fails otherwise. Every named-case file with
`--include-ignored`: nothing that passed at `85a2d2e` fails. On round 3's
own seeds against `85a2d2e`, 12 000 tilted on each kernel from 5543100000
and 5543200000, 4 000 slanted from 5543300000 and 4 000 turned through the
body from 5543600000: 134 failures against 135, 5543206708 repaired, no
seed that held fails.

Left, each ignored with its reason:

- **The tangency band's edge with a conic at the touch** (5543307115, a
  slanted draw): a rounded block's side touches its corner's wall along a
  line, and a cone of that axis cuts the side along a hyperbola whose
  vertex sits on that line; a turn's end plane of a hundredth of a degree
  crosses the side where the side and the cone stand 1.52e-8 apart at a
  tolerance of 1.6e-8. Two corners that far apart on the same four surfaces
  are parted by the hyperbola, 3e-8 from both (`canonical/corners.rs`,
  `parted`). No band is laid along a conic (decision 7): the issue's second
  default.
- **A wall touching a wedge at a point** (5543227890): a slot's round end
  touches, at one point, the wall of a turn of a hundredth of a degree
  joined to it; the slot's wall takes no sample there, and its triangle
  crosses the turn's end face by 1e-7 (`tessellation/contact/point.rs`).
- **Inherited from #533 and #536**, under their draws' bar: two regions
  with no point inside them, declined as a tie at `overlay/partition.rs`
  (5543305067, 5543616767), and two meshes drawn crossing (5543304420,
  5543610415).

### Decision 11: a lean of a hair laid square or along

After round 3, Tom decided on 2026-10-10 how #552 ends: below the kernel's
tolerance a plane's lean off square to, or along, a wall's axis is rounded;
above it, up to a small angle, the step is computed as drawn and the user
is warned that it is unstable, never blocked; the pieces with no hair come
first; two more rounds at most. This is the rounding, decision 11 of
`exact-kernel.md`. The warning and the plain pieces are lanes of their own.

- **The threshold is `G`, twenty tolerances, not one.** Tom's comment says
  "less than the tolerance across the part"; the method was left open. The
  turn is measured as the furthest it moves a point of the tool — the lean
  times half the diagonal of the tool's box — and made up to `G`, the
  furthest the kernel already moves an operand (decisions 8 and 10), which
  a line at a cosine of a twentieth sees as at most eight hundred
  tolerances. A prototype at one tolerance held 159 of round 3's 236
  shrunk failures within 1e-7°, and none of the 129 between 2e-7° and
  1e-3°; at `G`, 216 and 60. For a tool as large as the part, `G` is a
  lean of about 1e-8 radians; for a tool a tenth of the reach, ten times
  that. The 1e-5° and 1e-3° hairs the tilted draw makes stay as drawn.
- **Plane against axis only.** Turning for any pair of directions, two
  planes or two walls, held fourteen more shrunk failures and four more
  fresh seeds on the prototype, but broke one fresh seed (5549103968); set
  aside. Widening the rule is one predicate.
- **Set aside too:** laying a plane a hair off a cone's slope on it
  (5543220895, its tilt the slope less 1e-7°), outside Tom's rule.

At `85a2d2e` and on the branch, release, fifteen minutes a draw over round
3's seeds, counted over the seeds both reached:

| draw | first seed | cases | `85a2d2e`, failed (in a thousand) | decision 11 | repaired | broken |
| --- | --- | --- | --- | --- | --- | --- |
| tilted, exact kernel | 5543100000 | 21 823 | 118 (5.41) | 43 (1.97) | 78 | 3 |
| tilted, application's body | 5543200000 | 48 658 | 311 (6.39) | 135 (2.77) | 180 | 4 |
| slanted, exact kernel | 5543300000 | 15 078 | 10 (0.66) | 11 (0.73) | 0 | 1 |
| turned, application's body | 5543600000 | 17 085 | 17 (1.00) | 15 (0.88) | 3 | 1 |

Of round 3's 538 shrunk failures, 282 hold: 131 tilted on the exact
kernel, 144 through the application's body, 7 of the other draws. Sixteen
seeds named in `what_the_exact_campaigns_found_in_the_kernel.rs` hold and
are no longer ignored; nothing that passed there, in the triangles' file,
the tangency band's or the hair files fails.

**The nine seeds the base held that fail.** Three only by the path their
draw took: shrunk, they fail on `85a2d2e` as well (5543113170, a turn with
a hole cut by a floor 6e-7 off; 5543223121 and 5543242165, through the
application's body). Six are a lean laid exactly beside an offset hair the
lean had kept apart, or beside a turn's walls, named and ignored with
their reason:

- 5543112269 (`hair-from-tangent`): a start plane laid exactly along a
  leaning cylinder that slopes to a point 1.8e-6 off its corner;
- 5543218891 (`hair-from-tangent`): two floors 2e-8 apart laid along a
  wall at 30°, cutting it along two pairs of lines a hair apart;
- 5543312609 (`hair-from-tangent`, slanted): a turn's end planes laid
  exactly along walls they leaned 1e-7° off;
- 5543615040 (`hair-from-tangent`, turned): two turns' walls laid square
  by a plane's turn, meeting along the curve of two perpendicular
  cylinders;
- 5543235780 (`near-parallel-planes`): a slot's face laid parallel to a
  block's face a few tolerances off, a ring's walls crossing both;
- 5543120314 (`near-parallel-planes`): a point's end laid on a leaning
  shaft's end it crossed a hair off.

Refusing more turns is no cure. A turn refused at one step leaves
its tool at its lean beside one drawn at that lean and turned the step
before: refusing a turn that lays two skew walls square held 5543615040,
and broke 5543215175 and 5543231819, whose second step was refused.

One turn the first measurement made is refused: a plane through a point's
apex leaning 1e-7° off its axis, turned about the middle of its box along
the axis, stood a hair beside the apex and was cut along two rulings it
does not hold (5543232861, named, holds). A turn is not made where it
would lay a plane a hair beside a cone's apex; it cost one of the shrunk
failures the first measurement held (5543132867). Turning about the apex
instead was tried: it held that seed and broke four others.

**Left of the family.** A lean past `G` and under a small angle, built as
drawn, waits on the warning; 5549204865, a tube cut by a face 5e-7 off
tangent and 1.9e-6° off square — past the hair, failing on `85a2d2e` too —
is named under `hair-from-tangent`. The offset hairs and the planes a hair
apart are the families above, which the rounding reaches only where they
lean as well.

### Round 4 merged: the pieces with no hair, then decision 11

The plain pieces went in first, then decision 11 on top. Measured merged,
on round 3's seeds past the 12 000 the plain pieces had been measured on,
their reading of a plane through a cone's apex beyond the box broke three
seeds that held on `85a2d2e` (5543114315, 5543213205, 5543216548): two
rulings held for a plane steeper than the slope a hair off the apex,
declined as unverified, and a ruling of touch held for a plane steeper by
a third of a billionth, its face left undrawn. The conic cut held all
three. The ruling of touch is now held only where no conic is cut, the
case it was written for (decision 2); the three are named and hold.

Per seed, the way each was drawn, against `85a2d2e`'s campaigns over the
same seeds:

| Draw | First seed | Seeds | `85a2d2e` | Plain pieces | Both | Repaired | Broken |
| --- | --- | --- | --- | --- | --- | --- | --- |
| tilted, exact kernel | 5543100000 | 21 600 | 118 | 118 | 43 | 78 | 3 |
| tilted, application's body | 5543200000 | 48 400 | 308 | 302 | 127 | 185 | 4 |
| slanted, exact kernel | 5543300000 | 8 571 | 7 | 7 | 7 | 0 | 0 |
| turned, application's body | 5543600000 | 16 800 | 16 | 16 | 14 | 3 | 1 |

The plain pieces alone break no seed. The eight broken merged are decision
11's own, named above: three by the path of their draw, five named and
ignored; its sixth, 5543312609, lies past the slanted seeds reached. Of
round 3's 538 shrunk failures, 289 hold, the union of what each lane held
alone (9 and 282). With `--include-ignored`, nothing that passed on
either lane alone fails in any named-case file, and #498's eighteen cases
hold.

### The warning: a lean the kernel reads unstably

The second half of Tom's decision of 2026-10-10. A step whose plane leans
off square to, or along, a wall's axis by more than decision 11 lays
exactly and by a sine under a thousandth — some 0.057° — is built as drawn
and noted with its least lean; the application says it is an unstable
case, with the angle, once the step is applied and in the tree, and never
refuses it. A decline in the band stays a broken step, with its reason. The
design and the bound are in `exact-kernel.md`, step 10 of the boolean.

- **What the band covers.** On 20 000 fresh tilted seeds from 5549100000
  with decision 11, 1 820 cases (9.1 %) kept a lean in the band, and 25 of
  them failed: 1.4 %, against 0.18 % for the cases with none. The tilted
  draw is weighted towards hairs, so a user's part sees the notice far
  less often. 29 of that measurement's 57 failures were neither turned
  nor in the band.
- **What the warning does not change.** The judge of the campaigns does
  not excuse a step in the band: a failure there counts as any other. The
  campaigns count the steps built in the band, "steps built at an unstable
  lean", beside their rules.
- **Left in the band.** The families of round 3 between 2e-7° and 1e-3°
  that decision 11 does not reach, 5549204865 (a tube cut by a face 5e-7
  off tangent and 1.9e-6° off square), and the wedges of a hundredth of a
  degree (5543224272, 5543227890) are built with the notice where the
  kernel builds them, and broken with their reason where it declines.


### Round 5: the pieces with no hair

Round 4's measurement at `beaf75d` left eleven shrunk cases with no hair,
each failing on `85a2d2e` too. Three causes are fixed, each decided once,
and with them the two regions with no point inside them inherited from
#533 and #536 and two cases ignored since earlier campaigns:

- **A corner where two walls touch at a point** (5544320693): a rod joined
  to a post it touches, cut by a pin of the rod's radius whose axis meets
  the rod's at the touch. The node of the pin and the rod is that touch,
  and the line the pin touches the post along runs through it; the union
  left no edge there, so the node, found on the pin and the rod alone,
  never cut the line, and a region of the pin had no point inside it. A
  corner standing where two walls were decided to touch at a point — the
  node or the contact of two perpendicular cylinders — now lies on both
  (`canonical/apart.rs`, decision 5). It holds 5543305067 and 5543616767
  too. Read at a cone's apex as well, it broke 5545235515 in the round's
  own campaign, a plane at a cone's slope a hair off its apex, and taken
  onto a wall decided apart from the corner's, 5545618519, a post across
  two rods of one axis a hair apart in radius; both seeds are named.
- **A line inside a wall's section by rounding** (5544100660, 5544229207):
  a cut at 60°, or at 45°, through a wall's lowest line at its end. The
  line the cut shares with the end, 3e-14 inside the section, crossed the
  wall twice 3.2e-6 apart where the cut's ellipse touches the end once.
  Round 4 decided it for a cone's section alone; the line a plane square
  to a wall's axis shares with a plane crossing it steeply now touches any
  wall it stands inside by rounding alone, and the line, the circle and
  the conic of that corner reach the one touch
  (`relation/crossing/section.rs`, decision 4). Not beside a plane leaning
  along the axis: there the rule broke 5549116996, a hairpin whose vertex
  stands where rounding over its lean puts it. Nor for a circle no plane of
  the triple holds: read for any circle, it broke 5545108465 in the
  round's own campaign, a rim where a shaft gives way to a cone cut
  through the cone's apex a hair off its slope, whose hairpin crosses the
  rim twice; the touch is now read only for the edges lying on the
  triple's surfaces, and that seed is named. It holds 9304992 too.
- **Two arcs leaving a corner together** (5544515442): a rod along a
  rounded block's side, cut by a post touching the block's front where its
  corner's wall does. On the rod, the curves the post and the corner's
  wall meet it along leave their corner flat, touching to the fourth
  order; their chords a tenth along were drawn to places at different
  distances, the shorter one's reading below though it runs above, and
  the rod's cycles were traced across each other. Two such arcs are now
  ordered first by where each stands at one run, half the shorter (the
  overlay, step 5). It holds 6128869 too, ignored since campaign 6.

Left, each ignored with what was found:

- **A turn of 359.9° whose slit meets a tangent line at a rim**
  (5544504441): the 105° turn's inner wall touches the shaft's step along
  a line through the step's inner rim exactly at the slit, where the
  curve the shaft's wall meets that inner wall along leaves the rim
  touching it to the fourth order, within the tolerance of it across the
  whole slit. The step's face is lost: its rims, the slit's lines and the
  curves at the touch are left with one face each, declined as
  unverified. Reading the meeting curve on the step's plane as the rim, as
  a circle is read on a wall as the meeting curve (`domain/traced.rs`),
  removed a decline there and changed nothing else.
- **A cone face meeting itself along a circle** (5544323650): the body is
  verified, but the cone's face, whose wall runs on above and below the
  circle where the half ring's top and inner wall end — an edge with four
  uses — is not cut into triangles: its loop runs along that circle and
  back, and the sweep in the cone's parameters gives up
  (`tessellation/sweep.rs`). Left open.
- **Two walls of one radius tangent at a node** (5544618545, 5544629222):
  the body holds, and so do the exact kernel's own triangles. Drawn as the
  application draws, at a fifth of a thousandth of the reach, the sliver
  faces either side of the two ellipses, between the node and a pin's or
  a post's curves, stand 0.004 apart where their chords sag 0.003, and
  their triangles cross. The strips the contact of two perpendicular
  walls is cut in follow the rays of their circles, not those curves.
- **Crossings at a rim** (5544205878, 5544518834, 5544528726): a cone face
  and a bore face where a hyperbola meets the bore's rim, drawn as the
  application draws; a post's face and a floor at its rim, and two walls
  of the turned families of #533, at the exact kernel's own tolerance.
  Not diagnosed further.

Measured at `54aed83` against `f57a5ca`, release, fifteen minutes a draw
on fresh seeds, the two trees side by side on five threads each, counted
over the seeds both reached:

| Draw | First seed | Seeds | `f57a5ca` | Round 5 | Repaired | Broken |
| --- | --- | --- | --- | --- | --- | --- |
| tilted, exact kernel | 5545100000 | 40 756 | 106 | 105 | 1 | 0 |
| tilted, application's body | 5545200000 | 58 733 | 159 | 159 | 0 | 0 |
| slanted, exact kernel | 5545300000 | 39 459 | 26 | 25 | 1 | 0 |
| turned, exact kernel | 5545500000 | 30 545 | 19 | 19 | 0 | 0 |
| turned, application's body | 5545600000 | 39 087 | 17 | 16 | 1 | 0 |

The plain pieces are rare in fresh draws, so the counts barely move;
what the round measures is that no seed that held breaks. The two
measurements before this one broke three seeds (5545108465, 5545235515,
5545618519), each a reading wider than its cause asked for; each is
named, and the rules are narrowed to hold them. Every named-case file
with `--include-ignored`: nothing that passed at `f57a5ca` fails, and
6128869 and 9304992, ignored before, hold.

### Round 5: decision 11 set down where it meets

Round 4's measurement left five seeds that held on `85a2d2e` and fail by
decision 11's own turn (`decision-11-regressions`). The turn was made about
the middle of the second operand's box. Its directions came out exact, but
the operand landed a hair off what it had held against the first, up to
the lean times the distance from the middle of the box. The turn is now
made about the middle of where the two operands' boxes overlap. The
operand is then shifted square to the turn's axis, so that each pair the
turn lays parallel lies exactly where the two stood within the tolerance
(`canonical/placed.rs`, decision 11 in `exact-kernel.md`).

- **Held, four of the five:**
  - a wedge's side drawn on a disc's end (5544600162);
  - a post's end holding a line of a turn (5544510380);
  - a stepped turn's axis lying in a disc's end (5544600864);
  - a face laid square to a cone 0.98 of the tolerance short of the
    circle where the cone meets a shaft (5544232904). Drawn exactly
    square, that face fails as well.

  Three seeds named under the earlier keys hold with them:
  - 5543120314, a point's end on a leaning shaft's end;
  - 5543235780, a slot's face parallel to a block's;
  - 5543615040, two turns' walls laid square, the one refusing the turn
    for skew walls held before it broke two others.
- **Left, one.** 5544236192 is named under `near-parallel-planes`. A ring
  is laid square to a block 4e-8° off a quarter turn. Its end then leans
  7e-10 off the line a later 45° cut shares with the block's floor, a line
  the ring held exactly as drawn. A shift cannot lay a lean, wherever the
  turn is made about. Drawn with the ring and the block exact and the cut
  leaning 4e-8° about Z, with no turn made, the kernel meets the same tie
  in the overlay's star.
- **Laying from as far as `G` broke two seeds**, on the first
  measurement:
  - 5545209574: a cut's face was moved three tolerances onto something it
    only passed near;
  - 5545217348: a block's end was not laid on the plane of a circle it
    stood on.

  Pairs are now laid only within the tolerance, and the plane of a circle
  is laid on as a plane is. Both are named and hold.

Fresh seeds, release, against `f57a5ca` built in its own target directory,
counted over the seeds both reached:

| Draw | First seed | Cases | `f57a5ca` | Branch | Repaired | Broken |
| --- | --- | --- | --- | --- | --- | --- |
| tilted, application's body | 5545200000 | 32 263 | 90 (2.79) | 87 (2.70) | 3 | 0 |
| turned, application's body | 5545600000 | 17 776 | 4 (0.23) | 3 (0.17) | 1 | 0 |
| turned, exact kernel | 5545500000 | 8 383 | 6 (0.72) | 6 (0.72) | 0 | 0 |

In the named-case files with `--include-ignored`, the seven cases above
that fail on `f57a5ca` hold on the branch, the two new ones hold, and
nothing that held there fails.

### Round 5: the two lanes merged

The pieces with no hair and decision 11's setting down share no
production file, and merged as they stood (`afd06dd`). Measured merged,
ten minutes a draw against `f57a5ca`, the two trees side by side on five
threads each, they broke three seeds, all by decision 11's setting down,
and a version of the fix broke a fourth:

- **A straight edge touching a wall** (5546322326, 5546607658): a slot's
  side drawn on a leaning disc's wall, a rounded block's seam on a leaning
  ring's bore. Turned about where the operands meet, each stood a
  fraction of the tolerance off the wall, and was refused. A straight edge
  of either operand square to a cylinder's axis of the other, touching it
  within the tolerance, is now laid on it by the shift.
- **A flat near several lines that part** (5546116488): a cut drawn
  through a point a hair from a turn's axis, its slit's line and an
  earlier cut's corner, each a fraction of the tolerance from the others.
  It was laid on the axis, the nearest, and refused; as turned, between
  them, it held. A flat passing within the tolerance of lines that part
  by more than a thousandth of it now lies on none of them.
- **A flat laid on a wall it touches** (5546111832), by the first version
  of that fix: a face drawn on a turn's wall, a hair inside a second wall
  a hair larger, was laid on that one and refused. A flat is not laid on
  a wall.

Measured again on fresh seeds, decision 11 left two cuts reaching far
past the body as drawn: turned about where the operands meet, their far
ends would have moved past `G` (5547227649, a cut along a 270° turn's slit
face 1.4e-6° off; 5547233090, a face 9.5e-7° off square to a tube a hair
inside its end, which failed in release alone). The turn now falls back on
the middle of the cut's box where the meeting would move it past `G`, as
`f57a5ca` turned it; both are named and hold.

Left, two seeds named under `near-parallel-planes`, each held on
`f57a5ca` by where its turn happened to leave a line two planes a hair
apart cross on:

- 5547225260: a face 2e-8° off square to a ring's axis leaning 1e-5°,
  drawn through a point of its end. The two planes lean 1.7e-7 apart and
  cross along a line through the end's inner rim near its lowest point;
  decision 11 lays the face's sides along the axis, a turn of 3.5e-10
  radian, and about where the two meet the line lands 0.004 from that
  point instead of 0.001, and the triangles cross.
- 5548213817: a face drawn on a 0.01° wedge's face, 1e-7° off it. Laid
  along the wedge's axis, it still leans 1.5e-9 radian off that face, and
  the step is refused.

No pair of the two lies within the tolerance for the shift to lay.

Fresh seeds, release, the merged branch with its fixes against
`f57a5ca` built in its own target directory, counted over the seeds both
reached:

| Draw | First seed | Cases | `f57a5ca` | Branch | Repaired | Broken |
| --- | --- | --- | --- | --- | --- | --- |
| tilted, exact kernel | 5548100000 | 29 115 | 71 (2.44) | 69 (2.37) | 2 | 0 |
| tilted, application's body | 5548200000 | 40 735 | 109 (2.68) | 105 (2.58) | 5 | 1 |
| slanted, exact kernel | 5548300000 | 26 639 | 15 (0.56) | 15 (0.56) | 0 | 0 |
| turned, exact kernel | 5548500000 | 19 888 | 13 (0.65) | 13 (0.65) | 0 | 0 |
| turned, application's body | 5548600000 | 28 138 | 12 (0.43) | 11 (0.39) | 1 | 0 |

The one broken is 5548213817, above. In every named-case file with
`--include-ignored`, the fifteen cases above that fail on `f57a5ca` hold
on the branch, the round's eleven new seeds hold but the two ignored, and
nothing that held there fails; #498's eighteen cases hold.

### Round 6: the pieces with no hair

Round 5's measurement at `784c69e` left fourteen shrunk cases with no hair
(`plain-pieces`, named at `22df52e`), each failing as drawn on `f57a5ca`
too. Thirteen hold, by seven causes each decided once, and with them sixteen
cases ignored since earlier rounds:

- **Two walls of one operand touching at a point** (5550316656,
  5550230611): a rod beside a post it touches, or a slot's round end
  beside a turn, then a face of the other operand passing through the
  touch — a disc's floor, a block's side. Each wall's curve on that face
  touches the other wall there, and with no edge of the operand there no
  crossing found the corner: a tie in the face's overlay, or lines run
  the wrong number of times. The touch, kept in `Apart` (decision 2), is a
  corner candidate on the two walls and every surface of the other
  operand through it (`combine.rs`, `touched`).
- **A third surface crossing a band** (5550223366): a cone's wall crosses
  at 45° the band of a block's side and a 0.01° wedge's wall a hair
  inside it, leaving the circle it meets the wall along and the conic it
  cuts the side along between the same corners, 1.08e-8 apart at a
  tolerance of 9.5e-9. Two arcs both lying on two surfaces decided to
  touch are one up to the hair (`combine/identified.rs`, decision 6).
- **A plane holding one of the two ellipses** two walls of one radius meet
  along, their axes meeting square (5550232928, and 9501873, ignored as
  `kernel-declines` since #552's first rounds): the conic and the meet's
  component are read as one curve (`canonical/curves.rs`), and a curve of
  the conic's wall and the surface it crosses lying on its plane is the
  conic (`relation/crossing/conic.rs`). Where the component came first,
  as an operand's edge, it lay on the plane as a meet, and a plane is now
  told to see it as its conic (`domain/traced.rs`, 5552120815, below).
- **A triangle passing through another face's beside a corner they
  share** (5550316166, 5550409179, 5550412859, 5550427782, 5550629622,
  5550709462): at a corner where a face turns into the matter, a wall's
  triangle cut across it, true to the region in the wall's parameters,
  sags through the face beyond. The folds pass, which undid a triangle
  lying on another face across an edge, now undoes these too
  (`tessellation/folds.rs`). With them hold round 5's 5544205878,
  5544518834, 5544528726 and the two walls of one radius tangent at a
  node, 5544618545 and 5544629222, and from earlier rounds 3202168,
  533779006, 5364102153, 5365231546, 5365239260, 8515799, 8550600,
  8585454 and 8587307.
- **A corner at a point contact standing off one wall** (5550629812, and
  round 4's 5543227890): a pin touching a 0.01° wedge's wall has a vertex
  of the wedge 7.6e-9 outside it, decided on the pin, and the pin's
  samples anchored at it leaned its ruling across the wedge's end face.
  The touch point anchors a wall's samples for every vertex on both
  walls standing off that one (`tessellation/contact/point.rs`).
- **A rim passed a hair inside by a line** (5550106720): a pocket's side
  leaves a stepped turn's step 6e-4 inside its inner rim, whose chords sag
  0.06; the rim takes a sample there (`tessellation/sampling/beneath.rs`).
- **Two conics leaving a node together** (5550117749): a 15° face through
  a node of a turn's wall and a cylinder of its radius cuts both along
  conics tangent there, sampled on different walls' angles until one's
  chord crossed the other's samples and the horn of face between them
  could not be cut. Two conics of one plane on walls whose axes cross
  take each other's samples as rays (`tessellation/sampling/conic.rs`).

The round's own campaigns, run against `22df52e` while the round went,
broke four seeds, each found, shrunk, named and held by narrowing what
broke it:

- 5551116747 (tilted, exact kernel) and 5551409953 (slanted, through the
  body): a post floored 1e-7 under where a lying cylinder touches it, a
  pin touching a post whose end stands 6e-8 past the post's axis. A first
  version had the touch stand for every vertex on both walls; one standing
  on a wall anchors that wall's samples still.
- 5551609285 (turned, through the body): a ring and a cylinder inside it
  touching along a line a turn's end plane crosses. A first version gave
  any two tangent conics each other's samples; two parallel walls are
  sampled on the rays they share already.
- 5552120815 (tilted, exact kernel), open on `22df52e` as drawn and
  refused at `576396a`: a turn cut by a cylinder of its radius whose axis
  meets its own, then by a plane at 45° holding one of their ellipses —
  the meet came to lie on the plane, which saw it only as a chord. It is
  seen as the plane's conic now.

The one seed round 5 broke (`decision-11-regressions`, 5550133356, tilted,
exact kernel) holds too, by a lane of its own merged after this one: a half
turn cut by a face decision 11 lays square and by one leaning 9.4e-6° past
the hair leaves a sliver whose inner wall is a strip between a circle and a
conic ending at one corner, 0.2° short of a grid angle. The circle kept
that angle, dropping one beside an end only within a thousandth of `EPS`
of another surface, while the conic dropped it within a fifth, and the
strip's one triangle across the step lay on the leaning face. A circle now
drops a grid angle within a fifth of `EPS` of a plane whose conic on its
own wall ends at the same vertex (`tessellation/sampling/ends.rs`,
`pinching`). A first version giving the fifth to every plane an end lies
on broke 5561366633 (tilted, exact kernel, volume): a circle a hair off a
block's face all along lost a grid angle by each end. It is named and
holds.

Left, ignored with what was found:

- **A turn of 359.9° whose slit opens at a line of touch** (5550232595,
  as 5544504441): a tube's end face lies on the plane x = -15 a post
  touches along y = 0, and the tube's inner wall passes through that line
  at z = 5, its rim tangent to it, where the slit opens. The band lays the
  rim's arc across the slit on the post and a stretch of the curve the
  inner wall meets the post along on the end plane; neither is seen there
  but as a chord, the end face is lost, and the step is declined as
  unverified.

Every named-case file with `--include-ignored`: the twenty-nine cases
above that fail on `22df52e` hold, the four new ones hold, and nothing
that held there fails; #498's eighteen cases hold.

Measured at the round's tip on fresh seeds, release, fifteen minutes a
draw, the branch and `22df52e` side by side on five threads each, each
tree in its own target directory, counted over the seeds both reached:

| Draw | First seed | Cases | `22df52e` | Branch | Repaired | Broken |
| --- | --- | --- | --- | --- | --- | --- |
| tilted, exact kernel | 5553100000 | 41 295 | 112 (2.71) | 112 (2.71) | 0 | 0 |
| tilted, application's body | 5553200000 | 55 533 | 128 (2.30) | 126 (2.27) | 2 | 0 |
| slanted, exact kernel | 5553300000 | 39 366 | 17 (0.43) | 14 (0.36) | 3 | 0 |
| slanted, application's body | 5553400000 | 44 607 | 27 (0.61) | 24 (0.54) | 3 | 0 |
| turned, application's body | 5553600000 | 33 673 | 17 (0.50) | 17 (0.50) | 0 | 0 |
| square | 5553700000 | 94 491 | 6 (0.06) | 6 (0.06) | 0 | 0 |

The plain pieces are rare in fresh draws, so the counts barely move: what
the measurement holds is that no seed that held on `22df52e` fails. The
slanted draws lose about one failure in seven, the tilted draw through the
body two in a hundred and twenty-eight; the others are unchanged.

### Round 6: the two lanes merged

The pieces with no hair were merged first, then `decision-11-regressions`,
each measured on its own against `22df52e`: the gate, every named-case file
with `--include-ignored`, and ten minutes a draw of the tilted and slanted
draws on fresh seeds, release, the two trees side by side on five threads
each, each in its own target directory, counted over the seeds both reached.

The first merge broke one seed, 5564306306 (slanted, exact kernel, drawn
crossing): a post joined to a cylinder lying along X, 1e-7 wider than where
it would touch the post, ending square to X on a plane through the post's
axis. The rim puts a corner on both walls 1e-7 inside the post, on the end
face's plane, which holds the post's ruling there. The touch of the two
walls stood for that corner, as for the pin's and the slot's above, and the
post's ruling anchored 5e-8 short of it in that plane ran through the end
face. A corner on a plane parallel to a wall's axis anchors that wall's
samples still, whatever it stands off it (`tessellation/contact/point.rs`);
the seed is named and holds, and the measurement was run again.

| Tree | Draw | First seed | Cases | `22df52e` | Tree's | Repaired | Broken |
| --- | --- | --- | --- | --- | --- | --- | --- |
| pieces with no hair | tilted, exact kernel | 5564100000 | 23 941 | 55 | 53 | 2 | 0 |
| | tilted, application's body | 5564200000 | 24 146 | 58 | 56 | 2 | 0 |
| | slanted, exact kernel | 5564300000 | 19 710 | 16 | 14 | 3 | 1 |
| | slanted, application's body | 5564400000 | 26 346 | 12 | 10 | 2 | 0 |
| with the corner on a plane | tilted, exact kernel | 5566100000 | 17 573 | 44 | 43 | 1 | 0 |
| | tilted, application's body | 5566200000 | 36 003 | 81 | 80 | 1 | 0 |
| | slanted, exact kernel | 5566300000 | 25 995 | 9 | 9 | 0 | 0 |
| | slanted, application's body | 5566400000 | 29 585 | 14 | 13 | 1 | 0 |
| with decision 11's regression | tilted, exact kernel | 5565100000 | 28 225 | 69 | 67 | 2 | 0 |
| | tilted, application's body | 5565200000 | 36 569 | 95 | 88 | 7 | 0 |
| | slanted, exact kernel | 5565300000 | 25 976 | 16 | 14 | 2 | 0 |
| | slanted, application's body | 5565400000 | 29 718 | 23 | 22 | 1 | 0 |

In every named-case file with `--include-ignored`, after both merges, the
twenty-nine cases of the pieces with no hair and 5550133356 hold, as do the
six new named cases, and nothing that held on `22df52e` fails; #498's
eighteen cases hold. The turn of 359.9° (5550232595) is the one case of
the round left ignored.

### Round 6 measured

Measured at `05f20eb` against `22df52e` on fresh seeds from 5570100000 to
5570900000, every draw, with round 5's case counts; the table is the
journal's, 12 October. Twenty-nine seeds are repaired. One that held on
`22df52e` fails on every run: 5570235757, tilted through the application's
body, drawn crossing, a post and a half turn whose walls meet at one point,
cut by a face through that point 1e-5° off the plane holding the turn's
axis. Shrinking leaves it whole, and it bisects to `576396a`, two conics
of one plane leaving a node of two crossing walls sampled on each other's
places: the face cuts the post along an ellipse all but a circle and the
turn along a curve all but two lines, and the two leave the point of touch
together. It is named under `conics-at-a-node` in
`what_the_exact_campaigns_found_in_the_triangles.rs`, ignored. One more
seed changed the rule it breaks, 5570132168, drawn crossing on `22df52e`
and refused on the tip; its shrunk core, a whole turn cut by a block
2.3e-6° off square to its axis, is refused on both.

All 238 failures were shrunk, none killed or late. Six have no hair, every
one refused and failing on `22df52e` too: 5570126783, 5570207344,
5570319299, 5570201909, 5570420146, 5570526055 (listed below). The other
232 carry a hair: by an offset 143, a lean in the warning's band 68, a
lean of 1e-7° or less that decision 11 does not lay 21.

### Where #552 ends

**Tom's choice of 2026-10-10**, after round 3, with the tilted draws near
six in a thousand and three rounds of case-by-case fixes no longer moving
them. Half of what was left was a plane within 1e-7° of square or
parallel to a cylinder's or a cone's axis, most of the rest a plane 2e-7°
to 1e-3° off or a hair of an offset. Building the conic divides by the
tilt, so rounding is magnified by about one over it; at such tilts it
reaches the kernel's own tolerance, and the kernel answered both ways in
different places. A user never draws such a tilt on purpose, but the
solver can leave one, since a rule such as « Perpendiculaire » holds to
about 1e-9. The choice, as Tom made it:

1. **Below the kernel's tolerance, round.** A plane whose tilt from square
   or parallel to an axis moves it by less than the tolerance across the
   part is laid exactly square or parallel, decided once; the kernel then
   builds the circle or the lines it already holds exactly. Accepting the
   tilt as drawn was set aside: it leaves the ill-conditioned conic and its
   inconsistent answers, where rounding moves the geometry by less than the
   kernel already treats as one place.
2. **Above it, up to a small angle, compute as drawn and warn, never
   block.** The step is built when the kernel can build it, and the user
   sees an "unstable case" notice naming the near-square or near-parallel
   angle. If the kernel declines, the step is broken as before, with that
   reason.
3. **Pieces with no hair come first**: cones cut at a real slant that are
   refused or drawn crossing, and turns of a hundredth of a degree, before
   any hair family.
4. **Two more rounds at most**, then the pull request. Whatever is left is
   named in the issue and here, with this choice, so it is not lost.

How it was carried out is in `exact-kernel.md`: the rounding is decision
11, at `G` rather than one tolerance (measured, and one constant to move);
the warning is step 10 of the boolean, up to a sine of a thousandth, some
0.057°. Three rounds followed the choice, not two: round 4 took the pieces
with no hair, decision 11 and the warning; round 5 the pieces with no hair
round 4 left and decision 11's own regressions; round 6 the same again.
Round 6 changed no rule the choice set.

**The rates, round by round.** Failures in a thousand cases, every draw on
fresh seeds of its own each round, in release; 40 000 cases a tilted draw,
30 000 a slanted, turned or square one, 20 000 a profile one. Round 0 is
the branch as the review left it once its round was fixed.

| Draw | 0, `0d74713` | 1, `daf2079` | 2, `cf9093b` | 3, `0d35b52` | 4, `beaf75d` | 5, `784c69e` | 6, `05f20eb` |
| --- | --- | --- | --- | --- | --- | --- | --- |
| tilted, exact kernel | 10.60 | 6.55 | 6.00 | 5.50 | 2.45 | 2.42 | 2.02 |
| tilted, application's body | 9.78 | 6.38 | 5.98 | 6.25 | 2.60 | 2.83 | 2.17 |
| slanted, exact kernel | 0.50 | 0.57 | 0.63 | 0.63 | 0.63 | 0.60 | 0.47 |
| slanted, application's body | 0.73 | 0.60 | 0.60 | 0.20 | 0.47 | 0.83 | 0.50 |
| turned, exact kernel | 0.50 | 0.57 | 0.47 | 0.50 | 0.77 | 0.57 | 0.63 |
| turned, application's body | 0.80 | 1.07 | 0.23 | 0.63 | 0.53 | 0.47 | 0.33 |
| square | 0.00 | 0.07 | 0.10 | 0.07 | 0.07 | 0.13 | 0.07 |
| profiles, exact kernel | 0.20 | 0.15 | 0.35 | 0.25 | 0.20 | 0.05 | 0.35 |
| profiles, application's body | 0.15 | 0.10 | 0.10 | 0.10 | 0.20 | 0.30 | 0.15 |

Every draw but the two tilted ones ends under one failure in a thousand.
The tilted draws end at about two, a draw weighted towards hairs: of
round 6's 165 shrunk tilted failures, three have none. The bar is not met
there, and the families below are why.

**Left with no hair**, first:

- **A turn of 359.9° whose slit opens at a line of touch** (5550232595,
  5544504441): the slit's lines and rims are left with one face each,
  declined as unverified. Named under `plain-pieces`, ignored.
- **A cone face meeting itself along a circle** (5544323650): the body is
  verified, but the face's loop runs along the circle and back, and the
  sweep in the cone's parameters gives up; left open. Named under
  `plain-pieces`, ignored.
- **Six refused at round 6's measurement**, each failing on `22df52e` as
  well, not named: a quarter turn given a post and cut by a whole cone, the
  three about parallel axes (5570126783); a wedge of 0.01° of two cone
  bands joined to a slot (5570207344) or to a rounded rectangle
  (5570319299), the wedge round 5 already met; a whole cone cut by a block
  leaning 80.5° and by one square to it (5570201909); a block joined to a
  three-quarter turn of three cone bands about an axis leaning 45°
  (5570420146); a half turn joined to a cylinder across it and to a whole
  turn about an axis leaning 30° (5570526055).

**Broken by the branch**, one: 5570235757, two conics leaving a node of
two crossing walls (`conics-at-a-node`, above).

**Left with a hair**, by round 6's shrunk counts:

- **A hair from tangent by an offset**, 143 — a plane at a slant a hair
  from an edge, a rim or a wall's ruling, a conic a hair from where a wall
  ends. No band is laid along a conic: the issue's second default, and the
  follow-up a band on conics would be. Named under `hair-from-tangent`
  (5520109275, 5541111069, 5542121248, 5543126616, 5549204865), and
  5543307115, a hyperbola's vertex on a line of touch, under
  `kernel-declines`; three more are decision 11 laying a lean exactly
  beside such a hair (5543112269, 5543218891, 5543312609).
- **A lean in the warning's band**, 68: built as drawn with the notice
  where the kernel builds it, broken with its reason where it declines, as
  the choice says. Two planes a hair off each other are most of it, named
  under `near-parallel-planes` (5541126238, 5542234414, 5543139042,
  5547225260, 5548213817).
- **A lean of 1e-7° or less that decision 11 does not lay**, 21: a pair
  other than a plane against a wall's axis — two planes, two walls — or a
  turn decision 11 refuses (5542111371, 5544236192, under
  `near-parallel-planes`). Widening the rule to every pair of directions
  held more and broke one seed; it is one predicate, set aside.

**The gate.** Built and warm, best of three runs each, side by side on
12 October: 37.4 s at the tip, 33.1 s at `9e54a71`, the tip of #536 the
branch stands on. That is 4.3 s over it, past the 3 s the issue's fourth
default allows, and that default says to cut gate seeds first; it was
left to the pull request, and Tom accepted it as it stands (below).

### Merged as it stands

**Tom's word of 2026-10-12**, after trying the branch in the application:
merge it, knowing it is not perfect, and keep a written trace of what is
not. What went to `main` short of what was aimed at:

- **The tilted draws are not under one failure in a thousand.** They end
  at 2.02 and 2.17, nearly every failure a hair; every other draw is under
  the bar. The families above are why, and each is named.
- **One seed the branch broke**, 5570235757 (`conics-at-a-node`), named
  and ignored rather than fixed.
- **A plane touching a frustum along a ruling, the apex outside the
  part,** is listed as two arcs of ellipse 4e-7 apart with a ribbon of cone
  between them, not as the ruling. The body is closed, uncrossed, at the
  right volume, and nothing shows on screen; a fillet or a dimension on
  that edge would meet it. The plane drawn tangent passes a hair inside
  the cone, and no band is laid along a conic. With the apex inside the
  part, the same gesture gives the ruling.
- **The notice writes its angle with a point**, « 0.01° », where French
  writes « 0,01° ». Left as it is.
- **The gate is 4.3 s slower than at #536's tip**, past the 3 s the issue
  allowed, with no gate seed cut. Tom accepted it, and a little more if a
  later step needs it.
- **The follow-ups are not opened**: a band laid along a conic, decision
  11 for every pair of directions, #514's grid, exact predicates where
  floating point cannot tell. They are opened on Tom's word.

## #571: elliptic walls

What is left on elliptic walls when the application's body raised an
ellipse on the exact kernel for the first time (the flip). Each family is
named, with the seeds that hold it where one is in the gate's files,
ignored until it is fixed. Lane c, running the campaigns on the exact
kernel beside this one, counts them; what follows is what stood at the
flip.

- **A plane in the warning band through a wall's tangent line**, its arcs
  misordered round a corner at `overlay/star.rs`: #552's own default, on
  circular walls as well (`line-of-touch` in
  `a_surface_a_hair_from_a_line_of_touch.rs`).
- **The same ellipse turned by 1e-7 to 1e-4 of a radian beside a block's
  planes**: too far for decision 8 to make it one wall with itself, near
  enough for its crossings with itself to stand within the tolerance of
  the block's faces.
- **Two touches at the ends of a half-axis**, a crescent 1e-6 to 1e-5 thick
  between them, and a third wall across both.
- **A third surface through a node of a meet on an elliptic wall**: a
  hole's end cap through the node of a figure of eight an elliptic hole
  makes with a round pocket, left unverified (5571542016); a round bore's
  cap through the near-nodes an elliptic hole a hair narrower than a post
  makes with it, a tie (5571541652); a crescent between two parallel walls
  a hair apart, each at a node of an elliptic hole, read backwards
  (5571546569).
- **A round bar touching an elliptic hole at a point**, drawn crossing it
  there (5571548076).

An ellipse within decision 8's hair of round is not among them: Tom
decided on 2026-10-13 that a whole one is raised as a circle
(`exact-kernel.md`, "The body"), and on 2026-11-03 that an arc of one is
raised as an arc of the circle of its mean, which #574 does where the
arc lies between straight traits (below).

### The flip measured through the application's body

Each draw of ellipses for 300 s through the application's body, on fresh
seeds and five threads at `14eff46`, and the exact kernel on the same
seeds. In its 1200 s the kernel ran fewer of the crossed and tilted
seeds, being checked on its own triangles and listing too: the two are
compared over the seeds both ran.

| Draw | Seeds from | Body | Kernel, same seeds | Body only | Kernel only |
| --- | --- | --- | --- | --- | --- |
| ellipses | 5572000000 | 155 of 9296, 16.7 ‰ | 156 of 9296, 16.8 ‰ | 5 | 6 |
| crossed | 5572100000 | 12 of 9053, 1.3 ‰ | 5 of 4150, 1.2 ‰ | 1 | 0 |
| tilted | 5572200000 | 67 of 29001, 2.3 ‰ | 57 of 26599, 2.1 ‰ | 9 | 1 |

Nearly everything the body fails, the kernel fails on the same seed: the
kernel's families, which the rounds of #571 work on. In the ellipses'
draw most of them are an ellipse a hair from round against a circle of its
size, which Tom's decision of 2026-10-13 settles for a whole one, and the
same ellipse a hair off or turned a hair beside a block.

What the body adds is the application drawing finer than the kernel's own
triangles, none of it the flip's: no face is numbered otherwise, no click
lands elsewhere, and no step goes to the flats.

- **Undrawn at the application's fineness.** A face the body cannot cut
  into triangles at two ten-thousandths of the reach, which the kernel's
  coarser ones cover: an ellipse 1e-6 from round against a circle of its
  size (5572007115), the same ellipse 1e-6 wider beside a circle
  (5572005755), an ellipse touched at both ends of its major axis by
  a circle of that radius, beside a pin (5572001841).
- **Drawn crossing at the application's fineness.** An elliptic hole
  1e-6 narrower than its boss (5572001987); a plane leaning 1e-5° to 0.01°
  off along an elliptic wall's axis, near the line it touches the wall
  along (5572208524, 5572213933, 5572216256, 5572217185, 5572217381); a
  round post bored through by two elliptic walls square to each other
  (5572103167).
- **Late.** A case past the 30 s the campaign waits, through the body
  alone (5572008304).

## #576: spheres and tori

The rounded draw — `Case::drawn_rounded`, the campaign
`a_campaign_of_rounded_solids_on_the_exact_kernel_keeps_every_rule` — on the
exact kernel, in release with `--features campaigns` over 1200 s: five rounds,
each on its own tree and its own seeds, lane T's on five threads and lane R's
on eight. The lone-leaf twins and their body ran beside them on seeds 0..600
of the same draw. Every seed a family names below is a printed leaf of its
round, run again on the fixed tree where the family was fixed; triage draws
`Case::drawn_rounded_off_the_lattice` under `CAO_TRIAGE_DRAW=rounded`.

| campaign | tree | first seed | cases | broke a rule | per thousand | declined by design |
| --- | --- | --- | --- | --- | --- | --- |
| rounded, on the exact kernel, before | `ed8267e` | 5576400000 | 108 051 | 63 145 | 584.40 | 4 370 beyond the conics, 14 hair-thin |
| rounded, after lane T's first round | `e106f36` | 5576400000 | 53 187 | 9 724 | 182.83 | 14 994 |
| rounded, after lane T's second round | `ef7aacd` | 5576500000 | 49 472 | 8 049 | 162.70 | 14 558 |
| rounded, with relations and the boolean, round 1 | `bd79075` | 5576600000 | 68 625 | 2 601 | 37.90 | not recorded, Answers and Closed extracted alone |
| rounded, with relations and the boolean, round 2 | `f660771` | 5576668625 | 67 860 | 2 314 | 34.10 | 19 173 beyond the conics, 13 hair-thin |
| rounded, lane C round 1 | `b4f629b` | 5576800000 | 44 327 | 1 143 | 25.79 | 12 700 beyond the conics |
| rounded, lane C round 2 | `3105e02` | 5576850000 | 45 154 | 814 | 18.03 | 12 898 beyond the conics, 188 bowed sections the reading declines, 4 hair-thin |
| rounded, through the application's body, lane C round 2 | `3105e02` | 5576910000 | 8 198 | 187 | 22.81 | 2 396 beyond the conics |

**The bar is not met.** One case in a thousand is the issue's; the last round
stands at 18.0 in a thousand on the exact kernel and 22.8 through the
application's body. Lane C's two rounds, on seeds from 5576800000 and up past
every seed used before, moved `Answers` and `Listed` and left `Closed`,
`Uncrossed`, `Spans` and `Volume` where they were: those families did not move
in two rounds and are set aside, as the issue asks, below. The twelve older
draws — 200 cases of each on the exact kernel and 60 through the application,
from 5576950000 — give the same verdicts on `main` and on `d69c69c`, bit for
bit.

**Fixed:**

- **A round face had no triangles** (lane T, `c607037` to `ef7aacd`). The
  lone-leaf twin failed on all 62 bowed leaves of its 80 seeds — 43 `Open`, 19
  `Volume` — and its body on 227 of 227 leaves of seeds 0..300, 254 of 254 on
  300..600; the campaign counted `Closed` 44 083 and `Volume` 18 555. A face of
  a sphere or a torus is now cut in strips between the rows of its grid, a
  ball and a ring drawn from their poles and the cut of their chart alone; the
  twin passes on its 80 seeds and its body fails 0 of 227 and 1 of 254.
- **A ball a hair below the axis, and a ball or a ring on a leaning axis**
  (lane T, `e106f36` and `ef7aacd`): the box a rounded leaf spans read the
  bulge off the drawn arc while the kernel snapped the centre onto the axis,
  or square to one world axis where the arc's furthest point lay on another.
  1 494 `Volume` failures of round 1, seeds 5576400008, 5576400022 and
  5576400131 among them; 0 in round 2.
- **A ring's `Spans`, whole backwards or turned part way** (lane S2, `2cf1cf5`
  and `dbff58b`): a loopless ring face was closed along nothing, its parity
  put every point outside and its rays found no way out; 37 of the 42 `Spans`
  of seeds 0..300 and 48 of the 51 of 300..600, 0 after. The ring's chart cut
  is now its floor, at the window `(seam − π, seam + π]`.
- **A flipped round face's volume and poles** (lane S2, family C): a round
  groove in a bore, its matter outside the tube, held both poles and added
  their floors to its flux. One leaf each range, 0 after.
- **`Declined::Travel` on a ball or a cap** (lane S2, family B2): the quarter
  points of a bulge were read through `sin π`, 1.2e-16, and a pole at the
  frame's origin made a bulge of nothing. Two leaves on 0..300, one on
  300..600, 0 after.
- **The reading of a hair-off ball end** (lane S2, family B1, `adfa959` and
  `afa66a1`): five leaves on 0..300, three on 300..600, all a sphere a hair
  larger or smaller than its shaft, or a centre a hair above the axis with its
  end a hair off it, which the reading could not lay. An arc leaving a pole
  within the tolerance of tangent to the level its next run keeps is now laid
  tangent, the pole sliding along the axis onto the circle.
- **An apple's or a lemon's `Spans`** (lane S2, family D): a root of the
  quartic on the mirror lobe was polished onto a crossing of the own lobe
  already found. Five leaves on 0..300, three on 300..600, 0 after.
- **Coaxial pairs with a round** (lane R, `30792d0` to `f660771`): a sphere
  or a torus against a plane square to or holding its axis, a coaxial
  cylinder, cone, sphere or torus now gives rims and meridians, a touch built
  as one circle, `clear` passing a face apart, the side of a pair read from
  its rims, and a whole ring's chart cut moved out of what a boolean keeps.
  `Answers` fell from 6 220 at `ef7aacd` to 1 571 at `f660771`, `Spans` from
  1 802 to 36. A ring turned part of the way is cut by a coaxial shaft, and a
  ring a cylinder of another axis runs through declines (`f660771`).
- **A block and a rounded ring standing apart could not be joined** (lane
  F): the ring's torus face, an outer half bounded by its own two tangent
  latitude circles, was read as a whole ring whose rims part nothing, so the
  overlay dropped both circles and the join was declined as unverified. A
  ring's tangent rims are dropped now only where every face on the torus is
  whole. Found by the flip: `the_exact_kernel_turns.rs`'s turn of a rounded
  rectangle beside a block, which the flats had computed until then.
- **Two of round 1's refusals were the harness's own** (lane C, `7553448`,
  `d69c69c`): a bowed section the reading declines to lay — an arc whose end
  stands a hair off its circle, 138 of 153, or one crossing itself — is no
  answer the exact kernel owed, the application turning it on the flats; the
  campaign counts it apart, a section with no arc never. And the kernel boxes
  a face on a sphere or a torus by its whole surface, so a wall apart from a
  quarter torus may still be related and declined; the scope rule reads the
  same box. 257 of round 1's 630 `Answers`.
- **A sphere's face holding one pole while its loop runs through the other**
  (lane C, `59a2673`): the listing closed the loop along the floor of the pole
  it passes through, as a cone's apex, and found the face pointed or
  backwards; it winds once round the pole it holds. A face holding both poles
  has no outside loop. The bodies were right: 24 of round 1's 173 `Listed`.
- **The harness's rays through a spindle** (lane C, `3105e02`): a root of the
  reflection across the axis was polished onto a crossing of the tube's own
  circle; one leaf-alone `Spans`.

**Open:**

- **18.0 in a thousand** at `3105e02`, set aside: `Answers` 337, `Closed`
  170, `Uncrossed` 152, `Listed` 129, `Spans` 20, `Volume` 6; through the
  application's body `Answers` 137, `Uncrossed` 33, `Closed` 12, `Volume` 5.
  The families, from round 1's 1 143 sorted one by one:
  - **A round combined with a coaxial turn of another extent or sense** — a
    quarter of a ball joined to a whole coaxial band, a whole ring cut by a
    half turn: the kernel builds the boolean, its own check finds a loop on
    the ring torus that does not close across the chart's cut, or an edge used
    once, and declines `Unverified`, 249 of round 1's `Answers` (shrunk:
    5576800479, 5576800792, 5576800961, 5576800514). Through the body the step
    is refused. The same shape, in round numbers, is behind the `Closed` and
    `Listed` `Backwards` left (5576801941, 5576805400, 5576802264,
    5576808278). Re-joining the ring's loops across the cut built 82 more of
    round 1's cases right and 90 wrong — `Spans` finding no matter, `Open`,
    `Backwards`, `Crossing` — where the kernel had declined safely: set aside
    unlanded here, and taken up by #623 with what reads a face across the
    cut — see "#623" below, which counts the round again.
  - **Hairs**: 108 of 151 `Uncrossed` are a coaxial rim 1e-8 to 1e-7 inside or
    outside the sphere's (5576800624, 5576800706, 5576800899); 13 of 15
    leaf-alone `Spans`, the 6 leaf-alone `Volume`, a `Pointed` (5576809934).
  - **Ties**, 76 of round 1's `Answers`: a region no column brackets in
    `overlay/partition.rs` (41), the order of a star (18), a partition's cycle
    (8), `selection/wrapped.rs` (6), the ray's winding (3) — two curves
    touching more closely than their curvatures order (5576801218,
    5576800781).
  - **A straight edge traced on a round**, 45: a round meeting a circular
    prism whose plane stands a hair off its own (`0.5000002` against `0.5`;
    5576800874, 5576801314), declined at `domain/traced.rs`.
  - **Horn tori turned part way**, two leaves alone: an extra crossing at the
    pinch (5576818601), crossings outside the turn's window (5576842963).
  - A turned band with its axis a hair off, cut by a rounded prism, holds no
    round at all and is not #576's (5576800031).
- **The hair-off ball end of the harness**: a ball end a hair larger or
  smaller than its shaft, which the kernel
  lays tangent with its pole slid, while the harness's promise keeps the drawn
  radius and centre. One leaf in 481 lone leaves — `Spans` — and 5 of 49 472
  campaign cases, 0.1 in a thousand, `within_reach`; seeds 5576507978 and
  5576516429, and 5576692721, 5576695548, 5576696191 at `f660771`, all lone
  leaves with a 1e-5 hair.
- **A ring torus turned onto a part that already holds matter** was
  numbered by the flats' union in pieces: the boolean of the flats gives a
  number of its own to every piece of a face its planes leave apart, so a
  ring drawn as two halves of its tube and joined to a part on the flats
  names seven faces turned whole and sixteen at 150° where the exact kernel
  names its two halves, and four with the turn's two ends. On a part with
  nothing else in it the two count alike, the exact kernel and `main`. A
  drawing laid on one of those pieces may lose its face on replay — the
  precedent #571 set for the steps after it.
- **The harness handed a ring's section as two runs** where the application
  hands a whole circle as one: the flats' sampling takes arcs of one circle
  one after the other as one curve, and the exact kernel named a face per
  half (seed 25 of the rounded draw, a ring turned 150°: four numbers
  against three). The section's contour is now handed over a run per
  circle, and `a_rounded_leaf_names_its_faces_as_the_flats_name_them` holds.
- **A ball joined with a quarter shaft over its pole** fails `Backwards {
  face: 0, lap: 0 }`; the test was written and left out of `f660771` because
  it failed.
- **A fillet or a shoulder whose two tangencies are both a hair off** still
  declines in the reading, since its centre would have to move; the draw makes
  fillets on the lattice, so no leaf shows it.
- **The listing** accepts a face winding nought that holds two poles even with
  loops, so it could not catch the wrong poles of family C; a face bounded by
  a loop winding nought may legitimately hold both poles, and the reading is
  left as it is.
- **A coaxial wall a hair inside a ball** is not drawn in common with it
  (`joined::beside` answers false for a round); no leaf of two campaign rounds
  showed a crossing there.

## #623: a round and a coaxial turn of another extent

#576 left its largest family declining `Unverified`: a sphere or a torus
turned through one extent combined with a body turned about the same axis
through another — a quarter ring joined to a whole coaxial band it touches, a
whole ring cut by a coaxial half turn. The kernel built the boolean, and its
own check found a loop on the ring torus that did not close. The cause was
the chart, not the boolean: a ring torus is read in a window one turn wide
along the meridian, cut at the antipode of its seam, and a piece of surface
the cut runs through was read as a region against the window's bottom and
another against its top, or as one region bounded by both — and neither floor
is an edge. The regions were never joined back, nor their loops walked
through the cut.

**Fixed:**

- **The regions the cut parts** (`selection/parted/glued.rs`): the regions
  using the two floors along one stretch are one region, and its loops are
  walked again with the floors left out, an arc reaching a tip on one floor
  followed by the arc leaving the twin tip on the other, the two arcs of an
  edge the cut parts one use of it. A band the cut runs through, along two
  floors no meridian reaches, is one region too. The four seeds of the issue
  hold: 5576800479, 5576800514, 5576800792, 5576800961.
- **A face reaching across the cut, read** (`domain/floor/round/ring.rs`): the
  closure along the cut was laid only for a strip between two whole
  meridians; it is laid now for every stretch of the cut a face reaches, read
  off the meridians crossing it and the way each runs, and along the whole
  cut for a face holding it — a whole ring with holes in its tube, a band the
  cut runs through. The volume reads the same closure: a whole ring with a
  quarter of the top of its tube cut away had a volume of minus the slot.
- **Its triangles** (`tessellation/outline/round/crossed.rs`,
  `sampling/round.rs`): each loop is broken where it crosses the cut and laid
  to the window's top and bottom, as two whole meridians were; a face running
  round the axis is walled as a whole ring is, the walls standing only where
  the face does, a latitude circle running round the axis laid from wall to
  wall. A meridian arc on a ring is sampled at every level of its faces it
  runs across, the cut among them, where the strips end.
- **The grid of a ring whose face winds round the tube while its rims all
  stand inside** (`sampling/cone.rs`, `across_the_crest`): a ring touching a
  shaft along its inner equator was sampled for a circle of the shaft's
  radius, its triangles standing twice the tolerance off at the crest
  (5576851706).
- **A ring cut along a rim or through a vertex** (`combine/seamed.rs`,
  `reseamed`): a slab resting on a ring's lower crest when the chart is cut
  there, a wall of the ring's own radius meeting its end meridians at the
  crests. The cut is moved to the middle of the widest band between them
  and their antipodes. Without the antipodes, the first measure found three
  cases right on the base and wrong here (5576873290, 5576876097,
  5577106512): a single rim's band is the whole turn, its middle the rim's
  antipode, and the seam — the window's middle, where a whole ring's face
  is sampled — fell on the rim, on the crest a slab rests on, so the cut
  took the whole ring.
- **The listing** (`soundness/listed/turning.rs`, `turning/wound.rs`), the
  harness's own: loops winding round the tube are judged by their winding
  alone — two of them part a ring into two bounded strips, and only the way
  they run says which is the face —, a ring's face may be holes alone, a band
  the cut runs through sweeps its circles and a whole turn of the tube, and a
  loop crossing the cut reads its rims past it a turn on. Its fixture of a
  quarter ring ran both meridians down.

**Measured like for like**: base `45b2492` (#622's tip) against the branch,
on the same seeds, every verdict compared case by case.

| seeds | kernel | cases | base broke a rule | branch broke a rule | refused on the base, built wrong on the branch |
| --- | --- | --- | --- | --- | --- |
| lane C round 2's, from 5576850000 | exact | 45 154 | 814, 18.03 ‰ | 535, 11.85 ‰ | 12, 4 of them plain |
| fresh, from 5577000000 | exact | 45 000 | 868, 19.29 ‰ | 568, 12.62 ‰ | 20, 4 of them plain |
| lane C round 2's, from 5576910000 | the application's body | 8 198 | 186, 22.69 ‰ | 126, 15.37 ‰ | 3, all hairs |
| fresh, from 5577100000, but 5577103202 | the application's body | 7 999 | 162, 20.25 ‰ | 103, 12.88 ‰ | none |
| round 1's 1 143 failing seeds | exact | 1 143 | 860 | 543 | 18 |

No case the base built right is built wrong on the branch, on any of them.
`Answers` fell from 337 to 80 on round 2's seeds and from 371 to 93 on the
fresh ones; `Listed` from 129 to 112; `Closed`, `Uncrossed`, `Spans` and
`Volume` stayed where they were, give or take ten.

**A case a user types, and a hair.** Half the cases the rounded draw gives
carry a hair — a number a few units of 1e-7 off a round one, an axis 3e-7
off, a lean of 1e-5 — which the draw puts there on purpose. Read apart, a
case every number of which is a multiple of a thousandth, `plain` below,
fails half as often as the others, and less than one time in a hundred:

| seeds | kernel | plain, base | plain, branch | hair, base | hair, branch |
| --- | --- | --- | --- | --- | --- |
| from 5576850000 | exact | 317 of 21 265, 14.91 ‰ | 156, 7.34 ‰ | 497 of 23 889, 20.80 ‰ | 379, 15.87 ‰ |
| from 5577000000 | exact | 341 of 21 266, 16.03 ‰ | 166, 7.81 ‰ | 527 of 23 734, 22.20 ‰ | 402, 16.94 ‰ |
| from 5576910000 | the body | 67 of 3 949, 16.97 ‰ | 28, 7.09 ‰ | 119 of 4 249, 28.01 ‰ | 98, 23.06 ‰ |
| from 5577100000 | the body | 61 of 3 820, 15.97 ‰ | 28, 7.33 ‰ | 101 of 4 179, 24.17 ‰ | 75, 17.95 ‰ |

Through the application's body, what a plain case still breaks is a
refusal two times in three — `Answers`, 22 and 25 —, the rest `Uncrossed`
(6 and 2) and one `Volume`.

**Refused on the base, built wrong on the branch.** Every plain one is
`Closed`, the triangles of a face open at a pole or at a turn's end beside
the coaxial turn the branch now builds — the family the base had already
on those draws, now reached; the application's body refuses such a step as
`Undrawn`, as it refused the boolean before. The others are hairs:
`Closed` the same way, and `Uncrossed`, a coaxial rim a hair inside or
outside the round's, #626's — the three through the body among them.

**A seed the body takes forty minutes over**, 5577103202, is left out of
the fresh body seeds: a turned leaf on an axis 3e-7 off the plane's and
leaning 1e-5, which the exact kernel declines, so the application builds it
on the flats, and five steps on, two joins of the flats take 126 s and
1 343 s to give 745 415 triangles, which do not close (`Closed`, on the base
too). A few others take a minute or two through the body; none on the exact
kernel. Since #632 the body refuses that leaf as the exact kernel does, in a
few hundredths of a second, and the seed need be left out no more.

**What is left**, for the pass that takes the kernel as far as it goes —
535 of round 2's seeds and 568 of the fresh ones, by family, with what a
sample of the plain ones shrinks to (`CAO_TRIAGE_DRAW=rounded`, at
`1df9cc3`); every one of them broke a rule on the base too, or was refused
there:

- **`Closed`, 160 and 169, a third of them plain** — the largest. A sphere
  turned part of the way, its poles on the axis, combined with a coaxial
  turn of another extent or sense: a ball turned 300° cut by a coaxial tube
  turned 270° (5576853450), a quarter ball joined to a coaxial cone turned
  the other way (5576867141), a half ball and a cap cut by a slab turned 90°
  in the other plane (5576850917), the same at −240° and −270°
  (5576871098), at 270° both (5576860117), and one ring joined to a slab
  turned 345° (5576862224). The boolean is built, the triangles of a face
  are open at a pole or at a turn's end, and the application refuses the
  step as `Undrawn`. It is what #623 did for a ring's cut, still to do for
  a sphere's poles; the place to start is the outline of a sphere's face
  whose loops reach a pole along meridians a rim parts
  (`tessellation/outline/round.rs`, `laid_round`, and `floors.rs`). Taken
  up by #628, below: the strips, not the poles.
- **`Uncrossed`, 157 and 154, four in five hairs** — #626: a coaxial rim a
  hair inside or outside the round's. The plain ones are a whole ring
  touching a whole coaxial cylinder along its outer or inner equator
  (5576855166, 5576859244, 5576864048, 5576877759, 5576887458, 5576889779):
  the triangles of the two cross beside the equator they share, and the
  application's cross the same way.
- **`Listed`, 112 and 130, half of them plain** — the same shapes as
  `Closed`, on a sphere or a spindle turned part of the way with a coaxial
  turn of another extent (5576850021, 5576854214, 5576860674, 5576869948):
  four of six shrunk are refused by the application, so the faces are
  wrong there too; two keep every rule through it (5576865967,
  5576872826), where it is the listing's reading that is to be looked at
  first, not the body.
- **`Answers`, 80 and 93, four in five hairs** — ties (an order of curves
  or a tolerance, not a sign of points: "#585" below); a straight
  edge traced on a round; the plain ones a round against a cylinder of
  another axis (5576851103), a spindle against a coaxial half turn
  (5576864698), a ring cut through its tube's centre by a coaxial cylinder
  turned the other way (5576879605), a cylinder on an axis leaning a
  thousandth beside a block (5576891961, no round at all).
- **`Spans`, 20 and 15**, all hairs but one: a whole ring cut by a coaxial
  half slab whose two planes touch its two crests (5576855990), built with
  matter missing — the application's volume is wrong too.
- **`Volume`, 8 and 6**, all hairs but one: a ring joined to a coaxial
  spindle turned 315° (5576858970), the exact kernel's triangles a hundredth
  short, the application's right.

## #628: a ball turned part of the way beside a coaxial turn, drawn

After #623, the largest family on the rounded draw was `Closed`: a ball, a
spindle or a ring turned part of the way and combined with a coaxial turn of
another extent, the boolean built by the exact kernel and one face of it left
open by the triangles — so the application refused the step as `Undrawn`. Ten
shrunk seeds, six `Closed` and four `Listed`, all refused through the body.
Three causes in the triangles, one in the harness's listing:

- **A meridian parted by a vertex** (`tessellation/outline/strips.rs`,
  `passing`): where an edge of another face meets a meridian arc, the arc is
  two edges and the vertex between them a sample — and every sample's height
  was a level of the strips, so the meridian across the face, sampled at the
  rows alone, crossed a level with no sample on it and the strips declined.
  A point the boundary only passes through along a meridian, at no row and
  no level of the face, is no level now: the column runs on through it
  (5576867141).
- **A rim elsewhere on the face a meridian runs past**
  (`tessellation/sampling/round.rs`, `on_meridian`): #623 sampled a ring's
  meridian arcs at every level of their faces they run across, where the
  strips end; a sphere's and a spindle's are now sampled so too — a ball
  turned 300° cut by a coaxial tube turned 270°, a half ball joined to a
  coaxial half shaft turned from the other plane (5576853450, 5576850917,
  5576850021, 5576860674, 5576869948). A whole meridian keeps it for a ring
  alone: on a sphere, a whole great circle crosses both poles.
- **One meridian at two angles an ulp apart** (`outline/round.rs`,
  `one_angle_a_meridian`; `outline/round/floors.rs`, `read_again`): two edges
  of one meridian each read their angle about the axis off their own arc, and
  a loop laid through a pole's floor and then cut open at an angle was
  unwrapped twice, `θ + 2π − 2π` an ulp off `θ`; the strips compare every
  coordinate bit for bit and found a slanted segment. A face's meridians
  within `CLEAR` of each other are one angle, and a laid loop's angles are
  moved back onto the very bits they were read at before the cut unwraps
  them (5576871098, 5576860117, 5576862224).
- **A loop through both poles, in the listing** (`soundness/listed/turning.rs`,
  `turning/poles.rs`), the harness's own: a lune between two meridians was
  closed along the first pole's floor alone, by the loop's own turn, nought,
  so its area came out wrong and the face read `Backwards` or `Pointed`. Each
  pass through a pole is closed along that pole's floor now, the low pole's
  running the angle up and the high pole's down. Two meridians part a sphere
  into two lunes, both bounded, so a lune listed backwards reads as the other
  one listed right: the rays, the triangles and the volume judge it, as on a
  ring since #623. Of the six `Listed` shrunk, five keep every rule since
  (5576850021, 5576860674, 5576865967, 5576869948, 5576872826).

**Measured like for like**: `main` at `1af2ae7` (#623 merged) against the
branch, on #623's seed sets, every verdict compared case by case.

| seeds | kernel | cases | `main` broke a rule | branch broke a rule | built right on `main`, wrong here |
| --- | --- | --- | --- | --- | --- |
| from 5576850000 | exact | 45 154 | 535, 11.85 ‰ | 318, 7.04 ‰ | none |
| from 5577000000 | exact | 45 000 | 568, 12.62 ‰ | 338, 7.51 ‰ | 1, a hair |
| from 5576910000 | the application's body | 8 198 | 126, 15.37 ‰ | 79, 9.64 ‰ | none |
| from 5577100000, but 5577103202 | the application's body | 7 999 | 103, 12.88 ‰ | 66, 8.25 ‰ | 2, hairs |

**The draw is under one in a hundred** on both kernels, still far above #576's
bar of one in a thousand. Read apart:

| seeds | kernel | plain, `main` | plain, branch | hair, `main` | hair, branch |
| --- | --- | --- | --- | --- | --- |
| from 5576850000 | exact | 156, 7.34 ‰ | 65, 3.06 ‰ | 379, 15.87 ‰ | 253, 10.59 ‰ |
| from 5577000000 | exact | 166, 7.81 ‰ | 62, 2.92 ‰ | 402, 16.94 ‰ | 276, 11.63 ‰ |
| from 5576910000 | the body | 28, 7.09 ‰ | 13, 3.29 ‰ | 98, 23.06 ‰ | 66, 15.53 ‰ |
| from 5577100000 | the body | 28, 7.33 ‰ | 10, 2.62 ‰ | 75, 17.95 ‰ | 56, 13.40 ‰ |

`Closed` fell from 160 to 9 on round 2's seeds and from 169 to 15 on the fresh
ones, `Listed` from 112 to 32 and from 131 to 41.

**Built wrong where `main` built right or refused**, all hairs: a ball or a
cone cut by another a few units of 1e-8 inside it, which leaves a skin
thinner than the tolerance (5577012966, 5577100697, 5577107628 shrunk). On
`main`, both faces of the skin were sampled at the rows alone, their
triangles stood on the same points and cancelled; sampled now at their own
rims, a hair apart, they cross. The same through the body for four cases
`main` refused (5576914160, 5577100278, 5577103347, 5577106943). It is
#626's: a rim within a hair of a round's taken onto it would leave one rim.

**What is left**, for the pass that takes the kernel as far as it goes — 318
of round 2's seeds and 338 of the fresh ones on the exact kernel:

- **`Uncrossed`, 171 and 166, four in five hairs** — #626. The plain ones,
  30, are a whole ring touching a whole coaxial cylinder along its outer or
  inner equator (shrunk under "#623" above), the triangles of the two
  crossing beside the equator they share.
- **`Answers`, 80 and 95, four in five hairs** — ties (curve and tolerance
  decisions, "#585" below), a straight edge
  traced on a round; the plain ones, 17 and 16, a round against a wall of
  another axis (#624), a spindle against a coaxial half turn, a ring cut
  through its tube's centre by a coaxial cylinder turned the other way.
  Through the body, the plain cases left are 7 refusals in about 3 900.
- **`Listed`, 32 and 41, half of them plain**, among them a spindle holding a
  pole whose only loop is a slit along half a latitude circle (5576854214),
  refused `Undrawn` through the body: a face that holds one pole of two with
  no loop closing the other — the boolean's, not the triangles'.
- **`Spans` 18 and 15, `Closed` 9 and 15, `Volume` 8 and 6**, all hairs but a
  few: the half slab touching a ring's two crests (5576855990), a ring joined
  to a coaxial spindle turned 315° (5576858970).

## #632: a revolution the exact kernel cannot make is refused

The application's body turned on the flats what the reading declines to lay,
and every step after it was the flats' boolean. Since #632 it refuses it, as
the exact kernel does; the harness already counted such a decline apart
(`bowed`, `thin`), and counts the rest as no answer.

**Measured like for like**: the body's verdicts on #628's seed sets, case by
case against #628's own measure of them, which is the kernel `main` holds.
Every verdict that moved is a case the flats answered; none the exact kernel
built moved.

| seeds | cases | `main` broke a rule | branch broke a rule | the flats built right, refused here | the flats built open, refused here | the flats built open, no answer here |
| --- | --- | --- | --- | --- | --- | --- |
| from 5576910000 | 8 198 | 79, 9.64 ‰ | 68, 8.29 ‰ | 13 | 11 | 1 |
| from 5577100000, but 5577103202 | 7 999 | 66, 8.25 ‰ | 58, 7.25 ‰ | 17 | 8 | |

Of the fifty cases the flats answered, twenty came out open (`Closed`); the
thirty they built right are counted declined now, which is what the exact
kernel still lacks. One of the twenty is a decline the harness does not
excuse, neither a bowed section the reading declines nor a wall a hair thin,
so no answer (5576914427), as the two `laying-declines` seeds named in
`what_the_exact_campaigns_found_in_the_kernel.rs` are now, where the flats
left them open along a rim. Seed 5577103202 itself, kept out of the second
set because the flats took it forty minutes, was refused at its first leaf in
a few hundredths of a second, until #633 laid it.

## #633: a groove about an axis leaning a hair

The reading declined seed 5577103202's ring about an axis leaning 0.001°,
0.1°, 1° or 5° and laid it at 1e-5°, 0.01°, 15° or 30°: a groove whose half
round bottom is tangent to its two walls, the harness turning the section
rigidly with its axis, so that read against it the profile is the same at
every lean up to rounding. The cause was the bits, not the lean
(`turning/straight/bends.rs`, `landed`): an arc's end is moved onto its
circle along the coordinate its next run leaves free, and along a groove's
wall, square to the axis, that line is tangent to the circle — the root it
takes, of `radius² − fixed²`, is of nought up to the rounding the lean
leaves. An ulp below nought and the wall missed the circle, so the profile
was declined; an ulp above and the end was moved the root of it, some 1e-6,
off its level, tangent no longer.

Two rules lay it now:

- **An end laid on its centre's level stays**, when it stands within
  roundings of the radius from the centre: it is the tangent point.
- **A half circle ending tangent to the run leaving it** — a groove's
  bottom between two walls, a tube's wall rounded whole between its two
  shafts — has its centre laid at the middle of its two ends (`diameter`).
  Drawn square to its sketch and turned about an axis a hair off it, as #632
  drew the pulley in the application, the groove's walls are laid square and
  its centre kept as drawn stood a hair off their middle: the arc's end
  came out off its circle by more than the kernel's own tolerance, and the
  kernel refused it as no profile it reads.

A first version laid every half circle's centre so, a ball's included: a
ball's poles land along the axis, where nothing is tangent, and moving its
centre by an ulp turned two hair cases `main` builds right into crossing
triangles through the body (5576911550, 5576915729), and a third that broke
a rule already (5576913025). The rule is held to a tangent landing since,
and `a_ball_drawn_a_hair_off_the_middle_of_its_poles_keeps_its_centre_as_laid`
says so.

**Measured like for like**: `main` at `43c30b9` (#632 merged) against the
branch, every verdict compared case by case. On the body's sets `main` is
#632's own measure, its tree bit for bit; on the exact kernel's it is #628's,
which #632 left alone but for comments.

| seeds | kernel | cases | `main` broke a rule | branch broke a rule | bowed sections declined, `main` | branch | built right on `main`, wrong here |
| --- | --- | --- | --- | --- | --- | --- | --- |
| from 5576850000 | exact | 45 154 | 318, 7.04 ‰ | 320, 7.09 ‰ | 188 | 89 | none |
| from 5577000000 | exact | 45 000 | 338, 7.51 ‰ | 339, 7.53 ‰ | 181 | 86 | none |
| from 5576910000 | the application's body | 8 198 | 68, 8.29 ‰ | 68, 8.29 ‰ | 22 | 8 | none |
| from 5577100000, but 5577103202 | the application's body | 7 999 | 58, 7.25 ‰ | 58, 7.25 ‰ | 24 | 13 | none |

Some 220 sections the reading declined are laid and answered now. Three of
them, on the exact kernel, break a rule — every one a hair:
5576853629 (`Answers`, a ring cut by a ring a ten-millionth along), 5576888940
(`Uncrossed`, two coaxial rounds 2e-7 apart) and 5577009942 (`Listed`, a pole
6e-8 off the axis); through the body the second keeps every rule and the two
others are refused, as they were when their section was declined. Two cases
that broke a rule already break another one now (5577022081, 5577105811,
`Answers` to `Uncrossed`). The rate does not move: what the reading declined
was excused, and is answered now.

**What is left, by the reading**: a pulley drawn square to its sketch is
declined about an axis leaning between some 4e-6° and 1.2e-5°. Its walls,
fifteen long, are within the tolerance of square there, and its groove's two
ends, sixty apart along the axis, are not one level, so no circle is tangent
to both square walls. Below that window the ends are one level and the
centre is laid at their middle; above it the walls lean past the tolerance
and are slanted as drawn.

#636 decided to leave it. No lean a hand draws falls in the window, nor what
a constraint leaves: an axis typed a thousandth off square over 200 already
leans 5e-6 radians, past it, and a solver's residual is far below it. Inside
it the step is refused at once, with its reason. A groove `W` wide along the
axis between walls `D` deep is declined from a lean of some `tol / W` to
`tol / D` radians, wherever `W` is the larger. Laying the arc's two ends on
one level, each moved by up to twice the tolerance as a corner may be, would
start that window at `4 tol / W`: closed for the pulley, sixty against
fifteen, and still open for any groove more than four times as wide as its
walls are deep; and it would be a second levels rule, reaching across an
arc. Laying the walls as drawn would read cones within the tolerance of
planes, the ground decision 11 keeps off.

Laid, seed 5577103202's ring about a leaning axis is no longer parallel to
the case's second leaf, turned about a line parallel to the sketch's own
axis: the two meet along cylinders at a skew angle, which the exact kernel
declines as unsupported, so that step is refused at once, where about the
straight axis it is built and the next one refused. The whole case takes the
body under a second, as about the straight axis.

## #626: a coaxial surface a hair from tangent to a round

After #633, the largest family on the rounded draw was `Uncrossed`, four in
five of them hairs: a coaxial cylinder, cone or plane standing a few units
of 1e-8 from tangent to a sphere or a torus — a bore 2e-8 narrower than the
ball it runs through, a band 1e-7 inside a ball's equator, a disc 1e-8 under
a ring's two crests. The two cross along two circles `√(2rδ)` apart, a few
ten-thousandths: real geometry, thinner than any triangle laid between them.

**Fixed:**

- **Decision 12** (`canonical/tangent.rs`, `exact-kernel.md`): such a
  surface, within decision 8's hair `G`, is moved onto the touch with its
  operand — along the axis for a plane or a cone, away from it for a
  cylinder, its radius grown or shrunk by the hair
  (`canonical/carried/across.rs`). So is one standing within `G` of a rim
  the other operand drew on its round: a rounded zone cut by its own
  section moved 5e-8 along the axis left a ring 1.7e-8 wide (5576809934).
- **A disc resting on a ring's two crests, its wall through both**
  (`selection/parted.rs`), which the hair case becomes once moved and which
  `main` declined `Unverified` drawn exactly so: a rim along which one
  surface only touches a whole ring was read as parting nothing, though the
  disc's wall crossed the ring along it. It parts the ring now where a face
  of the crossing surface reaches the rim, and only there: a band's end
  plane through the circle a ring touches a post along, its face short of
  it, does not (5577022049).

**What the first measure found.** Built as above, the decision held 147 of
the two exact sets' `Uncrossed` and broke 17 cases `main` held, each shrunk
and replayed on both trees: ten were the decision's own — a move closing one
hair and opening another — and are refused since, each held by a test in
`a_wall_a_hair_from_tangent_to_a_round.rs`: a ball's equator taken off the
disc floor it stood on (5576859311), a tool's wall parted from a disc's it
was one with (5577024047), a cap laid onto its copy 1e-7 along, the lens
between them lost (5577028782), a ring a hair thinner than another about
the same circle brought to touch it inside (5576883559, 5577020735), a bar
grown onto the rim of a cap far along the axis from it (5576873713), and the
first `parted.rs` rule reading a plane short of a rim as crossing it
(5577022049). Two of those seeds, 5576873713 and 5577020735, held as
shrunk, still break a rule by another path through the draw, below.

**Measured like for like**: `main` at `c0ba3d0` (#637 merged; its verdicts
are #633's measure, its tree bit for bit) against the branch, on #623's and
#628's seed sets, every verdict compared case by case.

| seeds | kernel | cases | `main` broke a rule | branch broke a rule | `Uncrossed`, `main` → branch | held on `main`, built wrong here |
| --- | --- | --- | --- | --- | --- | --- |
| from 5576850000 | exact | 45 154 | 320, 7.09 ‰ | 255, 5.65 ‰ | 172 → 112 | 2 |
| from 5577000000 | exact | 45 000 | 339, 7.53 ‰ | 252, 5.60 ‰ | 167 → 100 | 2, and 1 declined on `main` |
| from 5576910000 | the application's body | 8 198 | 68, 8.29 ‰ | 57, 6.95 ‰ | 37 → 25 | none |
| from 5577100000, but 5577103202 | the application's body | 7 999 | 58, 7.25 ‰ | 46, 5.75 ‰ | 31 → 18 | none |

Read apart, a case every number of which is a multiple of a thousandth
against one that carries a hair:

| seeds | kernel | plain, `main` | plain, branch | hair, `main` | hair, branch |
| --- | --- | --- | --- | --- | --- |
| from 5576850000 | exact | 65 of 21 265, 3.06 ‰ | 60, 2.82 ‰ | 255 of 23 889, 10.67 ‰ | 195, 8.16 ‰ |
| from 5577000000 | exact | 62 of 21 266, 2.92 ‰ | 58, 2.73 ‰ | 277 of 23 734, 11.67 ‰ | 194, 8.17 ‰ |
| from 5576910000 | the body | 13 of 3 949, 3.29 ‰ | 13, 3.29 ‰ | 55 of 4 249, 12.94 ‰ | 44, 10.36 ‰ |
| from 5577100000 | the body | 10 of 3 820, 2.62 ‰ | 10, 2.62 ‰ | 48 of 4 179, 11.49 ‰ | 36, 8.61 ‰ |

**Held on `main`, wrong or refused here**, each shrunk on the branch and
replayed on both trees. Four reach, by another path through the draw, a
family `main` breaks on the shrunk case too: a whole ring touching a whole
coaxial cylinder along its equator (5576852238, 5577008284, both
`Uncrossed`; 5577042736, a post along a ring's inner equator, `Volume`),
a ring cut by one a hair thinner (5577020735, declined on `main`,
`Uncrossed` here). One is the harness's: a bar 1e-6 thinner than the rim
of a cap turned half a turn, grown onto the rim, stands that hair outside
the box of what was drawn on the side the half cap leaves bare
(5576873713, `Volume` by `within_reach`, as the hair-off ball end of #576);
the body is the one the bar drawn at the rim's radius gives, bit for bit.
Four are refused where `main` built them, each on a family `main` refuses
on its shrunk case too: 5576863438, 5576877739 on the exact kernel,
5576911895, 5577103737 through the body.

**What is left**, for the pass that takes the kernel as far as it goes —
round 2's `Uncrossed`, 112, sorted by what the shrunk case of `main` holds:

- **Plain, about 43** — a whole ring touching a whole coaxial cylinder
  along its outer or inner equator: the triangles of the two cross beside
  the circle they share (under "#623" above).
- **Two rounds a hair apart, about 31** — two balls of one radius, or two
  rings, a hair apart along the axis or in radius. Laying one onto the
  other is what decision 8 does for walls, but two rounds cross at a grazing
  angle all along their circle, and the lens merged away was seen by the
  rules' lines (5577028782): not this issue's, and set aside.
- **A wall a hair off a round still, about 35** — hairs past `G`, a few
  units of 1e-6 or 1e-5, real geometry the triangles do not keep apart; a
  ball of the second operand a hair inside a cylinder of the first, which no
  move of the ball settles (its poles stand on the axis); and moves a guard
  refuses.

## #585: which side a point stands on

#585 takes exactly the decisions of the kernel that are the sign of a
polynomial in held points (`orientation.rs`, and `exact-kernel.md`, "Which
side a point stands on", for the list): the side of a slant the laid
profile asks of a corner, which way a laid contour turns, which side of an
edge a folded triangle stands. The others are a tolerance or a curve, and
it leaves them as they are.

**Measured like for like**: `main` at `e411888` against #585's first
commit, on the same seeds, every verdict compared case by case. Its second
changes no answer: the early exit it adds returns the sign the exact sum
would. On the exact kernel, 20 000 cases a draw:

| draw | from seed | `main` broke a rule | branch broke a rule | the same seeds |
| --- | --- | --- | --- | --- |
| square | 5585100000 | 1 | 1 | yes |
| profiles | 5585200000 | 2 | 2 | yes |
| turned | 5585300000 | 9 | 9 | yes |
| slanted | 5585400000 | 11 | 11 | yes |
| tilted | 5585500000 | 53 | 53 | yes |
| rounded | 5585600000 | 95 | 95 | yes |
| ellipses | 5585700000 | 147 | 147 | yes |
| ellipses crossed | 5585800000 | 20 | 20 | yes |
| ellipses tilted | 5585900000 | 46 | 46 | yes |

Through the application's body, 2 000 cases a draw:

| draw | from seed | `main` broke a rule | branch broke a rule | the same seeds |
| --- | --- | --- | --- | --- |
| profiles | 5586200000 | 0 | 0 | yes |
| turned | 5586300000 | 0 | 0 | yes |
| slanted | 5586400000 | 0 | 0 | yes |
| tilted | 5586500000 | 5 | 5 | yes |
| rounded | 5586600000 | 7 | 7 | yes |
| ellipses | 5586700000 | 14 | 14 | yes |
| ellipses crossed | 5586800000 | 1 | 1 | yes |
| ellipses tilted | 5586900000 | 4 | 4 | yes |

No family moved, and no seed that held on `main` fails: the sign
decisions #585 made exact are not where the campaigns break.

**Where the refusals come from**, each refused seed of `main` run again as
drawn on a build that prints where a refusal is raised — 232 of the 384
failures of the nine draws:

- `Unverified` at `assembly.rs`, an edge of the result run more times one
  way than the other — 111: tilted 35, ellipses 34, tilted ellipses 23,
  rounded 8, slanted 3, turned 3, crossed ellipses 2, profiles 2,
  square 1. On the tilted draw, a plane a hair off parallel to another or
  a hair from an edge: #644.
- `Unsupported` at `combine/related.rs`, two parallel walls, one elliptic,
  the same ellipse a hair beside the other — 27, all on the ellipse draw:
  #574's first lead.
- `Unsupported` at `domain/traced/taken.rs`, a straight edge traced on a
  curved wall it was taken onto by decision 6 — 26.
- `Tie` — 50: 34 in `overlay/partition.rs` (a region no column crosses,
  following the cycles and heights of curves), 8 in `overlay/star.rs` (the
  order of curves at a corner, by tangent, bend and chord), 8 in
  `selection.rs` (every read of a region tied); shrunk, one more reached
  `ray.rs` (every direction doubtful). A curve or a tolerance decision each, none a sign
  of points: the "ties, #585" of #628's list above are not #585's.
- the rest, 18: a line lying on a round (`Unsupported` in
  `domain/traced.rs`, 10, rounded), a travel the kernel does not make
  (`Travel`, 4, crossed ellipses), a section the reading cannot lay
  (`Profile`, 3), a crossing it does not build (`combine/crossed.rs`, 1).

**What it costs**: nothing measurable. `main` and the branch's tip were
run side by side on the same seeds, 2 000 cases a draw on one thread
each, so that both met the same load:

| draw | `main` | branch |
| --- | --- | --- |
| square | 124.0 s | 124.0 s |
| turned | 328.9 s | 325.5 s |
| tilted | 267.1 s | 265.3 s |
| rounded | 351.0 s | 341.5 s |

The gate, `cargo test --workspace` built already and run three times on
each tree in turn: 94.1 s at best on `main`, 95.3 s on the branch, less
than one run differs from the next. A predicate builds its expansion only
when the plain product's error bound does not settle the sign.

## #644: the largest families, round after round

#644 runs the loop Tom asked for on #585: every failure of the last
measure rerun alone on a build that prints where a refusal is raised,
shrunk, and sorted into families; the largest family a change of the kernel
fixes is fixed, with a test that fails before; and every draw is measured
again on the same seeds, verdict against verdict. Flash sorted the rounded
and tilted-ellipse draws into families (`triage`, a brief that asks for a
mechanism per family and at most eight a draw); every family below was
read again on its shrunk cases before anything was changed.

**Round 1** took six families, each with a test in
`the_largest_failure_families.rs`:

- **Two planes a hair off parallel** (`canonical/squared.rs`, decision 11
  widened): a block's top and a slab's floor leaning 1e-7° off it had three
  corners within the tolerance and one 9 % past it, and the block was
  declined `Unverified` (5585501573). Decision 11 now lays two planes as it
  laid a plane and a wall; where no one turn lays them with the rest, the
  turn is sought again without them, so that what #552 laid is laid still.
  #552 had tried every pair and set it aside on one seed; two planes alone,
  measured on every draw, break none.
- **A ring face across its chart's cut laid from inside itself**
  (`tessellation/outline/round/crossed.rs`): a ring turned 359.9° against a
  post's end, and a ring notched between two whole latitudes, were laid from
  the widest gap between their own meridians, through the face, and left
  open — `Closed` (5585602707, 5585617453). The opening is read off the
  loops that do not run round the axis, and where no grid angle stands
  clear, it is the one gap both of whose sides are the face's meridians.
- **A ring round a post of its inner radius** (`tessellation/sampling/cone.rs`):
  the ring's chords along the circle they touch, on its own grid, cut
  across the post's between two of its steps — `Uncrossed` (5576855166). A
  round's grid is now a multiple of a coaxial cylinder's that touches it
  along an equator, when the two grids start at one angle.
- **A ring in a band of its outer radius** (`selection/parted.rs`): the
  ring stands a hair inside the circle for some way on either side of it,
  and so do the band's chords — `Uncrossed` (5585606247). A cylinder holding
  a whole ring and touching its outer equator now parts it there; at the
  inner equator parting broke the volume, and the grid does it instead.
- **A slit bounds no pole** (`topology/poles.rs`): a ball a half band
  touches along half its equator, slit there, was read as holding one pole
  of two — `Listed`, and refused through the body (5585606550).
- **A plane a hair from an edge** (decision 13, `canonical/edged.rs`): a
  cut at 45° through a point a third of the tolerance off a block's top
  edge — `Unverified` (5585500692, 5585502204). Its first form broke four
  seeds that held: a cut a hair off parallel to one of the edge's planes
  crosses it far from the edge, and laid through the edge takes a strip off
  it (5585501810, 5585505134, 5585513889); and two edges a hair apart leave
  the cut a hair off the one it is not laid through (5585505818). It now
  asks that the cut cross each plane along the edge within `G` of it, and
  that every edge it would be laid through share one gap. The narrowing
  gives back nine seeds the first form held, seven of the tilted ellipse
  draw and two of the tilted one: rerun with decision 13 traced, eight
  shrunk cases reach it, and in each the cut leans a hair off one plane
  along the edge, 5e-9 to 9e-4 rad, so the first condition lets it go (two
  of them, a cut between two edges at opposite gaps, the second too). Laid
  through the edge, those were built, where the four above were refused.
  The lean does not tell them apart, the same 6e-8 to 2e-7 rad in both,
  nor does where the cut crosses that plane: inside the face or past it,
  each happens on both sides. What does is the next pass's question.

**Round 2** sorted round 1's tip again, flash naming each failure's
mechanism. Its largest families are #574's first lead, a decision for Tom,
or set aside on #626 ("What is left", below); the one it took is the
harness's own:

- **A prism a few roundings thick** (`random_solids/building.rs`): the
  crossed ellipses draw raised a disc by a height a subtraction left
  1.8e-15 long, two faces closer than the draws promise never to ask
  (`NEAR`). The kernel declined it as no travel, rightly, and the campaign
  counted no answer. Such a prism is no solid now, and its case holds
  nothing (5585804304 and three more). The draw never asks whether a leaf
  is solid, so every other seed draws what it drew.

**Measured like for like**, #585's tip against round 1's, the same seeds,
every verdict compared case by case. On the exact kernel, 20 000 cases a
draw:

| draw | from seed | #585 broke a rule | #644 broke a rule | held now | broken now |
| --- | --- | --- | --- | --- | --- |
| square | 5585100000 | 1 | 1 | 0 | 0 |
| profiles | 5585200000 | 2 | 2 | 0 | 0 |
| turned | 5585300000 | 9 | 8 | 1 | 0 |
| slanted | 5585400000 | 11 | 10 | 1 | 0 |
| tilted | 5585500000 | 53 | 38 | 15 | 0 |
| rounded | 5585600000 | 95 | 76 | 19 | 0 |
| ellipses | 5585700000 | 147 | 147 | 0 | 0 |
| ellipses crossed | 5585800000 | 20 | 20 | 0 | 0 |
| ellipses tilted | 5585900000 | 46 | 27 | 19 | 0 |
| all | | 384 | 329 | 55 | 0 |

Through the application's body, 2 000 cases a draw:

| draw | from seed | #585 broke a rule | #644 broke a rule | held now | broken now |
| --- | --- | --- | --- | --- | --- |
| profiles | 5586200000 | 0 | 0 | 0 | 0 |
| turned | 5586300000 | 0 | 0 | 0 | 0 |
| slanted | 5586400000 | 0 | 0 | 0 | 0 |
| tilted | 5586500000 | 5 | 3 | 2 | 0 |
| rounded | 5586600000 | 7 | 7 | 0 | 0 |
| ellipses | 5586700000 | 14 | 14 | 0 | 0 |
| ellipses crossed | 5586800000 | 1 | 1 | 0 | 0 |
| ellipses tilted | 5586900000 | 4 | 1 | 3 | 0 |
| all | | 31 | 26 | 5 | 0 |

**What is left**, round 1's tip sorted by family:

- **The ellipse draw, 147, unchanged**: one ellipse a hair past decision 8
  beside another wall — the same ellipse turned 1e-7 to 1e-4 rad, moved a
  few units of 1e-6 or 1e-4, or resized within the hair, beside a block
  plane, a circle or a third ellipse. Flash sorted them by mechanism, and a
  spot check of their cases holds it:
  - two walls a hair apart running an edge unequally through a node,
    `Unverified` — 36;
  - the shell left open between them, `Closed` — 27;
  - the pair refused in `combine/related.rs`, `Unsupported` — 26;
  - their triangles crossing, `Uncrossed` — 22;
  - a curve's stretch not laid as a chord (`traced/taken.rs`), mostly a
    near-round ellipse beside a circle, `Unsupported` — 23;
  - a sliver between them no column crosses, `Tie` in `overlay/partition.rs`
    — 9; and four single cases.

  All but the chords are #574's first lead, which extends decisions 7 and 9,
  the band and grazing decisions of two circular walls, to elliptic pairs:
  the next pass's largest family, and #574's own. The chords are its second
  lead's shape, an arc of a near-round ellipse against a circle, which waits
  on Tom's rule.
- **The rounded draw, 76** (flash, on the shrunk cases):
  - two rounds a hair apart, set aside on #626 — 20;
  - a coaxial turn a hair off a round's equator, past `G`, set aside on
    #626 — 15;
  - a round ending a hair from the axis — 14, every one `Spans` or `Volume`:
    the kernel lays an arc's centre within `turning::ON_THE_AXIS` on the
    axis and reads its radius again, which moves the cap by about 1e-5, and
    the harness judges the arc as drawn. Whether the harness reads the
    kernel's band or the band narrows is **a decision for Tom**;
  - curves meeting at an edge that ties — 5; a straight edge a hair long
    lying on a ball or a ring, declined in `domain/traced.rs` — 4 (a chord
    of a sphere 1e-5 long, or the stretch of the axis inside a ring whose
    tube reaches a hair past it, where the ring's chart has no angle to
    trace it by); a ball's equator on a coaxial cylinder still — 3; a
    section's corner a hair off its circle — 2; and 13 single cases.
- **The tilted draw, 38**: 32 refused, 23 of them `Unverified` at
  `assembly.rs`, and 6 `Uncrossed`. The shrunk cases decision 13 lets go
  are a plane a hair from a turn's rim, which cuts a conic, and leans of
  1e-5 degrees: #552's warning band. Two are the seeds its narrowing gives
  back.
- **The tilted ellipse draw, 27**: the seven its narrowing gives back, and
  twenty flash sorted on its first form: a plane a hair from an elliptic or
  a round wall's tangent line — 11, and 3 more a hair off tangent and
  crossing it; a plane a hair off an edge where faces meet — 3; three
  single cases. #574's third lead, the warning band, again.
- **The crossed ellipse draw, 20, 16 once the harness's four are counted
  apart**: a side tool whose wall ends where the crossed wall's extreme
  stands, `Uncrossed` — 5; an elliptic arc taken for a chord — 4; a tool a
  hair off the face it was drawn to meet — 3; four single cases.
- **The small draws**: square 1, profiles 2, turned 8 and slanted 10, one
  seed each of the last two held by round 1.


## #574: the ellipse draws, round after round

#574 took up what #571 left on its three ellipse draws, with Tom's decision
of 2026-11-03 on its second lead and #571's stopping rule: a family that
does not move in two rounds is set aside and written here. Each round sorted
the failures of the last measure into families on a build that prints where
a refusal is raised, wrote the shrunk case of one family as a test that
fails on `main` (`walls_a_hair_from_an_ellipse.rs`), fixed it, and measured
the draw again on #644's seeds, verdict against verdict. A change was kept
only where no seed that held before broke.

### What was fixed

- **An arc of an ellipse a hair from round** (Tom's decision, the second
  lead; `piece/oval.rs`, `domain/traced.rs`): its half-axes within decision
  8's hair of one another, it is raised as an arc of the circle of their
  mean, its corners laid on that circle and the straight traits ending at
  them following, and a rim in its band is traced on it (5585703945,
  5585718355, 5585700308). The decision names no neighbour; the kernel
  raises only an arc between two straight traits, since a corner moved
  off a curve beside it would leave that curve, and an arc beside another
  curve keeps the tolerance.
- **The same ellipse turned a hair past decision 8** (`relation/walls.rs`):
  its feet all stood within the hair of the other, but between them the two
  walls part by more, and the kernel took them for one; the widest gap is
  now read between the feet, and the two cross (5585704198).
- **An ellipse and its osculating circle leaving a vertex together**
  (`overlay/star.rs`): ordered by where each has run to along its own arc,
  not by a share of its parameter (5585700934).
- **A whole ellipse a hair from round and a circle of its size about one
  axis** (`crescent/other_kind.rs`): one wall, as two circles are
  (5585703794, 5585717426).
- **An elliptic wall crossing another at a hair** (`sampling/abreast.rs`):
  the steps of its grid crowding the other wall are left out, as a circular
  wall's are (5585713456).
- **An edge beside an elliptic wall decision 2 grew onto a touch**
  (`combine/held.rs`): laid on the grown wall, from as far as its half-axes
  changed (5585800487).
- **A tunnel as wide as a tube's wall** (`canonical/touches/across.rs`): a
  cylinder a hair from touching two walls square to it at the two ends of
  its reach is moved midway and its radius set to touch both. The drawn
  tunnel is an ellipse within the hair of round, raised as a circle; an
  elliptic tunnel meets the tube's walls on the walls as they stand and
  needed no move (5585800308).
- **A plane leaning a hair off a wall's tangent** (`overlay/star.rs`,
  `along`), #574's third lead: the conic it cuts leaves the corner along the
  rim and then plunges, and read as a straight share of its first chord it
  was ordered above the rim; the chord the run ends in is measured again
  along its own chords. Shared with round walls, it holds two seeds of the
  tilted draw and breaks none (5585909807).

### Tried and taken back

- **The rays of a lens or a band a hair thick shared by its two rims**, and
  an ellipse sampled on the rays it shares with its twin a hair narrower,
  all round: the shrunk cases held, and the whole draw broke 31 seeds that
  held before. At the draw's coarser fineness the zone covered most of a rim
  and the two rims of one wall face disagreed. The shrunk cases hold only
  what failed: every rule since was measured on the whole draw first.
- **The paired drop**: where the two rims of a lens a hair thick end on one
  plane, the place one drops near the lens's end the other drops too
  (5585709823). Its first form broke 55 held seeds. Kept only at a lens's
  true ends, it held one of #644's seeds and broke none of them; on the
  fresh seeds it held two and broke three, among them the only two the
  branch broke of those holding on `main` — 5574700017, the same ellipse
  turned 1e-5 of a radian, and 5575703942 through the body, the same
  ellipse moved 5e-5 over one a circle was cut through. It was taken out.
- **Bounding decision 2's move of a turned ellipse** rather than laying the
  rims on the grown wall: it changes the bits of every turned elliptic node.

### Measured on #644's seeds

Failures in 20 000 seeds, each change measured against the one before it;
none broke a seed that held before it. The last two rounds were measured
apart, on the draws each could reach: the ellipses draw by the tunnel's,
the tilted draw by the tangent's. Taking the paired drop out gave back the
one seed it held, 5585709823, which fails on `main` too. Against `main`,
the branch's tip holds 55, 7 and 8 of these seeds and breaks none.

| Draw | `main` | Branch |
| --- | --- | --- |
| ellipses | 147 | 92 |
| crossed | 20 | 13 |
| tilted ellipses | 27 | 19 |

### Measured on fresh seeds

The criteria's measure: every draw on seeds no round was sorted on, 20 000
on the exact kernel and 5 000 through the application's body, `main` and
the branch's tip verdict against verdict.

| Draw | `main` | Branch | Held | Broken | Branch, ‰ |
| --- | --- | --- | --- | --- | --- |
| ellipses | 162 | 100 | 62 | 0 | 5.0 |
| crossed ellipses | 34 | 33 | 1 | 0 | 1.65 |
| tilted ellipses | 29 | 18 | 11 | 0 | 0.9 |
| square | 2 | 2 | 0 | 0 | 0.1 |
| profiles | 4 | 4 | 0 | 0 | 0.2 |
| turned | 7 | 7 | 0 | 0 | 0.35 |
| slanted | 6 | 6 | 0 | 0 | 0.3 |
| tilted | 22 | 20 | 2 | 0 | 1.0 |
| rounded | 76 | 76 | 0 | 0 | 3.8 |
| profiles, body | 1 | 1 | 0 | 0 | 0.2 |
| turned, body | 2 | 2 | 0 | 0 | 0.4 |
| slanted, body | 0 | 0 | 0 | 0 | 0 |
| tilted, body | 2 | 2 | 0 | 0 | 0.4 |
| rounded, body | 29 | 29 | 0 | 0 | 5.8 |
| ellipses, body | 43 | 36 | 7 | 0 | 7.2 |
| crossed ellipses, body | 11 | 11 | 0 | 0 | 2.2 |
| tilted ellipses, body | 9 | 7 | 2 | 0 | 1.4 |

No seed that holds on `main` breaks, on any draw. The tilted ellipse draw
ends under the bar; the ellipses and the crossed draws do not. The crossed draw held seven of #644's seeds and one fresh one.

### What stays above the bar

What the ellipses draw still fails, sorted at the round before last (91
on #644's seeds), with the crossed and tilted draws beside it:

- **A wall beside its near copy, past decision 8's hair** — 39, ellipses
  only. The same ellipse turned 1e-7 to 1e-4 of a radian, moved or resized
  1e-6 to 5e-4, or an ellipse near round against a circle of its size:
  twenty to ten thousand tolerances apart, so decision 8 rightly keeps two
  walls, and the kernel builds their true crossings — two long, nearly
  tangent curves and a crescent a hair thick, often with a block's face at
  a vertex of one copy. Left `Unverified` at `assembly.rs`, `Closed`,
  `Uncrossed` or a tie. The first lead, decisions 7 and 9 extended to
  elliptic pairs, was the shared-rays rule taken back above; widening
  decision 8's elliptic merge past its hair would hold the family, and
  #677 decided against it (below).
- **A touch at an ellipse's vertex, within a few tolerances** — 38 in the
  ellipses draw, 2 crossed. Mostly `Closed`: on a planar cap the ellipse's
  rim and the circle, flat or ellipse touching it cross, and the cap's sweep
  fails; three rounds on its sampling held one seed. The rest `Uncrossed`,
  `Unverified` where a third surface passes through the same ruling, or
  `Listed` with a wall one tolerance off a face.
- **A plane at a curved wall's tangent, the rest** — 9 tilted: two corners
  1.4e-14 apart never merged (5585909267), rays doubting round a region a
  hair thin (5585913058), a plane a hair inside a wall's extreme beside a
  meet with a tilted cylinder (`Unverified`, four), and the sliver's
  triangles crossing (`Uncrossed`, three).
- **A cut plane a hair off a block's face, leaning** — 7 tilted, no
  ellipse: plane against plane, in decision 11's band and the warned band
  past it.
- **A surface through or a hair off a node** — 6 crossed, 3 tilted: a third
  wall or a box's face six to thirteen tolerances off where two crossed
  walls meet.
- **Two crossed walls at an exact node** — 5 crossed: their triangles cross
  near it; `tessellation/contact` lays strips at nodes of two cylinders
  only, not of an elliptic wall (5585803427, 5585809805, 5585814173).
- **No ellipse at all** — 10 in the ellipses draw: rounded outlines, slots
  and circles within about a tolerance of each other or of flats.
- **Decision 8's own refusal** — 3: two elliptic walls within its hair, the
  second carrying corners off the first's wall, left two on purpose.

Tom said on 2026-11-04 to open the pull request once the two rounds then
running ended: below that, each family is a mechanism of its own.

#677 decided to leave the first family, and to take it up again if a user
meets it. Widened past `G`, decision 8 would merge walls the kernel can tell
apart: the part wrong by up to ten thousand tolerances where the step is
refused or left `Unverified` today, and the campaigns, held to the
arithmetic, would count it a failure still. Building the two nearly tangent
crossings is the answer, a mechanism of its own whose first attempt, the
shared rays, broke 31 held seeds. To fall in the family, the same ellipse is
drawn again turned 1e-7 to 1e-4 of a radian, or moved 1e-6 to 5e-4: a hand
lands far above that, a typed value repeats the first exactly, and a solver's
residual stays far below. The draw makes such near copies on purpose, far more
often than a drawing does, and the ellipses draw stays above its bar.

## #577: two walls at any angle

#577's lane 2 builds two cylinders or elliptic walls at a skew angle
(`docs/exact-kernel.md`, "Two walls at any angle"), and ships before the
cones as a pull request of its own. Before it, `main` declined every such
pair where their faces could meet, and the scope rule excused it; the
campaigns now hold those cases to every rule, and a case `main` declined
is a regression only where the branch answers it with a wrong body — a
decline against a decline is none, by the user's decision.

Five rounds of the walls draw (`random_walls_at_any_angle.rs`), and of the
draws already touching walls, each on fresh seeds for 120 seconds, `main`
measured on the same seeds:

| Round | What changed | Walls, exact | Walls, body |
| --- | --- | --- | --- |
| 1 | the walls a sliver past decision 11, at a touch along a stretch, declined | 6.3 ‰ | 6.8 ‰ |
| 2 | that decline only where faces may meet | 6.3 ‰ | 7.0 ‰ |
| 3 | every pair under a sine of 2e-7 that meets declined | 3.4 ‰ | 3.6 ‰ |
| 4 | walls closer than a chord's sag declined, the scope not yet excusing it (6.1 ‰ of those declines); a conic seen on a skew wall | 9.0 ‰ | 8.5 ‰ |
| 5 | the scope reading `closer_than_a_sag` | 2.9 ‰ | 2.2 ‰ |

### The last round, against `main`

| Draw | Cases | `main` | Branch | Branch only | `main` only | Branch, ‰ |
| --- | --- | --- | --- | --- | --- | --- |
| walls, exact | 10 266 | — | 30 | — | — | 2.9 |
| walls, body | 28 211 | — | 61 | — | — | 2.2 |
| square | 31 985 | 1 | 1 | 0 | 0 | 0.03 |
| crossed ellipses | 1 668 | 1 | 1 | 0 | 0 | 0.6 |
| tilted ellipses | 7 505 | 9 | 18 | 9 | 0 | 2.4 |
| slanted | 12 324 | 9 | 10 | 1 | 0 | 0.8 |
| tilted | 14 029 | 26 | 28 | 2 | 0 | 2.0 |

`main` has no walls draw. No seed `main` holds breaks on any draw. Every
seed only the branch fails, `main` declined as unsupported. Of the twelve,
seven the branch declines too, as `Unverified` or as unsupported
(81295817220299, 81295817220588, 81295817221510, 81295817221781,
81295817222522, 81295817223551, 78885331430629); four it answers with a
wrong body — 81295817222352 (`Closed`), 81295817223557 (`Uncrossed`),
81295817224067 (`Listed`) on the tilted ellipses, 78885331429875
(`Closed`) on the tilted draw — and one, 5622792629473 on the slanted
draw, fails the campaign where the step-by-step reading holds it. The
walls draw stays above one in a thousand.

### What stays above the bar

The families are on #707, their shrunk seeds under `#[ignore]` in
`random_walls_at_any_angle.rs`. Of the walls draw's 30 in the last round,
24 are wrong bodies and 5 declines of another reason; 20 hold a pair
nearly parallel in #552's band, 11 one nearly square.

- **Wrong bodies at a skew pair beside a third wall** — `Closed`,
  `Uncrossed`, `Volume`, `Listed`, 24 on the walls draw. A post bored
  square on its flank, then by its radius at 45° (86751972230147, the
  cap's mesh left open — fixed: a wall's face spanning more than a turn
  without winding had its hole laid a turn off, in the turn where the face
  does not hold it); an elliptic shaft with a skew bore and a skew
  post (86751972219231) and a post cut by an elliptic bore and a turned
  one (86751972224624), their triangles crossing; a bore 1e-3° off square
  to a shaft beside one at 30° (86751972230062), held at square and at
  0.1°; two posts of one radius at 45°, each a hair off, the triangles
  enclosing 2 689.80 where the body holds 2 696.04 (86751972229989), held
  at every other angle tried and with either hair alone — fixed: `folds`
  flipped a diagonal on a round face across two steps of the grid, its
  chord sagging four times the tolerance; a diagonal whose middle stands
  further from the surface than the tolerance, and than the old diagonal's
  middle, is no longer taken. A bound on its span in angle, half again the
  widest side's, was tried first and left folds on five seeds `main`
  holds.
- **A triple contact** — a pocket touching a block's side along a line
  and a bore at 75° reaching that very line (106731298824406). A conic
  seen on a skew wall a hair beside its plane, and a meet on a plane a
  hair beside one of its walls, are now traced through the other wall;
  the corners there are then left `Unverified`. A pocket moved 1e-7 off
  the side holds.
- **A tie near a touch** — a square pair standing two or three
  tolerances from a touch or a node, beyond decision 2's reach, and a
  third wall at a skew angle passing there (86751972220241,
  86751972222643), left as a tie; both hold without the hair. Whether
  decision 2 should reach `G` for walls, as decision 8 does, is open.
- **Declines of another reason** — `Unverified` and `Tie` where `main`
  declined as unsupported: five on the walls draw, and most of the twelve
  above.
- **Nearly parallel walls**, declined rather than built: past decision 11
  by a sliver, within the tolerance along a stretch, or closer than a
  chord's sag, where the two walls' grids, a hair turned, cross their
  triangles — some built by luck at one lean and not the next. #706 is to
  sample them on one grid and build them.

### Lane 3: a cone at any angle

Lane 3 builds a cone against a cylinder, an elliptic wall or a cone of
another axis (`docs/exact-kernel.md`, "A cone against another wall"),
where `main` declined every such pair. One replay, the seeds and the
counts of a 120-second round on each draw, `main` replayed on the same
seeds; "new" is a seed the branch fails and `main` holds — a decline,
there, every time:

| Draw | Cases | Declined as unsupported, `main` → branch | New failures | Gone |
| --- | --- | --- | --- | --- |
| walls, exact | 853 | 529 → 232 | 9 `Uncrossed`, 1 `Closed`, 1 `Answers` | 0 |
| walls, body | 1 337 | 797 → 349 | 13 `Uncrossed`, 1 `Answers` | 0 |
| slanted | 3 565 | 698 → 158 | 1 `Closed`, 1 `Answers` | 0 |
| tilted | 5 917 | 568 → 133 | 2 `Closed`, 1 `Answers` | 0 |
| turned | 6 701 | 34 → 34 | 0 | 0 |
| tilted ellipses | 5 986 | 114 → 111 | 0 | 0 |

On the walls draw about one case in a hundred is now wrong where `main`
declined it — 14 of 853 on the exact kernel, 16 of 1 337 through the
body, `main`'s own failures included. The bar of one in a thousand is not
met, and #577 stays open until #707 and #710 land. Two of the `Answers`,
93075255850138 and 122431270514532, are no cone's: their shrunk cases fail
on `main` too, which declined the cone step before reaching them.

What is left, by family:

- **Triangles crossing on a cone face away from its apex** (#710) — in
  `(θ, l)` a segment running along `l` across `θ` is no chord where the
  radius changes along it, and a sliver beside a hole, inside one column,
  crosses the wall the hole is cut by. Walls, exact: 93075255850034,
  93075255850115, 93075255850150, 93075255850326, 93075255850400,
  93075255850402, 93075255850476, 93075255850694, 93075255850884. Walls,
  body: 25875769009446, 25875769009839, 25875769009854, 25875769009915,
  25875769010058, 25875769010076, 25875769010112, 25875769010597,
  25875769010603, 25875769010631, 25875769010654, 25875769010704,
  25875769010757. 25875769009446 is a bore of nearly the cone's radius
  crossing it at a shallow angle, which `grazing` does not read.
- **Closed or refused beside a tangent or a hair** (#707) — a cone face
  bounded by a conic and the curve of a bore tangent to the plane of that
  conic, their chords crossing at the vertex (93075255850420); a cone's
  rim touching the body's wall within the tolerance (122431270514586 —
  fixed: two samples of a meet a few tolerances off its end vertex, out
  of order, are now merged into either end within a hair; only into an
  end, since merging any two samples that close drops the grid angle of one
  of two walls a hair apart, and the face is left uncut, 25875769010204); a
  ruling crossing a curve sampled coarse on a wide wall (28743870519278 —
  fixed: a sample of a conic lying on a parallel wall the conic's wall
  crosses was withheld as crowded; it stands on the line both walls take);
  a cone added a hair from a cylinder (28743870520828); a cone stepping
  1e-8 off a coaxial cylinder's rim, crossed by a wall at 60°, left
  `Unverified` (28743870518015); a body the application does not draw
  beside a near touch `grazing` misses (25875769010357).
- **A hole in a pointed cone's flank** (#711) — declined as unsupported
  (`on_a_pointed_face`): in `(θ, l)` the apex is a line, and the fan from
  it beside the hole lays two edges along one ruling.
- **A cone and a wall closer than a chord's sag** (#706) — declined
  (`grazing`) until `tessellation/contact` samples a cone in common with
  the other wall.

### #706: two cylinders closer than a sag, an attempt set aside

#706 tried to build the pairs lane 2 declines closer than a chord's sag
when both walls are cylinders, and was not merged: `main` keeps the
decline `closer_than_a_sag` for every pair. The attempt lives on the
branch `wip/706-crossing-line-and-targeted-declines`, and the commits named
below are on it: both are local to one machine and are not pushed, since
the branch's tip holds red tests and every pushed branch runs the CI. The
reasoning is carried by #706's comments. The cases it
reached stand as ignored tests in `random_walls_at_any_angle.rs`, to be
built uncrossed once the two walls share one order beside the line they
cross along; the cases it still declined run there against the decline
`main` gives.

**What it did.** It lifted the decline for two cylinders, kept it for an
elliptic wall and a cone, and sampled the line two such cylinders cross
along at its two ends alone, so that neither wall fanned across it. A band
wall was read as a parallel one where a wall ends on a cap. Where that was
not enough it declined again, as unsupported: where a third wall really
meets the pair inside the box they stand closer than a sag in, and where
one wall of the pair runs through a cap the other ends on at a slant.

**How it was measured.** Each round ran all eight draws, the branch for
120 seconds per draw and `main` (0516b97f) on exactly the seeds the branch
reached, compared seed by seed. A machine loaded by other campaigns is why
the counts are lower than in the rounds above.

**The first round**, at 2538a951 and again at be996413, from seed
318277455100000, failed the same seeds on both commits. On the walls draw
10 seeds `main` declined were answered with a wrong body — 4 on the exact
kernel and 9 through the body, 3 of them on both, `Uncrossed` but for one
`Closed`. Shrunk, each was two cylinders of one radius nearly coaxial,
leaning 1e-4° to 0.05°, most beside a third wall at 30° to 67°, one a
shaft and its bore alone. The targeted declines above came from it, and
the scope rule excused two cylinders closer than a sag whenever a third
leaf met their shared box. An intermediate tip, eee422fa, declined a tube
cut by a post of its outer radius and a shaft with such a post added,
unexcused — refusals on 7 of the first round's seeds and 16 of the fresh
ones below; none was left at 0a81ac0f.

**The last round**, at 0a81ac0f: the first round's seeds again, and a
fresh base, 331906274200000, on the case counts eee422fa reached in 120
seconds.

| Draw | Cases | `main` | Branch | Branch only | `main` only | Branch, ‰ |
| --- | --- | --- | --- | --- | --- | --- |
| walls, exact | 2 449 | 32 | 32 | 0 | 0 | 13.1 |
| walls, body | 8 424 | 94 | 94 | 0 | 0 | 11.2 |
| square | 25 730 | 0 | 0 | 0 | 0 | 0 |
| crossed ellipses | 927 | 2 | 2 | 0 | 0 | 2.2 |
| tilted ellipses | 3 407 | 5 | 5 | 0 | 0 | 1.5 |
| slanted | 2 732 | 11 | 10 | 0 | 1 | 3.7 |
| tilted | 7 005 | 19 | 19 | 0 | 0 | 2.7 |
| turned | 6 122 | 1 | 1 | 0 | 0 | 0.2 |

| Draw, fresh seeds | Cases | `main` | Branch | Branch only | `main` only | Branch, ‰ |
| --- | --- | --- | --- | --- | --- | --- |
| walls, exact | 7 166 | 98 | 98 | 0 | 0 | 13.7 |
| walls, body | 22 683 | 262 | 264 | 2 | 0 | 11.6 |
| square | 50 463 | 4 | 4 | 0 | 0 | 0.08 |
| crossed ellipses | 2 148 | 2 | 2 | 0 | 0 | 0.9 |
| tilted ellipses | 8 927 | 14 | 14 | 0 | 0 | 1.6 |
| slanted | 8 904 | 37 | 37 | 0 | 0 | 4.2 |
| tilted | 16 232 | 56 | 56 | 0 | 0 | 3.5 |
| turned | 13 915 | 18 | 18 | 0 | 0 | 1.3 |

The first round's wrong bodies were gone, and no seed failed under
another rule than on `main`. The slanted seed only `main` fails,
318277455102178, is a refusal the branch answered. Two seeds only the
branch failed, both `Uncrossed` through the body where `main` declined as
unsupported: 331906274213776 and 331906274218763, each shrunk to a shaft
and a bore of its own radius nearly coaxial, at 0.01° and 1e-4°, with no
third wall, the triangles crossing on the line between the cap and the
place the two axes cross — the family of the first round's
318277455102885, which held. The first crosses on the exact kernel too;
the second only through the body. The exact round on fresh seeds stopped
before it reached 331906274213776, which is why its row shows no seed only
the branch fails.

**Why it was set aside.** Beside the leaning crossing line, each wall
cuts the cell into triangles of its own. The rules excuse an overlap
thinner than `soundness::NEAR` times the reach as what rounding leaves,
and count two faces lying on each other over an area as a flaw
(`soundness/crossing.rs`). On those two seeds the two faces lie on each
other over an area wider than that margin; on the cases that held, the
overlap stayed thinner than it. The fix reached them by luck rather than
by laying both walls on one order.
The owner kept `main`'s decline on 2026-11-06. Sharing one order needs a
correspondence between the two walls' rays beside the line, a redesign
left for later.
