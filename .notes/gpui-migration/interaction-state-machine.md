# Interaction State Machine Notes

Status: working notes, not a committed design.

## Current Thinking

The GPUI frontend should not reuse the existing `core` state machine as its
source of truth. The existing state machine was useful for the TUI, but it mixes
domain data, app state, input grammar, transitions, and TUI-oriented keymap
concepts.

For GPUI, the state machine should not directly own application data.

Instead:

- Workspace data lives in `WorkspaceModel`.
- Editor text and dirty state live in editor buffer state.
- Dialog/opening state lives in the GPUI app model.
- The interaction state machine interprets focus, modes, and pending key
  sequences.
- The interaction state machine emits typed commands.

This keeps mouse, button, tab, tree, menu, and keyboard entry points compatible.

## Why Not Full Typestate

The previous direction tried to make invalid states impossible at compile time
by letting each state own the data it needed. That is attractive in Rust because
the compiler can reject many impossible transitions.

The tradeoff is too high for the GUI shape:

- GUI state has many orthogonal axes.
- Focus, active tab, tree selection, dialog state, drag state, backend state,
  loading, search, dirty buffers, and editor state can combine in many ways.
- Modeling all combinations as compile-time states causes state explosion.
- Adding a new UI entry point requires redesigning the graph instead of adding a
  command.

The practical boundary is to keep data strongly typed, but validate interaction
preconditions at runtime.

## Compile-Time vs Runtime Boundary

Use compile-time types for:

- `NoteId` vs `DirectoryId`.
- Backend configuration shapes.
- Command payloads.
- Workspace/editor model ownership.
- IDs and typed references passed into commands.

Use runtime validation for:

- Whether there is an active note.
- Whether a tree selection is renameable.
- Whether a dialog should capture input.
- Whether a command is allowed in the current focus or interaction mode.
- Whether a pending key sequence should be cancelled by mouse input.

Rejected commands should return explicit rejection reasons rather than silently
doing nothing.

## Proposed Shape

The app should converge toward:

```rust
enum FocusTarget {
    Workspace,
    Tree,
    Editor,
    Tabs,
    Dialog,
}

enum InteractionMode {
    Normal,
    RenamingNote(NoteId),
    RenamingDirectory(DirectoryId),
    MovingNote(NoteId),
    MovingDirectory(DirectoryId),
    Confirming,
}

enum KeySequence {
    Idle,
    Count(usize),
    Prefix(KeyPrefix),
    OperatorPending { op: Operator, count: usize },
}

enum Command {
    OpenNote(NoteId),
    CloseTab(NoteId),
    SelectTab(usize),
    ToggleDirectory(DirectoryId),
    FocusTree,
    FocusEditor,
    RenameSelected,
    MoveSelected,
    SaveActive,
}
```

The key layer converts input into commands or updates `KeySequence`.

Mouse and button actions should call the same command handlers as keyboard
shortcuts. When pointer-driven actions happen, pending key sequences usually
reset to `Idle`.

## Runtime Graph Option

A runtime graph can be useful for key sequence and mode validation:

- Verify there are no duplicate edges.
- Verify all target nodes exist.
- Verify unreachable nodes are intentional.
- Verify documented keymap entries are backed by real commands.
- Verify command preconditions are tested.

Avoid turning this into a fully generic FSM framework unless the app actually
needs it. A small table for key bindings and mode transitions is likely enough.

## Working Principle

State machines remain useful, but their scope should be interaction state, not
the whole application data model.

The data model answers: what exists?

The interaction state answers: how should this input be interpreted right now?

Commands bridge the two.
