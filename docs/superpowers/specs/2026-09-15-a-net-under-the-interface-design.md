# A net under the interface — design

Date: 2026-09-15
Branch: `docs/330-a-net-under-the-interface`
Epic: #329
State: agreed, ready for the implementation plan

> A record of a decision, kept as it was taken. File names, issue numbers and
> the state of the code below are the reading of the date at the head of it, and
> are not updated afterwards.

## 1. Why

#300 to #322 were merged without anyone opening the application. The first human
test afterwards opened eight issues in a day: #314, #315, #316, #317, #318,
#319, #320 and #321.

#324 answered by forbidding a merge before a human review. That rule is right
and it does not scale: it makes the human the net. What follows is the net under
them, so that a review is spent on taste rather than on finding out that a tool
arrives broken.

## 2. What the eight were

They are not one problem. Read one at a time they fall into three families, and
only one of them is a missing test.

### Family A — defects a headless test would have caught

- **#317** — a box dragged over the drawing took points, traits and circles, and
  left arcs behind. `Sketch::inside_band` walks four hand-written loops; there
  were three. The test enumerated what was implemented rather than what exists.
- **#321**, in its worst part — *the circular pattern could not be tested at all:
  no fields showed up*. A tool shipped not working. Which field a tool shows is
  a presenter's decision, buried in glue that has no test.
- **#318** — the grid of a sketch opened on a face is drawn where the base planes
  are, not in the sketch's own `WorkPlane`.
- **#319** — the matter between the camera and the sketch plane is hidden
  whatever the camera does. A missing condition, and a condition is a predicate.

Three of the four live under `crates/app/src/screens/viewport/`, named in
**What has no net** in `docs/code-map.md`. That is not a coincidence, and it is
the shortest statement of the problem: the places with no test are the places
that broke.

### Family B — the pull request delivered less than the issue asked

- **#314** — *It works, but it is not what was asked.*
- **#316** — the chamfer lays its geometry and none of the dimensions the issue
  asked for.

No test catches this, because the test was written against what was built. It is
a traceability hole, not a coverage hole, and it is the only family a machine
cannot close on its own: knowing something is missing means reading the issue.

### Family C — a decision nobody shared

- **#320** — #305 deliberately laid a bare copy, with no rule and no link to the
  original, and wrote so in its body. The human wanted the opposite and found out
  by testing.

No machine catches taste. What can be cut is the delay.

## 3. What this is not

`egui_kittest`, which #258 asks for, is the answer everyone looks at. Checked
against the four defects of family A, it catches one of them reliably.

Meanwhile this repository has already invented the technique that catches all
four, twice, and did not apply it where it was needed:

- `e1a7be4` — *the toolbar's presenter comes out from under its drawing*. A
  presenter out from under its view is tested with no window, and
  `a_presenter_never_takes_the_interface` in `crates/app/tests/architecture.rs`
  already keeps it that way. `screens/explorer/` and `screens/ribbon/` are both
  built this way today; they are the pattern, not a proposal.
- #257 — *what a painter pushes is a `Vec<cao_render::Vertex>`*, read back onto
  the sketch's plane to say what was drawn, with no window and no GPU.
  `screens/viewport/render/arc.rs` is the pattern.

So the net is those two moves applied where they were not, plus an exhaustiveness
the compiler holds rather than a convention anybody has to remember. `egui_kittest`
comes fourth, as a backstop rather than as the plan, and #258 is answered there.

## 4. The pieces

### 4.1 Three ratchets, before any file is worked

`crates/app/tests/architecture.rs` is where this repository turns a rule into a
failing build, and it already holds the debt it cannot pay yet as named lists
that may only shrink. Three rules are added there first, so that the net cannot
tear again behind the work that weaves it.

**A place with no net may leave the list, never join it.**
`a_place_said_to_carry_no_test_carries_none` holds `docs/code-map.md` and the
code in agreement in both directions today: a listed file that grows a `#[test]`
fails the test until its line comes out. What it does not refuse is a *new*
entry. It gains that refusal, against the list as it stands on the day it lands.

**A new file under `screens/<mode>/` has its presenter.** A mode's folder carries
a `state.rs` that decides and a `view.rs` that draws. Files that predate the rule
are named in a list that may only shrink, the same way the line budget names
its own.

**Every operation that sweeps the whole drawing has its exhaustive test.** The
list is in 4.2 and may only grow.

### 4.2 An exhaustive fixture the compiler keeps complete

The defect in #317 is not that a test was missing. A test existed. The defect is
that the test enumerated the kinds of element that were implemented, which is
the same act as writing the code, with the same blind spot.

