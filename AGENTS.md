# Repository Agent Instructions

- Always run `cargo clippy --all-targets -- -D warnings` before committing and ensure it passes.
- Run `cargo test` to verify all tests pass.
- After clippy and tests succeed, run `cargo fmt` on the entire workspace before committing changes.
