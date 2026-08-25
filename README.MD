# elm-win32

玩具项目：基于 The Elm Architecture (TEA) 的声明式 Win32 原生 GUI 框架。

## 特性

- 🦀 **The Elm Architecture (TEA)**：纯函数式状态管理（Model / Msg / Update / View）。
- 📐 **Taffy 布局引擎**：内置 `taffy` 现代 Flexbox 布局（支持 `Row`、`Column`、`spacing`、`padding`、`flex_grow`、`align_items` 等），告别手动绝对坐标计算。
- 📏 **固有尺寸测量 (Intrinsic Sizing)**：基于 GDI 字体文本测量，按钮、标签、复选框等未指定宽高时自动适配内容大小。
- 🪟 **Win32 原生控件**：Button, TextEdit, CheckBox, RadioButton, ListBox, ComboBox, ComboBoxEx, DateTime, Header, TabControl, Toolbar, Trackbar, GroupBox。
- ⚡ **响应式自适应**：窗口缩放（`WM_SIZE`）与系统 DPI 变更（`WM_DPICHANGED`）自动重新解算布局。

## 动态列表与 key

动态、有状态的子列表应使用稳定业务 ID 设置 key：

```rust
Column::new()
    .push(
        TextEdit::new(&item.text)
            .on_change(Msg::Edit(item.id))
            .key(item.id.to_string()),
    )
```

`.key(...)` 只在同一 `Column`/`Row` 的兄弟节点中匹配。插入、删除或重排时，相同 key 会复用原生控件；key 改变或 Widget 类型改变会重新创建控件。同一父节点重复 key 会 panic。未设置 key 的节点继续按位置匹配，因此回调消息应携带稳定业务 ID，不应依赖列表 index。

运行 keyed 动态列表示例：

```bash
cargo run -p gallery --bin keyed_list
```

## 运行示例

```bash
cargo run --bin gallery
```
