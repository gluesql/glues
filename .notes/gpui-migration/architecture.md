# Architecture Notes

## Current Shape

The current codebase has a useful separation between `core` and `tui`, but
`core` is not purely domain/backend logic. It currently mixes:

- Backend access and persistence.
- Notebook/domain state.
- App interaction state.
- TUI-oriented keymap/help concepts.
- Vim modal state and command grammar.

Important current files:

- `core/src/backend.rs`
- `core/src/glues.rs`
- `core/src/state.rs`
- `core/src/state/notebook.rs`
- `core/src/transition.rs`
- `tui/src/app.rs`
- `tui/src/context/notebook.rs`
- `tui/src/views/body/notebook/editor.rs`

The `core` backend side is valuable. The `core` UI state machine is much less
valuable if the TUI is being replaced and Vim modal editing is removed from the
first GPUI version.

## Recommended Boundary

Target architecture:

- `core`
  - Backend trait and implementations.
  - Shared data types.
  - Error types.
  - Backend opening helpers.
  - Sync jobs.
- `gpui`
  - App model.
  - Workspace model.
  - Focus model.
  - Tree state.
  - Tabs and active note.
  - Editor buffers and dirty state.
  - Dialogs, notifications, command palette.
  - Keyboard-first shortcuts.

UI construction should prefer composing existing `gpui-component` controls and
layout primitives where they fit, instead of hand-rolling every widget directly
with raw GPUI. The application model, backend commands, loading/error state, and
workspace behavior should remain Glues-owned.

In this direction, GPUI should not depend on the existing `Glues`, `State`,
`Event`, or `Transition` machinery except temporarily if it helps bootstrap.

## GPUI App Model

Suggested model split:

- `AppModel`
  - Runtime config.
  - Backend connection state.
  - Current workspace.
  - Global dialogs.
  - Notifications.
  - Global command dispatch.
- `WorkspaceModel`
  - Root tree snapshot.
  - Expanded directories.
  - Selected tree item.
  - Open tabs.
  - Active tab.
  - Sync/save status.
- `EditorBuffer`
  - `note_id`
  - text
  - dirty flag
  - load/save state
  - last loaded revision or timestamp if available
- `CommandLayer`
  - Open note.
  - Save note.
  - Add note/directory.
  - Rename.
  - Remove.
  - Move.
  - Focus tree/editor/dialog.

## Core Refactoring Implication

If the final product stops shipping the TUI, the existing `core` state machine
can be removed or heavily reduced after GPUI reaches parity.

Likely legacy/removal targets:

- `core/src/glues.rs`
- `core/src/state.rs`
- `core/src/event.rs`
- `core/src/transition.rs`
- `core/src/state/notebook/inner_state/editor/*`
- Vim-specific keymap/help descriptions.

Likely survivors:

- `CoreBackend`
- local backend implementation
- proxy backend implementation
- sync job logic
- shared note/directory/backend types
- error types

## Important Constraint

Do not try to preserve the current `core` state machine as the new source of
truth if GPUI owns focus, dialogs, editor behavior, and keyboard navigation. It
will create a split-brain app model.