So the enumeration stops being written by hand. Under `#[cfg(test)]` in
`cao_sketch`:

```rust
/// One of each. The match below has no wildcard arm, so a fifth kind of element
/// stops the build here rather than slipping past the tests that walk this list.
pub(crate) fn one_of_every_kind(sketch: &mut Sketch) -> Vec<Element>
```

`Element` has four variants — `Point`, `Segment`, `Circle`, `Arc`. The guarantee
is not a promise anyone keeps: a fifth variant fails to compile at that match,
in the gate, on the commit that adds it.

That gives completeness of the fixture. Adoption is a second question, and it is
a ratchet in `architecture.rs`:

```rust
/// Operations that sweep the whole drawing. Each must have a test that consumes
/// the exhaustive fixture. The list may only grow.
const OPERATIONS_THAT_SWEEP_THE_WHOLE_DRAWING: [(&str, &str); N]
```

It opens on the six places where *"and the arcs?"* is the question that gets
forgotten: `inside_band`, `pick`, the mirror, the two patterns, deleting, and the
`.caopart` round trip.

The same move is worth repeating afterwards for `Constraint`, `Operation` and
`ChamferMode`, each of which has matches scattered across crates. That is not in
this piece; it is what this piece makes cheap.

### 4.3 The issue's criteria, transcribed where the test is

Family B needs the issue's *Done when* list to be readable by something that is
not a person reading GitHub. So the branch transcribes it — at the moment the
task is opened, not at the end, because a list written after the work is a list
of what the work did.

The transcription does **not** go in a directory of its own. One file per issue
accumulates forever, goes stale on the first rename, and sits away from the code
it describes — against the grain of a repository where each test lives in the
file it covers. It goes in the module documentation of the test that covers it:

```rust
//! Closes #317.
//! - a box dragged over an arc takes the arc — `a_box_catches_an_arc`
//! - a mirrored arc comes back as an arc — `a_mirror_keeps_an_arc`
//! - the click was never broken — not done: `Sketch::pick` already returns arcs
```

A new `crates/app/tests/criteria.rs`, sibling to `gate.rs` and `architecture.rs`
and reading sources as text the same way, refuses:

- a bullet that names a test the file does not define;
- a bullet that names no test and gives no reason after `not done:`;
- a `Closes #n` block with no bullet at all.

Renames stay local — the test and the line that names it are in the same file,
so whoever renames one is looking at the other. Nothing accumulates. An issue
whose criteria span several files writes several blocks, and `criteria.rs` reads
their union.

**What this cannot tell.** Whether the transcription is faithful to the issue,
and whether the named test asserts what the bullet claims. Both need the issue
read, which is 4.4. Saying so here is the point: a check that oversells itself is
worse than none.

### 4.4 An agent that reads the issue and the diff

A workflow of its own on `pull_request`. It does not duplicate `ci.yml`, which
triggers on `push` alone and deliberately so.

It is given the issue as GitHub holds it, the diff, and the transcribed blocks
from 4.3, and it posts **one** comment: a table, criterion by criterion — covered,
not covered, the transcription has drifted from the issue, the named test does
not assert this. Every row quotes the criterion and the test line it read, so
that contradicting it costs a glance.

**It does not block.** A judge that is wrong and blocking is one people learn to
route around within a week, and then the deterministic layer goes with it.

It needs an `ANTHROPIC_API_KEY` repository secret and costs a little on each pull
request. The vehicle — a published action or a request written by hand — is
chosen when the piece is built and against what exists then, not from memory.

### 4.5 A decision set aside becomes an issue, not a sentence

Family C, and it is a line in `.claude/skills/open-a-task/SKILL.md` rather than
code.

#305 wrote its decision in its body. Bodies are read at review time, and #324 has
only just made review time exist. A decision set aside becomes an issue carrying
the `decision` label — *needs a human call before any code is written* — opened
in the same breath as the pull request that sets it aside. #320 would have
existed on the 14th instead of being found on the 15th.

### 4.6 The tools' presenters come out from under `viewport/input/`

The move of `e1a7be4`, applied to the folder that bled. The presenter decides —
which field the tool shows, what the next click means, whether the matter in
front is cut — and `a_presenter_never_takes_the_interface` keeps `&mut egui::Ui`
out of it, which is what makes it testable with no window.

This is where #321 and #319 are caught, and both as ordinary unit tests: *the
circular pattern tool exposes a field for the step and one for the count*, and
*the cut applies only within tolerance of the face-on view*.

