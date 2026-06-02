# Sources

These links informed the GPUI migration discussion. Re-check them before
implementation because GPUI and `gpui-component` are moving targets.

## GPUI

- GPUI crate docs: <https://docs.rs/gpui/latest/gpui/>
- `#[gpui::test]` docs: <https://docs.rs/gpui/latest/gpui/attr.test.html>
- GPUI testing example:
  <https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/testing.rs>
- GPUI test context source:
  <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/test_context.rs>
- GPUI headless screenshot context:
  <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/headless_app_context.rs>
- GPUI visual test context:
  <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/visual_test_context.rs>

Current inspected Zed commit:

```text
de14e3fcad093c2fc74409d2839321bbc6417bdc
```

## GPUI Component

- GPUI Component docs: <https://longbridge.github.io/gpui-component/>
- GPUI Component repository:
  <https://github.com/longbridge/gpui-component>
- Input state source:
  <https://github.com/longbridge/gpui-component/blob/main/crates/ui/src/input/state.rs>
- Input mode source:
  <https://github.com/longbridge/gpui-component/blob/main/crates/ui/src/input/mode.rs>
- Display map source:
  <https://github.com/longbridge/gpui-component/blob/main/crates/ui/src/input/display_map/mod.rs>
- Highlighter source:
  <https://github.com/longbridge/gpui-component/blob/main/crates/ui/src/highlighter/mod.rs>

Current inspected GPUI Component commit:

```text
2d2524d89efc47270e9a0ee18f10fa72cd573eff
```

## Zed Editor And Vim

- Zed editor crate:
  <https://github.com/zed-industries/zed/tree/main/crates/editor>
- Zed editor test context:
  <https://github.com/zed-industries/zed/blob/main/crates/editor/src/test/editor_test_context.rs>
- Zed Vim crate:
  <https://github.com/zed-industries/zed/tree/main/crates/vim>
- Zed Vim tests:
  <https://github.com/zed-industries/zed/blob/main/crates/vim/src/test.rs>
- Zed visual test runner:
  <https://github.com/zed-industries/zed/blob/main/crates/zed/src/visual_test_runner.rs>
- Zed visual test utilities:
  <https://github.com/zed-industries/zed/blob/main/crates/zed/src/zed/visual_tests.rs>
