#!/usr/bin/env sh
# The two local hooks call this script: .claude/hooks/gate-commit.sh and
# .githooks/pre-commit. The CI does not — .github/workflows/ci.yml replays the
# same commands as separate jobs, deliberately: for the per-job annotations, for
# the parallelism, and so that CAO_SKIP_GATE cannot reach them. What keeps the
# two lists from drifting apart is crates/app/tests/gate.rs.
set -u

export PATH="$HOME/.cargo/bin:$PATH"

if [ "${CAO_SKIP_GATE:-}" = "1" ]; then
    printf 'CAO_SKIP_GATE=1 — vérifications sautées. Ce commit passe sans filet.\n' >&2
    exit 0
fi

if ! command -v cargo >/dev/null 2>&1; then
    printf '%s\n' \
        'cargo est introuvable : la toolchain Rust est absente, ou hors du PATH.' \
        'Rien ne peut être vérifié, donc rien n’est certifié.' \
        '' \
        '  Installer :  curl --proto '"'"'=https'"'"' --tlsv1.2 -sSf https://sh.rustup.rs | sh' \
        '  Charger :    . "$HOME/.cargo/env"' \
        '' \
        '  Commiter quand même, en connaissance de cause :' \
        '    CAO_SKIP_GATE=1 git commit ...' >&2
    exit 1
fi

root=$(git rev-parse --show-toplevel 2>/dev/null) || root='.'
cd "$root" || exit 1

etape() {
    titre=$1
    shift
    printf '→ %s\n' "$titre" >&2
    if sortie=$("$@" 2>&1); then
        return 0
    fi
    printf '%s\n' "$sortie" >&2
    printf '\n✗ %s a échoué — le commit est refusé.\n' "$titre" >&2
    printf '  Corrige, puis recommence. Rien n’a été modifié dans le dépôt.\n' >&2
    exit 1
}

etape 'format (cargo fmt --all corrige)' cargo fmt --all --check
etape 'clippy (-D warnings)' cargo clippy --workspace --all-targets -- -D warnings
etape 'cargo test --workspace' cargo test --workspace

printf '✓ format, clippy et tests au vert.\n' >&2
