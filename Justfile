set shell := ["sh", "-eu", "-c"]

default:
    @just --list

# USE FROM SOURCE
i:
    just install
install:
    just install-cargo
install-cargo:
    cargo install --path ./crates/puggers-cli

r:
    just run
run *args:
    cargo run -p puggers --bin puggers -- {{ args }}

# TEST
t:
    just test
test:
    just test-cargo
test-cargo:
    cargo test

# STATIC ANALYSIS
f:
    just fmt
fmt:
    just fmt-cargo
fmt-cargo:
    cargo fmt --all
fmt-cargo-check:
    cargo fmt --all --check
c:
    just check
check:
    just check-cargo
    just check-clippy
    just check-wasm
check-cargo:
    cargo check --workspace
check-clippy:
    cargo clippy --workspace --all-targets -- -D warnings
check-wasm:
    cargo build -p dprint-plugin-pug --target wasm32-unknown-unknown --release

# BUILD SYSTEM
b:
    just build
build:
    just build-cargo
    just build-wasm
build-cargo:
    cargo build --workspace
build-wasm:
    cargo build -p dprint-plugin-pug --target wasm32-unknown-unknown --release

# RELEASE SYSTEM
n:
    just notes
notes:
    knope document-change

version:
    knope version

publish:
    just publish-crates

publish-crates:
    cargo publish -p puggers-core
    cargo publish -p puggers
    cargo publish -p dprint-plugin-pug
