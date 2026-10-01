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
- The five lanes came back and were merged, each reviewer having found and
  fixed real defects in its lane — a tangency of parallel cylinders a hair
  off one axis, two walls a hair apart taken for one surface, a hair of side
  leaving a tangent circle ordered by rounding, a listing that passed with a
  face bounded by nothing. Row 1 of the table keeps every rule on the exact
  kernel, its exact volume the arithmetic's to the last digit printed.
- The boolean landed, and so did the triangles of the curve two perpendicular
  cylinders meet along. **Every row of the table keeps every rule on the exact
  kernel** — all sixteen cases, with 11 and 12 in both their variants — with
  its exact volume the arithmetic's to the digits printed and each boolean
  under a millisecond. The last row to turn was the cylinder resting on a
  face: the kernel was right, and the listing's check read the slit the
  contact leaves in the block's top as a hole turned the wrong way.
- Campaign 0, five minutes from seed 1, as a first look: 69 256 cases, 1 338
  broke a rule (1.9 %) — Volume 533, Answers 523, Uncrossed 194, Closed 73,
  Listed 15. The shrunk cases pointed mostly at the triangles, not at the
  exact body, so the exact body's own crossings along every line are now a
  rule of their own (Spans), held before any triangle is laid; and every
  failing seed is named, so that a campaign's failures can be sorted into
  distinct ones afterwards.
- Campaign 1, the hour #498 asks for, started at 21:06 from seed 1 000 000
  on the kernel as it stands. Meanwhile two lanes fix what campaign 0
  shrank, one in the kernel, one in the triangles.
- Campaign 1 ended at 22:06: 51 298 cases, 980 broke a rule (1.91 %) —
  Answers 384, Volume 375, Uncrossed 156, Closed 47, Listed 18, **Spans
  none**. Whenever the kernel answered, its exact body held along every line
  what the arithmetic promised, to a millionth of the reach; every volume
  found wrong was the triangles'. A probe over three hundred cases checked
  that this is not a check that never runs: the exact crossings answered on
  every one of 686 592 lines. The campaign ran seventeen times slower than
  campaign 0, that check being the cost — one ray against every face, 2 304
  lines per body.
- A sample of each rule of campaign 1, 138 seeds, was shrunk and sorted by
  five readers, one per rule, each cause checked by a probe, then by a sixth
  merging what one cause shows under several rules: **nineteen distinct
  failures**, ten the kernel's, eight the triangles', one the harness's.
  [`exact-kernel-failures.md`](exact-kernel-failures.md) names them.
- The two fixing lanes came back and were merged at 23:57, each reviewer
  having confirmed and fixed more than it was handed. Run again, 107 of
  campaign 1's 138 shrunk seeds hold: twelve of the nineteen failures are
  gone entirely, six in part, one not at all. The sixteen cases stay green.
- Campaign 2 started at 23:57 from seed 2 000 000 on the merged kernel, for
  an hour, the same draw and the same rules as campaign 1.
- Not reviewed as written: the reviewers of the boolean and of the
  perpendicular triangles were cut short by the account's spending limit;
  the fixing lanes' reviewers read the same code again.
- What took the time: the decision that a whole circle is an edge with no
  vertex — as Parasolid holds it, and as the application will want to pick
  it — against the literal "its two vertices" of #498. A phantom vertex on a
  circle would be something the user can click on and that means nothing.

### Day 2 — 2026-10-01

- Campaign 2 ended at 00:57: 49 659 cases, **188 broke a rule (0.38 %),
  against 1.91 % for campaign 1** — five times fewer. Answers 161,
  Uncrossed 20, Listed 4, Closed 3; no Volume where campaign 1 had 375, and
  still no Spans. What remains is mostly the kernel declining.

| campaign | kernel | cases | broke a rule | per thousand |
| --- | --- | --- | --- | --- |
| 1 | 30 Sept., as first written | 51 298 | 980 | 19.1 |
| 2 | after round 1 | 49 659 | 188 | 3.8 |
| 3 | after round 2 | 298 949 | 370 | 1.24 |

- Campaign 2's sample of 67 was sorted: fifteen distinct failures, seven of
  campaign 1's still open, eight new — and every new one a configuration a
  round-1 fix made hold, moved by a hair. That is the tail #447 feared, seen
  arriving: each exact degeneracy fixed has a near one behind it.
- Round 2 was cut twice by the account's spending limit and resumed in the
  same worktrees. The kernel took a sixth decision, arc identity, and widened
  the refusals of decisions 3 and 5; the triangles took common sampling to the
  cases round 1 left. Merged at 02:59; two cases the lanes met on went back to
  the kernel declining and wait on it, with their diagnosis. Campaign 2's
  sample run again: 39 of 67 hold.
- Campaign 3, from seed 3 000 000 on the kernel after round 2, with no lane
  running beside it and the exact rays now skipping the faces a line's box
  misses: **298 949 cases, 370 failures, 1.24 per thousand**. Still no Spans.
  Volume came back, 13, the triangles' again. The fall slows: a fifth, then a
  third.
- Tom, asked what the 3D part should hold: after the sketch come all the 3D
  tools — revolutions with arcs and slanted traits, fillets and chamfers
  including where fillets meet at a corner, shell, draft, loft between
  profiles; not sweep along a path, not planned. Then assemblies with their
  own constraints, essential but not framed yet. Told that #497 names shell,
  draft, loft and corner fillets as what this road reaches late and
  OpenCascade on day one, he chose to stay on this road: some tools missing at
  first is not dramatic. The answer on #497 will say so.
- Round 3, from 11:30 to 20:00: campaign 3's sample of 125 sorted into
  twenty-one distinct failures, fifteen known and six new; then a kernel lane
  and a triangles lane, each reviewed. The agents were cut and relaunched
  several times by the spending limit, which is most of why it took
  eight hours; each relaunch picked up its own worktree. The kernel now merges
  faces across seams — the listing loses the edges a join left inside a face
  — and moves surfaces decided to touch onto the touch once. Run again, 84 of
  campaign 3's 125 shrunk seeds hold. One case the triangles lane fixed went
  back to the kernel declining at the merge, as in round 2.
- New distinct failures per campaign: 19, then 8, then 6. Each new one is a
  known configuration moved by a hair, or the harness catching up with a new
  decision. What stays open is one family, the tangency band.
- Campaign 4 started at 20:05 from seed 4 000 000 on the kernel after round 3.
