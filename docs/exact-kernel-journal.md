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
| 4 | after round 3 | 279 739 | 169 | 0.60 |
| 5a | after round 4 | 259 995 | 172 | 0.66 |
| 6a | after round 5 | 282 219 | 90 | 0.32 |
| 7a | after round 6 | 275 705 | 69 | 0.25 |

And on the profile draw, which adds rounded rectangles, slots and rings:

| campaign | kernel | cases | broke a rule | per thousand |
| --- | --- | --- | --- | --- |
| 5b | after round 4 | 158 854 | 1 069 | 6.73 |
| 6b | after round 5 | 174 256 | 310 | 1.78 |
| 7b | after round 6 | 170 352 | 191 | 1.12 |

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
- Campaign 4 ended at 21:05: 279 739 cases, **169 failures, 0.60 per
  thousand** — half campaign 3's. Uncrossed 80, Answers 77, Closed 7,
  Volume 3, Listed 2; still no Spans. Its sample of 72 is being sorted, and
  round 4 — the tangency band decided once, as decision 7 — is running.
- Campaign 4's sample of 72 sorted: fifteen distinct failures, nine known,
  six new. Seven tenths of the failures are the tangency band; the heaviest
  new one, a seventh, is round 3's own: moving a cylinder onto one touch
  breaks another it holds. New distinct failures per campaign: 19, 8, 6, 6.
  The rate keeps halving; the count of new causes no longer falls. Each
  decision added cures a family and opens a narrow one beside it.
- Tom, asked whether 0.60 per thousand is low enough to go on: yes — and the
  draw is to widen to what the application makes, rounded rectangles, slots
  and tubes, a lane started on it.
- Round 4, beside campaign 4: the band decided once, decision 7. A region
  standing within the tolerance of another surface all across is wound by
  the faces lying over it as the arena decided the pairs, never by a ray.
  Taken as the face's twin and covered by it, as first asked, it broke the
  closure of 23 named findings; wound beside it, it breaks none. Of the
  29 seeds of the band still failing after round 3, two hold as drawn and
  two more as shrunk; the Uncrossed nineteen are a skin of a cusp the band
  keeps as the exact geometry has it, in strips that are no regions, which
  a rule on regions cannot drop.
  Measured, decision 7 changes little: on campaigns of two and five minutes
  it answered for a handful of regions and fixed no seed. What the band
  leaves is no tie any more but a skin: where a cut crosses a tangency a
  hair from the touch, the cusp of matter left is thinner than the rules'
  NEAR, exact and right — the Spans rule holds — yet read by the triangles'
  rules as two faces laid on each other. Dropping it means the kernel cutting
  the matter at the band's edge, with edges there: a step of design, not a
  rule more.
- The draw widened, as Tom asked: rounded rectangles, slots and rings beside
  rectangles and circles, with their arithmetic along every line, as a
  second campaign; the square draw is untouched so campaigns stay
  comparable. Its first two minutes: **15.6 failures per thousand**, as many
  as campaign 1 — and 75 of its 80 are one configuration, two cylinders of
  one radius whose axes stand a hair apart: a slot's cap or a rounded corner
  on a hole of the same radius. The square draw almost never made it; the
  shapes a part is made of make it all the time. All of them are the kernel
  declining, none a wrong answer.
- Campaigns 5a (the square draw, seed 5 000 000) and 5b (the profile draw,
  seed 5 500 000) started at 22:46 on the kernel after round 4, an hour each.
- Round 5 started: decision 8, two parallel cylinders of one radius whose
  axes stand within a hundred tolerances are one surface — the kernel's own
  fuzzy tolerance, kept to the one pair whose crossing is ill conditioned;
  two parallel planes a hair apart never cross and keep their skin — and the
  moves onto a touch round 3 made, which campaign 4 found breaking a second
  touch.
- Campaigns 5a and 5b ended at 23:45. The square draw holds where campaign 4
  left it, 0.66 per thousand: round 4 changed little, as it had measured
  itself. The profile draw's first hour: 6.73 per thousand — Answers 622,
  Uncrossed 389, Closed 41, Volume 11, Listed 6, and still no Spans: the
  kernel is never wrong along a line, it declines or its triangles cross.

### Day 3 — 2026-10-02

