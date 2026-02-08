# numbers-station Justfile
# Root-level task runner for the project.
#
# Philosophy:
# - Operate from the repo root
# - Keep Rust tooling scoped to rust/numbers-station
# - Prefer explicit paths over implicit cwd assumptions

# -------------------------
# Configuration
# -------------------------

RUST_DIR := rust/numbers-station

# Default recipe
default:
    @just --list

# -------------------------
# Rust: build & run
# -------------------------

build:
    cd {{RUST_DIR}} && cargo build

run:
    cd {{RUST_DIR}} && cargo run

test:
    cd {{RUST_DIR}} && cargo test

check:
    cd {{RUST_DIR}} && cargo check

fmt:
    cd {{RUST_DIR}} && cargo fmt

clippy:
    cd {{RUST_DIR}} && cargo clippy --all-targets --all-features -- -D warnings

clean:
    cd {{RUST_DIR}} && cargo clean

# -------------------------
# Rust: quick iteration helpers
# -------------------------

watch:
    cd {{RUST_DIR}} && cargo watch -x run

test-watch:
    cd {{RUST_DIR}} && cargo watch -x test

# -------------------------
# Project hygiene
# -------------------------

tree:
    tree -L 3

status:
    git status -sb
