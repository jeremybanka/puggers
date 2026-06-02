# AGENTS

## Task Process

- Tasks live in `tasks/open`, `tasks/review`, and `tasks/closed`.
- New tasks should start from `tasks/issue-template.pug` and use a zero-padded id like `0002`.
- A task in `tasks/open` should use `issue(... status="open")`.
- When implementation is ready for another person to look at, move the task file to `tasks/review` and update it to `issue(... status="review")`.
- When review is complete and the work is accepted, move the task file to `tasks/closed` and update it to `issue(... status="closed")`.
- Keep acceptance criteria concrete and testable.
- Keep `review-items` focused on the remaining review conversation, not as a duplicate backlog.

## Testing Policy

- Prefer focused test suites in separate files instead of growing large inline test modules.
- For Rust integration coverage, put tests under the owning crate's `tests/` directory, for example `crates/puggers-core/tests/html_import.rs`.
- Use one file per feature or behavior slice so suites stay easy to scan and easy to run.
- Prefer names like `tests/<feature>.rs` over `tests/<feature>.test.rs`.
- `tests/<feature>.test.rs` may be technically possible, but it adds no value here and leads to less clean target names and conventions.
- Keep test inputs and assertions narrow. If a behavior needs broad corpus coverage, give that suite a descriptive name and document the fixture source nearby.
- Follow a red-green workflow for task-defining behavior changes: land focused failing tests that express the intended behavior, commit that red state, then implement the code that turns the tests green in a later commit.
- Run `cargo test` for normal verification.
- Before landing broader Rust changes, also run `cargo fmt --all`, `cargo check --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo build -p dprint-plugin-pug --target wasm32-unknown-unknown --release` when the affected area justifies it.