- Round 5 merged at 00:50. Decision 8: two parallel walls of one radius,
  one of each operand, their axes within twenty tolerances, are one wall,
  and the second operand moves onto it with what it built on it. The hair
  was measured before it was set — ten, twenty, fifty, a hundred and a
  thousand tolerances on the same campaigns — and set at twenty by the
  review, which found a line crossing the moved wall at a slant seeing a
  move of fifty: twenty is what no line the rules hold can see. On
  five-minute campaigns of the profile draw, failures fell from about 6.8
  per thousand to about 2. The moves onto a touch now gather every touch a
  surface holds before moving it (4-1), and never move the first operand's
  wall (4-2). The triangles' lane fixed its named cases, with no change a
  campaign could see.
- Campaigns 6a and 6b started at 00:53 on the kernel after round 5, an hour
  each, the square draw from 6 000 000 and the profile draw from 6 500 000.
- Campaigns 6a and 6b ended at 01:53. The square draw halves again, **0.32
  per thousand**; the profile draw falls fourfold, **1.78 per thousand**,
  decision 8 holding over an hour what five minutes had shown. Still no
  Spans on either: in six campaigns and some 1.6 million cases, the exact
  body has never been found wrong along a line.
- Campaign 6's samples sorted: nine failures in ten on both draws are the
  tangency band. Round 6 takes it as the heuristic this repository learned
  the hard way asks: every case of the band gathered as a test first, from
  every campaign and every finding left ignored, and only then the rule —
  the band's lines and corners laid on every surface of it, the change
  round 4 tried behind a switch and set aside for breaking seven findings
  it had not gathered.
- Round 6, the band. Its cases gathered first, as a test file of their own:
  157, of which 36 held. Then the rule, decision 9: the band of two surfaces
  decided to touch laid out once, the same on both — its corners on both,
  a line through each along the line of touch, a corner wherever a surface
  square to the band crosses one of its lines. Round 4's switch, run again,
  broke five findings; the lines through the band's corners are what it
  lacked. Three guards came out of the first campaigns on the rule, each
  with the case that found it. At the end, 113 of the corpus's 163 hold, and
  five-minute campaigns run side by side against the kernel before the round
  halve the profile draw's failures (31 to 15) and leave the square draw one
  (4 to 1), none made. What stays is mostly bands of pairs not decided to
  touch: a plane a hair inside a wall, two walls of one radius crossing at
  a grazing angle.
- Round 6's review. The rule held every shape built for it, its own and
  ten more, at thirty-four hairs and three orientations, and broke none;
  but three campaigns of the profile draw, longer than the round's, found
  it making failures where it fixed many more: two walls of one radius a
  hair beyond decision 8, a plane touching both. Two fixes hold most of
  what it made that a plane's gap or a skin read in the wrong frame
  explains; a third was taken back for breaking four cases that held. On
  some 113 000 profiles the failures go from 187 before round 6 to 112 at
  its head and 107 after the review, none made by the review; the
  crescent's tip under a plane is what is left of what round 6 made.
- Round 6 merged at 05:40, after the cases were gathered first: 197 cases of
  the band in `crates/solid/tests/the_tangency_band.rs` — every campaign-6
  seed of it, every band finding left ignored, five findings round 4's
  switch had broken, and nine shapes built at hairs from half a tolerance to
  two thousand — committed with the failing ones ignored before a line of the
  rule. Then decision 9: the band of two surfaces decided to touch is laid
  out once, the same on both — its corners, a line through each along the
  touch, the corners where a surface crosses those lines — so every strip of
  it is bounded by arcs both surfaces carry and decision 6 takes the twins as
  one. What round 4's switch lacked was the line through each corner. 132 of
  the 197 hold. On campaigns of a quarter of an hour on the same seeds, the
  square draw went from 18 failures to 9, none made; the profile draw from
  187 to 107 (1.65 to 0.95 per thousand), 93 fixed and 13 made. The review
  tried one more rule, a node of the band taken for the corner it falls on,
  saw it break more than it held, and took it back with the cases that say
  so.
- Campaigns 7a and 7b started at 05:45 on the kernel after round 6.
- Campaigns 7a and 7b ended at 06:45: **0.25 per thousand on the square
  draw, 1.12 on profiles**. Over the seven campaigns, some two million random
  parts; the exact body was never found wrong along a line.
