# Implementation Plan: `elm-win32` 声明式组件 Key

## Overview

按已评审的 `tasks/spec.md`，为动态 `Column`/`Row` 子列表增加同父节点范围的稳定 key。实现保留现有 Widget preorder、Taffy rect 和 runtime action-map 的位置对应关系，同时修复当前虚拟容器借用父 HWND 导致的所有权歧义。keyed 节点按 key 复用并重排已有 HWND；未 keyed 节点继续按绝对 sibling index 匹配。

本阶段只输出计划，不修改实现。

## Dependency Graph

```text
透明 key 表示与统一 Widget 访问
    │
    └── 可测试的保留节点所有权模型
            │
            └── 同父节点 keyed sibling 匹配
                    │
                    └── Win32 native z-order 重排
                            │
                            ├── runtime 映射资源清理
                            │
                            └── 动态列表示例与文档
                                    │
                                    └── workspace + 实际 UI 验证
```

## Architecture Decisions

### 1. Key 使用透明 Widget 包装，不给每个变体重复加字段

增加透明 keyed 表示，逻辑上仍算一个 Widget 节点：

```rust
TextEdit::new(&item.text).key(item.id.to_string())
```

`key()` 消费 builder/Widget，生成携带 `String` key 的 Widget。布局、控件创建、属性更新和事件映射必须通过统一的“去包装内容”访问器读取原 Widget；key 包装本身不增加 Taffy 节点、NodeTree preorder slot 或 HWND。

选择原因：避免给现有每个 Widget enum 变体和 builder 重复添加 `key: Option<String>`，也避免未来新增控件时忘记复制 key 字段。代价是内部所有直接枚举分派必须明确处理透明包装。

### 2. 保留 flat preorder，但让节点表达所有权和子树边界

不重写 layout/runtime 的 preorder 契约。将内部节点从仅有 `HWND` 提升为能区分虚拟节点和原生节点的记录，建议字段：

```text
NodeInfo
- hwnd: Option<HWND>       // Column/Row/None 为 None，叶子拥有 HWND
- parent_hwnd: HWND        // native z-order 分组
- subtree_len: usize       // 当前 flat preorder 子树范围
```

`subtree_len` 允许 reconciler 安全切分并移动完整旧子树；`Option<HWND>` 防止销毁虚拟 Column/Row 时误销毁它借用的 root HWND。NodeTree 最终仍按新 Widget preorder 排列，所以 layout 和 action map 不需要第二套索引体系。

### 3. 在副作用前验证整棵新树

reconcile 开始时，递归检查每个 `Column`/`Row` 的直接 children：所有 `Some(key)` 必须在该 sibling scope 唯一。重复 key 在 debug/release 中都 panic，并且必须发生在 `std::mem::take`、`DestroyWindow`、创建或更新 HWND 之前，保证失败不会留下半更新原生树。

### 4. Mixed keyed/unkeyed 使用明确的绝对 index 规则

每个新 child 的匹配规则：

- keyed child：只匹配同一父节点下未使用、相同 key 的旧 child；
- unkeyed child：只检查相同绝对 sibling index 的旧 child，且旧 child 也必须 unkeyed；
- 匹配后仍需底层 Widget 类型兼容；不兼容则卸载旧子树并挂载新子树；
- 未匹配旧 child 在所有新 child 完成匹配后按子树卸载。

首版使用 sibling key map 和 used 标记即可，不引入 Reactor 的 LIS。玩具项目优先确定性与可读性；仅当 native HWND 顺序实际改变时才执行一次线性重排，避免每次 render 无条件排序。

### 5. 原生顺序按最近真实 parent 分组

Column/Row 是虚拟容器，当前所有叶子控件实际挂在最近真实 HWND 下。reconcile 完成后，从新 preorder 中过滤 `Some(hwnd)`，按 `parent_hwnd` 分组；若某组相对 HWND 顺序变化，使用 `SetWindowPos` 和前一个 sibling anchor 调整 z-order：

```text
SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
```

