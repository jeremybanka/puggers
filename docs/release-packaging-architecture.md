# Puggers Release & Packaging Architecture

Puggers is a Rust workspace with a small family of related Pug tools:

- `puggers-core`: shared conversion and utility library
- `puggers`: CLI built on top of `puggers-core`
- `dprint-plugin-pug`: dprint formatter plugin
- `puggers` on npm: ESM TypeScript package with native Node-API bindings
- `@puggers/<target>` on npm: generated native platform packages

## Product Principles

- one repository
- one coordinated release version
- explicit release notes checked into the repo
- Cargo and npm are the publishers
- release metadata and changelog generation are handled separately from publishing

## Versioning Philosophy

All public crates and npm packages move together under one shared workspace
version.

Benefits:

- simpler release bookkeeping
- clearer compatibility between the CLI and library
- one changelog for the repo instead of three drifting ones
- one npm version that corresponds to the same Rust implementation version

## Release Orchestration

Puggers uses Knope as the declarative release layer.

Knope is responsible for:

- creating change files in `.changeset/`
- collecting those changes into release notes
- updating the shared workspace version
- updating dependent version references
- updating the npm package manifests used by `puggers` and the generated native
  package placeholders
- maintaining `CHANGELOG.md`

Knope is not the publisher. Publishing stays explicit in `Justfile`.
GitHub Actions automates the crate publish order for release PRs using
crates.io trusted publishing. npm publishing should use the same prepared Knope
version and publish the native platform packages before the top-level package.

## Release Workflow

### Feature PRs

Developers document user-facing changes with:

```sh
just notes
```

The resulting file in `.changeset/` should describe:

- what changed
- who it affects
- whether the change is patch, minor, or major

### Versioning

When it is time to cut a release:

```sh
just version
```

That runs Knope's release preparation workflow to:

- update `workspace.package.version`
- keep the CLI's dependency on `puggers-core` aligned
- keep npm package manifests aligned with the workspace version
- refresh `Cargo.lock`
- append the release entry to `CHANGELOG.md`

For automated releases, the `Create Release PR` GitHub Actions workflow runs on
pushes to `main` and executes Knope's `prepare-release` workflow with a GitHub
App token and the official `knope-dev/action` installer. That workflow:

- creates or resets the `release` branch
- prepares the version and changelog changes
- commits and pushes the branch
- opens a release pull request against `main`

The workflow skips commits whose message already starts with the release-prep
commit shape so merging the release PR does not immediately create another one.

### Publishing

After version preparation is reviewed and merged:

```sh
just publish-crates
```

Publish order matters:

1. `puggers-core`
2. `puggers`
3. `dprint-plugin-pug`

That order keeps downstream crates from referencing a version that has not been
published yet.

For npm, publish all native platform packages first, then the top-level package:

```sh
just publish-npm-native --target=<target>
just publish-npm
```

The top-level npm package uses `workspace:*` optional dependencies during local
development. pnpm rewrites those dependencies to the matching package versions
when packing or publishing, so the private `packages/native/*` placeholder
versions must stay aligned with the top-level package version.

In CI, the `Release` workflow runs when the `release` pull request merges. It
uses crates.io trusted publishing through GitHub Actions OIDC for the ordered
crate publish steps, then runs `knope release` to create the GitHub release.
The first crate and npm releases still need to be published manually before the
trusted publisher relationships can be used.
