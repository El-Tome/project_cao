# Contributing to CAO

Thank you for wanting to help. CAO is a young 3D design program, and it grows by
small steps, each one argued in an issue before it is written. This page says
how to take part without wasting your time.

By taking part you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Start with an issue

**Open an issue before you open a pull request.** Not to ask permission for a
typo, but because most of what is worth doing here involves a choice — how a
gesture should feel, which crate owns a rule — and the choice is cheaper to make
in a sentence than in a diff.

- **A bug**: use the *Bug report* form. The steps to make it happen again, and a
  `.caopart` file when there is one, are worth more than any theory.
- **A feature**: use the *Feature request* form, and describe the task you are
  stuck on rather than the button you would like.
- **A security problem**: never in a public issue, see [SECURITY.md](SECURITY.md).

Search the closed issues too: the symptom you found may already have been
decided one way or the other.

A maintainer answers with the criteria the work will be judged on — a **Done
when** list — and a label: `todo` (ready to start), `backlog` (waiting on
something) or `in-progress` (someone is on it). **Take an issue that is `todo`
and say so in it**, so that two people do not do the same work.

## Building

```sh
git clone git@github.com:helix-3d/project_cad.git
cd project_cad
git config core.hooksPath .githooks
cargo run -p cao_app
```

The Rust version is pinned by `rust-toolchain.toml`; `rustup` fetches it. See
[docs/build.md](docs/build.md) for Windows and the rest.

## Reading before writing

Before a structural change, read the whole picture:

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — the vision and the split
- [docs/code-layout.md](docs/code-layout.md) — where a new file goes, and what it may import
- [docs/code-map.md](docs/code-map.md) — which file carries which behaviour
- [docs/glossary.md](docs/glossary.md) — the words, and what they mean here

Two rules are checked by tests and will fail your branch if broken:

- **Language.** Everything a developer reads — code, comments, tests,
  documentation, commit messages, branch names, file names — is in English. The
  one exception is the text the user reads in the interface, which is in French
  and lives only in `cao_app`.
- **Modularity.** The crates keep their dependencies and the files keep their
  place and their size. `crates/app/tests/architecture.rs` is the authority.

## Working on an issue

One issue, one branch, one pull request.

1. Make the branch **from the issue**, so that GitHub links the two:

   ```sh
   gh issue develop <n> --base main --name <type>/<n>-<short-description> --checkout
   ```

2. Write the test first. Copy the issue's *Done when* into the module
   documentation of the test that answers it — one bullet per criterion — before
   writing the code. [`rust-tdd`](.claude/skills/rust-tdd/SKILL.md) has the
   detail.
3. Commit as often as you like. **Conventional commits** everywhere a name is
   written — the branch, every commit, the pull request title: `feat`, `fix`,
   `refactor`, `perf`, `docs`, `test`, `build`, `ci`, `chore`, with the crate as
   scope. What follows the colon says what the software can do now:

   ```
   feat(sketch): circles hold their tangencies, and a crash is written down
   fix(sketch): only the origin anchors, and a leaning shape says which way up
   ```

4. `git push` runs `cargo fmt --all --check` and `clippy -D warnings`; if they
   fail, the push does not happen. The tests run in the CI, on every pushed
   branch; run `cargo test --workspace` first if you want them sooner.

## The pull request

Fill in the template. It always has a body, and it opens on whether the change
is **visible to the user**. If it is, say how to try it: the gestures, and what
should be seen — the person who merges will try it in the app first, because
green tests do not reach the screen.

Then say what changed, what was decided and set aside, how it was verified, and
answer each criterion of the issue in turn. End on `Closes #<n>`.

**A maintainer merges, and only after trying it when it shows.** A green CI is
necessary and never enough.

## Licence

A contribution is released under the licence of the project, as the
[README](README.md#licence) states it. Do not send code you are not allowed to
release that way.
