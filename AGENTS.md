# Repository Agent Instructions

## Pre-commit Checklist
- Run `cargo clippy --all-targets -- -D warnings` and ensure it passes.
- Execute `cargo test` to verify the entire workspace.
- Finish with `cargo fmt` across the workspace before committing.
- Use file-based module declarations; do **not** introduce `mod.rs` files.
- Branch names must use lowercase `a`-`z`, dashes (`-`), or slashes (`/`).

## Coverage Workflow
- Collect data with `cargo coverage --no-report`.
- Inspect quick stats via `cargo coverage-report --summary-only`.
- For detailed analysis, generate JSON using `cargo llvm-cov report --json`.
- `--remap-path-prefix` only needs the flag; the workspace root is remapped automatically.

## Snapshot Tests
- Generate new snapshots by running the targeted test, e.g. `cargo test --package <pkg> --test <file> -- --nocapture`.
- Promote `.snap.new` files to `.snap` (commit only finalized `.snap`).
- Re-run the relevant test suite and `cargo test` afterwards.
- Finish by updating coverage (`cargo coverage --no-report`).
- Default to driving scenarios through real input; avoid mutating internal context (`App::context_mut`) from tests unless absolutely necessary.

## GitHub Issue Tips
- Compose multi-line bodies via a heredoc and pass `--body-file` to `gh issue create` / `gh issue edit` to avoid shell parsing issues.

## Personal Instructions
- Store collaborator-specific guidance in `AGENTS.local.md`. This file is gitignored and should remain local to each contributor.
