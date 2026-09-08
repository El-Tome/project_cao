//! The local gate and the CI hold two independent definitions of "the
//! repository is fine". They agree; nothing but this file keeps them agreeing.
//!
//! Merging them is not the answer — the CI has reasons to run the commands as
//! separate jobs rather than call `scripts/verifier.sh`: per-job annotations,
//! parallelism, and a `CAO_SKIP_GATE` that must not reach it. So the divergence
//! is made to fail a test instead, the way the architecture rules already are.
//!
//! The deliberate differences are named below. An intended gap stays visible;
//! an accidental one does not survive.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Steps that prepare a bare runner rather than check anything. A developer's
/// machine already has what they install.
const CI_STEPS_THAT_ONLY_PREPARE_THE_RUNNER: [&str; 3] = [
    "./scripts/ci-deps-ubuntu.sh",
    "sudo apt-get update && sudo apt-get install -y --no-install-recommends mingw-w64",
    "rustup target add x86_64-pc-windows-gnu",
];

/// What the CI checks and the local gate deliberately does not. Cross-compiling
/// for Windows wants `mingw-w64` and the target installed, which is more than a
/// commit should ask of every machine.
const CHECKS_THE_LOCAL_GATE_LEAVES_TO_THE_CI: [&str; 1] = ["./scripts/build-windows.sh"];

#[test]
fn the_local_gate_and_the_ci_check_the_same_things() {
    let ci = commands_the_ci_runs();
    let preparation: BTreeSet<String> = CI_STEPS_THAT_ONLY_PREPARE_THE_RUNNER
        .iter()
        .map(|step| step.to_string())
        .collect();

    let stale: Vec<&String> = preparation.difference(&ci).collect();
    assert!(
        stale.is_empty(),
        "CI_STEPS_THAT_ONLY_PREPARE_THE_RUNNER names steps the workflow no longer runs: {stale:?}",
    );

    let checked_by_the_ci: BTreeSet<String> = ci.difference(&preparation).cloned().collect();
    let expected: BTreeSet<String> = commands_the_verifier_runs()
        .into_iter()
        .chain(
            CHECKS_THE_LOCAL_GATE_LEAVES_TO_THE_CI
                .iter()
                .map(|step| step.to_string()),
        )
        .collect();

    assert_eq!(
        checked_by_the_ci, expected,
        "scripts/verifier.sh and .github/workflows/ci.yml no longer check the same things: \
         a step added to one belongs in the other, or in one of the two lists at the top of \
         this file that name what is meant to differ",
    );
}

fn commands_the_verifier_runs() -> BTreeSet<String> {
    let script = read("scripts/verifier.sh");
    let mut commands = BTreeSet::new();

    for line in script.lines() {
        let Some(rest) = line.trim().strip_prefix("etape ") else {
            continue;
        };
        let Some((_, command)) = rest.trim_start_matches('\'').split_once("' ") else {
            panic!("an etape line whose title is not in single quotes: {line}");
        };
        commands.insert(command.trim().to_string());
    }

    assert!(!commands.is_empty(), "scripts/verifier.sh runs no etape");
    commands
}

fn commands_the_ci_runs() -> BTreeSet<String> {
    let workflow = read(".github/workflows/ci.yml");
    let mut commands = BTreeSet::new();

    for line in workflow.lines() {
        let Some(command) = line.trim().strip_prefix("- run: ") else {
            continue;
        };
        let command = command.trim();
        assert!(
            !command.starts_with('|') && !command.starts_with('>'),
            "a multi-line run: block — this test reads one command per step",
        );
        commands.insert(command.to_string());
    }

    assert!(!commands.is_empty(), "the workflow runs no command");
    commands
}

fn read(path: &str) -> String {
    let path = workspace_root().join(path);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("a readable {}", path.display()))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root, two levels above crates/app")
        .to_path_buf()
}
