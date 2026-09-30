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
| 1-1 | Two parallel cylinders touching along a line: the grid step at the contact is withheld from every circle of both, far from the contact too | triangles | Volume, Closed | 27 % | open |
| 1-2 | A plane tangent to a cylinder a hair from a line crossing that plane: corners are put on the circle by distance, leaving a two-sided face thinner than the tolerance | kernel | Answers, Listed, Closed | 14 % | open |
| 1-3 | Two parallel cylinders crossing, on a grid angle or at a grazing angle: the step at the crossing is withheld from every circle of both | triangles | Volume, Closed | 13 % | open |
| 1-4 | A vertex partway up a cylinder wall no circle of the wall is sampled at: the triangle from it sags through, or lies on, the plane face meeting the wall there | triangles | Uncrossed | 11 % | open |
| 1-5 | Two perpendicular cylinders tangent inside, the curve a figure of eight: its node is cut into the curve at its first passage only | kernel | Answers | 8 % | open |
| 1-6 | The tolerance grows with a later leaf's reach: a gap an operand already holds apart falls under it, and its own lines and corners are merged across it | kernel | Uncrossed, Answers, Closed, Listed | 6 % | open |
| 1-7 | Winding rays cast for a region neither operand covers, from a point that lands on a face of the other | kernel | Answers | 5 % | open |
| 1-8 | Identity by distance chains through a third surface or line: three stand pairwise within the tolerance but not all together | kernel | Answers, Listed | 4 % | open |
| 1-9 | Two parallel cylinders of one radius a few tolerances apart, crossing at a grazing angle near a third surface | kernel | Answers, Listed | 3 % | open |
| 1-10 | Perpendicular cylinders tangent inside with a radius not exact in binary: the snap moves the curve's cylinder by a rounding, and the trace no longer knows its surface | kernel | Answers | 1.3 % | open |
| 1-11 | Two operands flush on two walls, each offset by a hair under the tolerance: their common edge and corner are not derived again | kernel | Answers | 1.3 % | open |
| 1-12 | A perpendicular cylinder whose cap is tangent to the other: the curve touches the cap's rim to the fourth order, and angle and curvature both tie | kernel | Answers | 1.3 % | open, declined by design |
| 1-13 | Two holes tangent at a point of a wall through both axes: the relations of a support are completed once rather than to the end | kernel | Answers | 1.3 % | open |
| 1-14 | A circle prism's wall holding the seam and a second loop: the outer loop is laid a whole turn away from its holes | triangles | Closed | 1.1 % | open |
| 1-15 | A circle printed on a face, internally tangent to another, its tool's cylinder gone from the body: no common sampling | triangles | Closed | 0.8 % | open |
| 1-16 | A corner of a face within a chord's sag of a curve of that face, but off it: the chord passes it on the wrong side | triangles | Closed | 0.6 % | open |
| 1-17 | Two cylinders sampled in common whose rays from vertices and from perpendicular curves go to one of them only | triangles | Closed | 0.6 % | open |
| 1-18 | Two circles of one radius a hair apart on one face, never sampled in common | triangles | Closed | 0.3 % | open |
| 1-19 | The listing's check misjudges the turn of a true face only a few tolerances across | harness | Listed | 0.3 % | open |

Three families stand out. Near-tangent pairs with a third surface near them
(1-2, 1-9) are one pattern: a tangency decided once as a line, then measured
again by distance near it. Identity by distance that never checks the support
it builds (1-6, 1-8, 1-11) is decision 5's refusal, written in the design and
nowhere in the code. And the triangles' contact rule withholds steps from
whole cylinders rather than near the contact (1-1, 1-3), which is two fifths
of the campaign by itself.
