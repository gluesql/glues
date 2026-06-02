# GUI Implementation Steps

## Core State Machine Policy

The new `gui` crate should not use the existing `core` app state machine as a
source of truth.

Allowed from `core`:

- `CoreBackend` and backend implementations.
- Shared `Directory`, `Note`, id, error, and result types.
- Backend opening helpers once they are extracted.
- Sync job primitives.

Avoid in `gui`:

- `Glues`
- `State`
- `Event`
- `Transition`
- `EntryState`
- `NotebookState`
- `core::state::notebook::*` tree/editor/modal types
- Vim/keymap/help descriptions tied to the TUI state machine

The existing state machine can be used as a behavior reference while developing
the GUI, but not as the model that GPUI renders. Reusing it would create two
owners for focus, dialogs, tabs, editor state, and keyboard behavior.

## Frontend Entry Policy

The GUI should not make backend selection a CLI-first workflow. Launching
`glues-gui` should open the desktop app, and storage selection should happen
inside the GUI.

Allowed command-line surface:

- Basic process-level flags if needed later, such as logging or debug capture.
- A developer/test-only shortcut only if it is clearly not the product path.

Avoid as product UX:

- `glues-gui file <path>`
- `glues-gui redb <path>`
- `glues-gui git ...`
- `glues-gui proxy ...`

The first real GUI milestone should include a GUI open screen or open dialog for
selecting storage backends, because existing real data must be inspectable
without falling back to terminal commands.

## First Milestone Scope

Target: read-only GUI over real backends.

The first milestone should stay close to the functionality already exposed by
the current TUI. It is not a product redesign, dashboard, command center, or new
interpretation of notebook state. The goal is simply to prove that the GUI can
open existing Glues backends and browse notes without using the old `core` state
machine.

Work included:

- Launch `glues-gui` into a desktop window.
- Build the visible UI by composing suitable `gpui-component` primitives where
  available.
- Show a GUI open screen before a workspace is loaded.
- Let the user choose from existing backend modes in the GUI:
  - memory
  - file
  - redb
  - git
  - mongo
  - proxy
- Open the selected backend through the extracted `core` backend opening helper.
- Load and render the root notebook tree read-only.
- Expand and collapse directories.
- Select notes and directories.
- Open a note and render its content read-only.
- Show loading and error states in the GUI.
- Provide deterministic demo data only for tests and visual review, not as the
  only product path.

First milestone UI shape:

- An open-workspace screen for selecting and configuring a backend.
- After opening, a two-pane workspace:
  - notebook tree
  - read-only note content
- Minimal visible chrome.
- Only the state needed to understand loading, open errors, selected item, and
  read-only content.

Work excluded:

- A custom widget system unless `gpui-component` lacks a needed primitive.
- Search.
- Recent workspaces.
- Real-time state/status dashboards.
- Sync indicators and sync commands.
- Keyboard shortcut layer.
- Status bar.
- Editing and saving notes.
- Creating, renaming, moving, or deleting items.
- Vim modal editing.
- Command palette.
- Full tab management.
- CLI-driven backend selection as product UX.
- CI visual screenshot jobs.
- Committed visual baselines.

This first milestone is completed by Steps 1 through 6 below. Those steps are
implementation slices, not separate product phases.

## Step 1: Add The `gui` Crate Skeleton

Goal: create a buildable GUI frontend crate without changing the default app.

Work:

- Add `gui/Cargo.toml`.
- Use package name `glues-gui`.
- Use library name `glues_gui`.
- Add a `glues-gui` binary.
- Add `gui` to workspace members.
- Add `glues-gui` to workspace dependencies only if `bin/glues` needs it later.
- Re-check the current GPUI and `gpui-component` versions before pinning
  dependencies.

Done when:

- `cargo check -p glues-gui` works.
- `cargo run -p glues-gui` can launch a minimal window or placeholder app.
- `bin/glues` still launches the TUI by default.

## Step 2: Extract Backend Opening

Goal: let non-TUI callers open storage without constructing `Glues` or
`NotebookState`.

Work:

- Add a narrow backend opening module under `core`, for example
  `core/src/backend/open.rs`.
- Introduce a backend config enum such as:
  - `Memory`
  - `File { path }`
  - `Redb { path }`
  - `Git { path, remote, branch }`
  - `Mongo { conn_str, db_name }`
  - `Proxy { url, auth_token }`
- Add `open_backend(config) -> Result<BackendBox>`.
- Keep the helper backend-only. It should not seed demo notes, select UI state,
  open tabs, or initialize any frontend model.
- Update the server to use the helper instead of its private `build_backend`.
- Update TUI entry opening only if the change stays mechanical and low risk.

Done when:

- A test can open a backend through `open_backend`, fetch the root directory,
  and list root notes/directories without `Glues`.
- Server backend construction no longer duplicates local backend selection.

## Step 3: Define GUI App Models

Goal: create the GUI-owned state that GPUI will render and mutate.

Work:

- Define `AppModel` for global app state:
  - backend connection state
  - active workspace
  - global dialogs
  - notifications
  - command dispatch
