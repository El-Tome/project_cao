#!/usr/bin/env sh
# SessionStart. N'échoue jamais : sa sortie standard devient du contexte.
set -u

export PATH="$HOME/.cargo/bin:$PATH"
cd "${CLAUDE_PROJECT_DIR:-.}" 2>/dev/null || exit 0
git rev-parse --git-dir >/dev/null 2>&1 || exit 0

git fetch --prune --quiet 2>/dev/null

branche=$(git rev-parse --abbrev-ref HEAD 2>/dev/null)
printf 'État du dépôt\n'
printf '  branche : %s\n' "$branche"

if git rev-parse --verify --quiet origin/main >/dev/null 2>&1; then
    devant=$(git rev-list --count origin/main..HEAD 2>/dev/null)
    derriere=$(git rev-list --count HEAD..origin/main 2>/dev/null)
    printf '  vs origin/main : %s commit(s) devant, %s derrière\n' "$devant" "$derriere"
    [ "${derriere:-0}" -gt 0 ] && printf '  ⚠ en retard sur origin/main — un `git pull` s’impose avant de commencer.\n'
fi

sales=$(git status --porcelain 2>/dev/null | wc -l | tr -d ' ')
printf '  travail non commité : %s fichier(s)\n' "$sales"

printf '  branches distantes récentes :\n'
git for-each-ref --sort=-committerdate --count=6 \
    --format='    %(committerdate:short)  %(refname:short)' refs/remotes/origin 2>/dev/null \
    | command grep -vE '  origin$'

printf '\nAvant toute tâche : vérifier qu’aucune de ces branches ne fait déjà le travail demandé.\n'

if [ "$(git config core.hooksPath 2>/dev/null)" != '.githooks' ]; then
    printf '\n⚠ Le hook git n’est pas activé sur ce clone. Une fois pour toutes :\n'
    printf '    git config core.hooksPath .githooks\n'
fi

command -v cargo >/dev/null 2>&1 \
    || printf '\n⚠ cargo introuvable : rien ne pourra être compilé ni testé.\n'
