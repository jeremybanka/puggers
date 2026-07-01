set shell := ["sh", "-eu", "-c"]

default:
    @just --list

# REPO SETUP
prepare:
    git config core.hooksPath .githooks
    chmod +x .githooks/pre-commit

# USE FROM SOURCE
i:
    just install
install:
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
    just test-npm
test-cargo:
    cargo test
test-npm:
    pnpm --filter puggers test

# STATIC ANALYSIS
f:
    just fmt
fmt:
    cargo fmt --all
fmt-cargo-check:
    cargo fmt --all --check
c:
    just check
check:
    just check-scripts
    just check-versions
    just check-cargo
    just check-npm
check-scripts:
    pnpm run check:scripts
check-versions:
    node scripts/check-version-alignment.ts
check-cargo:
    just check-cargo-workspace
    just check-cargo-clippy
    just check-cargo-wasm
check-cargo-workspace:
    cargo check --workspace
check-cargo-clippy:
    cargo clippy --workspace --all-targets -- -D warnings
check-cargo-wasm:
    cargo build -p dprint-plugin-pug --target wasm32-unknown-unknown --release
check-npm:
    pnpm --filter puggers exec tsc

# BUILD SYSTEM
b:
    just build
build:
    just build-cargo
    just build-wasm
    just build-npm
build-cargo:
    cargo build --workspace
build-wasm:
    cargo build -p dprint-plugin-pug --target wasm32-unknown-unknown --release
build-npm *args:
    just build-npm-js
    just build-npm-native {{ args }}
build-npm-js:
    pnpm --filter puggers build
build-npm-native *args:
    just build-npm-native-binaries {{ args }}
    just build-npm-native-emplace {{ args }}
build-npm-native-binaries *args:
    node scripts/build-native-binaries.node.ts {{ args }}
build-npm-native-emplace *args:
    node scripts/assemble.node.ts emplace {{ args }}

# RELEASE SYSTEM
n:
    just notes
notes:
    knope document-change

version:
    knope version

publish-crates:
    just publish-crates-core
    just publish-crates-cli
    just publish-crates-dprint
publish-crates-core:
    cargo publish -p puggers-core
publish-crates-cli:
    cargo publish -p puggers
publish-crates-dprint:
    cargo publish -p dprint-plugin-pug
publish-npm-native *args:
    node scripts/assemble.node.ts dist {{ args }}
    pnpm --dir "$(node scripts/assemble.node.ts print-dist-path {{ args }})" publish --access public
publish-npm-native-provenance *args:
    node scripts/assemble.node.ts dist {{ args }}
    pnpm --dir "$(node scripts/assemble.node.ts print-dist-path {{ args }})" publish --access public --provenance
publish-npm:
    pnpm --filter puggers publish --access public
publish-npm-provenance:
    pnpm --filter puggers publish --access public --provenance