不得设置 `SWP_NOZORDER`。官方文档明确 `SetWindowPos` 可改变 child window 的 z-order，`hWndInsertAfter` 指定它在 z-order 中跟随的窗口：

- https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos
- https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features#z-order

首版不调用 `SetParent`，不支持跨真实 parent 移动。微软文档说明 child window 的 parent 关系与 client coordinate、可见性和销毁生命周期绑定；跨 parent 不是普通 sibling reorder。

注意：`SetWindowPos` 证明的是 z-order 调整，不据此宣称 Windows 对话框 Tab 遍历一定改变。`GetNextDlgTabItem` 文档描述其搜索顺序来自对话框模板创建顺序：

- https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlgtabitem

本功能保证已聚焦 HWND 在 same-key reorder 中不被销毁；不额外承诺通用 Tab traversal 重写。

### 6. Reconciler 建立私有 native backend 测试边界

当前 reconciler 直接调用 `widgets::create_hwnd`、`DestroyWindow`、`SendMessageW`、`SetWindowPos`，现有单元测试无法验证真实行为。引入 crate-private backend/test seam，生产实现调用 Win32，测试 fake 生成确定性 HWND 并暴露最终 mounted handle 顺序和销毁结果。

测试断言状态结果：哪些 handle 被保留、销毁、创建，以及最终顺序；不把内部 key-map 调用次数当契约。

### 7. Runtime 映射每次 render 与新树一致

`hwnd_to_action` 继续在 reconcile 后从新树重建。`hwnd_to_colors` 也必须清除 stale HWND，并在清除/覆盖前释放已创建的 GDI brush；否则 keyed 删除会残留颜色映射和资源。该修复限定在 spec 已要求的 stale runtime mapping 清理。

## Task List

## Task 1: 引入透明 keyed Widget 表示

**Description:** 在不改变 preorder slot 数量的前提下加入内部透明 key 包装和统一访问器，并让 layout、widget dispatch、runtime action mapping 与现有 positional reconciler 对包装后的内容保持原行为。暂不启用 keyed sibling 匹配。

**Acceptance criteria:**
- [ ] keyed 包装不新增布局节点、NodeTree slot 或 HWND；`variant_eq`、`children`、bounds/layout 均委托到底层 Widget。
- [ ] 所有现有 Widget builder 和未 keyed 代码保持编译；包装后的叶子仍正确创建、更新和建立事件映射。
- [ ] `Widget::None` 可被包装但仍不创建 HWND。

**Verification:**
- [ ] 先加入失败的透明性测试，再运行：`cargo test -p elm-win32 widget layout runtime`
- [ ] 编译检查：`cargo check --workspace`

**Dependencies:** None

**Files likely touched:**
- `crates/elm-win32/src/widget.rs`
- `crates/elm-win32/src/layout.rs`
- `crates/elm-win32/src/widgets/mod.rs`
- `crates/elm-win32/src/reconciler.rs`
- `crates/elm-win32/src/runtime.rs`

**Estimated scope:** Medium（5 files；一个原子表示变更）

## Task 2: 建立安全可测试的保留节点模型

**Description:** 保留 flat preorder 外部契约，将虚拟节点和原生 HWND 所有权分开，记录子树范围与 native parent，并加入 fake native backend，使子树挂载、更新、卸载可在无真实窗口的单元测试中验证。

**Acceptance criteria:**
- [ ] Column/Row/None 节点不拥有 HWND；卸载虚拟子树只销毁其中真实叶子且每个恰好一次。
- [ ] `subtree_len` 精确覆盖嵌套 preorder 范围，layout/action map 的位置映射保持一致。
- [ ] fake backend 能断言 mounted handles 和销毁集合，生产 backend 保持现有创建、更新、字体设置行为。

**Verification:**
- [ ] 先加入复现虚拟容器错误销毁和多节点子树删除的失败测试，再运行：`cargo test -p elm-win32 reconciler`
- [ ] 编译检查：`cargo check --workspace`

**Dependencies:** Task 1

