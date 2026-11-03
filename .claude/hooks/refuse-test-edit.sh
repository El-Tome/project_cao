#!/usr/bin/env sh
# PreToolUse on Edit and Write, for the implementer subagent only.
set -u

payload=$(cat)

if command -v jq >/dev/null 2>&1; then
    path=$(printf '%s' "$payload" | jq -r '.tool_input.file_path // ""' 2>/dev/null) || path=$payload
else
    path=$payload
fi

case "$path" in
    */tests/*)
        printf '%s\n' \
            "The implementer does not edit tests: $path" \
            'If the test is wrong, return without a change and say why; the test writer decides.' >&2
        exit 2
        ;;
esac

exit 0
