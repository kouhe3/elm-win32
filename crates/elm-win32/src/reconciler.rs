use std::collections::{HashMap, HashSet};

use crate::widget::Widget;
use crate::widgets;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::Graphics::Gdi::HFONT;
use windows::Win32::UI::WindowsAndMessaging::{
    DestroyWindow, HWND_TOP, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SendMessageW, SetWindowPos,
    WM_SETFONT,
};

#[derive(Debug, Clone)]
pub(crate) struct NodeInfo {
    pub hwnd: HWND,
    owns_hwnd: bool,
    parent_hwnd: HWND,
    subtree_len: usize,
}

#[derive(Default)]
pub(crate) struct NodeTree {
    nodes: Vec<NodeInfo>,
}

trait NativeOps {
    fn create<Msg>(&mut self, parent: HWND, widget: &Widget<Msg>) -> HWND;
    fn update<Msg>(&mut self, hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>);
    fn destroy(&mut self, hwnd: HWND);
    fn set_font(&mut self, hwnd: HWND, font: HFONT);
    fn reorder(&mut self, parent: HWND, hwnds: &[HWND]);
}

struct Win32Ops;

impl NativeOps for Win32Ops {
    fn create<Msg>(&mut self, parent: HWND, widget: &Widget<Msg>) -> HWND {
        widgets::create_hwnd(parent, widget).expect("failed to create HWND")
    }

    fn update<Msg>(&mut self, hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
        widgets::update_hwnd(hwnd, old, new);
    }

    fn destroy(&mut self, hwnd: HWND) {
        unsafe {
            let _ = DestroyWindow(hwnd);
        }
    }

    fn set_font(&mut self, hwnd: HWND, font: HFONT) {
        unsafe {
            SendMessageW(
                hwnd,
                WM_SETFONT,
                Some(WPARAM(font.0 as usize)),
                Some(LPARAM(1)),
            );
        }
    }

    fn reorder(&mut self, _parent: HWND, hwnds: &[HWND]) {
        let mut insert_after = HWND_TOP;
        for &hwnd in hwnds {
            unsafe {
                SetWindowPos(
                    hwnd,
                    Some(insert_after),
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                )
                .expect("failed to reorder child HWND");
            }
            insert_after = hwnd;
        }
    }
}

impl NodeTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reconcile<Msg: Clone>(
        &mut self,
        root_hwnd: HWND,
        old: Option<&Widget<Msg>>,
        new: &Widget<Msg>,
        font: Option<HFONT>,
    ) {
        let mut ops = Win32Ops;
        self.reconcile_with(root_hwnd, old, new, font, &mut ops);
    }

    fn reconcile_with<Msg: Clone, O: NativeOps>(
        &mut self,
        root_hwnd: HWND,
        old: Option<&Widget<Msg>>,
        new: &Widget<Msg>,
        font: Option<HFONT>,
        ops: &mut O,
    ) {
        validate_keys(new);

        let old_nodes = std::mem::take(&mut self.nodes);
        let old_native_order = native_order(&old_nodes);
        self.nodes = reconcile_node(root_hwnd, old, new, &old_nodes, font, ops);
        reorder_changed_parents(&old_native_order, &self.nodes, ops);
    }

    pub fn get_by_position(&self, pos: usize) -> Option<&NodeInfo> {
        self.nodes.get(pos)
    }
}

fn reconcile_node<Msg: Clone, O: NativeOps>(
    parent_hwnd: HWND,
    old_widget: Option<&Widget<Msg>>,
    new_widget: &Widget<Msg>,
    old_nodes: &[NodeInfo],
    font: Option<HFONT>,
    ops: &mut O,
) -> Vec<NodeInfo> {
    let old_content = old_widget.map(Widget::without_key);
    let new_content = new_widget.without_key();
    let can_update = old_widget
        .is_some_and(|old| old.key_value() == new_widget.key_value() && old.variant_eq(new_widget));

    if !can_update {
        destroy_nodes(old_nodes, ops);
        return mount_node(parent_hwnd, new_widget, font, ops);
    }

    let old_widget = old_widget.unwrap();
    let old_content = old_content.unwrap();
    let old_root = old_nodes
        .first()
        .expect("old widget tree and retained node tree diverged");
    let mut result = Vec::new();

    if new_content.is_container() {
        result.push(NodeInfo {
            hwnd: parent_hwnd,
            owns_hwnd: false,
            parent_hwnd,
            subtree_len: 0,
        });
        reconcile_children(
            parent_hwnd,
            old_content.children(),
            new_content.children(),
            &old_nodes[1..],
            font,
            ops,
            &mut result,
        );
    } else if matches!(new_content, Widget::None) {
        result.push(NodeInfo {
            hwnd: parent_hwnd,
            owns_hwnd: false,
            parent_hwnd,
            subtree_len: 1,
        });
    } else {
        ops.update(old_root.hwnd, old_widget, new_widget);
        result.push(NodeInfo {
            hwnd: old_root.hwnd,
            owns_hwnd: true,
            parent_hwnd,
            subtree_len: 1,
        });
    }

    result[0].subtree_len = result.len();
    result
}

