#!/usr/bin/env bash

set -euo pipefail

set -x
cargo clippy "$@" --release --all-targets --all-features -- --deny warnings
set +x

if [[ -n "${GITHUB_ACTIONS+x}" || -n "${CLIPPY_ONLY:-}" ]]; then
    exit 0
fi

which cargo-machete >/dev/null 2>&1 || exit 0
cargo machete

which typos >/dev/null 2>&1 || exit 0
typos --config typos.toml
