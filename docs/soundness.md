# Soundness: the rules every solid keeps

A geometry kernel has no expected answer to be tested against. Nobody can
write down the faces two tilted cylinders should come out as, so a boolean
cannot be checked the way a function returning a number is. What can be
written down is what **every** result must satisfy, whatever it is — and a
generator of random solids that throws cases at the kernel until one of them
breaks a rule. That is #448.

The scenario tests in `crates/solid/src/boolean/tests.rs` and
`crates/part/tests/extruding.rs` stay: a named case that once failed is worth
more than a random one that passes. This adds the cases nobody listed.

## The rules

They live in `cao_solid::soundness` (`crates/solid/src/soundness/`), behind
the `test-support` feature: the application never checks its own solids while
it runs. They read **triangles and nothing else**, which is what lets them
hold any kernel to the same account — the one written here, or one adopted
behind `cao_solid` (#447).

| Rule | Function | What breaks it |
| --- | --- | --- |
| Closed | `closed` | a stretch of edge with more faces running one way along it than the other: a hole, a crack, a face laid twice |
| Uncrossed | `uncrossed` | two faces passing through each other, or lying on each other over an area |
| Volume | `enclosed`, `Lines` | the matter enclosed is not what the operation promised |
| Repeatable | `repeatable` | the same input answered twice, differently, down to one bit |
| Undone | `first_difference`, in `cao_part`'s tests | an operation and its undo give back a part other than the one they started from |
| Answers | `answer` | the kernel panics, does not come back within the patience given, or declines to raise a solid its input describes |

A few decisions are worth knowing before reading a report.

**Closed is counted along stretches of edge, not whole edges.** A kernel that
cuts faces leaves T-junctions — the corner of one face partway along the edge
of its neighbour — and the surface is still closed. Every edge is split at the
corners lying on it before the faces along it are counted. The count asked for
is *as many one way as the other*: one each on an ordinary edge, two each where
two solids touch along an edge and nothing more, which is a legitimate result.

**The volume is promised two ways.** A solid raised on its own is held against
arithmetic: a prism against its area times its height, exactly — the kernel
was handed the flats a circle was sampled into, and those are what it promised
to raise; a revolution against Pappus, less at most a fifth of a percent for
the flats laid round the curve. A boolean is held against **the same lines of
measure**: a bundle of parallel lines is laid across the case, and along every
one of them the length inside the result must equal the length inside the
operation applied to its inputs — union or difference of stretches of a line,
which is arithmetic nobody can get wrong. Measuring both sides along the same
lines is what lets the tolerance be tight: the grid's own error is the same on
both sides and cancels. When the rule breaks, the report names the line the two
disagree most along. The lines only cover the box the inputs span, so matter a
result leaves outside that box breaks the rule on its own.

**Every tolerance is relative** to how far the solid reaches, for the reason
`boolean.rs` gives: the noise of a coordinate grows with its size.

## The cases

Two generators, one per level:

- **Solids** — `crates/solid/tests/random_solids/`: a first solid and up to
  five steps that add to it or cut into it. Prisms of rectangles, circles
  sampled into 48 flats as the application samples them, star-shaped outlines
  and rings; revolutions of a rectangle, on the axis, off it, and a hair either
  side of it; on the three planes of the origin or on a tilted one.
- **Parts** — `crates/part/tests/an_undo_gives_back_the_part.rs`: drawings on
  the planes of the origin and on flat faces of the part, rectangles and
  circles, raised, cut or turned. Every gesture is undone and redone, which is
  the only way to reach the fifth rule.

A kernel does not fail on solids in general position, so both are weighted
towards the places it does. Corners sit on a lattice of round numbers, so that
faces land in one plane on their own. Half the tools are drawn from a solid
before them: the same plane, the same height, the same centre, a radius that
makes two circles touch. And a coincidence is now and then **missed by a
hair** — from half a billionth of the part's size up to the solver's own
tolerance — which takes in the band where the kernel written here judges two
faces one plane at some corners and not at others. Never finer than five times
the rules' own tolerance: below that, a thin skin the kernel was right to leave
could not be told from two faces laid on each other. Some cases are drawn thirty times larger, where a
tolerance taken in absolute units stops holding.

## Running a campaign

A campaign is not a unit test. It is ignored by the gate and run by hand,
bounded by a deadline rather than a count. The campaigns over solids are not
even compiled unless asked for, with `--features campaigns`: work on any other
issue never pays for them (#526).

```sh
CAO_FUZZ_SECONDS=300 cargo test --release -p cao_solid --features campaigns \
    --test every_solid_keeps_its_rules -- --ignored --nocapture

CAO_FUZZ_SECONDS=300 cargo test --release -p cao_part \
    --test an_undo_gives_back_the_part -- --ignored --nocapture
```

`CAO_FUZZ_SEED` starts from a given seed rather than from the clock;
`CAO_FUZZ_PATIENCE` is how many seconds a single case may take before it counts
as no answer. `--release` is worth it: the same minutes try several times more
cases.

The deadline is checked between cases, so a run can overshoot it by one
patience. The seed being tried is written on the standard error as it goes: a
kernel that blows its stack ends the program, which nothing can catch, and the
last seed written is the one to run again.

The campaigns of `random_exact_solids` and `random_turned_solids` hand their
seeds to every core of the machine (`campaign_across`, #549); `CAO_FUZZ_THREADS`
sets how many threads instead. They name the same failing seeds, in seed order,
as one thread would. Several campaigns run side by side should share the cores
between them: a machine crowded past its cores slows every case, and a case
slowed past `CAO_FUZZ_PATIENCE` counts as no answer.

A campaign shrinks the first three failures of each rule and counts the rest.
One rule broken by a common cause can hide a rarer one behind it: several
campaigns from different seeds see more than one long one.

## Reading a finding

Every case that breaks a rule is shrunk: steps are dropped, curves made
straight, planes put square to the axes and numbers rounded, for as long as
the smaller case still breaks **the same rule**. A case shrunk into a different
failure is a different bug, and would describe neither. The report prints how
many cases broke each rule, and for the first few of each, the flaw and the
shrunk case as a test ready to paste:

```text
── Closed, seed 1727701234 ──
as drawn: Open { from: …, to: …, one_way: 1, other_way: 0 }
shrunk:   Open { from: …, to: …, one_way: 1, other_way: 0 }

#[test]
fn seed_1727701234_keeps_every_rule() {
    random_solids::holds(&Case::new(
        Leaf::prism(Plane::xy(0.0), Outline::rectangle([0.0, 0.0], [10.0, 10.0]), 10.0),
        vec![
            Step::cut(Leaf::prism(Plane::xy(5.0), Outline::circle([10.0, 5.0], 2.0), 5.0)),
        ],
    ));
}
```

Numbers are printed the way `{:?}` prints an `f64` — the shortest text that
reads back as the same bits — so the case pasted is the case that failed and
not a neighbour of it.

That is also the shape of a good bug report for a kernel adopted from
elsewhere: a minimal input, an operation, and a rule that broke.

## Where a finding goes

Each distinct failure is **its own issue**, with the shrunk case in it. Fixing
it is not the harness's job, and some will not be worth fixing in a
representation about to be left (#447).

A case whose issue is still open is kept as a named test under
`#[ignore = "#n"]` — a solid in `crates/solid/tests/what_random_solids_found.rs`,
a part beside its campaign in `crates/part/tests/an_undo_gives_back_the_part.rs`
— so that it stays in the repository and runs with `--ignored`. The commit that
fixes it takes the `ignore` off, and from then on the gate holds it.

The first campaigns, over some hundred thousand random solids and five hundred
random parts, found nine: #486 to #494. Whether campaigns run on a timer is
#495.
