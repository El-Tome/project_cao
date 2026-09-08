---
name: rust-tdd
description: Write Rust test-first on this CAO repository. Use as soon as a feature is added, a bug is fixed, or the solver, the constraints, the geometry or the booleans are touched — and whenever one wonders where to put a test, how to compare f64, or how to test code with no net.
---

# Test-first, in Rust, here

## The loop

A failing test, the least code that makes it pass, then the next one.

```
RIGHT:  test1 → impl1 → test2 → impl2 → test3 → impl3
WRONG:  test1, test2, test3 → impl1, impl2, impl3
```

Writing every test first and then all the code produces bad tests: in a batch,
one tests an **imagined** behaviour, and ends up checking the *shape* of
things — signatures, structures — rather than what the system does. A test
written just after the matching piece of code knows what actually matters.

A test written **after** all the code is worse still: it validates what exists
instead of what should exist.

**Never refactor while a test is red.** Green first.

## Where to put the test

**Colocated**, at the bottom of the file, is the convention of the repository:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tangent_circle_keeps_touching_after_the_line_moves() {
        // ...
    }
}
```

**Integration**, in `crates/<crate>/tests/`, when the test crosses several
modules or hammers the system. An existing model:
`crates/core/tests/stress_tangent.rs`.

Test names are **complete English sentences** that say the behaviour, not the
function called. That is the established usage:

```
a_crash_is_written_down_with_its_hour_and_its_stack
cutting_a_pocket_takes_matter_away
turning_backwards_still_faces_outwards
```

If the name is not enough to understand the test, rename the test — do not add
a comment. A test is never commented.

## Looping fast

The whole workspace takes ~12 s. During the red/green loop, aim narrower:

```sh
cargo test -p cao_sketch                      # one crate
cargo test -p cao_sketch tangent              # the tests whose name holds "tangent"
cargo test -p cao_core --test stress_tangent  # one integration file
```

The whole workspace once, before committing — the gate will do it anyway.

## `f64`, never with `==`

The core computes in floating point. Two different paths of computation give
two results a hair apart, and `0.1` is not representable in binary.

```rust
// NO
assert_eq!(circle.radius, 12.5);

// YES — with a tolerance chosen, and stated
const TOLERANCE: f64 = 1e-9;
assert!(
    (circle.radius - 12.5).abs() < TOLERANCE,
    "radius expected 12.5, got {}",
    circle.radius,
);
```

Choose the tolerance from what is being measured, do not copy it across: `1e-9`
for a direct geometric comparison, wider after an iterative solver or a run of
boolean operations. A tolerance widened to make a red test pass hides a bug —
that is the moment to stop.

## The places with no net

`sketch/src/solver.rs`, `sketch/src/constraints.rs` and the whole of
`crates/app/` have **no test at all**. The solver alone gathers four recent
fixes.

Touching them goes in three moves:

1. **Characterise first.** Write a test that describes what the code does
   *today*, however shaky, and watch it pass. That is the net.
2. **Then a red test** for the wanted behaviour.
3. **Then the change.** If the characterisation test breaks, the change has a
   side effect — which is exactly what one wanted to learn.

On the solver, prefer **properties** to hard-coded values: after solving, a
tangency still holds, an equality constraint stays true, an entirely
constrained figure no longer moves. The exact numbers of an iterative solver
change at the slightest adjustment; the properties do not.
`crates/core/tests/stress_tangent.rs` gives the model: a pseudo-random
generator hammers a configuration and checks it never falls apart.

## What is tested, and how

| Target | Where | Dependencies |
| --- | --- | --- |
| Pure geometry (`plane`, `construct`, `regions`) | colocated | none |
| Solver, constraints | colocated + properties | none |
| Mesh, booleans | colocated | none |
| `PartState::apply`, history | colocated | none |
| Persistence (`document`, `settings`, `recents`) | colocated | writes to disk — see below |
| Camera, cube, rendering geometry | colocated | pure arithmetic, no GPU |
| wgpu pipelines | not unit tested | visual check through the `offscreen` example |
| The presenter of a screen (`state.rs`) | colocated | none — that is the point |
| The drawing of a screen (`view.rs`), `ui/` primitives | not tested | would want a window |

**On screens:** a screen splits into a presenter (`state.rs`) and a view
(`view.rs`). The presenter never takes `&mut egui::Ui` — the architecture test
refuses it — and that is precisely what makes it testable with no window and no
GPU. When adding a behaviour in `crates/app/`, the question is not "is this
testable", it is "what decides here, and why is it in the file that draws". See
`docs/code-layout.md`.

**On persistence:** those tests write into `std::env::temp_dir()` today and
clean up with `remove_dir_all`. It is slow, and two tests that pick the same
folder tread on each other. If a test is added there, give it a folder with a
unique name. The real answer — a port for the file system — is described in the
`architecture-rust` skill, and is a separate piece of work.

## Before committing

`scripts/verify.sh` runs `cargo fmt --all --check`, `clippy -D warnings` then
`cargo test --workspace`. The gate does it by itself at commit time; running it
by hand first saves a round trip.
