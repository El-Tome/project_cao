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
`..._in_the_triangles.rs`.

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