#[allow(clippy::too_many_arguments)]
fn reconcile_children<Msg: Clone, O: NativeOps>(
    parent_hwnd: HWND,
    old_children: &[Widget<Msg>],
    new_children: &[Widget<Msg>],
    old_nodes: &[NodeInfo],
    font: Option<HFONT>,
    ops: &mut O,
    result: &mut Vec<NodeInfo>,
) {
    let old_ranges = child_ranges(old_children, old_nodes);
    let mut old_by_key = HashMap::new();
    for (index, child) in old_children.iter().enumerate() {
        if let Some(key) = child.key_value() {
            old_by_key.insert(key, index);
        }
    }

    let mut used = vec![false; old_children.len()];
    for (index, new_child) in new_children.iter().enumerate() {
        let candidate = if let Some(key) = new_child.key_value() {
            old_by_key
                .get(key)
                .copied()
                .filter(|&old_index| !used[old_index])
        } else if index < old_children.len()
            && old_children[index].key_value().is_none()
            && !used[index]
        {
            Some(index)
        } else {
            None
        };

        let (old_child, old_slice) = if let Some(old_index) = candidate {
            used[old_index] = true;
            let range = &old_ranges[old_index];
            (Some(&old_children[old_index]), &old_nodes[range.clone()])
        } else {
            (None, &[][..])
        };
        result.extend(reconcile_node(
            parent_hwnd,
            old_child,
            new_child,
            old_slice,
            font,
            ops,
        ));
    }

    for (index, range) in old_ranges.iter().enumerate() {
        if !used[index] {
            destroy_nodes(&old_nodes[range.clone()], ops);
        }
    }
}

fn mount_node<Msg, O: NativeOps>(
    parent_hwnd: HWND,
    widget: &Widget<Msg>,
    font: Option<HFONT>,
    ops: &mut O,
) -> Vec<NodeInfo> {
    let content = widget.without_key();
    let mut result = Vec::new();
    if content.is_container() {
        result.push(NodeInfo {
            hwnd: parent_hwnd,
            owns_hwnd: false,
            parent_hwnd,
            subtree_len: 0,
        });
        for child in content.children() {
            result.extend(mount_node(parent_hwnd, child, font, ops));
        }
    } else if matches!(content, Widget::None) {
        result.push(NodeInfo {
            hwnd: parent_hwnd,
            owns_hwnd: false,
            parent_hwnd,
            subtree_len: 1,
        });
    } else {
        let hwnd = ops.create(parent_hwnd, widget);
        if let Some(font) = font {
            ops.set_font(hwnd, font);
        }
        result.push(NodeInfo {
            hwnd,
            owns_hwnd: true,
            parent_hwnd,
            subtree_len: 1,
        });
    }
    result[0].subtree_len = result.len();
    result
}

fn child_ranges<Msg>(children: &[Widget<Msg>], nodes: &[NodeInfo]) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::with_capacity(children.len());
    let mut start = 0;
    for _ in children {
        let len = nodes
            .get(start)
            .expect("widget children and retained node tree diverged")
            .subtree_len;
        ranges.push(start..start + len);
        start += len;
    }
    assert_eq!(
        start,
        nodes.len(),
        "retained child subtree has trailing nodes"
    );
    ranges
}

fn validate_keys<Msg>(widget: &Widget<Msg>) {
    let content = widget.without_key();
    if !content.is_container() {
        return;
    }
    let mut keys = HashSet::new();
    for child in content.children() {
        if let Some(key) = child.key_value() {
            assert!(keys.insert(key), "duplicate key among siblings: {key}");
        }
        validate_keys(child);
    }
}

fn destroy_nodes<O: NativeOps>(nodes: &[NodeInfo], ops: &mut O) {
    for node in nodes.iter().rev() {
        if node.owns_hwnd {
            ops.destroy(node.hwnd);
        }
    }
}

fn native_order(nodes: &[NodeInfo]) -> HashMap<isize, Vec<HWND>> {
    let mut by_parent = HashMap::<isize, Vec<HWND>>::new();
    for node in nodes {
        if node.owns_hwnd {
            by_parent
                .entry(node.parent_hwnd.0 as isize)
                .or_default()
                .push(node.hwnd);
        }
    }
    by_parent
}

