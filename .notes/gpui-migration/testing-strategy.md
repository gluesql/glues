# Testing Strategy

## Summary

GPUI does not map one-to-one to the current Ratatui snapshot strategy. The
closest practical replacement is semantic snapshot testing:

- Simulate input through GPUI test contexts.
- Serialize app/editor state into stable strings or structs.
- Compare that output in normal tests or snapshots.
- Use screenshots as local human review artifacts first, and later as optional
  CI artifacts or automated visual regression inputs for a small number of
  high-value UI layouts.

## GPUI Test Infrastructure

Observed GPUI/Zed patterns:

- `#[gpui::test]` creates a test app context.
- `TestAppContext` tests entities, actions, async work, globals, clipboard,
  prompts, windows, and task scheduling.
- Current Zed GPUI also exposes `HeadlessAppContext`,
  `PlatformHeadlessRenderer`, and `Window::render_to_image()` under
  `test-support`; this allows PNG screenshot generation from an integration
  test without driving the macOS `screencapture` command.
- `VisualTestContext` adds a window-aware test harness.
- It supports simulated keystrokes, text input, mouse movement/clicks, resize,
  focus, and action dispatch.
- Zed editor and Vim tests mostly assert semantic editor state after simulated
  keystrokes.
- Zed also has a separate visual test runner that captures screenshots and
  compares them against PNG baselines.

## What Replaces TUI Snapshots

Current TUI tests snapshot a terminal buffer. For GPUI, prefer snapshots of
semantic state:

- Visible tree rows.
- Expanded directory ids.
- Selected tree item.
- Focus target.
- Open tabs.
- Active tab.
- Current note id/path/title.
- Editor text.
- Cursor or selection state.
- Dirty/save status.
- Active dialog.
- Notifications.

Example shape:

```text
focus: editor
tabs:
  * note:README
tree:
  v root
    > docs
    - README
editor:
  note: README
  dirty: false
  text:
    # Glues
```

This is not a pixel snapshot, but it captures the app behavior that matters most
for a keyboard-first note app.

## Editor State Snapshot

Zed's editor tests use marked text for cursor and selection state. A similar
format would work well for Glues:

```text
hello ˇworld
```

For selection:

```text
hello «worˇld»
```

This is especially useful if Vim returns later, but it is still useful for a
non-modal editor because it verifies cursor movement, selection, search, and
editing commands.

## Recommended Test Layers

Layer 1: backend contract tests

- Stay in `core`.
- Validate create/read/update/delete/move/sync behavior across backend types.
- These should not know about GPUI.

Layer 2: app model tests

- Test `WorkspaceModel`, `EditorBuffer`, and command execution without drawing.
- Use fake or memory backends.
- This should cover most business behavior.

Layer 3: GPUI interaction tests

- Use `#[gpui::test]` and `VisualTestContext`.
- Simulate key sequences and commands.
- Assert semantic snapshots.

Layer 4: visual review and regression tests

- Initially capture screenshots locally on macOS for human review during GUI
  development, especially after layout, theme, editor, and dialog changes.
- Keep the output easy to inspect side-by-side with the current app state.
- Treat CI screenshot artifacts as a later option, not an initial requirement.
- Use automated pixel/image comparison only for a small set of stable layouts:
  - initial workspace
  - tree + editor split
  - dialog overlay
  - narrow window
  - theme rendering
- Keep automated visual comparison optional or separate from the normal fast
  test path.

## Initial Visual Workflow

The first visual workflow should be local and macOS-focused:

- Run a cargo integration test that launches deterministic GUI states through
  GPUI test support.
- Capture screenshots into a predictable local output directory.
- Leave local `main` vs current-branch comparison as a later option.
- Use this workflow during collaborative development before adding CI visual
  jobs.
- Do not commit visual screenshots or baselines at this stage.
- Keep generated screenshots under ignored output such as
  `target/visual-tests/`.
- Do not fail normal `cargo test` on screenshot differences.

Current command:

```text
cargo test -p glues-gui --features visual-tests --test visual
```

Current output:

```text
target/visual-tests/glues-gui/empty-viewer.png
target/visual-tests/glues-gui/long-note.png
target/visual-tests/glues-gui/open-demo.png
target/visual-tests/glues-gui/open-screen-recents.png
target/visual-tests/glues-gui/workspace-narrow.png
```

This test asserts that the screenshot has a plausible device-pixel size and is
not a uniform image. It does not compare against a committed baseline yet.
The Cargo test target stays as one runner, while individual screenshot cases
live under `gui/tests/visual/*.rs`.

## Local Visual Diff Workflow

The current first-phase workflow intentionally stops at generated screenshots.
Local `main` vs current-branch comparison can be revisited later if visual
review starts needing side-by-side or diff output.

