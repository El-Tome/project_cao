#!/usr/bin/env sh
# The two local hooks call this script before a push: .claude/hooks/gate-push.sh
# and .githooks/pre-push. The tests are not here: they run in the CI only, which
# replays these commands as separate jobs — for the per-job annotations, for the
# parallelism, and so that CAO_SKIP_GATE cannot reach them. What keeps the two
# lists from drifting apart is crates/app/tests/gate.rs.
set -u

export PATH="$HOME/.cargo/bin:$PATH"

if [ "${CAO_SKIP_GATE:-}" = "1" ]; then
    printf 'CAO_SKIP_GATE=1 — checks skipped. This push goes through without a net.\n' >&2
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
        '  Push anyway, knowingly:' \
        '    CAO_SKIP_GATE=1 git push ...' >&2
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
    printf '\n✗ %s failed — the push is refused.\n' "$title" >&2
    printf '  Fix it, then go again. Nothing in the repository was changed.\n' >&2
    exit 1
}

step 'format (cargo fmt --all fixes it)' cargo fmt --all --check
step 'clippy (-D warnings)' cargo clippy --workspace --all-targets -- -D warnings

printf '✓ format and clippy green. The tests run in the CI.\n' >&2
