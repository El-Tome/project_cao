# Agent tooling — design

Date: 2026-09-07
Branch: `refactor/architecture`
State: agreed, ready for the implementation plan

> A record of a decision, kept as it was taken. Every figure, file name and
> branch name below is the reading of the date at the head of it, and is not
> updated afterwards: `esquisse.md` has since become `sketch.md`, and the counts
> have all moved. The amendments dated later are marked where they apply.

## 1. Why

The repository has no agent tooling at all: no `.claude/`, no skill, no hook,
no CI, no toolchain configuration. All that is left is `CLAUDE.md` (37 lines)
and `docs/` (1 881 lines).

That emptiness has measurable effects.

`CLAUDE.md` states rules with no procedure: nothing says *how* to carry out a
task, the tests are never mentioned although the repository holds 176 of them,
and the instruction "pull and check whether a branch already exists" leans on
no tool — so it is forgotten every session.

`docs/` is product prose, not a map of the code. `esquisse.md` describes over
869 lines what the sketch *does*, never *where to act*. No file ties a
behaviour to a module, nor lists the invariants not to break.

And the test coverage, left to discipline alone, has hollowed out exactly where
the risk is greatest:

| Area | Lines | Tests |
| --- | ---: | ---: |
| `sketch/sketch.rs` | 2 372 | 62 |
| `core/state.rs` | 1 280 | 26 |
| `sketch/solver.rs` | 1 470 | **0** |
| `sketch/constraints.rs` | 283 | **0** |
| `crates/app/` (whole) | ~6 400 | **0** |

The constraint solver has no test and gathers four recent fixes
(`fix/solver-anchoring`, `fix/tangent-circles`, `fix/circle-handling`,
`fix/dimension-handling`). That is exactly the profile of an area where a
regression goes unnoticed.

## 2. Baseline measurements

Taken on 2026-09-07 on Apple Silicon, Rust 1.98.1 stable, after installing the
toolchain (absent from the machine until then).

| Command | Duration | Result |
| --- | ---: | --- |
| `cargo fmt --all --check` | < 1 s | **fails** — 90 differences |
| `cargo clippy --workspace --all-targets -- -D warnings` | 13 s | **passes, 0 warning** |
| `cargo test --workspace` | 11 s (36 s cold) | **passes** |

Two consequences for the design.

`clippy` in strict mode already passes on 19 822 lines: the gate can be
uncompromising from day one without breaking anything.

`rustfmt` fails, but repairing it wants a global reformatting commit that would
conflict with the twenty or so remote branches open. One of the 90 differences
is not a fault either: the keyboard mapping table of `app.rs` is compacted by
hand, as a table, and rustfmt bursts it into 26 lines. No `rustfmt.toml`
setting preserves that layout; only `#[rustfmt::skip]` does.

## 3. Decisions

| Question | Decision |
| --- | --- |
| Scope | **Tooling only.** No `.rs`, no `Cargo.toml`, no existing document is modified — except `CLAUDE.md`. |
| Level of constraint | **Blocking hooks**, not rules merely written down. |
| Structure | **Skills specific to this workspace**, not a generic pack imported. |
| Gate layers | **All three**: Claude Code hook, git `pre-commit` hook, GitHub CI. |
| rustfmt | **Outside the gate** for now. `rustfmt.toml` is shipped, the CI checks the format as a non-blocking warning. — *taken up again on 2026-09-08, see below the table* |
| MCP | **`context7` alone.** |

