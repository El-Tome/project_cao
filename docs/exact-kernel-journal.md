# An exact kernel of our own: the two weeks of #498

On `test/498-an-exact-kernel-of-our-own`, a branch that sits on #449's and is
never merged. The answer is written on #497; this is where the days are
accounted for, as #498 asks: what was reached on which day, what took the
time, and whether the failures a campaign finds go down from one run to the
next — the one sign that tells a tail that ends from one that does not.

Two weeks at most, from 2026-09-30 to 2026-10-13. Past that, nothing without
Tom's word.

## The cases

The stock is #449's: a cylinder of radius 20 standing on the XY plane from 0
to 10, and tools of radius 5. The block is 40 by 40 by 10, centred on the
origin, from 0 to 10.

| # | case | from |
| --- | --- | --- |
| 1 | a circle raised | #449 |
| 2 | a hole bored flush, off the axis, at (8, 0) | #449 |
| 3 | a hole bored through, off the axis | #449 |
| 4 | a hole bored flush, tangent to the wall, at (15, 0) | #449 |
| 5 | a hole bored through, tangent to the wall | #449 |
| 6 | a hole bored flush, on the axis | #449 |
| 7 | a hole bored through, on the axis | #449 |
| 8 | a hole bored flush but for a hair, from 1e-7 | #449 |
| 9 | a boss standing on the top | #449 |
| 10 | a boss sunk into the top | #449 |
| 11 | a boss overhanging the top's edge | truck failed it |
| 12 | two blocks side by side sharing a wall, flush or offset | truck failed it |
| 13 | a hole tangent to a side of a block | truck failed it |
| 14 | a cylinder resting against a flat face | truck failed it |
| 15 | two holes whose circles touch | truck failed it |
| 16 | a pocket flush with a side and the top | truck failed it |

## The days

### Day 1 — 2026-09-30

- Branch made from #498 on top of #449's. #449's table runs again as it was
  left: the flats keep every rule on nine of the ten cases, truck on four.
- Four designs of the kernel written independently — a classic boolean made
  consistent by a pool of corners, a plane-based one after Campen and Kobbelt,
  a cell complex of both boundaries, and a risk-first one — and judged by three
  readers against the sixteen cases, the harness and the product, before any
  code. The three judges agreed on the decisions and disagreed on the base, so
  the synthesis took the decisions, not a design: one relative tolerance and
  each fact decided once and read back as a symbol; the boolean as one overlay
  per surface with a winding rule; the scope cut to planes and parallel or
  perpendicular cylinders, anything else declined and counted; an oracle for
  the campaign written from the leaves by arithmetic, apart from the kernel.
  [`exact-kernel.md`](exact-kernel.md) has it.
- The vocabulary every lane builds against is frozen and committed: canonical
  surfaces and curves, the arenas of the topology, the listing, and a curve's
  trace in a surface's parameters.
- Five lanes started in parallel, each with an adversarial reviewer: the
  geometry of pairs (relations, the perpendicular curve), the body (raise,
  exact volume, point in face), the triangles, the overlay, and the harness
  (the listing's check, the analytic promise, the campaign, the table).
- What took the time: the decision that a whole circle is an edge with no
  vertex — as Parasolid holds it, and as the application will want to pick
  it — against the literal "its two vertices" of #498. A phantom vertex on a
  circle would be something the user can click on and that means nothing.
