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

The issues say the same thing faster. Three labels carry their state:

```sh
gh issue list --label todo          # ready to start, nothing blocks it
gh issue list --label in-progress   # a branch exists and a pull request is open
gh issue list --label backlog       # waiting on a decision or on another issue
```

An epic carries none of the three — it is tracked by its sub-issues. Move the
label as you go: `in-progress` when the branch is made, off it when the pull
request is opened and the work is handed back.

## The branch

One issue, one branch, one pull request. The branch is created **from the
issue**, never with a bare `git checkout -b`, so that GitHub links the two and
the issue shows what is being done about it:

```sh
git checkout main && git pull
gh issue develop <n> --base main --name <type>/<n>-<short-description> --checkout
```

The name is a **conventional commit** prefix, the issue number, and a short
description: `refactor/24-storage-error-becomes-two-errors`,
`fix/17-dimension-leader`, `feat/circles`. **English, hyphenated, short.** A
French branch name is a slip, not a variant.

If there is no issue yet, open one first. It is where the reasoning goes, and
it is what the next person reads before touching the same file.

Never work on `main` directly.

### Stacking

An issue whose dependency is still in review does **not** wait for a merge. It
branches off that branch and targets it as base:

```sh
gh issue develop <n> --base <parent-branch> --name <type>/<n>-<description> --checkout
gh pr create --base <parent-branch>
```

The pull request body says what it sits on.

**Merging the stack is where it goes wrong.** Deleting the parent branch closes
every pull request based on it, and a closed pull request cannot be retargeted
or reopened. Merge the bottom **without** `--delete-branch`, rebase the child on
`main` and force-push it, `gh pr edit <child> --base main`, and only then delete
the parent branch. `CLAUDE.md` §How work is delivered has the commands.

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

A **conventional commit** prefix, then an evocative sentence that says **what
the software can do now** — not what you typed. The prefix does not replace the
sentence, it precedes it:

```
feat(sketch): circles hold their tangencies, and a crash is written down
fix(sketch): only the origin anchors, and a leaning shape says which way up
refactor(core): a part archive and a configuration file fail apart
perf(sketch): the solver starts again while it is still gaining
docs(contexts): Command is the vocabulary of intent, and goes with the preferences
```

`feat`, `fix`, `refactor`, `perf`, `docs`, `test`, `build`, `ci`, `chore`. The
scope is the crate when the change has one, and is left out when it does not.

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

Push, then open the pull request. Its title carries the same conventional
commit prefix as the commits, and it **always has a body**:

```sh
git push -u origin <branch>
gh pr create --base main --title "<type>(<scope>): <sentence>" --body "..."
```

The body says what changed, what was decided and what was set aside, how it was
verified — the gate, the test count, the command whose output you read — and
ends on `Closes #n`. Never the list of files touched: `git` already has it. A
pull request with no body is one nobody can review a month later.

Then move the issue's label from `in-progress`, and say to the human what was
done and above all what was **not**: a part left aside, a decision deferred, a
test you could not write. What is not said at that moment is lost.
