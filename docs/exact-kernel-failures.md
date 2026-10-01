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
cure — the band decided once, as one decision, rather than pair by pair — and
it is not written yet.

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
