_default:
    @just --list --unsorted

init:
    cargo install --locked prek
    cargo install --locked koji
    cargo install --locked mdbook --version 0.5.4
    cargo install --locked mdbook-mermaid
    prek install

run *args:
    cargo run {{ args }}

commit:
    koji

check: lint check-docs test

lint:
    cargo fmt --all --check
    scripts/clippy.sh

check-docs:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

test:
    cargo test

fix:
    cargo fmt --all
    scripts/clippy.sh --fix --allow-dirty
    typos --write-changes

docs:
    mdbook serve website --hostname 127.0.0.1 --port 3000 --open

docs-build:
    mdbook build website

docs-check: docs-build
    #!/usr/bin/env bash
    set -euo pipefail
    typos website/src
    # Mirror the Pages URL prefix so root-relative links resolve locally.
    site_root="$(mktemp -d)"
    trap 'rm -rf "$site_root"' EXIT
    ln -s "$PWD/website/book" "$site_root/bevy_advanced_item_system"
    lychee --offline --no-ignore --include-fragments --root-dir "$site_root" --index-files index.html 'website/book/**/*.html'

docs-api:
    cargo doc --workspace --no-deps --document-private-items --open
