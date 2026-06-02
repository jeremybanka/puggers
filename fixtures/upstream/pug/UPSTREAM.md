# Upstream Pug Fixtures

This directory vendors official fixtures and examples from the upstream
[`pugjs/pug`](https://github.com/pugjs/pug) repository.

Imported snapshot:

- Upstream repository: `https://github.com/pugjs/pug`
- Upstream tag: `pug@3.0.4`
- Upstream commit: `c323ed3e630b931bc04790efb509d34ac5927040`
- Imported on: `2026-06-02`

Vendored source directories:

- `packages/pug/examples`
- `packages/pug/test`
- `packages/pug-lexer/test`
- `packages/pug-filters/test`
- `packages/pug-linker/test`

Vendored upstream license files:

- `packages/pug/LICENSE`
- `packages/pug-lexer/LICENSE`
- `packages/pug-filters/LICENSE`
- `packages/pug-linker/LICENSE`

Why this is vendored instead of tracked as a git submodule:

- Tests stay stable and self-contained inside this repository.
- Contributors do not need submodule setup to run or inspect fixtures.
- Fixture diffs stay reviewable in the same pull request as formatter changes.

License notes:

- The vendored upstream packages identify their license as MIT.
- The corresponding upstream package `LICENSE` files are copied into this tree
  so the license notice travels with the vendored material.
- If we later vendor fixtures from additional upstream Pug packages, copy their
  package-level `LICENSE` files too and list them above.

Organization policy:

- Preserve upstream relative paths under `fixtures/upstream/pug/packages/...`.
- Treat this tree as a pinned snapshot, not a hand-edited working area.
- Add project-specific fixtures outside this tree so upstream material remains
  easy to diff against future refreshes.
- When refreshing from upstream, record the new tag or commit here and keep the
  imported directory list current.