### 4.7 The painters hand back data that is read back

The move of #257, applied to `screens/viewport/render.rs` and its children. A
painter returns what it would draw; the test reads it back and says where it
lands. #318 becomes one assertion: every vertex of the grid lies in the sketch's
`WorkPlane`.

`render.rs` is several times over the line budget, so splitting it serves this
rule and the budget at once.

**Only the files that bled get an issue of their own** — the tool presenters, the
grid painter, the cut predicate. The rest of **What has no net** empties as work
opens those files anyway. The ratchet of 4.1 is what makes that safe: the list
can only shrink, so at-the-water's-edge is a direction rather than a hope.

### 4.8 `egui_kittest`, behaviour first

The finding that unblocks #258: **the platform question exists only for images.**

`egui_kittest` does two separable things. It drives the application through its
AccessKit tree — find a widget by its label, click it, type — and asserts on what
the tree holds. And it renders a frame offscreen and compares it to a committed
image. Only the second has anything to say about a GPU, a font or an operating
system.

So the behaviour tests come first, and no decision is owed before writing them.
*The circular pattern tool shows a field for the step and one for the count* is
#321 end to end, and it is a statement about the tree.

Images come after, and the answer to the question #258 posed is the third of the
three it offered: **one platform, in the CI, skipped elsewhere.** The goal is to
notice a regression, not to certify a rendering across three operating systems;
one reference image per platform is three times the images to keep for a question
nobody is asking.

`egui_kittest` is a new dev-dependency and goes through the `architecture-rust`
skill before it is added, as `CLAUDE.md` requires. #258 says it is published at
the version `egui` is pinned to here and carries an `eframe` feature; that is
verified against the registry when the piece is built, not taken from #258.

## 5. Two tiers

| | Commit gate | CI, every push |
| --- | --- | --- |
| `cargo fmt --all --check` | yes | yes |
| `cargo clippy --workspace --all-targets -D warnings` | yes | yes |
| `cargo test --workspace` | yes | yes |
| `criteria.rs`, reading text | yes | yes |
| `scripts/build-windows.sh` | no | yes, already |
| harness tests, behaviour and images | no | yes |
| the agent of 4.4 | no | yes, on `pull_request` |

The tests of 4.1, 4.2, 4.6 and 4.7 are ordinary unit tests. They stay in the gate
and the gate stays fast. Only the harness leaves.

`crates/app/tests/gate.rs` already keeps the two lists from drifting, and already
names the one check the gate deliberately leaves to the CI. It gains the harness
alongside it. An undeclared difference fails a test, which is the property worth
keeping.

**The risk this creates** is a test nobody runs locally, which rots. Against it:
the harness has a named script beside `scripts/verify.sh`, and `review-rust` says
to run it when `screens/` was touched.

## 6. Order

| | Piece | Catches | Size |
| --- | --- | --- | --- |
| 1 | The three ratchets — 4.1 | nothing behind, everything ahead | small |
| 2 | The exhaustive fixture — 4.2 | #317 | small |
| 3 | `criteria.rs` — 4.3, and the line in `open-a-task` for 4.5 | #314, #316, and cuts C's delay | small |
| 4 | The agent — 4.4 | a soft transcription | medium |
| 5 | The tool presenters — 4.6 | #321, #319 | large, cut per file |
| 6 | The painters — 4.7 | #318 | large, cut per file |
| 7 | `egui_kittest` — 4.8 | the rest, and closes #258 | medium |

1 comes first because every later piece adds code to places the ratchets are
meant to hold. 4 comes after 3 because an agent given the transcribed criteria
judges better than one given only a diff and an issue.

## 7. Set aside, knowingly

- **A blocking agent review.** Reasoned in 4.4.
- **A reference image per platform.** Reasoned in 4.8.
- **Emptying What has no net as a project of its own.** It would be a body of
  work nobody asked for, competing with #320, #321 and #314, against the scope
  rule in `CLAUDE.md`. Only the files that bled are scheduled; 4.1 makes the rest
  a direction.
- **Driving the running application from outside it.** #258 measured that macOS
  does not route synthetic input to an application that is not active, and that
  the accessibility tree of the real window exposes almost nothing. The answer is
  not to open a window, which is what 4.8 does.
- **Exhaustive fixtures for `Constraint`, `Operation` and `ChamferMode`.** Worth
  doing, not in piece 2. Piece 2 is what makes them cheap.
- **A part drawn to order.** #327 and #328 build one, for measurement and for
  putting something rich in front of the application. It is a neighbour of this
  epic, not a part of it.
