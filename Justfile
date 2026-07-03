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
    just build-dprint-plugin
build-dprint-plugin:
    cargo build -p dprint-plugin-pug --target wasm32-unknown-unknown --release
build-npm *args:
    just build-npm-js
    just build-npm-native {{ args }}
    just build-npm-dprint-plugin
build-npm-js:
    pnpm --filter puggers build
build-npm-native *args:
    just build-npm-native-binaries {{ args }}
    just build-npm-native-copy-binaries {{ args }}
build-npm-native-binaries *args:
    node scripts/build-native-binaries.node.ts {{ args }}
build-npm-native-copy-binaries *args:
    pnpm npm-stage copy-binaries {{ args }}
build-npm-dprint-plugin:
    just build-dprint-plugin
    pnpm npm-stage copy-dprint-plugin
    pnpm npm-stage copy-dprint-plugin --destination=staging
    staging_path="$(pnpm --silent npm-stage print-dprint-plugin-staging-path)"; \
    pnpm --dir "$staging_path" pack --pack-destination "{{ justfile_directory() }}/target/npm"


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
    pnpm npm-stage copy-binaries --destination=staging {{ args }}
    pnpm npm-stage write-manifest {{ args }}
    staging_path="$(pnpm --silent npm-stage print-staging-path {{ args }})"; \
    pnpm --dir $staging_path publish --access public
publish-npm-dprint-plugin:
    just build-npm-dprint-plugin
    staging_path="$(pnpm --silent npm-stage print-dprint-plugin-staging-path)"; \
    pnpm --dir $staging_path publish --access public
publish-npm:
    pnpm --filter puggers publish --access public
