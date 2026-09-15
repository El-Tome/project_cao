# A net under the interface — implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the deterministic half of the net described in the design, so that
a regression or a pull request delivering less than its issue asked fails a test
instead of waiting for a human to open the application.

**Architecture:** Three ratchets in `crates/app/tests/architecture.rs` first, so
that nothing added later re-tears what is woven. Then an exhaustive fixture in
`cao_sketch` that the compiler keeps complete, consumed by the operations that
sweep the whole drawing. Then a new `crates/app/tests/criteria.rs` that reads the
issue's criteria where they are transcribed — in the test module that covers
them — and refuses a criterion with neither a test nor a reason.

**Tech Stack:** Rust, `cargo test --workspace`, no new dependency. The repository
tests read sources as text; every helper this plan needs
(`workspace_root`, `sources_of`, `every_source`, `production`, `rust_files`,
`places_with_no_net`) already exists in `architecture.rs`.

**Spec:** `docs/superpowers/specs/2026-09-15-a-net-under-the-interface-design.md`

## Global Constraints

- Everything a developer reads is in English — file names, test names, assertion
  messages, commit messages, branch names. `crates/app/tests/language.rs`
  enforces it.
- No comment unless it carries a constraint invisible from the file. At most one
  per file written or modified.
- A file over four hundred lines has to be split; the files already over it are
  named in `FILES_OVER_THE_LINE_BUDGET` with the length they had, and none may
  grow. Files under `crates/*/tests/` are outside that rule —
  `a_file_that_outgrew_its_budget_has_to_be_split` walks `crates/*/src` only.
- One issue, one branch, one pull request. The branch is made with
  `gh issue develop <n> --base <base> --name <type>/<n>-<description> --checkout`.
- Every commit goes through the gate: `cargo fmt --all --check`, `clippy -D
  warnings`, `cargo test --workspace`. `CAO_SKIP_GATE` belongs to the human.
- No pull request is merged by an agent. Open it and stop.
- No line count in prose, in any document this plan writes.

## Amendment to the spec, found while planning

§4.1 says a place with no net *"may leave the list, never join it"*. A test that
does not read git history cannot tell an addition from a list that was always
that long. What every ratchet in `architecture.rs` does instead — and what this
plan does — is make the addition **impossible to do silently**: the list lives in
a named constant whose documentation says it may only shrink, and the diff that
grows it has to grow that constant too, in front of a reviewer. That is the
strength the repository already chose for
`FILES_OVER_THE_LINE_BUDGET` and `FILES_ALLOWED_TO_REACH_OUTSIDE`, and matching
it is worth more than inventing a stronger mechanism for one rule.

§4.1 also lists the sweep ratchet as the third of three. It belongs with the
fixture it holds, and is Task 5 below rather than part of Piece 1.

---

## Piece 1 — Two ratchets, before any file is worked

Issue: to open, `refactor(app)` scope. Branch base: `main`.
No dependency. This is what makes Pieces 5 and 6 safe to do at the water's edge.

### Task 1: A place with no net may only leave the list

**Files:**
- Modify: `crates/app/tests/architecture.rs`

**Interfaces:**
- Consumes: `places_with_no_net() -> Vec<String>`, already defined in the file.
- Produces: `PLACES_ALLOWED_TO_HAVE_NO_NET`, a constant later pieces shrink as
  they cover a place.

- [ ] **Step 1: Write the failing test**

Add below `NO_NET_HEADING`, and take the entries from what
`places_with_no_net()` returns today — run
`cargo test -p cao_app --test architecture -- --nocapture` after a temporary
`dbg!` if the list is not obvious, rather than copying it from this plan, which
was written on 2026-09-15 and is not the authority.

```rust
/// Every place `docs/code-map.md` is allowed to list under
/// [`NO_NET_HEADING`]. The list may only shrink: an entry added here is a
/// place that went into the repository with no test, which is the one move
/// that empties the rule of meaning.
const PLACES_ALLOWED_TO_HAVE_NO_NET: [&str; 0] = [];
```

and the test, beside `a_place_said_to_carry_no_test_carries_none`:

```rust
#[test]
fn the_places_with_no_net_are_the_ones_already_named() {
    let allowed: BTreeSet<&str> = PLACES_ALLOWED_TO_HAVE_NO_NET.iter().copied().collect();
    let listed: BTreeSet<String> = places_with_no_net().into_iter().collect();

    let joined: Vec<&String> = listed
        .iter()
        .filter(|place| !allowed.contains(place.as_str()))
        .collect();
    assert!(
        joined.is_empty(),
        "docs/code-map.md has gained {joined:?} under \"{NO_NET_HEADING}\". \
         A place with no test is not added to this repository: write the test, \
         or say in PLACES_ALLOWED_TO_HAVE_NO_NET why this one is owed.",
    );

    let paid: Vec<&&str> = allowed
        .iter()
        .filter(|place| !listed.contains(**place))
        .collect();
    assert!(
        paid.is_empty(),
        "{paid:?} carry a test now and docs/code-map.md says so. \
         Drop them from PLACES_ALLOWED_TO_HAVE_NO_NET.",
    );
}
```

- [ ] **Step 2: Run it and watch it fail**

```sh
cargo test -p cao_app --test architecture the_places_with_no_net_are_the_ones_already_named
```

Expected: FAIL, listing every place `docs/code-map.md` names — the empty
constant claims none is owed.

- [ ] **Step 3: Fill the constant from the failure message**

Copy the places the assertion printed into `PLACES_ALLOWED_TO_HAVE_NO_NET`,
sorted, and set the array length. Nothing else changes: the constant records
what is there, it does not grant anything new.

- [ ] **Step 4: Run it and watch it pass**

```sh
cargo test -p cao_app --test architecture
```

Expected: PASS, and every other test in the file still passing.

- [ ] **Step 5: Prove the ratchet bites**

Add a line under `## What has no net` in `docs/code-map.md` naming any covered
file, run the test, read the failure, then take the line out again. A ratchet
nobody has seen refuse anything is a ratchet nobody knows is wired up.

```sh
cargo test -p cao_app --test architecture the_places_with_no_net_are_the_ones_already_named
```

Expected: FAIL naming the file you added. Then remove the line and re-run:
PASS.

- [ ] **Step 6: Commit**

```sh
git add crates/app/tests/architecture.rs
git commit -m "test(app): the places with no net can be covered, never added to"
```

### Task 2: A mode keeps what it knows apart from what it draws

**Files:**
- Modify: `crates/app/tests/architecture.rs`

**Interfaces:**
- Consumes: `workspace_root() -> PathBuf`.
- Produces: `MODES_WITHOUT_A_PRESENTER`, shrunk by Piece 5.

- [ ] **Step 1: Write the failing test**

```rust
/// Modes under `screens/` that have not yet split what they know from what
/// they draw. `explorer` and `ribbon` show the shape: a `state.rs` that
/// decides and answers, a `view.rs` that draws. The list may only shrink.
const MODES_WITHOUT_A_PRESENTER: [&str; 0] = [];

#[test]
fn a_mode_keeps_what_it_knows_apart_from_what_it_draws() {
    let owed: BTreeSet<&str> = MODES_WITHOUT_A_PRESENTER.iter().copied().collect();
    let screens = workspace_root()
        .join("crates")
        .join("app")
        .join("src")
        .join("screens");

    let mut seen: BTreeSet<String> = BTreeSet::new();
    for entry in fs::read_dir(&screens).expect("a readable screens/").flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let split = path.join("state.rs").exists() && path.join("view.rs").exists();

        if owed.contains(name.as_str()) {
            seen.insert(name.clone());
            assert!(
                !split,
                "screens/{name} has its state.rs and its view.rs now. \
                 Drop it from MODES_WITHOUT_A_PRESENTER.",
            );
            continue;
        }
        assert!(
            split,
            "screens/{name} draws and decides in the same place, so nothing in \
             it can be tested without a window. A mode carries a state.rs that \
             decides and a view.rs that draws — screens/ribbon is the shape.",
        );
    }

    let gone: Vec<&&str> = owed.iter().filter(|name| !seen.contains(**name)).collect();
    assert!(gone.is_empty(), "{gone:?} are gone but still listed as owing a presenter");
}
```