**Files likely touched:**
- `crates/elm-win32/src/reconciler.rs`
- `crates/elm-win32/src/runtime.rs`

**Estimated scope:** Medium（2 files）

## Checkpoint: Representation and ownership

- [ ] `cargo test -p elm-win32`
- [ ] `cargo check --workspace`
- [ ] 未 keyed gallery 仍可启动并显示原布局

## Task 3: 实现同父节点 keyed sibling 匹配

**Description:** 暴露已确认的 `.key(impl Into<String>)` builder API；在每个 Column/Row scope 预检重复 key，并按 key/绝对 index 规则选择旧子树，实现 keyed 插入、删除、重排的逻辑 handle 复用。

**Acceptance criteria:**
- [ ] same-key/same-type 复用完整旧子树；key 改变或 same-key/different-type 执行卸载后挂载。
- [ ] keyed 插入、删除、反转及 mixed keyed/unkeyed 均符合 `tasks/spec.md`，key 不跨父节点匹配。
- [ ] 新树任意 sibling scope 重复 key 在任何 native 副作用前始终 panic。

**Verification:**
- [ ] 逐项先写失败测试，再运行：`cargo test -p elm-win32 reconciler widget`
- [ ] 编译 public API 示例：`cargo check --workspace`

**Dependencies:** Task 2

**Files likely touched:**
- `crates/elm-win32/src/widget.rs`
- `crates/elm-win32/src/reconciler.rs`

**Estimated scope:** Medium（2 files）

## Task 4: 同步 keyed HWND 原生 z-order

**Description:** 将新 preorder 中每个真实 parent 下的 native HWND 顺序同步到 Win32 z-order；顺序不变时零调用，变化时用非激活、非移动、非缩放的 `SetWindowPos` anchor 链完成重排。

**Acceptance criteria:**
- [ ] fake backend 中 `A,B,C → C,A,B` 保留三个 handle，最终 native 顺序为 `C,A,B`，没有 destroy/create。
- [ ] 插入和删除后的 native 顺序与新 preorder 一致；虚拟容器从不作为 SetWindowPos 目标或 anchor。
- [ ] 顺序未变化不执行重排；重排失败显式失败，不静默留下逻辑/native 顺序分叉。

**Verification:**
- [ ] 先加入失败的 reorder 状态测试，再运行：`cargo test -p elm-win32 reconciler`
- [ ] 编译 Win32 production backend：`cargo check --workspace`

**Dependencies:** Task 3

**Files likely touched:**
- `crates/elm-win32/src/reconciler.rs`

**Estimated scope:** Small（1 file）

## Task 5: 清理删除节点的 runtime 映射资源

**Description:** 使 action/color 映射严格反映 reconcile 后的新树，并在颜色映射移除或重建时释放已有 GDI brush，防止 keyed 删除留下 stale HWND 或资源泄漏。

**Acceptance criteria:**
- [ ] 删除 keyed TextEdit 后，其旧 HWND 不再存在于 action/color map。
- [ ] render 重建颜色映射和 Runtime 销毁时，已创建 brush 恰好释放一次。
- [ ] retained same-key TextEdit 的新回调和颜色来自新 Widget 树。

**Verification:**
- [ ] 先加入失败的映射生命周期测试，再运行：`cargo test -p elm-win32 runtime`
- [ ] 编译检查：`cargo check --workspace`

**Dependencies:** Task 3

**Files likely touched:**
- `crates/elm-win32/src/runtime.rs`

**Estimated scope:** Small（1 file）

## Checkpoint: Core keyed reconciliation

- [ ] `cargo test -p elm-win32`
- [ ] `cargo check --workspace`
- [ ] 插入、删除、反转、类型替换和重复 key 契约全部有行为测试

## Task 6: 添加 keyed 动态列表示例和使用文档

**Description:** 添加最小独立动态列表示例，使用稳定业务 ID 为含 TextEdit 的行设置 key，并提供头部插入、中间删除、反转操作；README 说明 key 的同父作用域、重复 key panic 和 unkeyed 位置语义。

