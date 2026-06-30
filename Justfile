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
    just test-npm
test-cargo:
    cargo test
test-npm:
    pnpm --filter puggers test

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
    just check-scripts
    just check-versions
    just check-cargo
    just check-clippy
    just check-wasm
    just check-npm
check-scripts:
    pnpm run check:scripts
check-versions:
    node scripts/check-version-alignment.ts
check-cargo:
    cargo check --workspace
check-clippy:
    cargo clippy --workspace --all-targets -- -D warnings
check-wasm:
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
build-npm-native:
    cargo build -p puggers --release --locked
    cargo build -p puggers-node --release --locked
    node scripts/npm-native.ts local
build-npm:
    just build-npm-native
    pnpm --filter puggers build
package-npm-native *args:
    node scripts/npm-native.ts package {{ args }}
pack-npm-native *args:
    node scripts/npm-native.ts pack {{ args }}
pack-npm:
    pnpm --filter puggers pack --pack-destination target/npm

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
publish-npm-native *args:
    node scripts/npm-native.ts publish {{ args }}
publish-npm-native-provenance *args:
    PUGGERS_NPM_PROVENANCE=1 node scripts/npm-native.ts publish {{ args }}
publish-npm:
    pnpm --filter puggers publish --access public
publish-npm-provenance:
    pnpm --filter puggers publish --access public --provenance
