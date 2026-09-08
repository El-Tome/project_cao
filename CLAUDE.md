# Rules for this project

Read [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the whole picture before
any structural change, [`docs/contexts.md`](docs/contexts.md) for where the
seams are and where they are going, [`docs/code-layout.md`](docs/code-layout.md)
for where a new file goes and what it may import,
[`docs/glossary.md`](docs/glossary.md) for the words, and
[`docs/code-map.md`](docs/code-map.md) to find where things live.

## How to work

Six skills carry the detail, in `.claude/skills/`:

| Skill | When |
| --- | --- |
| `open-a-task` | at the very start, before reading any code |
| `code-map` | to find where to act |
| `rust-tdd` | to write the test before the code |
| `refactor-rust` | to move code that already works, without changing it |
| `architecture-rust` | before adding a crate, a module, a dependency, or any I/O |
| `review-rust` | before committing |

The `review-architecture-rust` subagent reads a diff in a separate context.

On a fresh clone, enable the git hook once:

```sh
git config core.hooksPath .githooks
```

`git commit` then runs `cargo fmt --all --check`, `clippy -D warnings` and
`cargo test --workspace` (~12 s). On failure the commit does not happen. The
`CAO_SKIP_GATE=1` valve exists for work in progress: it belongs to the human, an
agent never reaches for it on its own.

**Every branch you push is checked**, pull request or not: the CI triggers on
push, on any branch. What that does not check is the branch *merged with* `main`
— only its own tip. A branch that went green a week ago can still break `main`,
which is what the rebase before a merge is for.

For an API question on `egui`, `wgpu` or `glam`, use `context7` rather than
memory: this project is on `egui 0.36`, `wgpu 30` and `glam 0.33`, crates whose
API breaks on every minor release.

## Language — not negotiable

`crates/app/tests/language.rs` enforces what follows, in the gate. It is the
authority; this section is the summary.

**Everything a developer reads is in English.** Documentation, comments,
assertion messages, test names, commit messages, branch names — and **file and
folder names**: `docs/render.md`, not `docs/rendu.md`; `scripts/verify.sh`, not
`scripts/verifier.sh`. There is no "when it is touched anyway" clause: that
clause is what kept half of this repository French, because a document nobody
has a reason to open never gets touched.

**The one exception is the interface.** Text the user reads is in **French**,
and lives only in `cao_app`. Layers below return a named case —
`ExtrusionMode::Cut`, not "Enlèvement de matière" — and the interface decides
how it is said. That is what will make translation a wiring job rather than a
rewrite; the i18n system itself is still to come. A separate rule, in
`architecture.rs`, keeps those sentences from sinking any lower.

A document quoting such a string word for word keeps it quoted: writing that a
button reads "Nouvelle esquisse" is not writing in French, and the test skips
what sits between quotes or backticks.

Git history is not rewritten: the commits that predate the rule stay as they
are.

## Modularity — not negotiable

`crates/app/tests/architecture.rs` enforces what follows, in the gate. It is the
authority; this section is the summary.

- `cao_core` never depends on a UI crate (`egui`, `eframe`, …). That is the only
  way to keep it reusable by a future tablet or web front-end.
- `cao_sketch` and `cao_solid` take nothing but `glam` and `serde`. They are the
  real domains; a geometry rule goes in one of them.
- No `std::fs`, `directories` or `Utc::now()` below a domain boundary without a
  trait. Four files in `cao_core` predate the rule and are listed in the test;
  there will be no fifth.
- A new mode (sketching, assembly, …) is a new `Screen` variant
  (`crates/app/src/screens/mod.rs`) plus its own module in `screens/`. Never
  several modes piled into one file or one match.
- Once a mode outgrows a single screen, it becomes its own crate (`cao_sketch`,
  `cao_assembly`, …) rather than swelling `cao_app`.
- `cao_app` stays a thin shell: window and routing between modes, no business
  logic.
- Mind the name: whatever its documentation claims, `cao_core` is not the
  domain. It depends on `cao_sketch` and `cao_solid` and orchestrates sketch,
  solid, history and persistence — it is the application layer.

## Where a file goes

Also in the gate. [`docs/code-layout.md`](docs/code-layout.md) is the detail;
this is what you need before creating a file.

**The role is the folder, never a suffix in the name.** A Rust module name is an
identifier, so `button.ui.rs` and `part-repository.rs` cannot name a module.
Files and folders are snake_case, and the path says the job:

```
crates/core/src/            crates/app/src/
├── model/                  ├── ui/                  primitives: egui only
├── ports/                  └── screens/<mode>/
├── adapters/                   ├── state.rs         the presenter, never draws
└── services/                   └── view.rs          the drawing
```

A folder appears only where the role exists. `cao_sketch` and `cao_solid` do
mathematics and stay flat: a port there removes no disk, no clock, no network,
and buys nothing.

- **`ui/` knows no `cao_*` crate.** A primitive takes plain values and hands
  back what the user did. A screen that dresses an `egui::Slider` by hand is
  writing the same slider for the twenty-third time.
- **A presenter never takes `&mut egui::Ui`.** That is what makes it testable
  with no window. Drawing lives in `view.rs`.
- **`services/` never imports `adapters/`.** It names the trait it needs; the
  wiring is decided above it.
- **`adapters/` is the only place allowed to reach the disk or the clock.**
- **No `utils`, `helpers`, `common`, `misc`, `shared`, `manager`, `handler`.** A
  name that says nothing is where responsibilities come to hide.
- **400 lines per file.** Seventeen files are already over and are named in the
  test with their current length; none of them may grow.

## How work is delivered

One issue, one branch, one pull request. Nothing lands on `main` any other way.

The branch is created **from the issue**, so GitHub links the two and the issue
shows its own branch:

```sh
gh issue develop <n> --base main --name <type>/<n>-<short-description> --checkout
```

**Conventional commit everywhere a name is written** — the branch, every commit
title, and the pull request title. `feat`, `fix`, `refactor`, `perf`, `docs`,
`test`, `build`, `ci`, `chore`, with the crate as scope when there is one.

The prefix does not replace the evocative sentence this repository asks for; it
precedes it. What follows the colon still says what the software can do now:

```
refactor(core): a part archive and a configuration file fail apart
docs/24-storage-error-becomes-two-errors
```

**A pull request always carries a body.** What changed, what was decided and
what was set aside, how it was verified, and `Closes #n`. A pull request with
no body is one nobody can review a month later.

**Stack rather than wait.** An issue whose dependency is still in review
branches off *that* branch and targets it as base, instead of blocking on a
merge. The body names what it sits on.

**Merge a stack from the bottom, and delete nothing on the way.** Deleting the
branch a pull request is based on does not retarget that pull request — GitHub
**closes** it, and a closed pull request can be neither retargeted nor reopened.
The branch and its commits survive; the body and the review thread do not. That
is how #74 had to come back as #78. The order that works:

```sh
gh pr merge <parent> --squash            # no --delete-branch
git checkout <child> && git rebase main  # the squash gave the parent new hashes
git push --force-with-lease
gh pr edit <child> --base main
git push origin --delete <parent-branch> # only now, then repeat one level up
```

Retargeting every child to `main` before merging anything works just as well. A
merge commit or a rebase merge skips the replay step altogether, which is an
argument for not squashing a stack.

**`git rebase --onto main $(git merge-base HEAD @{u})` is a trap** on a branch
level with its upstream: the merge base is `HEAD`, so nothing is replayed and
the branch ends up on `main` with its own commit gone from the tip. The reflog
gets it back. Plain `git rebase main` is what is wanted.

**Three labels carry the state of an issue** — `todo` for ready to start,
`backlog` for waiting on something, `in-progress` for a branch that exists.
Epics carry none of the three: they are tracked by their sub-issues. Labels
rather than a Project board because the `gh` token here has no `read:project`
scope; if that changes, the board replaces them.

## Rules that do not apply here

`~/.claude/*.md` describes a TypeScript stack — Effect, Redux, hexagonal in
`.port.ts` / `.adapter.ts`, kebab-case files, React presenter hooks. **None of
it governs this repository.** The ideas survive the translation; the notation
does not. Where the two disagree, this file and the architecture test win.

## Scope

- Do not anticipate real-time collaboration or the final file format (a function
  tree) while the need is not concrete — those are explicitly deferred
  decisions, see ARCHITECTURE.md.
- Do not add a feature, a crate or an abstraction nobody asked for. This project
  grows by small steps the user asked for explicitly.

## Style

- No comments except for a non-obvious reason (a hidden constraint, a
  workaround). The code should stand on its own.
- Commit and branch freely, to be able to step back if something goes wrong;
  pushing to the remote is fine.
- Keep the documentation spread over several files rather than one, so that
  finding the part that covers a given file stays easy.
- **No line count in prose.** A figure written in a document is exact at the
  commit that writes it and false at the next one — a `cargo fmt` was enough.
  Say the order of magnitude, or name the largest. The only lengths that carry a
  rule live in `crates/app/tests/architecture.rs`, which fails when they drift.
  A counter that *is* the point of a document — the split of `cao_core` in
  `docs/contexts.md`, the wording budget — is the exception, and it earns a test.
- Ask, at any point, when something is unclear, when the request is ambiguous,
  or when information is missing — before acting.
- Before starting a task, pull, and check whether an existing branch already
  covers the request: several people work on this project.