fn reorder_changed_parents<O: NativeOps>(
    old_order: &HashMap<isize, Vec<HWND>>,
    new_nodes: &[NodeInfo],
    ops: &mut O,
) {
    for (parent, new_order) in native_order(new_nodes) {
        if old_order.get(&parent) != Some(&new_order) {
            ops.reorder(HWND(parent as *mut _), &new_order);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::{Column, KeyExt, Label, Row};

    #[derive(Default)]
    struct FakeOps {
        next: isize,
        destroyed: Vec<isize>,
        reorders: Vec<Vec<isize>>,
    }

    impl NativeOps for FakeOps {
        fn create<Msg>(&mut self, _parent: HWND, _widget: &Widget<Msg>) -> HWND {
            self.next += 1;
            HWND(self.next as *mut _)
        }

        fn update<Msg>(&mut self, _hwnd: HWND, _old: &Widget<Msg>, _new: &Widget<Msg>) {}

        fn destroy(&mut self, hwnd: HWND) {
            self.destroyed.push(hwnd.0 as isize);
        }

        fn set_font(&mut self, _hwnd: HWND, _font: HFONT) {}

        fn reorder(&mut self, _parent: HWND, hwnds: &[HWND]) {
            self.reorders
                .push(hwnds.iter().map(|h| h.0 as isize).collect());
        }
    }

    fn keyed_labels(keys: &[&str]) -> Widget<()> {
        let mut column = Column::new();
        for key in keys {
            column = column.push(Label::new(key).key(*key));
        }
        column.into()
    }

    #[test]
    fn virtual_nodes_never_destroy_the_root_window() {
        let root = HWND(99usize as *mut _);
        let old: Widget<()> = Column::new().push(Row::new().push(Label::new("A"))).into();
        let new: Widget<()> = Widget::None;
        let mut tree = NodeTree::new();
        let mut ops = FakeOps::default();
        tree.reconcile_with(root, None, &old, None, &mut ops);
        tree.reconcile_with(root, Some(&old), &new, None, &mut ops);
        assert_eq!(ops.destroyed, vec![1]);
    }

    #[test]
    fn keyed_reorder_reuses_handles_and_changes_native_order() {
        let root = HWND(99usize as *mut _);
        let old = keyed_labels(&["a", "b", "c"]);
        let new = keyed_labels(&["c", "a", "b"]);
        let mut tree = NodeTree::new();
        let mut ops = FakeOps::default();
        tree.reconcile_with(root, None, &old, None, &mut ops);
        ops.reorders.clear();
        tree.reconcile_with(root, Some(&old), &new, None, &mut ops);
        let handles: Vec<_> = tree
            .nodes
            .iter()
            .filter(|n| n.owns_hwnd)
            .map(|n| n.hwnd.0 as isize)
            .collect();
        assert_eq!(handles, vec![3, 1, 2]);
        assert!(ops.destroyed.is_empty());
        assert_eq!(ops.reorders, vec![vec![3, 1, 2]]);
    }

    #[test]
    fn keyed_insert_and_delete_only_change_affected_handles() {
        let root = HWND(99usize as *mut _);
        let old = keyed_labels(&["a", "b"]);
        let new = keyed_labels(&["x", "a"]);
        let mut tree = NodeTree::new();
        let mut ops = FakeOps::default();
        tree.reconcile_with(root, None, &old, None, &mut ops);
        tree.reconcile_with(root, Some(&old), &new, None, &mut ops);
        let handles: Vec<_> = tree
            .nodes
            .iter()
            .filter(|n| n.owns_hwnd)
            .map(|n| n.hwnd.0 as isize)
            .collect();
        assert_eq!(handles, vec![3, 1]);
        assert_eq!(ops.destroyed, vec![2]);
    }

    #[test]
    fn unkeyed_children_keep_absolute_index_identity() {
        let root = HWND(99usize as *mut _);
        let old: Widget<()> = Column::new()
            .push(Label::new("plain"))
            .push(Label::new("keyed").key("k"))
            .into();
        let new: Widget<()> = Column::new()
            .push(Label::new("new").key("n"))
            .push(Label::new("plain"))
            .push(Label::new("keyed").key("k"))
            .into();
        let mut tree = NodeTree::new();
        let mut ops = FakeOps::default();
        tree.reconcile_with(root, None, &old, None, &mut ops);
        tree.reconcile_with(root, Some(&old), &new, None, &mut ops);
        let handles: Vec<_> = tree
            .nodes
            .iter()
            .filter(|n| n.owns_hwnd)
            .map(|n| n.hwnd.0 as isize)
            .collect();
        assert_eq!(handles, vec![3, 4, 2]);
        assert_eq!(ops.destroyed, vec![1]);
    }

    #[test]
    fn same_key_with_different_widget_type_remounts() {
        let root = HWND(99usize as *mut _);
        let old: Widget<()> = Column::new().push(Label::new("A").key("item")).into();
        let new: Widget<()> = Column::new().push(Row::new().key("item")).into();
        let mut tree = NodeTree::new();
        let mut ops = FakeOps::default();
        tree.reconcile_with(root, None, &old, None, &mut ops);
        tree.reconcile_with(root, Some(&old), &new, None, &mut ops);
        assert_eq!(ops.destroyed, vec![1]);
        assert!(tree.nodes.iter().all(|node| !node.owns_hwnd));
    }

    #[test]
    #[should_panic(expected = "duplicate key among siblings: duplicate")]
    fn duplicate_keys_panic_before_native_changes() {
        let root = HWND(99usize as *mut _);
        let duplicate: Widget<()> = Column::new()
            .push(Label::new("A").key("duplicate"))
            .push(Label::new("B").key("duplicate"))
            .into();
        let mut tree = NodeTree::new();
        let mut ops = FakeOps::default();
        tree.reconcile_with(root, None, &duplicate, None, &mut ops);
    }
}
