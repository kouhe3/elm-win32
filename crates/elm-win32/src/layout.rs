use crate::style::Rect;
use crate::widget::Widget;
use taffy::prelude::*;
use windows::Win32::Foundation::SIZE;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, DeleteDC, GetTextExtentPoint32W, HFONT, HGDIOBJ, SelectObject,
};

/// Length representation for layout dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Length {
    #[default]
    Auto,
    Px(f32),
    Percent(f32),
    Fill,
}

impl Length {
    pub const fn px(v: f32) -> Self {
        Length::Px(v)
    }

    pub const fn percent(p: f32) -> Self {
        Length::Percent(p)
    }

    pub fn is_auto(self) -> bool {
        matches!(self, Length::Auto)
    }

    pub fn to_dimension(self) -> Dimension {
        match self {
            Length::Auto => Dimension::auto(),
            Length::Px(v) => Dimension::length(v),
            Length::Percent(p) => Dimension::percent(p / 100.0),
            Length::Fill => Dimension::percent(1.0),
        }
    }

    pub fn to_length_percentage(self) -> LengthPercentage {
        match self {
            Length::Auto => LengthPercentage::length(0.0),
            Length::Px(v) => LengthPercentage::length(v),
            Length::Percent(p) => LengthPercentage::percent(p / 100.0),
            Length::Fill => LengthPercentage::percent(1.0),
        }
    }

    pub fn to_length_percentage_auto(self) -> LengthPercentageAuto {
        match self {
            Length::Auto => LengthPercentageAuto::auto(),
            Length::Px(v) => LengthPercentageAuto::length(v),
            Length::Percent(p) => LengthPercentageAuto::percent(p / 100.0),
            Length::Fill => LengthPercentageAuto::percent(1.0),
        }
    }
}

impl From<f32> for Length {
    fn from(v: f32) -> Self {
        Length::Px(v)
    }
}