- [ ] **Step 2: Run it and watch it fail**

```sh
cargo test -p cao_app --test architecture a_mode_keeps_what_it_knows_apart
```

Expected: FAIL on the first mode folder that has no `state.rs`/`view.rs` pair.

- [ ] **Step 3: Fill the constant from the failure**

Run the test repeatedly, adding each mode it names to
`MODES_WITHOUT_A_PRESENTER`, until it passes. Do not guess the list: the test
is the authority on what the folder holds today.

- [ ] **Step 4: Run the whole file**

```sh
cargo test -p cao_app --test architecture
```

Expected: PASS.

- [ ] **Step 5: Prove the ratchet bites**

```sh
mkdir -p crates/app/src/screens/nowhere && touch crates/app/src/screens/nowhere/mod.rs
cargo test -p cao_app --test architecture a_mode_keeps_what_it_knows_apart
rm -r crates/app/src/screens/nowhere
```

Expected: FAIL naming `screens/nowhere`, then PASS once it is removed.

- [ ] **Step 6: Commit**

```sh
git add crates/app/tests/architecture.rs
git commit -m "test(app): a new mode splits what it knows from what it draws, or it does not land"
```

### Task 3: Say in the documentation what the two rules are

**Files:**
- Modify: `docs/code-layout.md`
- Modify: `.claude/skills/architecture-rust/SKILL.md`

- [ ] **Step 1: Write the two rules into `docs/code-layout.md`**

Under the section that describes `screens/<mode>/`, one short paragraph: a mode
carries `state.rs` and `view.rs`, the modes that do not yet are named in
`architecture.rs`, and the list only shrinks. And beside the sentence that points
at **What has no net**: the list is closed, a new place with no test is not added
to it.

- [ ] **Step 2: Point the skill at the tests rather than restating them**

`architecture-rust` names the two constants and says the tests are the authority.
Do not copy the lists — `docs/code-map.md` learned that lesson already and says
so under **What has no net**.

- [ ] **Step 3: Run the gate**

```sh
scripts/verify.sh
```

Expected: format, clippy and tests green. `language.rs` reads both files.

- [ ] **Step 4: Commit and open the pull request**

```sh
git add docs/code-layout.md .claude/skills/architecture-rust/SKILL.md
git commit -m "docs: the two rules that keep the net from tearing are written where they are read"
git push -u origin <branch>
gh pr create --base main --title "test(app): the net cannot tear behind the work that weaves it" --body "..."
```

The body says what changed, that the ratchets record today's debt and grant
nothing, how it was verified, and `Closes #<n>`. Then stop — the merge is the
human's.

---

## Piece 2 — An exhaustive fixture the compiler keeps complete

Issue: to open, `test(sketch)` scope.
**Branch base: `fix/317-a-mirrored-arc-comes-back-as-an-arc`, not `main`.**
`Sketch::inside_band` only takes arcs on that branch; based on `main`, Task 4
would be red for a reason that is already fixed and in review. The pull request
body says what it sits on, and the stack is merged bottom-first per `CLAUDE.md`.

### Task 4: One of every kind, and the box that has to catch them all

**Files:**
- Modify: `crates/sketch/src/banding.rs` (its `#[cfg(test)]` module)
- Create or modify: wherever `cao_sketch` keeps its shared test scaffolding —
  check first with `grep -rn "cfg(test)" crates/sketch/src/lib.rs` and follow
  whatever is already there rather than inventing a second place.

**Interfaces:**
- Produces: `one_of_every_kind(&mut Sketch) -> Vec<Element>`, `pub(crate)` and
  under `#[cfg(test)]`. Tasks 5 and 6 consume it.

- [ ] **Step 1: Write the fixture and the failing test**