**Acceptance criteria:**
- [ ] 示例可以执行插入、删除、反转，所有消息携带业务 ID 而非可变 index。
- [ ] README 展示 `.key(item.id.to_string())`，明确 key 只需 sibling 唯一且不支持跨父节点移动。
- [ ] 示例构建命令和实际运行命令准确可执行。

**Verification:**
- [ ] 构建示例：`cargo build -p gallery --bins`
- [ ] 启动实际示例并交互检查：聚焦某行 TextEdit，反转/插入后焦点和编辑状态仍跟随该业务 key；删除该行后 HWND 正常销毁。

**Dependencies:** Tasks 4 and 5

**Files likely touched:**
- `examples/src/bin/keyed_list.rs`
- `README.MD`

**Estimated scope:** Small（2 files）

## Task 7: 完成全量验证、审查修复和清理

**Description:** 对完成后的公开 API、行为、文档和示例执行项目级验证，并修复 PR2 审查发现的空 retained tree panic 与 HWND 复用后的 z-order 同步问题。

**Acceptance criteria:**
- [x] `tasks/spec.md` 的全部 success criteria 均有自动化或实际 UI 证据。
- [x] 首次渲染 `Widget::None` 不 panic，虚拟节点正常安装。
- [x] 同 key 类型替换在 HWND 数值复用时仍强制同步父节点 z-order。
- [x] 无临时测试入口、调试输出、未使用兼容 shim 或 stale 计划状态。

**Verification:**
- [x] `cargo fmt --all -- --check`
- [x] `cargo test --workspace`：23 passed
- [x] `cargo build --workspace`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] 新增空视图和 HWND 复用 z-order 回归测试。

**Dependencies:** Task 6

## Checkpoint: Complete

- [x] 所有自动化验证通过
- [x] 实际 Windows keyed TextEdit 列表 smoke 通过
- [x] 公开 API、README、spec、plan、todo 一致
- [x] PR2 审查完成，修复提交 `43e9688` 已推送；等待合并

## Parallelization

核心实现必须顺序执行：Tasks 1 → 2 → 3 → 4。它们共享 `widget.rs`/`reconciler.rs` 的表示和契约，平行编辑会制造冲突并破坏中间可编译性。

Task 5 在 Task 3 后理论上可与 Task 4 并行，因为只修改 `runtime.rs`；但项目较小，顺序执行更易保持验证证据。Task 6 必须等待 Tasks 4、5 的稳定行为。最终验证只运行一次。

## Risks and Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| 透明 key 包装意外增加 preorder/layout slot | High | Task 1 用 layout rect 数和 NodeTree 位置测试固定“一逻辑 Widget 一 slot” |
| 当前虚拟容器记录 root HWND，子树删除误销毁 root | High | Task 2 先用失败测试复现，再改为 `Option<HWND>` 所有权模型 |
| 重复 key panic 发生在部分 Win32 变更之后 | High | reconcile 第一条操作是全树 duplicate preflight，fake backend 断言零副作用 |
| mixed keyed/unkeyed 语义不确定 | Medium | 固定“unkeyed 只匹配相同绝对 sibling index 且双方都无 key”并逐例测试 |
| logical preorder 与 native z-order 分叉 | High | NodeInfo 记录 parent；重排后 fake backend 对最终顺序做状态断言 |
| SetWindowPos 被误认为能保证 Tab traversal | Medium | 只承诺 z-order 与 focus HWND 保留；README 不扩张到未验证的 Tab 契约 |
| keyed 删除残留回调/颜色 brush | Medium | Task 5 重建映射并验证 brush 生命周期 |
| 为减少 moves 引入过度复杂 LIS | Low | 首版只在顺序变化时做线性完整重排，暂不引入 LIS |

## Open Questions

无。若实施发现必须改变 key 作用域、duplicate 行为、公开方法签名或跨 parent 语义，先更新 `tasks/spec.md` 并重新评审，不在代码中临时决定。
