---
name: refactor-rust
description: Move Rust code on this CAO repository without changing what it does. Use when extracting a function into another crate, splitting a file that has grown too large, moving a responsibility across a boundary, renaming a crate, or paying down the ratchets in the architecture test. Not for adding behaviour — that is rust-tdd.
---

# Moving code without changing it

A refactor and a feature have opposite contracts. `rust-tdd` starts from a red
test, because the behaviour does not exist yet. Here the behaviour exists and
must survive untouched, so the loop is reversed: **the test is green before, and
green after, and it never went red in between.**

If a test had to change to keep passing, you did not refactor. You changed the
software and called it tidying.

## Before touching anything

Read [`docs/contexts.md`](../../../docs/contexts.md). A refactor without a
target is rearrangement — the code ends up somewhere else, not somewhere right.
If what you are about to do is not in that map, either it belongs there and the
map is updated first, or it should not happen.

Then check the net is under you:

```sh
scripts/verify.sh
```

Green before you start, or you will not know which failure you caused.

## The loop

```
characterise → move → green → commit → move → green → commit
```

**1. Characterise.** If the code you are about to move has no test, write one
that describes what it does **today**, even if today is wrong. It must pass
immediately. That is the whole point: it is not a specification, it is a
witness. Anything it fails to pin down is something you are free to break
without noticing.

`crates/app/` has no tests at all. Nothing moves out of it before a
characterisation test exists for the piece being moved.

**2. Move.** One thing at a time, and let the compiler carry it. Cut the
function, paste it, fix what goes red. Rust is unusually good at this: an
extraction that compiles has almost certainly kept its meaning.

**3. Green.** `scripts/verify.sh`. If a test fails, the move changed
behaviour — that is information, not an obstacle. Undo and look at why, rather
than adjusting the test.

**4. Commit.** Now, not at the end of the day.

## One commit does one thing

Never a move and a change in the same commit. Not because it is untidy, but
because a diff that both moves and edits cannot be read: `git` shows a deletion
and an addition, and the one line that actually changed hides in three hundred
that only travelled.

If you want to fix something you notice while moving it, move it first, commit,
then fix it. Two commits, both small, both reversible.

Renames go alone too. `git` follows a pure rename and shows nothing; a rename
with an edit inside shows a rewrite.

## What survives a move, and what does not

**Behaviour survives.** Same inputs, same outputs, same errors, same order.

**Visibility does not automatically.** A function that was private and becomes
`pub` because it crossed a crate boundary has widened the public surface. Ask
whether it needed to. `pub(crate)` first.

**Wording does not travel down.** Extracting a rule from `viewport.rs` into
`cao_sketch` must leave its French labels behind, in `cao_app`. The layer below
returns a named case; the interface says it. The architecture test refuses the
alternative.

**I/O does not travel down either.** If the thing you are moving reads the disk
or the clock, it needs a trait before it can cross a domain boundary. See
`architecture-rust`.

## Moving a file into the folder its role calls for

The most common move in this repository from here on, and the one the layout
rules exist to make routine.
[`docs/code-layout.md`](../../../docs/code-layout.md) says which folder.

`git mv` alone, then `mod` declarations, then compile — nothing else in that
commit. A path change and an edit in the same diff cannot be read.

Two of these moves change more than a path and are worth naming:

**Splitting a screen into `state.rs` and `view.rs`.** Everything that decides
goes to the presenter, everything that draws stays in the view, and the presenter
must come out with no `egui::Ui` in any signature. What resists the split is
usually the interesting part: a decision taken mid-draw, from a value only the
frame had. Pass it in as an argument rather than following it back into the view.

**Lifting a widget into `ui/`.** The primitive takes plain values and hands back
what the user did — no `cao_*` import survives the move. The first caller to
migrate defines the signature; the second one is what tells you whether it was
the right one. Do not lift a widget that has only ever had one caller.

Both lower a ratchet, and the commit message should say which.

## The ratchets

`crates/app/tests/architecture.rs` holds four: files still reaching for the disk
or the clock, lines of user-facing wording still below `cao_app`, files past the
400-line budget, and widgets a screen still dresses by hand.

**A figure only ever goes down.** Lowering one is the visible result of a
refactor and belongs in its commit message. Raising one to make the gate pass
is the single move that voids the file: it turns a rule into a record of
whatever happened to be true.

If a legitimate change genuinely needs a figure raised, that is a decision for
the human, not a hurdle to clear.

## When to stop

- **The tests are red and you do not understand why.** Undo the step. A
  refactor whose effect you cannot explain is a rewrite in disguise.
- **The move needs a new abstraction to work.** Then it is not a move. Say what
  is needed and ask; this repository grows by explicitly requested steps.
- **The file is not smaller and nothing else is better.** Some code is large
  because the problem is. `solver.rs` is dense on purpose.

## Done

- [ ] `scripts/verify.sh` green, and it was green at every commit in between.
- [ ] No test was modified to accommodate the change. A test that moved with its
      code is fine; one whose assertions changed is not.
- [ ] No ratchet figure went up.
- [ ] The public surface did not widen for convenience.
- [ ] Each commit does one thing, and its message says what the software can do
      now — in English, as a sentence.
- [ ] `docs/code-map.md` still describes where things are. If files moved,
      it is now wrong.

For an independent read of the diff, the `review-architecture-rust` subagent applies
`review-rust` and `architecture-rust` in a separate context. Whoever just moved
the code is the worst placed to see what it took with it.