- Tom's word, on the morning of day 3: every criterion of #498 is met, the
  maquette stops here, and #499 comes next. The answer was written on #497
  (https://github.com/El-Tome/project_cao/issues/497#issuecomment-5948313733):
  a kernel written here holds, on planes and cylinders; the hardest part is
  near-coincidence, the tangency band; the rest reads as weeks for the next
  tools and months for shell, loft and corner fillets. Tom also said what
  matters most: he cannot check a kernel by reading numbers. The answer
  says how anyone can run them again, and the check that counts will be the
  application's, once #499 puts this kernel behind `Body`.
- Three days of the fourteen were used.

## After the maquette: #526 puts the kernel behind the application

### 2026-10-05

- Campaigns 8a and 8b, the hour each draw has always had, run on the branch
  of #526 and, beside them on the same seeds, on `c778b20` — the kernel as
  #498 left it, ported to `main` before #526 changed anything — each tree
  built in its own target directory: the square draw from seed 8 000 000,
  the profile draw from 8 500 000. A third hour, 8c, ran the profile draw
  through the application's body (`cao_solid::Body`: face numbers, the
  drawing as it builds, its declines), from 8 500 000 too. Six campaigns
  shared the machine, so each tried about a third of what an hour tried
  alone on day 3.

| campaign | kernel | cases | broke a rule | per thousand |
| --- | --- | --- | --- | --- |
| 8a | `c778b20`, square draw | 88 775 | 14 | 0.16 |
| 8a | #526, square draw | 88 833 | 14 | 0.16 |
| 8b | `c778b20`, profile draw | 54 256 | 55 | 1.01 |
| 8b | #526, profile draw | 54 104 | 55 | 1.02 |
| 8c | #526, profile draw through the body | 89 821 | 111 | 1.24 |

- **#526 made no failure and fixed none on the kernel.** On the seeds both
  trees reached, the same seeds fail, each breaking the same rule, and each
  shrinks to the same case, to the last digit. Square draw: Uncrossed 9,
  Answers 5. Profile draw: Answers 30, Uncrossed 22, Volume, Listed and
  Closed one each. Still no Spans.
- Every one of the 69 shrunk cases is the tangency band: two surfaces a hair
  of 1e-8 to 1e-5 from touching or from being one. Sorted by hand, about a
  third of the square draw's and half the profile draw's are two walls of
  one radius whose axes stand a hair apart, beyond decision 8's; the rest a
  plane a hair from a face, a wall or a line of touch. None outside the
  band.
- Through the body, the profile draw fails 1.24 per thousand rather than
  1.02, though it is held to fewer rules: neither to the listing nor to
  Spans, which read the kernel underneath. On the seeds 8b reached, 5 fail
  only through the body, all Uncrossed, and 2 only on the kernel — a Listed
  the body is not held to, and an Uncrossed its finer triangles draw
  right. The 5 are no regression of #526:
  the body draws at a fifth of a thousandth of the reach where the
  campaign's kernel draws at a thousandth, and the kernel of `c778b20`
  drawn that fine crosses its triangles on the same 5, as it fails every
  one of the 111 seeds 8c named. Finer triangles show more of the band; the
  body adds nothing to it. Two seeds the kernel drew open or crossed are
  declined by the body instead, as #526 asks of a body it cannot draw
  whole.
- The part campaign of #448 (`an_undo_gives_back_the_part.rs`), half an
  hour on each tree from seed 8 000 000, started again past every part that
  did not answer within ten seconds: 3 248 parts on #526, 293 broke a rule
  (90 per thousand) — 175 Closed, 118 late — against 296 on the same seeds
  before it. Every Closed of the 175 shrinks to a part with a turn in it,
  the flats' own (#486 to #492). #526 closed five of them, and two parts
  answered late on #526 that answered in time before: both join exact
  matter and then a turn, and take two to three times as long, 13 to 15
  seconds rather than 5 to 8, because the exact body handed to the flats is
  drawn finer than the flats draw themselves. Both keep every rule given
  the time.

- #528's lane of the crescent's tip: two walls of one radius a hair apart
  beyond decision 8's, decided once to graze and their band laid out as a
  line of touch's (`exact-kernel.md`, decisions 2, 5, 6 and 9). 46 of the
  family's 59 shrunk seeds hold, and 15 of `the_tangency_band.rs`'s
  ignored cases and one of the triangles'. Ten minutes of each draw beside
  the branch's head, on the seeds both reached: the square draw 3 failures
  in 14 221 on both; the profile draw 9 in 9 996 before, 5 after; through
  the body 11 in 15 062 before, 7 after. None made. What is left, and the
  merge taken back, in `exact-kernel-failures.md`.

