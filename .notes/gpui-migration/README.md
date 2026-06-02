# GPUI Migration Notes

Date: 2026-05-09

This directory is intentionally ignored by git. It preserves the design analysis
from the Codex thread about replacing the current TUI frontend with a GPUI
desktop frontend.

External GPUI/Zed facts were researched in the thread before this note was
created. Re-check upstream before acting on version-sensitive details.

## Current Direction

The most realistic direction is:

- Keep the current `tui + core` implementation as a development-time reference.
- Build a new GPUI frontend alongside it during development.
- Do not plan for long-term dual frontend support.
- Use the existing backend layer from `core`, but let the GPUI app own its app
  model, focus model, editor integration, dialogs, shortcuts, and workspace UI.
- Move toward a keyboard-first desktop app first, without making Vim modal
  editing part of the first GPUI milestone.
- Treat Vim modal editing as a later optional layer or a separate editor effort.

This means the product can switch to GPUI in one user-visible move, while the
development work still proceeds in controlled milestones.

## Key Documents

- `architecture.md`: Proposed boundary between backend/shared data and GPUI app
  state.
- `migration-plan.md`: Phased read-first implementation plan.
- `implementation-steps.md`: Concrete implementation sequence for the new
  `gui` crate.
- `testing-strategy.md`: How to replace terminal snapshots with GPUI-friendly
  tests.
- `editor-and-vim.md`: Editor component, Vim modal editing, and upstream
  contribution considerations.
- `open-questions.md`: Decisions still worth revisiting before implementation.
- `sources.md`: Upstream GPUI, `gpui-component`, and Zed source links that
  informed the analysis.

## One-Line Summary

Treat the GPUI rewrite as a new keyboard-first desktop frontend over the Glues
backend, not as a direct Ratatui renderer replacement.