```rust
/// One of each. The match below has no wildcard arm, so a fifth kind of
/// element stops the build here rather than slipping past every test that
/// walks this list.
pub(crate) fn one_of_every_kind(sketch: &mut Sketch) -> Vec<Element> {
    let drawn = vec![
        Element::Point(/* a free point inside the band */),
        Element::Segment(/* both ends inside */),
        Element::Circle(/* centre and radius inside */),
        Element::Arc(/* all of the curve inside */),
    ];

    let mut kinds = BTreeSet::new();
    for element in &drawn {
        kinds.insert(match element {
            Element::Point(_) => "point",
            Element::Segment(_) => "segment",
            Element::Circle(_) => "circle",
            Element::Arc(_) => "arc",
        });
    }
    assert_eq!(kinds.len(), drawn.len(), "a kind is drawn twice and another not at all");
    drawn
}

#[test]
fn a_box_over_the_whole_drawing_catches_every_kind_of_element() {
    let mut sketch = Sketch::new();
    let drawn = one_of_every_kind(&mut sketch);
    let caught = sketch.inside_band(
        DVec2::new(-1000.0, -1000.0),
        DVec2::new(1000.0, 1000.0),
        AnnotationMetrics::default(),
    );

    for element in drawn {
        assert!(
            caught.contains(&Selection::Element(element)),
            "{element:?} was inside the box and the box did not take it",
        );
    }
}
```

The geometry is left to whoever writes it: put each element well inside the
band, and use the constructors `banding.rs`'s existing tests already use.
`AnnotationMetrics::default()` is what those tests pass — check and follow them.

- [ ] **Step 2: Run it**

```sh
cargo test -p cao_sketch a_box_over_the_whole_drawing_catches_every_kind
```

Expected: PASS on this branch, because #325 fixed the arc. That is the point of
the task: the test that could not have been written wrong is now in place, so
the fifth kind of element cannot repeat #317.

- [ ] **Step 3: Prove the compiler is the one holding it**

Temporarily add a variant to `Element` in `crates/sketch/src/element.rs`.

```sh
cargo test -p cao_sketch 2>&1 | head -30
```

Expected: the build fails **at the match inside `one_of_every_kind`**, not at a
test assertion. Remove the variant. If the build fails anywhere else first, the
fixture is not doing its job — the match has to be the narrowest place a new
variant lands.

- [ ] **Step 4: Commit**

```sh
git add crates/sketch/src/banding.rs
git commit -m "test(sketch): a box is shown to catch one of every kind, and a fifth kind stops the build"
```

### Task 5: The other five sweeps, and the ratchet that names them

**Files:**
- Modify: the test module of each operation named below
- Modify: `crates/app/tests/architecture.rs`

**Interfaces:**
- Consumes: `one_of_every_kind` from Task 4.
- Produces: `OPERATIONS_THAT_SWEEP_THE_WHOLE_DRAWING`.

- [ ] **Step 1: Find the five**

```sh
grep -rn "fn pick\|fn mirror\|fn delete" crates/sketch/src crates/part/src | head -20
```

The design names six sweeps: `inside_band` (Task 4), `pick`, the mirror, the two
patterns, deleting, and the `.caopart` round trip. Confirm each one's real name
and file before writing its test; the names above are the design's words, not
necessarily the code's.

- [ ] **Step 2: Write one test per sweep, each walking the fixture**

Each asserts the same shape as Task 4: every element the fixture drew is
answered for. For the round trip, that a part holding one of every kind comes
back from `.caopart` holding one of every kind.

- [ ] **Step 3: Run them**

```sh
cargo test --workspace
```

Expected: PASS. **A failure here is a finding, not a mistake in the test** —
write it down and raise it before changing the test to agree with the code. That
is how #317 looked the first time.

- [ ] **Step 4: Add the ratchet**

```rust
/// Operations that sweep the whole drawing, each with the test that walks the
/// exhaustive fixture. The list may only grow: an operation that answers for
/// every kind of element and is not named here is one nobody will notice going
/// quiet. The test names are checked, so a rename is caught here.
const OPERATIONS_THAT_SWEEP_THE_WHOLE_DRAWING: [(&str, &str); 6] = [
    ("crates/sketch/src/banding.rs", "a_box_over_the_whole_drawing_catches_every_kind_of_element"),
    // the five others, file and test name
];

#[test]
fn an_operation_that_sweeps_the_drawing_is_shown_to_answer_for_every_kind() {
    for (path, test) in OPERATIONS_THAT_SWEEP_THE_WHOLE_DRAWING {
        let source = fs::read_to_string(workspace_root().join(path))
            .unwrap_or_else(|_| panic!("a readable {path}"));
        assert!(
            source.contains(&format!("fn {test}(")),
            "{path} is said to sweep the whole drawing, and defines no {test}. \
             Either the test was renamed and this line follows it, or the sweep \
             lost the one thing that kept it honest.",
        );
        assert!(
            source.contains("one_of_every_kind"),
            "{path}'s {test} does not walk one_of_every_kind, so it enumerates \
             the kinds someone remembered. That is how #317 happened.",
        );
    }
}
```

