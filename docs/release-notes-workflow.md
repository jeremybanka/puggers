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
updates `CHANGELOG.md`. The same version is applied to the Rust workspace, the
top-level npm package, and the native platform packages used for pnpm's local
`workspace:*` resolution.

The GitHub Actions `Create Release PR` workflow runs automatically on pushes to
`main`. It follows Knope's pull-request-driven recipe directly: install Knope
with `knope-dev/action`, run `knope prepare-release`, commit the generated
changes, push the `release` branch, and keep the release pull request updated
automatically.

## Publish

```sh
just publish-crates
```

Crate publishing remains explicit and ordered:

1. `puggers-core`
2. `puggers`
3. `dprint-plugin-pug`

Knope is the source of truth for release intent and changelog generation.
Cargo and npm remain the actual publishers.

npm publishing is also ordered. Publish every generated native platform package
for the release version before publishing the top-level `puggers` package:

```sh
just publish-npm-native --target=<target>
just publish-npm
```

The merged `release` pull request triggers the `Release` workflow. It calls the
shared npm package-group workflow with publishing enabled, publishes the Rust
crates with crates.io trusted publishing after npm succeeds, and then runs
`knope release`. npm provenance comes from trusted publishing through GitHub
Actions OIDC once each npm package has a trusted publisher entry for the calling
`.github/workflows/release.yml`.
