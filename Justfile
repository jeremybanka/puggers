set shell := ["sh", "-eu", "-c"]

default:
    @just --list

# REPO SETUP
prepare:
    just prepare-hooks
prepare-hooks:
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
    just check-npm-types
check-npm-types:
    pnpm --filter puggers exec tsc
check-clippy:
    just check-cargo-clippy
check-wasm:
    just check-cargo-wasm

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
    just build-npm-native-cli
    just build-npm-native-addon
    just build-npm-native-local
build-npm-native-cli:
    cargo build -p puggers --release --locked
build-npm-native-addon:
    cargo build -p puggers-node --release --locked
build-npm-native-local:
    node scripts/npm-native.ts local
build-npm:
    just build-npm-native
    just build-npm-package
build-npm-package:
    pnpm --filter puggers build

# DISTRIBUTION ARTIFACTS
d:
    just dist
dist:
    just dist-npm
dist-npm:
    just dist-npm-native
    just dist-npm-package
dist-npm-native *args:
    just dist-npm-native-directory {{ args }}
    just dist-npm-native-tarball {{ args }}
dist-npm-native-directory *args:
    node scripts/npm-native.ts directory {{ args }}
dist-npm-native-tarball *args:
    node scripts/npm-native.ts tarball {{ args }}
dist-npm-package:
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
    just publish-npm-native-package {{ args }}
publish-npm-native-package *args:
    node scripts/npm-native.ts publish {{ args }}
publish-npm-native-provenance *args:
    PUGGERS_NPM_PROVENANCE=1 just publish-npm-native-package {{ args }}
publish-npm:
    just publish-npm-package
publish-npm-package:
    pnpm --filter puggers publish --access public
publish-npm-provenance:
    just publish-npm-provenance-package
publish-npm-provenance-package:
    pnpm --filter puggers publish --access public --provenance