- [ ] **Step 5: Run the gate and commit**

```sh
scripts/verify.sh
git add -A
git commit -m "test(sketch): every sweep of the drawing answers for one of every kind"
```

- [ ] **Step 6: Open the pull request against the parent branch**

```sh
gh pr create --base fix/317-a-mirrored-arc-comes-back-as-an-arc --title "..." --body "..."
```

The body names what it sits on and why. Stop there.

---

## Piece 3 — The issue's criteria, where the test is

Issue: to open, `test(app)` scope. Branch base: `main`. No dependency on Pieces 1
and 2, so it can be worked in parallel with them.

### Task 6: `criteria.rs` refuses a bullet naming a test that is not there

**Files:**
- Create: `crates/app/tests/criteria.rs`

**Interfaces:**
- Produces: nothing other tasks consume. It reads sources as text, the way
  `gate.rs` and `architecture.rs` do, and needs the same `workspace_root` and
  `rust_files` helpers — copy them rather than reach across test binaries, which
  is what `gate.rs` already does.

- [ ] **Step 1: Write the file's documentation first**

It says what the block looks like and why it lives beside the test rather than in
a directory of its own — the reasoning is in §4.3 of the spec, in two sentences.

- [ ] **Step 2: Write the failing test on a fixture string**

Test the parser against strings held in the test, not against the repository:
a parser tested on the repository passes the day the repository is empty of
blocks.

```rust
#[test]
fn a_bullet_naming_a_test_the_file_does_not_define_is_refused() {
    let source = "\
//! Closes #317.
//! - a box over an arc takes the arc — `a_box_catches_an_arc`

#[test]
fn something_else() {}
";
    let complaints = complaints_about(source, "somewhere.rs");
    assert_eq!(complaints.len(), 1);
    assert!(complaints[0].contains("a_box_catches_an_arc"));
}

#[test]
fn a_bullet_with_neither_a_test_nor_a_reason_is_refused() { /* ... */ }

#[test]
fn a_bullet_that_says_not_done_and_why_is_let_through() { /* ... */ }

#[test]
fn a_closes_block_with_no_bullet_at_all_is_refused() { /* ... */ }

#[test]
fn a_file_with_no_closes_block_is_not_this_test_s_business() { /* ... */ }
```

- [ ] **Step 3: Run them and watch them fail**

```sh
cargo test -p cao_app --test criteria
```

Expected: FAIL, `complaints_about` not defined.

- [ ] **Step 4: Write `complaints_about`**

```rust
/// Every complaint one file's `Closes #n` blocks earn. A file with no such
/// block earns none: transcribing is asked of the branch that closes an issue,
/// not of every file in the repository.
fn complaints_about(source: &str, path: &str) -> Vec<String>
```

It reads the `//!` lines, takes the ones after a `Closes #` line, and for each
bullet requires either a name in backticks that `source` defines as `fn <name>(`,
or the words `not done:` followed by something that is not only whitespace.

- [ ] **Step 5: Run them and watch them pass**

```sh
cargo test -p cao_app --test criteria
```

Expected: PASS, all five.

- [ ] **Step 6: Walk the repository with it**

```rust
#[test]
fn every_criterion_transcribed_in_this_repository_is_answered_for() {
    let mut complaints = Vec::new();
    for file in rust_files(&workspace_root().join("crates")) {
        let source = fs::read_to_string(&file).unwrap_or_default();
        complaints.extend(complaints_about(&source, &file.display().to_string()));
    }
    assert!(complaints.is_empty(), "{}", complaints.join("\n"));
}
```

