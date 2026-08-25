# Key Feature Task Checklist

规格：`tasks/spec.md`  
实施计划：`tasks/plan.md`

## Phase 1: Representation and Ownership

- [x] Task 1: 引入透明 keyed Widget 表示
- [x] Task 2: 建立安全可测试的保留节点模型

### Checkpoint: Representation and Ownership

- [x] `cargo test -p elm-win32`
- [x] `cargo check --workspace`
- [x] 未 keyed gallery 编译通过

## Phase 2: Keyed Reconciliation

- [x] Task 3: 实现同父节点 keyed sibling 匹配
- [x] Task 4: 同步 keyed HWND 原生 z-order
- [x] Task 5: 清理删除节点的 runtime 映射资源

### Checkpoint: Core Keyed Reconciliation

- [x] `cargo test -p elm-win32`
- [x] `cargo check --workspace`
- [x] 插入、删除、反转、类型替换、重复 key 均有行为测试

## Phase 3: Example and Verification

- [x] Task 6: 添加 keyed 动态列表示例和使用文档
  - `cargo build -p gallery --bins` 通过；README 已加入 `.key(...)` 契约和运行命令。
  - 实际 smoke：启动 `keyed_list.exe`，枚举三个 Edit HWND `134216,199778,134334`；点击 Reverse 后为 `134334,199778,134216`，集合保持不变、顺序改变。

- [x] Task 7: 完成全量验证、审查修复和清理
  - `cargo fmt --all -- --check` 通过
  - `cargo test --workspace`：23 passed
  - `cargo build --workspace` 通过
  - `cargo clippy --workspace --all-targets -- -D warnings` 通过
  - 修复首次 `Widget::None` 渲染的空 retained tree panic
  - 修复 HWND 数值复用时重建父节点 z-order 未同步
  - 新增两个回归测试
  - PR2 审查完成，修复提交 `43e9688` 已推送

### Checkpoint: Complete

- [x] 自动化验证通过
- [x] keyed TextEdit 实际 Windows smoke 通过
- [x] 完成自审并交给用户 review
- [x] tasks/ 未提交，按用户要求保留为未跟踪文件
