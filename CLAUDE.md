# Rules for this project

Read [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the whole picture before
any structural change, [`docs/contexts.md`](docs/contexts.md) for where the
seams are and where they are going, [`docs/glossary.md`](docs/glossary.md) for
the words, and [`docs/carte-du-code.md`](docs/carte-du-code.md) to find where
things live.

## How to work

Six skills carry the detail, in `.claude/skills/`:

| Skill | When |
| --- | --- |
| `ouvrir-une-tache` | at the very start, before reading any code |
| `carte-du-code` | to find where to act |
| `rust-tdd` | to write the test before the code |
| `refactor-rust` | to move code that already works, without changing it |
| `architecture-rust` | before adding a crate, a module, a dependency, or any I/O |
| `revue-rust` | before committing |

The `revue-archi-rust` subagent reads a diff in a separate context.

On a fresh clone, enable the git hook once:

```sh
git config core.hooksPath .githooks
```

`git commit` then runs `clippy -D warnings` followed by
`cargo test --workspace` (~12 s). On failure the commit does not happen. The
`CAO_SKIP_GATE=1` valve exists for work in progress: it belongs to the human, an
agent never reaches for it on its own.

For an API question on `egui`, `wgpu` or `glam`, use `context7` rather than
memory: this project is on `egui 0.36`, `wgpu 30` and `glam 0.33`, crates whose
API breaks on every minor release.

## Language

Code, test names, documentation, branch names and commit messages are in
**English**.

Text the user reads is in **French**, and lives only in `cao_app`. Layers below
return a named case — `ExtrusionMode::Cut`, not "Enlèvement de matière" — and
the interface decides how it is said. That is what will make translation a
wiring job rather than a rewrite; the i18n system itself is still to come.

Documents and commits written before this rule are in French. They are
translated when touched anyway, never in a sweep of their own, and history is
not rewritten.

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
- Ask, at any point, when something is unclear, when the request is ambiguous,
  or when information is missing — before acting.
- Before starting a task, pull, and check whether an existing branch already
  covers the request: several people work on this project.
