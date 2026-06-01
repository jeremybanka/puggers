# Puggers Release & Packaging Architecture

Puggers is a Rust workspace with a small family of related Pug tools:

- `puggers-html`: HTML-to-Pug conversion library
- `puggers-cli`: CLI built on top of `puggers-html`
- `dprint-plugin-pug`: dprint formatter plugin

## Product Principles

- one repository
- one coordinated release version
- explicit release notes checked into the repo
- Cargo is the publisher
- release metadata and changelog generation are handled separately from publishing

## Versioning Philosophy

All public crates move together under one shared workspace version.

Benefits:

- simpler release bookkeeping
- clearer compatibility between the CLI and library
- one changelog for the repo instead of three drifting ones
- easier future expansion if this workspace gains npm or editor targets

## Release Orchestration

Puggers uses Knope as the declarative release layer.

Knope is responsible for:

- creating change files in `.changeset/`
- collecting those changes into release notes
- updating the shared workspace version
- updating dependent version references
- maintaining `CHANGELOG.md`

Knope is not the publisher. Publishing stays explicit in `Justfile`.

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
- keep the CLI's dependency on `puggers-html` aligned
- refresh `Cargo.lock`
- append the release entry to `CHANGELOG.md`

### Publishing

After version preparation is reviewed and merged:

```sh
just publish
```

Publish order matters:

1. `puggers-html`
2. `puggers-cli`
3. `dprint-plugin-pug`

That order keeps downstream crates from referencing a version that has not been
published yet.