- Define `WorkspaceModel` for notebook state:
  - root tree snapshot
  - expanded directory ids
  - selected tree item
  - open tabs
  - active tab
  - focus target
  - save/sync status
- Define `EditorBuffer`:
  - note id
  - text
  - dirty flag
  - load/save status
  - last loaded timestamp or revision if available
- Define a small command layer:
  - open note
  - save note
  - add note/directory
  - rename
  - remove
  - move
  - focus tree/editor/dialog

Done when:

- The model can load a memory backend root tree without creating GPUI elements.
- Basic commands can be unit-tested against a memory backend.

## Step 4: Build Semantic Test Harness

Goal: make GUI behavior testable before visual polish.

Work:

- Add a `WorkspaceTester` or `GuiTester`.
- Support:
  - `open_memory_workspace(...)`
  - `snapshot_workspace()`
  - `snapshot_editor()`
  - command simulation without drawing
- Prefer stable semantic snapshots over pixel snapshots.
- Add a local macOS screenshot review workflow for important GUI states through
  a GPUI screenshot integration test.
- Have the visual test generate PNG screenshots and fail if launch, capture, or
  basic image validation fails.
- Write screenshots under a stable directory such as `target/visual-tests/`.
- Keep first-phase visual screenshots as ignored local artifacts, not committed
  baselines.
- Keep automated pixel comparison rare and targeted.

Done when:

- Tree loading, note opening, tab selection, and dirty editor state can be
  asserted without a rendered window.
- A local macOS workflow can generate visual review screenshots without
  requiring pixel baseline comparison.
- `cargo test -p glues-gui --features visual-tests --test visual` writes review
  screenshots under `target/visual-tests/`.
- Individual visual cases are split under `gui/tests/visual/*.rs` while sharing
  one Cargo test target.
- Local `main` vs current-branch comparison remains a later option.

## Step 5: Add GPUI Shell

Goal: launch the new desktop shell with a real GPUI window.

Work:

- Initialize GPUI app/window.
- Initialize and use `gpui-component` UI primitives for standard controls and
  layout where practical.
- Add basic theme setup.
- Render the app frame with a tree region and content region.
- Add simple error/notification surfaces.
- Start with a GUI-owned open screen or open dialog for choosing storage.
- Add a way to capture current shell screenshots locally on macOS for human
  review.

Done when:

- `glues-gui` opens a window.
- The window can open a backend through GUI controls and show root notebook
  structure.
- A local macOS workflow can generate screenshots of the initial shell for
  visual review.

## Step 6: Complete Read-Only Workspace

Goal: complete the first milestone by making the opened workspace useful as a
simple read-only notebook viewer.

Work:

- Render the directory tree.
- Expand/collapse directories.
- Select notes and directories.
- Open a note from the tree.
- Render note content read-only.
- Keep the UI to tree plus read-only note content unless a tiny path label is
  needed for orientation.

Done when:

- A user can open a real backend from the GUI, browse the tree, and read notes
  without using the TUI.

## Step 7: Basic Editor Integration

Goal: support daily editing without Vim modal semantics.

Work:

- Spike `gpui-component` editor against the first editor scope.
- Wrap the editor behind a thin local abstraction if needed for tests and save
  state.
- Support plain text editing first.
- Add syntax highlighting, search, and line numbers only if they fit cleanly.
- Add standard desktop shortcuts for save and navigation.

Done when:

- A note can be edited in the GUI model.
- Cursor/text state can be tested semantically.
- No Vim modal grammar is required.

## Step 8: Basic Writes

Goal: persist common note operations.

Work:

- Save note content.
- Add note.
- Add directory.
- Rename note/directory.
- Use reload-after-write first, then optimize later if needed.
- Surface save errors in the GUI instead of panicking.

Done when:

- Common note creation and editing workflows work through the GUI.

## Step 9: Workspace Parity

Goal: cover the workflows that still require the TUI.

Work:

- Remove note/directory.
- Move note/directory.
- Add autosave policy if selected.
- Show sync status.
- Run sync commands.
- Add command palette if selected for first release.
- Polish keyboard-first navigation.
- Promote a small set of stable screenshots to OS-specific committed baselines
  if remote CI visual diffs are useful by this point.

Done when:

- Normal daily usage no longer needs the TUI.

## Step 10: Cutover

Goal: make GUI the user-facing default.

Work:

- Change `bin/glues` so running it without a subcommand launches the GUI by
  default.
- Keep a temporary `tui` or `--tui` path only if it is useful for migration.
- Update README, screenshots, install notes, and release notes.
- Re-run full workspace checks.

Done when:

- `glues` launches the GUI by default.
- The TUI is no longer the product path.

## Step 11: Legacy Removal

Goal: remove the old model once GUI is stable.

Work:

- Remove the TUI crate if it is no longer needed.
- Remove `core` state/event/transition/Vim machinery.
- Keep backend/data/error/sync code.
- Rename or restructure `core` only if the churn is justified.

Done when:

- The repository no longer carries two frontend app models.