Expected: PASS over an empty set today. Task 7 gives it its first block.

- [ ] **Step 7: Commit**

```sh
git add crates/app/tests/criteria.rs
git commit -m "test(app): a criterion transcribed from an issue names the test that answers it"
```

### Task 7: Transcribe the first one, so the rule has a worked example

**Files:**
- Modify: `crates/sketch/src/banding.rs`, or whichever file Piece 2 gave the
  #317 tests to

- [ ] **Step 1: Read the issue rather than remembering it**

```sh
gh issue view 317
```

Take its **Done when** bullets verbatim.

- [ ] **Step 2: Write the block above the test module**

One bullet per criterion, each naming the test that answers it or saying
`not done:` and why.

- [ ] **Step 3: Run the walk**

```sh
cargo test -p cao_app --test criteria
```

Expected: PASS, and a deliberate typo in a test name inside the block makes it
FAIL. Try it, then put it back.

- [ ] **Step 4: Commit**

```sh
git add -A
git commit -m "test(sketch): what #317 asked for is written beside the tests that answer it"
```

### Task 8: Ask for the transcription at the moment the task opens

**Files:**
- Modify: `.claude/skills/open-a-task/SKILL.md`

- [ ] **Step 1: Add the transcription to the opening, not the closing**

Under **The branch**: the issue's *Done when* is transcribed into the test module
that will cover it, before the first line of code. A list written afterwards is a
list of what the work did.

- [ ] **Step 2: Add the rule for a decision set aside — spec §4.5**

Under **Closing**: a decision set aside becomes an issue carrying the `decision`
label, opened in the same breath as the pull request that sets it aside, not a
sentence in a body. Name #320 as what that costs when it is not done.

- [ ] **Step 3: Run the gate, commit, open the pull request**

```sh
scripts/verify.sh
git add .claude/skills/open-a-task/SKILL.md
git commit -m "docs: a task opens by transcribing what it will be judged on"
git push -u origin <branch>
gh pr create --base main --title "test(app): what an issue asked for is answered for beside the test" --body "..."
```

Stop. The merge is the human's.

---

## Pieces 4 to 7 — what has to be true before each is planned

These are not planned here, and each one's reason is a condition rather than a
preference.

**Piece 4, the agent that reads the issue and the diff.** Needs an
`ANTHROPIC_API_KEY` secret on the repository, which only the human can add, and
the vehicle has to be chosen against what exists on the day it is built rather
than from memory. It is also worth more after Piece 3 has produced a few real
blocks to read: an agent given the transcription judges better than one given a
diff and an issue.

**Piece 5, the tools' presenters.** One issue per file, and the first three are
the ones that bled: the circular pattern's fields (#321), the cut predicate
(#319), and `viewport/input/mod.rs`, which is over the line budget and is where
the tools' glue collects. Each of those is a `refactor` that moves code without
changing it — the `refactor-rust` skill, not this plan. The plan for them is
written once Piece 1's `MODES_WITHOUT_A_PRESENTER` exists, because that constant
is the list they work down.

**Piece 6, the painters hand back data.** Same shape, starting with the grid
(#318). `render.rs` is several times over its budget, so the split comes with it.

**Piece 7, `egui_kittest`.** A new dev-dependency, so it goes through the
`architecture-rust` skill first, and the version and features are verified
against the registry at the time. Behaviour tests first — they owe no platform
decision. Images afterwards, on one platform, in the CI, skipped elsewhere.

## Self-review against the spec

- §4.1 two ratchets → Tasks 1 and 2; the third moved to Task 5 and the move is
  recorded in the amendment above.
- §4.2 fixture and sweeps → Tasks 4 and 5.
- §4.3 criteria → Tasks 6 and 7.
- §4.4 agent → not planned; the condition is named.
- §4.5 a decision becomes an issue → Task 8, step 2.
- §4.6, §4.7, §4.8 → not planned; the conditions are named.
- §5 two tiers → nothing to do until Piece 7 adds the first check the gate does
  not run. `gate.rs` gains its entry there, not here.
- §6 order → Pieces 1, 2 and 3 are independent of each other; 2 stacks on #325.