**Decision of 2026-09-08 (#60) — `rustfmt` blocks.** The exemption had a named
reason: the keyboard mapping table of `app.rs`, compacted by hand, which rustfmt
burst into 26 lines, and a global reformatting commit that would have conflicted
with the twenty or so remote branches open at the time (§ 2). That commit has
happened — `4f81da6`, 22 files — and `grep -rn 'rustfmt::skip' crates/` returns
nothing: rustfmt's formatting has been accepted everywhere, there is no
exception left to protect. `cargo fmt --all --check` exits 0 on the current
tree, so putting it back in the gate blocks nothing that exists and costs about
0.3 s per commit. `continue-on-error` disappears from `ci.yml` — with that
option the conclusion of the job was `success`, so a badly formatted branch lit
nothing up anywhere, even declared a required check.

### Why these choices

**Three gate layers, because they do not protect the same people.** A Claude
Code hook intercepts only the commands the agent runs: it sees nothing of a
commit made from a terminal or from JetBrains. The git hook covers every local
commit. The CI covers the remote branch, and therefore the contributors who
have not set up their hooks.

| Layer | Blocks | Bypassable by |
| --- | --- | --- |
| Claude Code hook | the agent | nothing |
| git `pre-commit` hook | every local commit | `git commit --no-verify` |
| GitHub CI | every branch pushed | nothing |

**One single MCP.** The project depends on `egui 0.36`, `wgpu 30` and
`glam 0.33` — crates whose API breaks at every minor version and on which a
model's knowledge drifts. `context7` serves versioned docs and repairs that
precise gap. The MCP servers wrapping `rust-analyzer` are barely maintained and
`cargo check` does better: none is added.

**Skills that know this code.** A generic skill about SOLID is worth nothing. A
skill that says "before touching `solver.rs`, write a test characterising what
is there, because it has none" is worth something. That is the sorting
criterion for everything that follows.

## 4. What is delivered

```
.claude/
├── settings.json                       versioned: hooks + permissions
├── skills/
│   ├── code-map/SKILL.md
│   ├── rust-tdd/SKILL.md
│   ├── architecture-rust/SKILL.md
│   ├── review-rust/SKILL.md
│   └── open-a-task/SKILL.md
├── agents/
│   └── review-architecture-rust.md
└── hooks/
    ├── session-start.sh
    └── gate-commit.sh
.githooks/pre-commit
.mcp.json
rust-toolchain.toml
rustfmt.toml
clippy.toml
.github/workflows/ci.yml
CLAUDE.md                               rebuilt
docs/code-map.md                        new
.gitignore                              + .claude/settings.local.json
```

## 5. The skills

Each skill is a `SKILL.md` with a `name` + `description` frontmatter. The
`description` says **when** to trigger the skill — it is the only field read
before loading, and it must hold the words one would naturally use.

The skills are written in **French**, like `CLAUDE.md` and `docs/`. The code and
the test names stay in English, following the established usage of the
repository (`a_crash_is_written_down_with_its_hour_and_its_stack`). *Reversed on
2026-09-08 (#95): everything a developer reads is English, names included.*

### 5.1 `code-map`

The missing bridge between `docs/` and the 40 files of `crates/`.

Contents: a table behaviour → crate → file → entry function, covering the five
crates. Then the invariants of the project, each with its reason:

- the core computes in `f64`, the camera and the rendering in `f32`, and the
  conversion happens at the last moment at each crossing of the boundary;
- the geometry is never saved: it is replayed from the history by
  `PartState::rebuild`, which makes undo, redo and going back to a step
  identical;
- a new mode is a variant of `Screen` plus a module in `screens/`, never a
  branch added to an existing module;
- `cao_core` depends on no UI crate.

Trigger: as soon as it is a matter of finding where to act in the code.

### 5.2 `rust-tdd`

The red/green/refactor loop applied to this repository.

- **One test at a time.** Never all the tests then all the code: tests written
  in a batch check an imagined behaviour, not the real one.
- **Where to put it.** A colocated `mod tests` at the bottom of the file, the
  convention of the repository; `crates/<crate>/tests/` for integration, on the
  model of `crates/core/tests/stress_tangent.rs`.
- **How to loop fast.** `cargo test -p cao_sketch <filter>` rather than the
  whole workspace.
- **Never `==` between two `f64`.** Comparison by tolerance, with a tolerance
  chosen and justified, not copied.
- **Areas with no net.** `solver.rs` and `constraints.rs` have no test and
  gather four recent fixes; `crates/app/` has none over ~6 400 lines. Any work
  in those files begins with a test that characterises what is there before
  changing it.

### 5.3 `architecture-rust`

SOLID and ports & adapters applied to this workspace.

- The dependency graph allowed between the five crates.
- The rule of ports: no `std::fs`, `directories`, or `chrono::Utc::now()` below
  a domain boundary without going through a trait. The concrete justification:
  the persistence tests write into `std::env::temp_dir()` today and do
  `remove_dir_all`, which makes them slow and not safely parallelisable.
- The Rust pattern for introducing a port: trait, real implementation, test
  implementation.
- **The current gaps, named.** `cao_core` depends on `cao_sketch` and
  `cao_solid`: it is in truth the application layer, not the domain, whatever
  its documentation says. The I/O is hard-coded in `document.rs`, `recents.rs`,
  `settings.rs` and `storage.rs`. Those places are known debt and must never
  serve as a model to copy.

### 5.4 `review-rust`

A review checklist before committing.

- SRP, with figures — `viewport.rs` is 4 179 lines and 105 functions,
  `sketch.rs` 2 372; those are the deterrents.
- OCP on exhaustive `match`es, `PartState::apply` as the example: every new
  operation forces the function open again.
- `unwrap()`, `expect()`, `panic!()` outside test code.
- `pub` placed by default where the field or the function could stay private.
- Errors: `thiserror`, never a variant carrying a free `String`.
- Allocation inside a rendering loop.

### 5.5 `open-a-task`

The procedure `CLAUDE.md` asks for without giving it a tool.

`git fetch`, comparison with the existing remote branches — there are a score
of them — so as not to redo a feature already under way, creation of a `feat/…`
or `fix/…` branch, then the commit message style of the repository: an
evocative French sentence, on the model of "Cercles : poignées, tangences
tenues, plantages tracés".

## 6. The subagent

`.claude/agents/review-architecture-rust.md` — a read-only subagent applying
`review-rust` and `architecture-rust` to a diff and returning a list of findings
ranked by severity. Tools: reading, searching, `cargo` read-only. It changes
nothing.

It exists so that the review happens in a context separate from the writing
context: whoever has just written the code is badly placed to judge it.

## 7. The hooks

All three scripts export `PATH="$HOME/.cargo/bin:$PATH"` at the head: the hooks
run in a non-interactive shell that does not read `~/.zprofile`, and `cargo`
would otherwise be nowhere to be found there.

### 7.1 `session-start.sh` — informative

Trigger: `SessionStart`. Never blocks.

Does a `git fetch --prune`, then shows the current branch, its distance from
`origin/main`, the most recent remote branches, and the state of the working
tree. Reports `cargo` missing where that is the case.

The effect aimed at: the "pull and check the branches" instruction of
`CLAUDE.md` stops depending on the agent's memory.

### 7.2 `gate-commit.sh` — blocking

Trigger: `PreToolUse` on `Bash`. The script reads the JSON on its standard
input and does something only if the command holds `git commit`.

Chains, stopping at the first failure:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`

The format comes first because it is the cheapest — decision § 3.

On failure: exit code 2, the commit is never run, the output of the offending
command is returned to the agent, which fixes it and goes again. The git index
is not touched.

If `cargo` is nowhere to be found: an explicit refusal, with the message
"rustup not installed, the gate cannot be checked", rather than a
`command not found` swallowed.

The valve: if `CAO_SKIP_GATE=1` is in the environment, the gate steps aside and
says so. The `open-a-task` skill carries the instruction that the agent never
sets that variable on its own — it belongs to the human, for their work in
progress.

### 7.3 `.githooks/pre-commit` — blocking, versioned

The same chain, for every local commit whoever its author. Versioned in the
repository, therefore readable and modifiable like the rest of the code.

Enabling, once per clone: `git config core.hooksPath .githooks`. That command
is documented in `CLAUDE.md` and recalled by `session-start.sh` for as long as
it has not been run.

Bypassable with `git commit --no-verify`, which is the expected behaviour of a
git hook and the reason the third layer exists.

## 8. `settings.json`

Versioned, and therefore shared by the team. Declares the two hooks and
pre-authorises the reading and checking commands the agent runs in a loop:
`cargo test`, `cargo clippy`, `cargo check`, `cargo build`, `cargo fmt`,
`git status`, `git diff`, `git log`, `git fetch`.

`.claude/settings.local.json` — personal preferences — is added to the
`.gitignore`.

## 9. `.mcp.json`

Declares `context7` over HTTP, with its API key read from the environment
(`CONTEXT7_API_KEY`), on the model of the plugin already installed globally. A
skill is not needed: the natural trigger is an API question about `egui`,
`wgpu` or `glam`, and `CLAUDE.md` carries the instruction to reach for it
rather than trusting a dated memory.

## 10. Outside `.claude/`

### `rust-toolchain.toml`

Pins `stable` 1.98.1 with the `clippy` and `rustfmt` components, so that
everybody's machine and the CI compile the same code. The 2024 edition of the
workspace wants at least 1.85.

### `rustfmt.toml` and `clippy.toml`

Shipped from day one, while `fmt` was not yet in the gate: their presence fixed
the style aimed at until it got there.

### `.github/workflows/ci.yml`

On every push, whatever the branch. Toolchain pinned by
`rust-toolchain.toml`, `Swatinem/rust-cache` cache.

| Job | Command | Blocking |
| --- | --- | --- |
| `fmt` | `cargo fmt --all --check` | yes |
| `clippy` | `cargo clippy --workspace --all-targets -- -D warnings` | yes |
| `test` | `cargo test --workspace` | yes |
| `build-windows` | `scripts/build-windows.sh` | yes |

Amended on 2026-09-08 (#57): `build-windows` ran only on `main`, and therefore
never before a merge. It now runs on every push; only sending the artefact stays
reserved to `main`.

Amended on 2026-09-08 (#58): `on: push` covered `main` only, so a branch pushed
with no PR open was checked nowhere. It now covers every branch, and
`on: pull_request` disappears in exchange — the two together would run the
workflow twice at every push. What is given up: GitHub used to check the branch
*merged* with its base, it now checks its tip. A branch green but behind `main`
stays possible; the rebase before merging is what catches it.

The Ubuntu runner gets an `apt-get` step installing the system dependencies of
`winit`/`wgpu` (`libxkbcommon-dev`, `libwayland-dev`, `libxcb*`): `eframe` is
compiled with the `x11` and `wayland` features, and without them the CI fails
at linking, not at the tests.

No GPU test is run. The tests of `cao_render` (`camera.rs`, `geometry.rs`,
`cube.rs`) are pure arithmetic and pass with no graphics card.
`examples/offscreen.rs` needs a real `wgpu` device: it is compiled, not run.
The visual check of the three PNGs stays manual, locally.

### `CLAUDE.md` rebuilt

The existing rules are kept word for word. Added to them: the working loop, a
pointer to the skills, the command that enables the git hooks, the instruction
to reach for `context7` for the `egui`/`wgpu`/`glam` APIs.

One correction: `CLAUDE.md` and `ARCHITECTURE.md` present `cao_core` as the
crate of "the domain types". That is false — it depends on `cao_sketch` and
`cao_solid` and orchestrates sketch, solid, history and persistence. It is the
application layer. The wording is corrected in `CLAUDE.md`; `ARCHITECTURE.md` is
not touched, the scope excludes it, and the point is recorded in
`architecture-rust` as known debt.

### `docs/code-map.md`

The reference content of the `code-map` skill, in a form readable by a human
arriving on the project too.

## 11. Out of scope

Explicitly excluded from this task:

- any change to a `.rs` or a `Cargo.toml`;
- the `cargo fmt --all` reformatting commit and the `#[rustfmt::skip]` that go
  with it;
- the architectural refactor: inverting `cao_core`, ports on persistence,
  splitting `viewport.rs`;
- writing the missing tests of the solver and of `crates/app/`;
- the `x86_64-pc-windows-gnu` target and `mingw-w64`, not installed.

Those points are real and documented, but each is a separate task to be decided
on its own.

## 12. Acceptance criteria

1. `.claude/skills/` holds five skills, each with a valid `name` and
   `description` frontmatter.
2. A session opened in the repository shows the git state without being asked.
3. A `git commit` attempted while a test fails is refused, and the failure
   message of the test is visible.
4. The same commit passes once the test is repaired.
5. `CAO_SKIP_GATE=1 git commit` passes, reporting that the gate was skipped.
6. The gate refuses with an explicit message if `cargo` is absent from the
   `PATH`.
7. `git config core.hooksPath .githooks` is enough to make the git hook play.
8. The CI passes on the branch, `fmt` as a warning, `clippy` and `test` green.
9. `cargo test --workspace` and
   `cargo clippy --workspace --all-targets -- -D warnings` stay green: no source
   file has been touched.
