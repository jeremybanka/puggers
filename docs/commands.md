# Repository commands

Run these commands from the repository root with `pnpm run <command>`. `mise.toml` selects the toolchain. Package-level commands keep the same meaning while narrowing their scope.

| Command | Contract |
| --- | --- |
| `fmt` | Apply the repository formatting policy. |
| `check:fmt` | Validate formatting without rewriting maintained files; language-specific validators are listed below. |
| `check` | Run every static check listed below. Generated prerequisites and caches may be written; source fixes are explicit. |
| `test` | Run the normal test suite once and return a failing status when tests fail. |
| `test:watch` | Watch the available interactive test suites. |
| `build` | Build distributable artifacts. |
| `verify` | Run the repository checks, tests, builds, and implemented coverage or compatibility gates. |
| `change` | Author pending release notes. |
| `release:version` | Prepare versions and release metadata without publishing. |

## Static checks

- `check:cargo`: `just check-cargo-workspace`.
- `check:clippy`: `just check-cargo-clippy`.
- `check:fmt`: `just fmt-cargo-check`.
- `check:tsc`: `tsc -p scripts/tsconfig.json && pnpm --filter puggers exec tsc`.
- `check:versions`: `node scripts/check-version-alignment.ts`.
- `check:wasm`: `just check-cargo-wasm`.

## Verification

`pnpm run verify` executes `pnpm run check && pnpm run build && pnpm run test`. CI can run these constituent commands in separate jobs. Check failures must propagate to the caller.

The package scripts expose the shared command names and delegate Rust/native work to Just. `fmt` and `check:fmt` use the existing Rust formatting policy. `test:watch` watches the npm suite; `test` also runs Rust tests. Multi-platform publication remains in the Release workflow. Existing Just recipes remain available.

## Migration

Use `check:fmt` for formatting validation and `check:<tool>` for static checks. Use the canonical commands directly; superseded names have been removed.
