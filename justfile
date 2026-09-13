_default:
    @just --list --unsorted

init:
    cargo install --locked prek
    cargo install --locked koji
    cargo install --locked mdbook
    cargo install --locked mdbook-mermaid
    prek install

run *args:
    cargo run --features dev {{ args }}

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
    mdbook serve docs --hostname 127.0.0.1 --port 3000 --open

docs-build:
    mdbook build docs

docs-check: docs-build
    typos docs/src
    lychee --offline --no-ignore --include-fragments 'docs/book/**/*.html'

docs-api:
    cargo doc --workspace --no-deps --document-private-items --open