If added later, comparison should reuse the same GPUI screenshot integration
test path instead of launching the real app and using macOS `screencapture`.

Useful review states for early increments:

- Initial shell with memory backend.
- Tree and read-only content split.
- Open note with realistic multiline content.
- Long note names and deep tree nesting.
- Dialog overlay.
- Dirty editor status.
- Narrow window layout.

## Future CI Shape

If visual review moves into CI later, the GUI test pipeline can have separate
jobs with different purposes:

Required fast jobs:

- `core` backend contract tests.
- `gui` model/unit tests.
- `gui` semantic snapshot tests.
- GPUI interaction tests that exercise focus, actions, and keyboard input.

Optional visual artifact job:

- Launch deterministic GUI states in CI.
- Capture screenshots for the agreed review states.
- Fail if the app cannot launch, the window cannot render, or screenshots cannot
  be produced.
- Upload screenshots as CI artifacts.
- Do not initially fail on pixel differences unless the state has been promoted
  to an automated regression baseline.

Optional or later-gated visual regression job:

- Compare a small set of stable screenshots against approved baselines.
- Use this for high-value layouts only.
- Run on demand, nightly, or as a required job once the rendering environment is
  stable enough.

## Screenshot Baseline Strategy

Image snapshots should not be treated exactly like the current TUI `.snap`
files. Terminal snapshots are text and mostly platform-independent. GUI
screenshots are affected by OS, GPU, font rendering, display scale, theme
defaults, and window manager behavior.

There are three useful modes:

Mode 1: local or artifact-only review

- A local script generates screenshots, or CI builds the PR and uploads
  screenshots.
- The job fails only if launch/render/capture fails.
- Reviewers inspect the artifacts manually.
- This should exist from the first GUI shell milestone.

Mode 2: compare against `main`

- CI compares PR screenshots against screenshots generated from `main`.
- This can be done by rebuilding `main` in the same CI environment, or by
  downloading screenshots from the latest successful `main` visual job.
- This avoids committing many PNG baselines.
- It is more complex and slower because it needs two render outputs for the same
  OS/rendering environment.

Mode 3: committed OS-specific baselines

- Store approved screenshots in git for the remote CI environments that are
  actually enforced.
- Organize by platform and rendering profile, for example:
  `gui/tests/visual/baselines/macos/default/*.png` and
  `gui/tests/visual/baselines/linux/default/*.png`.
- This is simpler for required remote CI because the expected images travel with
  the code.
- It should be limited to a small, stable set of screens to avoid repo bloat and
  noisy churn.

Recommended path:

- Start with Mode 1 as a local macOS visual review workflow.
- Keep first-phase screenshots untracked and uncommitted.
- Add CI artifacts only when PR review needs remote screenshots.
- Add Mode 2 if review needs objective diffs before the UI is stable enough for
  committed baselines.
- Promote only stable, high-value states to Mode 3.
- Keep baseline approval explicit. An intentional visual change should update
  the baseline in the same PR that changes the UI.

Local workflow should remain first-class even if remote CI is added:

- Do not assume local machines can compare cleanly against committed CI
  baselines.
- Provide a script that renders both `main` and the current branch in controlled
  local worktrees, then produces side-by-side images or an HTML diff report.
- Let local developers generate screenshots for inspection without updating
  committed baselines.
- Treat committed image baselines as CI-environment-specific unless the local
  environment intentionally matches that profile.

## Human Visual Review

Screenshots are valuable even before they become automated regression tests.
The GUI should start with a local macOS visual review workflow that produces
current screenshots for important states so a person can inspect spacing,
hierarchy, theme balance, clipping, overlap, focus indicators, and overall feel.

Suggested review states:

- Empty or first-run workspace.
- Tree + editor with realistic note names and note content.
- Long names and deep nesting.
- Dirty editor with active save/error status.
- Dialog overlay.
- Narrow window.
- Light and dark themes if both exist.

These review screenshots should be generated locally first. They can later be
generated in CI and uploaded as PR artifacts if that becomes useful. They do not
all need to be committed as baselines.

## Why Not Heavy Automated Pixel Tests

Automated pixel snapshots are costly because fonts, renderer behavior, OS,
display scale, theme changes, and antialiasing can produce noise. They are
useful for layout breakage but poor as the main safety net for app behavior.

For Glues, semantic snapshots should be the main regression guard. Pixel tests
should be rare and targeted.

## Initial Harness Proposal

Create a `GpuiTester` or `WorkspaceTester` helper with:

- `open_memory_workspace(...)`
- `simulate_keys(...)`
- `simulate_text(...)`
- `snapshot_workspace() -> String`
- `snapshot_editor() -> String`
- `assert_snapshot(...)`

This preserves the spirit of the current TUI test harness while fitting GPUI's
test model.
