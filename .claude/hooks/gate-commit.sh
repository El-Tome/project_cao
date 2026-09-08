#!/usr/bin/env sh
# PreToolUse on Bash. Does something only when the command is a commit.
set -u

payload=$(cat)

if command -v jq >/dev/null 2>&1; then
    command_line=$(printf '%s' "$payload" | jq -r '.tool_input.command // ""' 2>/dev/null) || command_line=$payload
else
    command_line=$payload
fi

case "$command_line" in
    *'git commit'*) ;;
    *) exit 0 ;;
esac

root=${CLAUDE_PROJECT_DIR:-$(git rev-parse --show-toplevel 2>/dev/null)}
verify="$root/scripts/verify.sh"

if [ ! -x "$verify" ]; then
    printf 'scripts/verify.sh not found or not executable: the gate cannot be checked.\n' >&2
    exit 2
fi

"$verify" || exit 2
