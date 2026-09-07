---
name: revue-rust
description: Review Rust before committing it on this CAO repository. Use before any commit, when reading a diff, when wondering whether a file has grown too large, or to check that a change respects the project's rules.
---

# Reviewing before committing

Review the **diff**, not the file. For each point below the question is "does my
change introduce this", not "does the repository contain this".

## Size and responsibility

The repellents, to calibrate against: `viewport.rs` is 4 234 lines and 105
functions, `sketch.rs` 2 384, `solver.rs` 1 466, `state.rs` 1 297. All of
`crates/app/src` is 7 454 lines with one test file.

- [ ] Is the file I touched growing again? If so, could what I am adding live
      elsewhere?
- [ ] Does a function run past a screen? Does it do more than one thing?
- [ ] Did I add a parameter to a function that already had five? That is usually
      a missing struct.
- [ ] Does a new type carry fields that only matter half the time? That is two
      types, not one.

## What should raise an eyebrow

- [ ] `unwrap()`, `expect()` or `panic!()` outside test code. Inside a test,
      `expect("the file exists")` is normal and wanted.
- [ ] `pub` added by reflex. A field or function not used outside its module
      stays private — that is what lets it change later.
- [ ] `==` between two `f64`. Always a tolerance, chosen and justified.
- [ ] `as f32` or `as f64` far from a boundary. The conversion happens at the
      last moment, crossing to the GPU or to `egui`.
- [ ] An allocation (`Vec::new`, `to_string`, `collect`) inside a paint or
      hit-test loop that runs every frame.
- [ ] `clone()` to quiet the borrow checker. Often a reference does; otherwise
      the split is wrong.
- [ ] An error variant carrying a free `String` — the caller can decide nothing.
      See `architecture-rust`.
- [ ] An `enum` of constants replaced by bare integers or strings.

## The architecture

The gate runs `crates/app/tests/architecture.rs`, so a violation of the crate
graph fails on its own. What the test cannot see:

- [ ] Did a geometry rule end up in `crates/app/` rather than in `cao_sketch` or
      `cao_solid`? The test checks dependencies, not where a rule lives.
- [ ] Is a new mode a variant of `Screen`, or a branch grafted somewhere else?
- [ ] **Did a ratchet figure go up?** Lowering one is a result; raising one
      empties the file of meaning and is a decision for the human.

## The project's rules

- [ ] **No comments**, except for what the code cannot say: a constraint
      invisible from the file, a discarded alternative that would be tried again
      without the note, a rule coming from outside the code. A paraphrase of the
      line below is deleted, not rewritten. The story of your own change goes in
      the commit message.
- [ ] **Never a comment on a test.** The test name is the sentence.
- [ ] Code, test names, documentation, branch names and commit messages in
      **English**. Text the user reads in **French**, and only in `cao_app`.
- [ ] No new French wording below `cao_app` — the layer underneath returns a
      named case.
- [ ] `cao_core` imports no interface crate.
- [ ] No feature, crate or abstraction that was not asked for. This project
      grows by small, explicitly requested steps.

## The tests

- [ ] Does the added behaviour have a test? If it touches `solver.rs`,
      `constraints.rs` or `crates/app/`, is there first a test that
      characterises what exists?
- [ ] Does the test describe an observable **behaviour** through the public
      interface, or the implementation? A test that breaks when a private
      function is renamed was testing the wrong thing.
- [ ] Is the test name a sentence saying what the software does?
- [ ] Was a tolerance widened to make a red test pass? If so there is a bug
      underneath — stop there.
- [ ] If this was a refactor: did any test's assertions change? They should not
      have. See `refactor-rust`.

## Before validating

```sh
scripts/verifier.sh
```

`clippy -D warnings` then `cargo test --workspace`, ~12 s. The gate will do it
again at commit time, but running it first saves a round trip.

For an independent read, the `revue-archi-rust` subagent applies this skill and
`architecture-rust` to a diff, in a separate context — whoever just wrote the
code is badly placed to judge it.
