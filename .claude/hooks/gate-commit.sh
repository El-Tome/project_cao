#!/usr/bin/env sh
# PreToolUse sur Bash. Ne fait quelque chose que si la commande est un commit.
# Sortie 2 = l'appel d'outil est refusé et stderr est renvoyé à l'agent.
set -u

charge=$(cat)

if command -v jq >/dev/null 2>&1; then
    commande=$(printf '%s' "$charge" | jq -r '.tool_input.command // ""' 2>/dev/null) || commande=$charge
else
    commande=$charge
fi

case "$commande" in
    *'git commit'*) ;;
    *) exit 0 ;;
esac

racine=${CLAUDE_PROJECT_DIR:-$(git rev-parse --show-toplevel 2>/dev/null)}
verifier="$racine/scripts/verifier.sh"

if [ ! -x "$verifier" ]; then
    printf 'scripts/verifier.sh introuvable ou non exécutable : gate non vérifiable.\n' >&2
    exit 2
fi

"$verifier" || exit 2
