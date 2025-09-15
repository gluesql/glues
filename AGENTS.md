# Repository Agent Instructions

- Always run `cargo clippy --all-targets -- -D warnings` before committing and ensure it passes.
- Run `cargo test` to verify all tests pass.
- After clippy and tests succeed, run `cargo fmt` on the entire workspace before committing changes.
- Use file-based module declarations; do **not** create `mod.rs` files.
- Branch names may contain only lowercase `a`-`z`, dashes (`-`), and slashes (`/`).
- Insta snapshots: commit only `.snap`; never commit `.snap.new`.
