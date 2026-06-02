# Editor And Vim Notes

## Current Position

The first GPUI version should not require Vim modal editing. The project can
remain keyboard-first without being Vim-modal.

This distinction is important:

- Keyboard-first means all major workflows are usable without a mouse.
- Vim-modal means normal/insert/visual/operator-pending modes, motions,
  text objects, registers, repeat, and Vim-specific editing semantics.

The first GPUI milestone can keep the former and defer the latter.

## Current TUI Editor Situation

The TUI frontend currently uses `edtui`, but there are workarounds around editor
limitations:

- Viewport/scroll control is patched around in the view layer.
- Selection construction needs hacks because the underlying API does not expose
  enough.
- Unicode/CJK-aware word movement is implemented manually.

Moving to GUI can remove some terminal/editor widget limitations, especially
around viewport, focus, overlays, mouse, IME, and rendering. It does not
automatically solve modal editing semantics.

## `gpui-component` Editor

The `gpui-component` editor is not Zed's editor engine. It is a GPUI-based code
editor/input implementation with:

- Rope-backed text storage.
- Display mapping for wrapping/folding.
- Syntax highlighting.
- Search and line numbers.
- Some LSP/completion/diagnostics support.

It is closer to a capable code input/editor widget than a full editor engine
designed for deep external control.

Potential issue for Vim:

- Programmatic selection control may not be sufficiently public.
- Precise viewport recentering may not be sufficiently public.
- Undo grouping may not match Vim command semantics.
- Input routing may need hooks to intercept raw key input before normal editor
  insertion.
- Linewise/blockwise/register semantics would need to be built separately.

## Zed Comparison

Zed's Vim implementation is not proof that Vim can be layered over
`gpui-component` as-is.

Zed has its own `crates/editor` engine, and `crates/vim` integrates deeply with
that engine. Zed can control selections, transactions, cursor shape, editor mode,
input routing, and editor internals in a way a consumer of a public component
usually cannot.

Useful lesson from Zed:

- Vim works well when the editor core exposes deep control points.

Wrong lesson:

- A generic editor widget automatically has enough public API for full Vim.

## Upstream Contribution Direction

If full Vim support becomes a goal over `gpui-component`, the clean path is
likely upstream contribution rather than long-term local hacks.

Potential upstream API needs:

- Public selection getter/setter.
- Public selected text/range access.
- Explicit viewport/reveal control.
- Explicit edit transaction grouping.
- Range replacement APIs with predictable side effects.
- Input interception or mode-specific input enable/disable.
- Stable hooks for external command layers.

Frame this upstream as advanced editor integration support, not only Vim support.
The same APIs help command palettes, macros, structural editing, and test
harnesses.

## Go/No-Go Spike For Vim

If Vim is reconsidered, run a short spike before committing:

- `v` selection extend/shrink.
- `ciw`.
- `dd` plus linewise `p`.
- `zt`, `zz`, `zb`.
- `i hello Esc` followed by one undo operation.

If more than one of these requires private/internal API, assume stock
`gpui-component` is not enough.

## First GPUI Editor Scope

Recommended first scope:

- Plain/code editing.
- Syntax highlight if available.
- Search if easy.
- Line numbers if useful.
- Keyboard shortcuts for save, focus, open, command palette.
- No Vim modal grammar.

This gets the GPUI frontend to product value faster and keeps the editor risk
contained.
