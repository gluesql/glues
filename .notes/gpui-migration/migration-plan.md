# Migration Plan

## Product Strategy

The intended product strategy is a one-time switch from TUI to GPUI. The
development strategy should still be staged:

- Keep current `tui + core` as the reference implementation while building GPUI.
- Avoid adding new product features to TUI during migration.
- Build GPUI in read-first phases.
- Switch the default binary only after GPUI covers daily usage.
- Remove legacy TUI/core state machine code after the switch.

## Phase 0: Freeze Current TUI

Goal: keep the current app available as a behavior reference.

Work:

- Avoid feature work in `tui`.
- Allow bug fixes only when they affect migration confidence or backend parity.
- Use existing TUI tests as reference while designing GPUI behavior.

Done when:

- New UI/product work is happening in the GPUI path.
- TUI remains runnable for comparison.

## Phase 1: Backend Opening

Goal: make the backend usable without the current `Glues` state machine.

Work:

- Extract or add a backend-opening helper from entry-state logic.
- Ensure memory/file/redb/git/mongo/proxy backends can be opened directly.
- Keep the helper narrow and backend-focused.

Done when:

- A non-TUI caller can open a backend and read notebook data without constructing
  the old app state machine.

## Phase 2: GUI Read-Only Workspace

Goal: create a GPUI app that opens existing backends from the GUI and browses
notebooks read-only.

Work:

- Add a new GPUI frontend crate.
- Build window shell and theme initialization using `gpui-component` primitives
  where practical.
- Implement storage open flow inside the GUI, not as a CLI-first product path.
- Support opening existing real backend data from the GUI open screen.
- Load root/tree/note metadata.
- Display a two-pane workspace: notebook tree plus read-only note content.
- Expand and collapse directories.
- Open notes from the tree.
- Show note content.
- Show loading and open-error states.
- Add local macOS visual review screenshots for this first read-only milestone.
- Defer tabs, breadcrumb/path UI, search, command palette, status-heavy chrome,
  and keyboard shortcut layers unless they become necessary for the first
  read-only workflow.

Done when:

- The GPUI app launches, opens a real backend through GUI controls, and lets the
  user navigate the tree and read notes without touching the TUI.

## Phase 3: Basic Writes

Goal: make the GPUI app useful for daily editing without Vim modal editing.

Work:

- Integrate `gpui-component` editor or a selected editor layer.
- Support plain/code editing.
- Save note content.
- Add note and directory.
- Rename.
- Use simple reload-after-write behavior before optimizing optimistic updates.

Done when:

- Common note editing and creation workflows work in GPUI.

## Phase 4: Full Workspace Parity

Goal: cover the current TUI daily workflows.

Work:

- Remove note/directory.
- Move note/directory.
- Autosave policy.
- Sync status and sync commands.
- Error surfaces.
- Command palette if selected for first release.
- Polish keyboard-first navigation.

Done when:

- The TUI is no longer needed for normal usage.

## Phase 5: Cutover

Goal: make GPUI the default product.

Work:

- Switch `bin/glues` to launch GPUI by default.
- Update README/docs/screenshots.
- Decide whether TUI remains temporarily available through a hidden/dev path.

Done when:

- The user-facing app is GPUI.

## Phase 6: Legacy Removal

Goal: simplify the codebase after the switch.

Work:

- Remove TUI crate if no longer needed.
- Remove legacy `core` state/event/transition/Vim machinery.
- Rename or restructure `core` only if the churn is justified.

Done when:

- The codebase reflects the new architecture instead of carrying both models.
