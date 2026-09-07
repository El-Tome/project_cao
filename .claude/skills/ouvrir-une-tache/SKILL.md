---
name: ouvrir-une-tache
description: Open and close a task on this CAO repository. Use at the very start, before reading any code, to check whether the work is already under way somewhere, to branch correctly, and at the end to say what was done and what was not.
---

# Opening a task, and closing it

## Before reading a line of code

More than one person works here. The first question is not "how", it is
"is someone already doing this".

```sh
git fetch --prune
git status
git for-each-ref --sort=-committerdate --count=15 \
    --format='%(committerdate:short)  %(refname:short)' refs/remotes/origin
```

The `SessionStart` hook prints the essentials when a session opens. If it says
you are behind `origin/main`, catch up before starting.

**If an existing branch looks like it is doing the work, say so and ask** rather
than starting again. `git log origin/feat/xxx --oneline` usually says enough.

## The branch

From an up-to-date `main`:

```sh
git checkout main && git pull
git checkout -b feat/short-description
```

`feat/` for a feature, `fix/` for a correction. **English, hyphenated, short** —
`feat/circles`, `feat/oriented-dimensions`, `fix/solver-anchoring`,
`fix/tangent-circles`. A French branch name is a slip, not a variant.

Never work on `main` directly.

## While you work

The test before the code — see `rust-tdd`. Moving code that already works
without changing it — see `refactor-rust`. To find where to act,
`carte-du-code`. Before committing, `revue-rust`.

For an API question on `egui`, `wgpu` or `glam`, **use `context7`** rather than
memory: this project is on `egui 0.36`, `wgpu 30` and `glam 0.33`, crates whose
API breaks on every minor release.

## The language

Code, test names, documentation, branch names and commit messages are in
**English**. What the user reads is in French, and only in `cao_app` — the
layers below return a named case and let the interface say it.

Documents still in French are translated when they are touched anyway, not in a
sweep of their own.

## The commit message

One title line, an evocative sentence that says **what the software can do
now** — not what you typed:

```
The architecture is a test now, and the debt cannot grow
Circles: handles, tangencies held, crashes written down
Only the origin anchors, and a leaning shape has to say which way up
The solver starts again while it is still gaining, and a figure moves whole
```

A body only if it carries something: the why, an alternative that was tried and
dropped, a consequence that is not obvious. Never the list of files touched —
`git` already has it.

That is also where the story of your change goes, the one you were tempted to
leave in a comment.

Commits before this policy are in French. They stay as they are; history is not
rewritten for a naming rule.

## The gate

`git commit` runs `clippy -D warnings` then `cargo test --workspace` (~12 s).
On failure the commit does not happen and the index is untouched: fix it and go
again.

There is a valve, `CAO_SKIP_GATE=1`, for committing work in progress knowingly.
**It belongs to the human.** Never reach for it yourself, not even after several
failures: a failing gate is saying something true about the code. If you cannot
get it green, explain why, and ask.

On a fresh clone the git hook needs enabling, once:

```sh
git config core.hooksPath .githooks
```

## Closing

```sh
git push -u origin feat/short-description
```

Then say what was done, and above all what was **not**: a part left aside, a
decision deferred, a test you could not write. What is not said at that moment
is lost.
