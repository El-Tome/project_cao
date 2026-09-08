#!/usr/bin/env sh
# The two local hooks call this script: .claude/hooks/gate-commit.sh and
# .githooks/pre-commit. The CI does not — .github/workflows/ci.yml replays the
# same commands as separate jobs, deliberately: for the per-job annotations, for
# the parallelism, and so that CAO_SKIP_GATE cannot reach them. What keeps the
# two lists from drifting apart is crates/app/tests/gate.rs.
set -u

export PATH="$HOME/.cargo/bin:$PATH"

if [ "${CAO_SKIP_GATE:-}" = "1" ]; then
    printf 'CAO_SKIP_GATE=1 — checks skipped. This commit goes through without a net.\n' >&2
    exit 0
fi

if ! command -v cargo >/dev/null 2>&1; then
    printf '%s\n' \
        'cargo is nowhere to be found: the Rust toolchain is missing, or off the PATH.' \
        'Nothing can be checked, so nothing is certified.' \
        '' \
        '  Install:  curl --proto '"'"'=https'"'"' --tlsv1.2 -sSf https://sh.rustup.rs | sh' \
        '  Load:     . "$HOME/.cargo/env"' \
        '' \
        '  Commit anyway, knowingly:' \
        '    CAO_SKIP_GATE=1 git commit ...' >&2
    exit 1
fi

root=$(git rev-parse --show-toplevel 2>/dev/null) || root='.'
cd "$root" || exit 1

step() {
    title=$1
    shift
    printf '→ %s\n' "$title" >&2
    if output=$("$@" 2>&1); then
        return 0
    fi
    printf '%s\n' "$output" >&2
    printf '\n✗ %s failed — the commit is refused.\n' "$title" >&2
    printf '  Fix it, then go again. Nothing in the repository was changed.\n' >&2
    exit 1
}

step 'format (cargo fmt --all fixes it)' cargo fmt --all --check
step 'clippy (-D warnings)' cargo clippy --workspace --all-targets -- -D warnings
step 'cargo test --workspace' cargo test --workspace

printf '✓ format, clippy and tests green.\n' >&2
