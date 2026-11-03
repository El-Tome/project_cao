---
name: resolve-issue
description: Carry an issue of this CAO repository from its branch to its pull request, one subagent per step. Use as soon as an issue is to be worked — "take #n", "fix #n", "do the next todo" — and follow it as the orchestrator, rather than writing the tests and the code yourself.
---

# Resolving an issue, one step per subagent

You are the **orchestrator**. You do not write the test, nor the code, nor the
pull request body. You hand each step to the subagent that owns it, read back
its summary, and decide what comes next. Your context stays small: a summary
per step, never a page of cargo output.

The point of the split is that **the hand that writes the code never wrote the
test it has to pass**, and that the diff and the issue are read at the end by
eyes that did neither.

## The steps

| Step | Subagent | Model | Writes |
| --- | --- | --- | --- |
| 1. Open | `task-opener` | sonnet | git and `gh` |
| 2. Locate | `code-locator` | haiku | nothing |
| 3. Red | `test-writer` | opus | tests only |
| 4. Green | `implementer` | opus | production code only |
| 5. Gate | `gate-runner` | haiku | nothing |
| 6. Mutants | `mutation-hunter` | haiku | nothing |
| 7. Review | `review-architecture-rust` | opus | nothing |
| 8. Criteria | `criteria-checker` | sonnet | nothing |
| 9. Pull request | `pr-writer` | sonnet | `gh` |

The model is in each agent's frontmatter. Override it only when the step is
plainly lighter or heavier than usual — a one-line fix does not need opus to
write it.

### 1. Open

`task-opener` gets the issue number. It fetches, checks that no branch already
does the work, moves the label and branches from the issue. It returns the
branch, the **Done when** word for word, and anything that made it stop.

**If it stops, you stop.** An existing branch doing the work, an issue with no
**Done when**, a criterion two people would read two ways: those go to the
human, not to the next step.

### 2. Locate

`code-locator` gets the criteria and returns where each one is decided today:
files, functions, the tests already around them. Skip it when the issue names
the file itself.

### 3 and 4. Red, then green — one criterion at a time

`rust-tdd` asks for `test1 → impl1 → test2 → impl2`, never all the tests first.
The split keeps that rhythm: for each criterion, `test-writer` writes one test
and watches it fail, then `implementer` makes it pass.

On its first turn, `test-writer` transcribes every criterion into the module
documentation of the test file that will answer it, as `open-a-task` asks —
before any test, since the names it gives there are the tests it then writes.

Keep **one** `test-writer` and **one** `implementer` for the whole issue, and
continue them with `SendMessage` from one criterion to the next rather than
spawning new ones: each keeps what it learnt of the code.

Between the two, **stage the red test** — `git add -A`. What `implementer` then
changes is exactly the unstaged diff, and you check it before going on:

```sh
git diff --stat              # the implementer's work only
git diff -- '*/tests/*'      # must be empty
```

A hunk in `tests/` or inside a `#[cfg(test)]` module, from `implementer`, is
reverted — `git checkout -- <file>` on that hunk's file if it touched nothing
else, by hand otherwise — and the turn is redone. When `implementer` says a
test is wrong, it gives the reason; pass that reason to `test-writer`, and let
it decide. Never settle it yourself by editing the test.

Once green, `git add -A` again and go to the next criterion.

**Red tests are never committed on their own.** The gate would refuse them, and
`CAO_SKIP_GATE` belongs to the human. A test and the code that makes it pass
land in the same commit.

**A refactor** has no red step: `test-writer` pins what the code does today, as
`refactor-rust` says, and `implementer` moves the code under that pin.

### 5. Gate

`gate-runner` runs `scripts/verify.sh` and returns green, or the failure in a
few lines: the step, the file and line, the message. Red goes back to
`implementer` with those lines.

### 6. Mutants — domains only

Only when the diff touches `crates/sketch/` or `crates/solid/`, where a weak
test costs most. `mutation-hunter` runs `cargo mutants` on the diff alone and
returns the surviving mutants. A survivor in code this issue added goes back to
`test-writer`: the test exists but does not hold. If `cargo-mutants` is not
installed, the step is skipped and the pull request says so. It never blocks
on its own and is never part of the gate.

### 7 and 8. Review, then criteria

Both read; neither writes. Run them in parallel.

Write the diff to a file first and hand them its **path** — a subagent handed
"read the diff" on its own can burn its budget fighting a wrapper that
truncates `git` output. On a machine where `rtk` rewrites commands:

```sh
rtk proxy git diff main...HEAD > <scratchpad>/branch.diff
```

`review-architecture-rust` returns findings ranked by severity.
`criteria-checker` reads the issue, the transcription and the diff, criterion by
criterion, and returns one verdict per criterion. A finding that lands, or a
criterion not answered, goes back to the step that owns it — `test-writer` for
a test that asserts something adjacent, `implementer` for the rest.

### 9. Commit, then the pull request

You commit — the gate runs again through the hook — in the voice `open-a-task`
asks for. Then `pr-writer` gets the branch, the issue number, the
`criteria-checker` verdicts, what `mutation-hunter` said or that it was
skipped, and what was set aside. It pushes, opens the pull request, and stops.
**Nobody merges without the human's word.**

## The loop, and when it stops

Gate red, a review finding, a criterion unanswered: back to the step that owns
it, then forward again from there. **After three rounds on the same failure,
stop** and hand it to the human with what was tried.

Before a third attempt, **attack the premise**: when two fixes that share one
premise fail the same way, write that premise in one sentence and question it,
rather than trying a third fix built on it.

## What may be skipped

Nine agents for a typo is waste. Say which steps you skip, and why, in the pull
request body.

- **Locate** — the issue names the file.
- **Red** — nothing a test can reach changes: a document, a skill, a comment.
  The criterion then says `no test:` and why.
- **Mutants** — the diff touches neither `cao_sketch` nor `cao_solid`.
- **Review** — never on a change to `crates/`. On documentation only, yes.

**Gate** and **Criteria** are never skipped.

## What the human hears at the end

What was done, and above all what was **not**: a criterion left unanswered, a
decision set aside — opened as a `decision` issue in the same breath, as
`open-a-task` says — a step skipped, a survivor kept on purpose.
