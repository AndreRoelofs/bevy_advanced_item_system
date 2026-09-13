#!/usr/bin/env bash

set -uo pipefail

for marker in MERGE_HEAD REBASE_HEAD CHERRY_PICK_HEAD REVERT_HEAD; do
    if [[ -e $(git rev-parse --git-path "$marker") ]]; then
        exit 0
    fi
done

{ exec < /dev/tty; } 2>/dev/null && koji --hook || true