- #528's six lanes merged, measured like for like against the night's
  campaigns: the same three draws from the same first seeds, each stopped
  after the number of cases it tried that night (`CAO_FUZZ_CASES`), so
  that a machine shared three ways rather than six tries the same cases.

| campaign | kernel | cases | broke a rule | per thousand | fixed | made |
| --- | --- | --- | --- | --- | --- | --- |
| 8a | #526, square draw | 88 833 | 14 | 0.16 | | |
| 8a | #528, square draw | 88 833 | 4 | 0.05 | 10 | 0 |
| 8b | #526, profile draw | 54 104 | 55 | 1.02 | | |
| 8b | #528, profile draw | 54 104 | 13 | 0.24 | 44 | 2 |
| 8c | #526, profile draw through the body | 89 821 | 111 | 1.24 | | |
| 8c | #528, profile draw through the body | 89 821 | 26 | 0.29 | 87 | 2 |

- The first measure of the merge made three failures: 8528992, 8515425
  and 8542629, held the night before, failed on both profile draws. The
  lanes' own ten-minute campaigns had not reached them. 8528992 was
  decision 10's move carrying a rounded rectangle's upper corners' walls
  onto its lower ones, a hair below, and is fixed: a move is not made
  where it would lay a surface of the operand on another of its own. The
  other two are the line-of-touch lane's corner on two planes, laid on
  the line they share a hair off a wall: a disc's wall crossing a bore's
  band (8515425), and a disc's rim touching a block's side, whose corner
  had every rim of the wall sampled on the side (8542629). Kept where they
  were found, or laid where the wall crosses the line, those corners
  fixed one or both and broke up to eight seeds the lanes hold; one rule
  held all eight and fixed 8550377 besides, but only for the order the
  walls happen to be listed in. None was kept: the two stay failing,
  their shrunk cases ignored with the reason, and the lane keeps its
  gains (8010303, 8010399, 8055558 and three of the band's).
- What is left, 4 seeds on the square draw, 13 on the profiles and 26
  through the body, is the tangency band still: a third surface a hair
  from a line of touch, the larger part on every draw; a plane a hair from
  a face; two walls of one radius a hair apart under a third surface; and
  a few the triangles draw crossing. Six seeds fail as drawn though their
  shrunk cases hold, and want shrinking again.
- The gate's tests, built, take 23 to 24 seconds where they took 19.6 to
  20 at `b9d4fea`, the base the lanes started from: the crescent's tip's
  cases, ignored there and held now, cost
  about two and a half seconds of it, and the band's newly held cases half
  a second. No test binary that held the same cases got slower.

## 6 October: the turned campaigns of #533

Two hours each, on the exact kernel and through the application's body, on
`01c30e2`: 53 failures in 71 475 cases, and 72 seeds in 13 520. Every one
shrunk and filed by family in `exact-kernel-failures.md`; one seed of each
family is named in `what_the_exact_campaigns_found_in_the_kernel.rs`.

- Most failures through the body, 64 of 72, are the flats turning a hole a
  hair inside its outline, which the exact kernel declines and the
  application hands them: main's sweep, unchanged.
- On the exact kernel the families are those prisms meet: a surface a hair
  from another, an axis leaning a hair, surfaces touching. Two cases of
  the last fail alike with the turned cylinder raised as a prism.
- The one finding of turning's own: the ends of a partial turn, planes
  holding its axis, cutting a cylinder turned about that axis along its
  rulings, declined as a tie (328889).
- An earlier fifteen-minute run, before the commit that counts a wall a
  hair thin as the flats' to turn, is superseded by these.

## 6 October: the night's campaigns of #533

The turned campaigns four hours each on the branch, and the square,
profile and part campaigns on the branch and on a copy of `main` over the
same seeds, all side by side in release.

- The square and profile draws fail the same seeds on both trees, 6 in
  150 000 and 42 in 95 000, shrunk into the same cases: #533 changed
  nothing a prism meets.
- The part campaign found what the solids' campaigns cannot: sixteen
  parts failing on the branch alone, every one a circle drawn across a
  sketch axis, which `main` dropped. Joining its two sides on the flats
  took most of a minute. A whole turn now turns only the sides no other
  side's turn holds, and the circle turns in milliseconds; seven of the
  ten such seeds in the range run again hold. The part-way turns, two
  solids touching along the axis or back to back, stay with the flats'
  join (#418), and a part made round by such a turn joins as slowly
  afterwards as `main` does any round part (#492).
- The turned campaigns' failures fall in the families the two-hour runs
  found: the flats turning a wall a hair thin through the body, the
  kernel's hairs, touches and leaning axes on the exact kernel, each
  sample run again with a prism instead failing alike. One more finding of
  turning's own, a sector a hundredth of a degree wide about a cylinder's
  axis (533603007), joins the partial turn's tie.
- The gate's tests, built, take about 30 s on the branch where they take
  about 23 s on `main`: the turned harness's kernel tests and the part's
  slanted turn on the flats are most of the difference.

## 6 October: round 1 of #533, under one failure in a thousand

Five lanes, each started from the night's families and merged one by one
onto `1492eed`:

- flats-join: a turned profile that touches itself once its levels are
  laid is read as the matter its laid outline bounds, so a hole a hair
  inside its outline becomes a notch, a slot a hair wide closes, and the
  exact kernel turns the section instead of handing it to the flats. This
  was most of the night's failures through the application's body.
- kernel-drawing: sampling decides once where two edges touch without
  crossing, so a face's outline no longer meets itself mid-segment and a
  body drawn through it closes.
- flats-ends: a turn on the flats is closed by its ends whichever way it
  turns, and a turn a hair short of whole is swept whole.
- kernel-declines: three facts the boolean decided twice — a wall taken
  for another a hair off now brings its operand onto it, a node grown
  slides along the shaft it rests on, and two planes a hair apart are read
  where they stand.
- rounded-a-hair: a straight run whose ends are one corner within the
  tolerance is left out of a raise, so a rounded rectangle a hair wider
  than its corners is raised.

Measured on fresh seeds at `4e326a9`, in release, the four draws side by
side, and the first 15 000 seeds of each run again on `1492eed` beside
them:

| Draw | First seed | Cases | Failures | Per thousand | `1492eed`, first 15 000 |
| --- | --- | --- | --- | --- | --- |
| turned, exact kernel | 534500000 | 30 000 | 12 | 0.40 | 0.47 |
| turned, application's body | 534600000 | 30 000 | 21 | 0.70 | 6.33 |
| square | 534700000 | 30 000 | 3 | 0.10 | 0.07 |
| profiles | 534800000 | 20 000 | 6 | 0.30 | 0.33 |

- Every draw is now under one failure in a thousand. No seed that holds
  on `1492eed` fails at `4e326a9`; through the body, 83 of the base's 95
  failures over the shared seeds hold.
- What is left, not yet shrunk into families: results drawn crossing
  (`Uncrossed`), most of the failures on every draw, and cases refused or
  too slow (`Answers`), ten of the 21 through the body, run with up to
  eight campaigns sharing the machine.
- The gate's tests, built, take about 30 s; the turned harness about
  2.9 s of it, still above its 2.5 s budget.

## 8 October: round 1 of #536, measured

Three lanes merged onto `3286cee` (a slanted section that touches itself,
a cone's triangles, a cone's boolean), measured at `2e1e3ec` on fresh
seeds, in release, the seven draws side by side, and the first 15 000
seeds of each slanted draw run again on `3286cee` and on the tip:

| Draw | First seed | Cases | Failures | Per thousand |
| --- | --- | --- | --- | --- |
| slanted, exact kernel | 5365100000 | 40 000 | 35 | 0.88 |
| slanted, application's body | 5365200000 | 40 000 | 42 | 1.05 |
| turned, exact kernel | 5365300000 | 30 000 | 21 | 0.70 |
| turned, application's body | 5365400000 | 30 000 | 15 | 0.50 |
| square | 5365500000 | 30 000 | 2 | 0.07 |
| profiles, exact kernel | 5365600000 | 20 000 | 9 | 0.45 |
| profiles, application's body | 5365700000 | 20 000 | 7 | 0.35 |

- Over the slanted draws' first 15 000 seeds the exact kernel goes from
  3.87 failures in a thousand at `3286cee` to 0.93, the application's
  body from 4.87 to 1.13. No seed there that holds on `3286cee` fails on
  the tip.
- The fresh seeds found three that do, each run again on every commit of
  the round, each named and ignored in
  `what_the_exact_campaigns_found_in_the_kernel.rs`. A pointed cone cut
  away by a cone of its apex a hair wider is refused from `eba1fa2` on
  (5365211513), with both tips on one apex too. Taking a cone about an
  axis a hair off, from `d961d6d` on, leaves a slot's round end 1e-5
  off a cone's axis open along a ruling (5365114896), and refuses a shaft
  ending on a band a hair long beside a quarter cone (5365205368), whose
  shrunk case fails on `3286cee` too.
- Every other failure, run again on `3286cee`, fails there too. The
  turned, square and profile draws' are prisms' and straight turns'
  families, those of #533. The slanted draws' are, by their shrunk
  cases: no cone at all; a partial turn of a cone or about its axis; a
  single whole cone whose section has a step, a band or a hole a hair
  off; coaxial cones with prisms; and a cone beside a turn about another
  axis.
- The gate's tests, built, take about 31.5 s, against about 31 s at
  `3286cee`.

## 8 October: round 2 of #536, measured

Three lanes merged onto `ae20fa9` (laying's declines, a cone's triangles,
the kernel's declines), measured at `37ad3cc` on fresh seeds, in release,
the seven draws side by side, and the first 15 000 seeds of round 1's
slanted draws run again on `ae20fa9` and on the tip:

| Draw | First seed | Cases | Failures | Per thousand |
| --- | --- | --- | --- | --- |
| slanted, exact kernel | 5366100000 | 40 000 | 22 | 0.55 |
| slanted, application's body | 5366200000 | 40 000 | 29 | 0.73 |
| turned, exact kernel | 5366300000 | 30 000 | 10 | 0.33 |
| turned, application's body | 5366400000 | 30 000 | 21 | 0.70 |
| square | 5366500000 | 30 000 | 2 | 0.07 |
| profiles, exact kernel | 5366600000 | 20 000 | 10 | 0.50 |
| profiles, application's body | 5366700000 | 20 000 | 9 | 0.45 |

- Every draw is under one failure in a thousand. Over round 1's first
  15 000 slanted seeds the exact kernel goes from 0.93 failures in a
  thousand at `ae20fa9` to 0.40, the application's body from 1.07 to
  0.20.
- Eight seeds that hold on `ae20fa9` fail on the tip, each run again on
  every commit of the round. From `8a008ea` on (a plane its operand drew
  corners on is not moved onto a touch): 5365110795, 5366205754,
  5366215526, 5366611312 and 5366614710, four declined as unverified,
  two of them prisms alone, a rounded block a hair off a block's side.
  From `beb95aa` (a wall about a cone's axis is not moved off it):
  5365100952, a cylinder cut by a coaxial section with a band sloping by
  6e-7, declined as unverified. From `af8e71a`: 5366415073, a post cut by
  a post a hair off its end, drawn crossing through the body. From
  `6fd4c2a`: 5366116515, a stepped cone joined to a partial turn, drawn
  crossing. The last four shrink to cases that fail on `ae20fa9` too; the
  regression is in the case as drawn. None is named yet.
- Every other failure, run again on `ae20fa9`, fails there too. Shrunk,
  the slanted draws' 51 are: no cone at all, 25, most of them straight
  turns; a cone beside a turn about another axis, 9; coaxial turns with a
  cone, 8; a cone with prisms, 6; and a single cone with a band a hair
  long, 3, left open (`Closed`). The turned, square and profile draws'
  are prisms' and straight turns' families, those of #533. By the rule
  broken: drawn crossing (`Uncrossed`) and refused (`Answers`), nearly
  half each.
- The gate's tests, built, take about 32 s, against about 31 s at
  `ae20fa9`.

## 8 October: round 3 of #536, measured

Three lanes merged onto `a344fdb` (two faces folded onto each other across
an edge, a slide carrying a wall onto the first operand's, a cone holding a
wall on its axis), measured at `40afb2d` on fresh seeds, in release, the
seven draws side by side, and the first 15 000 seeds of round 1's slanted
draws run again on `a344fdb` and on the tip:

| Draw | First seed | Cases | Failures | Per thousand |
| --- | --- | --- | --- | --- |
| slanted, exact kernel | 5369100000 | 40 000 | 14 | 0.35 |
| slanted, application's body | 5369200000 | 40 000 | 16 | 0.40 |
| turned, exact kernel | 5369300000 | 30 000 | 9 | 0.30 |
| turned, application's body | 5369400000 | 30 000 | 13 | 0.43 |
| square | 5369500000 | 30 000 | 0 | 0 |
| profiles, exact kernel | 5369600000 | 20 000 | 4 | 0.20 |
| profiles, application's body | 5369700000 | 20 000 | 3 | 0.15 |

- Every draw stays under one failure in a thousand, each lower than in
  round 2. Over round 1's first 15 000 slanted seeds the exact kernel
  goes from 0.40 failures in a thousand at `a344fdb` to 0.33, the
  application's body from 0.20 to 0.13.
- Three seeds that hold on `a344fdb` fail on the tip, all drawn crossing
  (`Uncrossed`), all from `82eec7b` (two faces folded onto each other are
  cut the other way), each shrunk to a case that holds on `a344fdb`:
  5365104153, a stepped post ending in a pointed cone joined to a sixth
  of a turn of a coaxial post; 5369119536, three quarters of a pointed
  cone cut by three quarters of a coaxial post; 5369239472, through the
  application's body, a whole cone bored by a coaxial post as wide as its
  narrow end and joined to a block 6e-7 off that end. None is named yet.
- Every other failure, run again on `a344fdb`, fails there too, two of
  them by another rule: drawn crossing there, refused here. Shrunk, the
  slanted draws' 30 are: no cone at all, 14, most of them straight turns;
  coaxial turns with a cone, 9, five of them partial; a cone with prisms,
  5; and a single cone alone, 2. A turn drawn in another plane about the
  same line counts as coaxial here, which round 2's sorting missed. The
  turned and profile draws' are prisms' and straight turns' families,
  those of #533. By the rule broken, over all 59: refused (`Answers`) 31,
  drawn crossing 24, left open (`Closed`) 3, a volume off 1.
- The gate's tests, built, take about 33 s, against about 32.4 s at
  `a344fdb`.

## 9 October: round 4 of #536, measured

One lane merged onto `dcce010`, and half of it taken back: two faces
folded onto each other are flipped only across triangles that open by
more than rounding, while a fan of collinear samples is a fold again.
Measured at `594dec4` on fresh seeds, in release, the seven draws side by
side, and the first 15 000 seeds of round 1's slanted draws run again on
`dcce010` and on the tip:

| Draw | First seed | Cases | Failures | Per thousand |
| --- | --- | --- | --- | --- |
| slanted, exact kernel | 5371100000 | 40 000 | 20 | 0.50 |
| slanted, application's body | 5371200000 | 40 000 | 8 | 0.20 |
| turned, exact kernel | 5371300000 | 30 000 | 14 | 0.47 |
| turned, application's body | 5371400000 | 30 000 | 15 | 0.50 |
| square | 5371500000 | 30 000 | 4 | 0.13 |
| profiles, exact kernel | 5371600000 | 20 000 | 2 | 0.10 |
| profiles, application's body | 5371700000 | 20 000 | 3 | 0.15 |

- Every draw stays under one failure in a thousand. Against round 3, the
  slanted exact, turned and square draws fail more often and the slanted
  application's body less; since every one of these failures fails on
  `dcce010` too, that is the draw of seeds, not the code.
- Over round 1's first 15 000 slanted seeds, `dcce010` and the tip fail
  on the same seeds: five on the exact kernel (0.33 in a thousand), two
  through the application's body (0.13). 5365104153 is among them, open
  since round 3.
- Every failure on fresh seeds, run again on `dcce010`, fails there too,
  by the same rule. No seed that holds on `dcce010` fails on the tip.
- Shrunk, the slanted draws' 28 are: coaxial turns with a cone, 9, five
  of them partial; no cone at all, 11, eight of them straight turns; a
  cone with prisms, 4; a single cone alone, 3, two of them refused; a
  cone beside a turn about another axis, 1.
  The turned, square and profile draws' 38 are straight turns, 26, and
  prisms alone, 12: the families of #533.
- By the rule broken, over all 66: refused (`Answers`) 37, drawn crossing
  (`Uncrossed`) 23, left open (`Closed`) 2, a volume off 2, a span off
  along a line 1, a face listed backwards 1.
- The gate's tests, built, take about 33 s, as at the merge.
