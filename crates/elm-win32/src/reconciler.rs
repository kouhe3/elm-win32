use crate::widget::Widget;
use crate::widgets;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::Graphics::Gdi::HFONT;
use windows::Win32::UI::WindowsAndMessaging::{DestroyWindow, SendMessageW, WM_SETFONT};

#[derive(Debug, Clone)]
pub(crate) struct NodeInfo {
    pub hwnd: HWND,
}

#[derive(Default)]
pub(crate) struct NodeTree {
    nodes: Vec<NodeInfo>,
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
        let old_nodes = std::mem::take(&mut self.nodes);
        let mut new_nodes = Vec::new();
        let mut old_idx = 0usize;
        self.reconcile_recursive(
            root_hwnd,
            old,
            new,
            &old_nodes,
            &mut old_idx,
            &mut new_nodes,
            font,
        );
        for (_, node) in old_nodes.iter().enumerate().skip(old_idx) {
            destroy_node(node);
        }
        self.nodes = new_nodes;
    }

    #[allow(clippy::too_many_arguments)]
    fn reconcile_recursive<Msg: Clone>(
        &self,
        parent_hwnd: HWND,
        old_widget: Option<&Widget<Msg>>,
        new_widget: &Widget<Msg>,
        old_nodes: &[NodeInfo],
        old_idx: &mut usize,
        new_nodes: &mut Vec<NodeInfo>,
        font: Option<HFONT>,
    ) {
        let old_node = old_nodes.get(*old_idx);

        match (old_node, old_widget) {
            (Some(old_node), Some(old_widget)) if old_widget.variant_eq(new_widget) => {
                widgets::update_hwnd(old_node.hwnd, old_widget, new_widget);
                new_nodes.push(old_node.clone());
                *old_idx += 1;

                if new_widget.is_container() {
                    let old_children = old_widget.children();
                    for (index, child) in new_widget.children().iter().enumerate() {
                        self.reconcile_recursive(
                            old_node.hwnd,
                            old_children.get(index),
                            child,
                            old_nodes,
                            old_idx,
                            new_nodes,
                            font,
                        );
                    }
                    for _ in new_widget.children().len()..old_children.len() {
                        if *old_idx < old_nodes.len() {
                            destroy_node(&old_nodes[*old_idx]);
                            *old_idx += 1;
                        }
                    }
                }
            }
            (old_node, _) => {
                if let Some(node) = old_node {
                    destroy_node(node);
                    *old_idx += 1;
                }

                let hwnd = if new_widget.is_container() || matches!(new_widget, Widget::None) {
                    parent_hwnd
                } else {
                    let hwnd = widgets::create_hwnd(parent_hwnd, new_widget)
                        .expect("failed to create HWND");
                    if let Some(font) = font {
                        unsafe {
                            SendMessageW(
                                hwnd,
                                WM_SETFONT,
                                Some(WPARAM(font.0 as usize)),
                                Some(LPARAM(1)),
                            );
                        }
                    }
                    hwnd
                };

                new_nodes.push(NodeInfo { hwnd });

                if new_widget.is_container() {
                    for child in new_widget.children() {
                        self.reconcile_recursive(
                            hwnd, None, child, old_nodes, old_idx, new_nodes, font,
                        );
                    }
                }
            }
        }
    }

    pub fn get_by_position(&self, pos: usize) -> Option<&NodeInfo> {
        self.nodes.get(pos)
    }
}

fn destroy_node(node: &NodeInfo) {
    unsafe {
        let _ = DestroyWindow(node.hwnd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_tree_roundtrip() {
        let tree = NodeTree::new();
        assert!(tree.get_by_position(0).is_none());
    }

    #[test]
    fn test_destroy_node_no_panic() {
        let node = NodeInfo {
            hwnd: HWND::default(),
        };
        destroy_node(&node);
    }
}
