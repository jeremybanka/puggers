# Release Notes Workflow

This repository uses [Knope](https://knope.tech/) for declarative release
notes and coordinated versioning.

## Create A Change File

```sh
just notes
```

That command runs `knope document-change` and writes a small markdown file in
`.changeset/` describing the user-facing change and its semver impact.

## Prepare A Release

```sh
just version
```

That command runs `knope version`, which reads the pending change files,
updates the shared workspace version, rewrites dependent version references, and
updates `CHANGELOG.md`.

The GitHub Actions `Prepare Release` workflow runs automatically on pushes to
`main`. It runs the richer `knope prepare-release` bot workflow, which also
commits those changes, pushes `knope/release`, and opens the release pull
request automatically.

## Publish

```sh
just publish
```

Publishing remains explicit and ordered:

1. `puggers-core`
2. `puggers`
3. `dprint-plugin-pug`

Knope is the source of truth for release intent and changelog generation.
Cargo remains the actual publisher.

After the initial manual publish, GitHub Actions publishes with crates.io
trusted publishing instead of a long-lived crates.io token.
