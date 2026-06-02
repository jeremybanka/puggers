# Fixture Corpus

This directory holds regression corpora for `puggers`.

The fixture tree is organized by source first, then by expected test role:

- `upstream/`
  A pinned snapshot of official upstream fixture material. Keep upstream paths
  recognizable and do not hand-edit vendored content except when refreshing the
  snapshot or correcting import layout.
- `hand-authored/idempotence/`
  Project-owned fixtures that should format back to themselves exactly.
- `hand-authored/normalization/`
  Project-owned fixtures that intentionally normalize to a different output.
- `hand-authored/failure/`
  Project-owned fixtures that exercise malformed or unsupported input handling.

Guidelines:

- Prefer upstream fixtures when the behavior already exists in the official Pug
  corpus.
- Add hand-authored fixtures when we need narrower assertions, clearer names,
  or behavior that upstream does not isolate well.
- Keep one feature or behavior slice per file when possible so suites remain
  easy to scan and easy to run.
- Treat checked-in reports such as `upstream/pug/REGISTER.md` as auditable test
  artifacts. If the behavior baseline changes intentionally, update the report
  in the same change.
