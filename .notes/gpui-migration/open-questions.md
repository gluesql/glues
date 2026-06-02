# Open Questions

## Product Decisions

- Is the GPUI app still positioned as a note-taking app first, or as a general
  notebook/workspace app?
- Should tabs remain in the first GPUI version?
- Should breadcrumb/path UI remain, or should tree selection be enough?
- Should command palette be part of the first usable version?
- Should the first version support only local backends, or all existing backend
  modes?

## Keyboard-First UX

- What are the default focus targets?
- What are the keys for moving focus between tree, editor, tabs, and dialogs?
- Should shortcuts follow common desktop conventions first?
- Which old TUI shortcuts should be preserved?
- Which old TUI shortcuts should be intentionally dropped?

## Write Model

- Should note saves be explicit, automatic, or both?
- If autosave exists, should it be timer-based, blur-based, or command-based?
- Should writes update the local app model optimistically?
- Should phase 1 writes simply reload from backend after each mutation?
- How should save conflicts be surfaced?

## Tree Loading

- Should the tree load eagerly or lazily?
- Should expanded directory state persist?
- Should selected tree item and open tabs persist across launches?

## Testing

- What should the canonical semantic snapshot format be?
- Should snapshots use inline string assertions first, or a snapshot crate?
- Which visual states deserve PNG regression tests?
- Should visual tests be optional/manual at first?

## Editor

- Is `gpui-component` editor enough for first non-modal editing?
- Do we need a thin wrapper around the editor from day one?
- What public APIs are missing for reliable tests?
- At what point do we fork or contribute upstream?

## Cutover

- How long should TUI remain in the repo after GPUI becomes default?
- Should the binary keep a hidden `--tui` path during transition?
- When should legacy `core` state machine code be deleted?