impl From<i32> for Length {
    fn from(v: i32) -> Self {
        Length::Px(v as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Align {
    pub fn to_align_items(self) -> Option<AlignItems> {
        match self {
            Align::Start => Some(AlignItems::FLEX_START),
            Align::Center => Some(AlignItems::CENTER),
            Align::End => Some(AlignItems::FLEX_END),
            Align::Stretch => Some(AlignItems::STRETCH),
            _ => None,
        }
    }

    pub fn to_justify_content(self) -> Option<JustifyContent> {
        match self {
            Align::Start => Some(JustifyContent::FLEX_START),
            Align::Center => Some(JustifyContent::CENTER),
            Align::End => Some(JustifyContent::FLEX_END),
            Align::Stretch => Some(JustifyContent::STRETCH),
            Align::SpaceBetween => Some(JustifyContent::SPACE_BETWEEN),
            Align::SpaceAround => Some(JustifyContent::SPACE_AROUND),
            Align::SpaceEvenly => Some(JustifyContent::SPACE_EVENLY),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Edges {
    pub left: Length,
    pub right: Length,
    pub top: Length,
    pub bottom: Length,
}

impl Edges {
    pub const ZERO: Self = Self {
        left: Length::Px(0.0),
        right: Length::Px(0.0),
        top: Length::Px(0.0),
        bottom: Length::Px(0.0),
    };

    pub fn all(l: impl Into<Length>) -> Self {
        let len = l.into();
        Self {
            left: len,
            right: len,
            top: len,
            bottom: len,
        }
    }

    pub fn symmetric(vertical: impl Into<Length>, horizontal: impl Into<Length>) -> Self {
        let v = vertical.into();
        let h = horizontal.into();
        Self {
            left: h,
            right: h,
            top: v,
            bottom: v,
        }
    }

    pub fn new(
        top: impl Into<Length>,
        right: impl Into<Length>,
        bottom: impl Into<Length>,
        left: impl Into<Length>,
    ) -> Self {
        Self {
            top: top.into(),
            right: right.into(),
            bottom: bottom.into(),
            left: left.into(),
        }
    }

    pub fn to_taffy_rect(self) -> taffy::Rect<LengthPercentage> {
        taffy::Rect {
            left: self.left.to_length_percentage(),
            right: self.right.to_length_percentage(),
            top: self.top.to_length_percentage(),
            bottom: self.bottom.to_length_percentage(),
        }
    }

    pub fn to_taffy_rect_auto(self) -> taffy::Rect<LengthPercentageAuto> {
        taffy::Rect {
            left: self.left.to_length_percentage_auto(),
            right: self.right.to_length_percentage_auto(),
            top: self.top.to_length_percentage_auto(),
            bottom: self.bottom.to_length_percentage_auto(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutStyle {
    pub width: Length,
    pub height: Length,
    pub min_width: Length,
    pub min_height: Length,
    pub max_width: Length,
    pub max_height: Length,
    pub padding: Edges,
    pub margin: Edges,
    pub gap_row: Length,
    pub gap_col: Length,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub align_items: Option<Align>,
    pub justify_content: Option<Align>,
    pub align_self: Option<Align>,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            width: Length::Auto,
            height: Length::Auto,
            min_width: Length::Auto,
            min_height: Length::Auto,
            max_width: Length::Auto,
            max_height: Length::Auto,
            padding: Edges::ZERO,
            margin: Edges::ZERO,
            gap_row: Length::Auto,
            gap_col: Length::Auto,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            align_items: None,
            justify_content: None,
            align_self: None,
        }
    }
}

/// Helper to measure text width and height using Win32 GDI.
pub fn measure_text(text: &str, font: Option<HFONT>) -> (f32, f32) {
    if text.is_empty() {
        return (0.0, 16.0);
    }
    unsafe {
        let hdc = CreateCompatibleDC(None);
        let mut old_font = HGDIOBJ::default();
        if let Some(f) = font {
            old_font = SelectObject(hdc, HGDIOBJ(f.0));
        }

        let wide: Vec<u16> = text.encode_utf16().collect();
        let mut size = SIZE::default();
        let _ = GetTextExtentPoint32W(hdc, &wide, &mut size);

        if !old_font.is_invalid() {
            SelectObject(hdc, old_font);
        }
        let _ = DeleteDC(hdc);

        (size.cx as f32, size.cy as f32)
    }
}

/// Layout engine for elm-win32.
pub struct LayoutEngine {
    taffy: TaffyTree<()>,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
        }
    }

    /// Compute layouts for all widgets in the tree and return a list of resolved `Rect`s
    /// in preorder matching the NodeTree traversal order.
    pub fn compute<Msg: Clone>(
        &mut self,
        root: &Widget<Msg>,
        available_width: f32,
        available_height: f32,
        font: Option<HFONT>,
    ) -> Vec<Rect> {
        self.taffy.clear();
        let root_node = self.build_node(root, font);

        let space = Size {
            width: if available_width > 0.0 {
                AvailableSpace::Definite(available_width)
            } else {
                AvailableSpace::MinContent
            },
            height: if available_height > 0.0 {
                AvailableSpace::Definite(available_height)
            } else {
                AvailableSpace::MinContent
            },
        };

        let _ = self.taffy.compute_layout(root_node, space);

        let mut results = Vec::new();
        self.collect_rects(root_node, 0.0, 0.0, &mut results);
        results
    }

    fn build_node<Msg: Clone>(&mut self, widget: &Widget<Msg>, font: Option<HFONT>) -> NodeId {
        let widget = widget.without_key();
        match widget {
            Widget::Column {
                children, layout, ..
            }
            | Widget::Row {
                children, layout, ..
            } => {
                let flex_direction = match widget {
                    Widget::Row { .. } => FlexDirection::Row,
                    _ => FlexDirection::Column,
                };

                let child_nodes: Vec<NodeId> =
                    children.iter().map(|c| self.build_node(c, font)).collect();
                let style = taffy::Style {
                    display: Display::Flex,
                    flex_direction,
                    size: Size {
                        width: layout.width.to_dimension(),
                        height: layout.height.to_dimension(),
                    },
                    min_size: Size {
                        width: layout.min_width.to_length_percentage_auto(),
                        height: layout.min_height.to_length_percentage_auto(),
                    },
                    max_size: Size {
                        width: layout.max_width.to_length_percentage_auto(),
                        height: layout.max_height.to_length_percentage_auto(),
                    },
                    padding: layout.padding.to_taffy_rect(),
                    margin: layout.margin.to_taffy_rect_auto(),
                    gap: Size {
                        width: layout.gap_col.to_length_percentage(),
                        height: layout.gap_row.to_length_percentage(),
                    },
                    align_items: layout.align_items.and_then(|a| a.to_align_items()),
                    justify_content: layout.justify_content.and_then(|j| j.to_justify_content()),
                    flex_grow: layout.flex_grow,
                    flex_shrink: layout.flex_shrink,
                    ..Default::default()
                };

                self.taffy
                    .new_with_children(style, &child_nodes)
                    .expect("Failed to create layout node with children in TaffyTree")
            }
            Widget::None => {
                let style = taffy::Style {
                    size: Size {
                        width: Dimension::length(0.0),
                        height: Dimension::length(0.0),
                    },
                    ..Default::default()
                };
                self.taffy.new_leaf(style).unwrap()
            }
            _ => {
                let explicit_bounds = widget.bounds();
                let layout = widget.layout_style();
                let (default_w, default_h) = self.default_widget_size(widget, font);

                // Priority: explicit layout style > explicit bounds > default intrinsic size
                let width = if !layout.width.is_auto() {
                    layout.width.to_dimension()
                } else if explicit_bounds.w > 0.0 {
                    Dimension::length(explicit_bounds.w)
                } else if default_w > 0.0 {
                    Dimension::length(default_w)
                } else {
                    Dimension::auto()
                };

                let height = if !layout.height.is_auto() {
                    layout.height.to_dimension()
                } else if explicit_bounds.h > 0.0 {
                    Dimension::length(explicit_bounds.h)
                } else if default_h > 0.0 {
                    Dimension::length(default_h)
                } else {
                    Dimension::auto()
                };

                let style = taffy::Style {
                    size: Size { width, height },
                    min_size: Size {
                        width: layout.min_width.to_length_percentage_auto(),
                        height: layout.min_height.to_length_percentage_auto(),
                    },
                    max_size: Size {
                        width: layout.max_width.to_length_percentage_auto(),
                        height: layout.max_height.to_length_percentage_auto(),
                    },
                    margin: layout.margin.to_taffy_rect_auto(),
                    flex_grow: layout.flex_grow,
                    flex_shrink: layout.flex_shrink,
                    align_self: layout.align_self.and_then(|a| a.to_align_items()),
                    ..Default::default()
                };

                self.taffy.new_leaf(style).unwrap()
            }
        }
    }

    fn default_widget_size<Msg: Clone>(
        &self,
        widget: &Widget<Msg>,
        font: Option<HFONT>,
    ) -> (f32, f32) {
        match widget.without_key() {
            Widget::Button { text, .. } => {
                let (tw, th) = measure_text(text, font);
                ((tw + 32.0).max(60.0), (th + 14.0).max(28.0))
            }
            Widget::Label { text, .. } => {
                let (tw, th) = measure_text(text, font);
                (tw + 4.0, th.max(20.0))
            }
            Widget::TextEdit { .. } => (180.0, 26.0),
            Widget::CheckBox { text, .. } => {
                let (tw, th) = measure_text(text, font);
                (tw + 34.0, th.max(24.0))
            }
            Widget::RadioButton { text, .. } => {
                let (tw, th) = measure_text(text, font);
                (tw + 34.0, th.max(24.0))
            }
            Widget::ComboBox { .. } => (160.0, 26.0),
            Widget::ComboBoxEx { .. } => (160.0, 26.0),
            Widget::ListBox { .. } => (180.0, 100.0),
            Widget::DateTime { .. } => (160.0, 26.0),
            Widget::GroupBox { .. } => (200.0, 150.0),
            Widget::Header { .. } => (300.0, 28.0),
            Widget::TabControl { .. } => (300.0, 200.0),
            Widget::Toolbar { .. } => (360.0, 28.0),
            Widget::Trackbar { trackbar_style, .. } => {
                if trackbar_style & 0x0002 != 0 {
                    // TBS_VERT
                    (30.0, 200.0)
                } else {
                    (200.0, 30.0)
                }
            }
            _ => (0.0, 0.0),
        }
    }

    fn collect_rects(&self, node: NodeId, parent_x: f32, parent_y: f32, results: &mut Vec<Rect>) {
        let layout = self.taffy.layout(node).unwrap();
        let abs_x = parent_x + layout.location.x;
        let abs_y = parent_y + layout.location.y;

        results.push(Rect {
            x: abs_x,
            y: abs_y,
            w: layout.size.width,
            h: layout.size.height,
        });

        if let Ok(children) = self.taffy.children(node) {
            for child in children {
                self.collect_rects(child, abs_x, abs_y, results);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::{Button, Column, Label, Row};

    #[test]
    fn test_column_layout_spacing() {
        let mut engine = LayoutEngine::new();
        let root: Widget<()> = Column::new()
            .spacing(10.0)
            .push(Button::new("A").height(30.0).width(80.0))
            .push(Button::new("B").height(30.0).width(80.0))
            .push(Button::new("C").height(30.0).width(80.0))
            .into();

        let rects = engine.compute(&root, 500.0, 500.0, None);
        assert_eq!(rects.len(), 4); // Root + 3 children

        // Child 1 (Button A): y = 0.0, h = 30.0
        assert_eq!(rects[1].y, 0.0);
        assert_eq!(rects[1].h, 30.0);

        // Child 2 (Button B): y = 30 + 10 = 40.0
        assert_eq!(rects[2].y, 40.0);
        assert_eq!(rects[2].h, 30.0);

        // Child 3 (Button C): y = 40 + 30 + 10 = 80.0
        assert_eq!(rects[3].y, 80.0);
        assert_eq!(rects[3].h, 30.0);
    }

    #[test]
    fn test_row_layout_spacing() {
        let mut engine = LayoutEngine::new();
        let root: Widget<()> = Row::new()
            .spacing(15.0)
            .push(Button::new("A").width(50.0).height(25.0))
            .push(Button::new("B").width(60.0).height(25.0))
            .into();

        let rects = engine.compute(&root, 500.0, 500.0, None);
        assert_eq!(rects.len(), 3); // Root + 2 children

        // Child 1 (Button A): x = 0.0, w = 50.0
        assert_eq!(rects[1].x, 0.0);
        assert_eq!(rects[1].w, 50.0);

        // Child 2 (Button B): x = 50 + 15 = 65.0, w = 60.0
        assert_eq!(rects[2].x, 65.0);
        assert_eq!(rects[2].w, 60.0);
    }

    #[test]
    fn test_nested_column_row_padding() {
        let mut engine = LayoutEngine::new();
        let root: Widget<()> = Column::new()
            .padding_all(20.0)
            .spacing(10.0)
            .push(Label::new("Title").height(20.0))
            .push(
                Row::new()
                    .spacing(8.0)
                    .push(Button::new("Btn1").width(50.0).height(30.0))
                    .push(Button::new("Btn2").width(50.0).height(30.0)),
            )
            .into();

        let rects = engine.compute(&root, 400.0, 400.0, None);
        assert_eq!(rects.len(), 5); // Root Column, Label, Inner Row, Btn1, Btn2

        // Label
        assert_eq!(rects[1].x, 20.0);
        assert_eq!(rects[1].y, 20.0);

        // Inner Row
        assert_eq!(rects[2].x, 20.0);
        assert_eq!(rects[2].y, 50.0); // 20 (padding) + 20 (label h) + 10 (spacing)

        // Btn1 inside Row
        assert_eq!(rects[3].x, 20.0);
        assert_eq!(rects[3].y, 50.0);

        // Btn2 inside Row
        assert_eq!(rects[4].x, 20.0 + 50.0 + 8.0); // 78.0
        assert_eq!(rects[4].y, 50.0);
    }
}
