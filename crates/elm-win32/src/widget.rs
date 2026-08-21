use crate::style::{Align, Edges, LayoutStyle, Length, Style};

pub use crate::style::Rect;

#[derive(Debug, Clone)]
pub enum Widget<Msg> {
    None,
    Column {
        children: Vec<Widget<Msg>>,
        layout: LayoutStyle,
    },
    Row {
        children: Vec<Widget<Msg>>,
        layout: LayoutStyle,
    },
    Button {
        text: String,
        on_click: Option<Msg>,
        bounds: Rect,
        layout: LayoutStyle,
        button_style: u32,
    },
    Label {
        text: String,
        bounds: Rect,
        layout: LayoutStyle,
    },
    TextEdit {
        text: String,
        on_change: Option<fn(String) -> Msg>,
        bounds: Rect,
        layout: LayoutStyle,
        edit_style: u32,
        text_color: Option<u32>,
        bg_color: Option<u32>,
    },
    ListBox {
        items: Vec<String>,
        on_select: Option<fn(usize) -> Msg>,
        bounds: Rect,
        layout: LayoutStyle,
    },
    ComboBox {
        items: Vec<String>,
        selected: Option<usize>,
        on_select: Option<fn(usize) -> Msg>,
        on_edit_change: Option<fn(String) -> Msg>,
        combo_style: u32,
        bounds: Rect,
        layout: LayoutStyle,
    },
    ComboBoxEx {
        items: Vec<String>,
        selected: Option<usize>,
        on_select: Option<fn(usize) -> Msg>,
        bounds: Rect,
        layout: LayoutStyle,
    },
    #[allow(clippy::type_complexity)]
    DateTime {
        format: u32,
        on_change: Option<fn(u16, u16, u16, u16, u16, u16) -> Msg>,
        bounds: Rect,
        layout: LayoutStyle,
    },
    CheckBox {
        text: String,
        check_state: i32,
        checkbox_style: u32,
        on_toggle: Option<fn(i32) -> Msg>,
        bounds: Rect,
        layout: LayoutStyle,
    },
    RadioButton {
        text: String,
        checked: bool,
        group: bool,
        on_toggle: Option<fn(i32) -> Msg>,
        bounds: Rect,
        layout: LayoutStyle,
    },
    GroupBox {
        text: String,
        bounds: Rect,
        layout: LayoutStyle,
    },
    Header {
        columns: Vec<(String, f32)>,
        header_style: u32,
        on_column_click: Option<fn(usize) -> Msg>,
        bounds: Rect,
        layout: LayoutStyle,
    },
    TabControl {
        tabs: Vec<String>,
        selected: Option<usize>,
        tab_style: u32,
        on_tab_change: Option<fn(usize) -> Msg>,
        bounds: Rect,
        layout: LayoutStyle,
    },
    Toolbar {
        buttons: Vec<String>,
        toolbar_style: u32,
        on_button_click: Option<fn(usize) -> Msg>,
        bounds: Rect,
        layout: LayoutStyle,
    },
}

impl<Msg> Default for Widget<Msg> {
    fn default() -> Self {
        Widget::None
    }
}

impl<Msg> Widget<Msg> {
    pub fn is_container(&self) -> bool {
        matches!(self, Widget::Column { .. } | Widget::Row { .. })
    }

    pub fn children(&self) -> &[Widget<Msg>] {
        match self {
            Widget::Column { children, .. } | Widget::Row { children, .. } => children,
            _ => &[],
        }
    }

    pub fn variant_eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    pub fn bounds(&self) -> Rect {
        match self {
            Widget::Button { bounds, .. }
            | Widget::Label { bounds, .. }
            | Widget::TextEdit { bounds, .. }
            | Widget::ListBox { bounds, .. }
            | Widget::ComboBox { bounds, .. }
            | Widget::ComboBoxEx { bounds, .. }
            | Widget::DateTime { bounds, .. }
            | Widget::CheckBox { bounds, .. }
            | Widget::RadioButton { bounds, .. }
            | Widget::GroupBox { bounds, .. }
            | Widget::Header { bounds, .. }
            | Widget::TabControl { bounds, .. }
            | Widget::Toolbar { bounds, .. } => *bounds,
            _ => Rect::ZERO,
        }
    }

    pub fn layout_style(&self) -> LayoutStyle {
        match self {
            Widget::Column { layout, .. }
            | Widget::Row { layout, .. }
            | Widget::Button { layout, .. }
            | Widget::Label { layout, .. }
            | Widget::TextEdit { layout, .. }
            | Widget::ListBox { layout, .. }
            | Widget::ComboBox { layout, .. }
            | Widget::ComboBoxEx { layout, .. }
            | Widget::DateTime { layout, .. }
            | Widget::CheckBox { layout, .. }
            | Widget::RadioButton { layout, .. }
            | Widget::GroupBox { layout, .. }
            | Widget::Header { layout, .. }
            | Widget::TabControl { layout, .. }
            | Widget::Toolbar { layout, .. } => *layout,
            Widget::None => LayoutStyle::default(),
        }
    }
}

// ---- Builder types ----

pub struct Column<Msg> {
    children: Vec<Widget<Msg>>,
    layout: LayoutStyle,
}

impl<Msg> Column<Msg> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            layout: LayoutStyle {
                width: Length::Fill,
                ..Default::default()
            },
        }
    }

    pub fn push(mut self, widget: impl Into<Widget<Msg>>) -> Self {
        self.children.push(widget.into());
        self
    }

    pub fn spacing(mut self, s: impl Into<Length>) -> Self {
        let val = s.into();
        self.layout.gap_row = val;
        self.layout.gap_col = val;
        self
    }

    pub fn gap(self, g: impl Into<Length>) -> Self {
        self.spacing(g)
    }

    pub fn padding(mut self, p: Edges) -> Self {
        self.layout.padding = p;
        self
    }

    pub fn padding_all(mut self, p: impl Into<Length>) -> Self {
        self.layout.padding = Edges::all(p);
        self
    }

    pub fn align_items(mut self, a: Align) -> Self {
        self.layout.align_items = Some(a);
        self
    }

    pub fn justify_content(mut self, j: Align) -> Self {
        self.layout.justify_content = Some(j);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.layout.width = w.into();
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.layout.height = h.into();
        self
    }

    pub fn flex_grow(mut self, g: f32) -> Self {
        self.layout.flex_grow = g;
        self
    }

    pub fn flex_shrink(mut self, s: f32) -> Self {
        self.layout.flex_shrink = s;
        self
    }
}

impl<Msg> Default for Column<Msg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Msg> From<Column<Msg>> for Widget<Msg> {
    fn from(c: Column<Msg>) -> Self {
        Widget::Column {
            children: c.children,
            layout: c.layout,
        }
    }
}

pub struct Row<Msg> {
    children: Vec<Widget<Msg>>,
    layout: LayoutStyle,
}

impl<Msg> Row<Msg> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            layout: LayoutStyle {
                width: Length::Fill,
                ..Default::default()
            },
        }
    }

    pub fn push(mut self, widget: impl Into<Widget<Msg>>) -> Self {
        self.children.push(widget.into());
        self
    }

    pub fn spacing(mut self, s: impl Into<Length>) -> Self {
        let val = s.into();
        self.layout.gap_row = val;
        self.layout.gap_col = val;
        self
    }

    pub fn gap(self, g: impl Into<Length>) -> Self {
        self.spacing(g)
    }

    pub fn padding(mut self, p: Edges) -> Self {
        self.layout.padding = p;
        self
    }

    pub fn padding_all(mut self, p: impl Into<Length>) -> Self {
        self.layout.padding = Edges::all(p);
        self
    }

    pub fn align_items(mut self, a: Align) -> Self {
        self.layout.align_items = Some(a);
        self
    }

    pub fn justify_content(mut self, j: Align) -> Self {
        self.layout.justify_content = Some(j);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.layout.width = w.into();
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.layout.height = h.into();
        self
    }

    pub fn flex_grow(mut self, g: f32) -> Self {
        self.layout.flex_grow = g;
        self
    }

    pub fn flex_shrink(mut self, s: f32) -> Self {
        self.layout.flex_shrink = s;
        self
    }
}

impl<Msg> Default for Row<Msg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Msg> From<Row<Msg>> for Widget<Msg> {
    fn from(r: Row<Msg>) -> Self {
        Widget::Row {
            children: r.children,
            layout: r.layout,
        }
    }
}

pub struct Button<Msg> {
    text: String,
    on_click: Option<Msg>,
    style: Style,
}

impl<Msg> Button<Msg> {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            on_click: None,
            style: Style::new(),
        }
    }

    pub fn on_click(mut self, msg: Msg) -> Self {
        self.on_click = Some(msg);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn flex_grow(mut self, g: f32) -> Self {
        self.style = self.style.flex_grow(g);
        self
    }

    pub fn align_self(mut self, a: Align) -> Self {
        self.style = self.style.align_self(a);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<Button<Msg>> for Widget<Msg> {
    fn from(b: Button<Msg>) -> Self {
        Widget::Button {
            text: b.text,
            on_click: b.on_click,
            bounds: b.style.bounds,
            layout: b.style.layout,
            button_style: b.style.button_style.win32_style(),
        }
    }
}

pub struct Label<Msg> {
    text: String,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> Label<Msg> {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn flex_grow(mut self, g: f32) -> Self {
        self.style = self.style.flex_grow(g);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<Label<Msg>> for Widget<Msg> {
    fn from(l: Label<Msg>) -> Self {
        Widget::Label {
            text: l.text,
            bounds: l.style.bounds,
            layout: l.style.layout,
        }
    }
}

pub struct TextEdit<Msg> {
    text: String,
    on_change: Option<fn(String) -> Msg>,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> TextEdit<Msg> {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            on_change: None,
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn on_change(mut self, f: fn(String) -> Msg) -> Self {
        self.on_change = Some(f);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn flex_grow(mut self, g: f32) -> Self {
        self.style = self.style.flex_grow(g);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<TextEdit<Msg>> for Widget<Msg> {
    fn from(e: TextEdit<Msg>) -> Self {
        Widget::TextEdit {
            text: e.text,
            on_change: e.on_change,
            bounds: e.style.bounds,
            layout: e.style.layout,
            edit_style: e.style.edit_style.win32_style(),
            text_color: e.style.text_color.map(|c| c.to_colorref()),
            bg_color: e.style.bg_color.map(|c| c.to_colorref()),
        }
    }
}

pub struct ListBox<Msg> {
    items: Vec<String>,
    on_select: Option<fn(usize) -> Msg>,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> ListBox<Msg> {
    pub fn new(items: &[&str]) -> Self {
        Self {
            items: items.iter().map(|s| s.to_string()).collect(),
            on_select: None,
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn on_select(mut self, f: fn(usize) -> Msg) -> Self {
        self.on_select = Some(f);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn flex_grow(mut self, g: f32) -> Self {
        self.style = self.style.flex_grow(g);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<ListBox<Msg>> for Widget<Msg> {
    fn from(lb: ListBox<Msg>) -> Self {
        Widget::ListBox {
            items: lb.items,
            on_select: lb.on_select,
            bounds: lb.style.bounds,
            layout: lb.style.layout,
        }
    }
}

pub struct ComboBox<Msg> {
    items: Vec<String>,
    selected: Option<usize>,
    on_select: Option<fn(usize) -> Msg>,
    on_edit_change: Option<fn(String) -> Msg>,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> ComboBox<Msg> {
    pub fn new(items: &[&str]) -> Self {
        Self {
            items: items.iter().map(|s| s.to_string()).collect(),
            selected: None,
            on_select: None,
            on_edit_change: None,
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn selected(mut self, idx: usize) -> Self {
        self.selected = Some(idx);
        self
    }

    pub fn on_select(mut self, f: fn(usize) -> Msg) -> Self {
        self.on_select = Some(f);
        self
    }

    pub fn on_edit_change(mut self, f: fn(String) -> Msg) -> Self {
        self.on_edit_change = Some(f);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<ComboBox<Msg>> for Widget<Msg> {
    fn from(cb: ComboBox<Msg>) -> Self {
        Widget::ComboBox {
            items: cb.items,
            selected: cb.selected,
            on_select: cb.on_select,
            on_edit_change: cb.on_edit_change,
            combo_style: cb.style.combobox_style.win32_style(),
            bounds: cb.style.bounds,
            layout: cb.style.layout,
        }
    }
}

pub struct CheckBox<Msg> {
    text: String,
    check_state: i32,
    checkbox_style: crate::style::CheckBoxStyle,
    on_toggle: Option<fn(i32) -> Msg>,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> CheckBox<Msg> {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            check_state: 0, // BST_UNCHECKED
            checkbox_style: crate::style::CheckBoxStyle::Auto,
            on_toggle: None,
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn checked(mut self) -> Self {
        self.check_state = 1; // BST_CHECKED
        self
    }

    pub fn check_state(mut self, state: i32) -> Self {
        self.check_state = state;
        self
    }

    pub fn checkbox_style(mut self, s: crate::style::CheckBoxStyle) -> Self {
        self.checkbox_style = s;
        self
    }

    pub fn on_toggle(mut self, f: fn(i32) -> Msg) -> Self {
        self.on_toggle = Some(f);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<CheckBox<Msg>> for Widget<Msg> {
    fn from(cb: CheckBox<Msg>) -> Self {
        Widget::CheckBox {
            text: cb.text,
            check_state: cb.check_state,
            checkbox_style: cb.checkbox_style.win32_style(),
            on_toggle: cb.on_toggle,
            bounds: cb.style.bounds,
            layout: cb.style.layout,
        }
    }
}

pub struct RadioButton<Msg> {
    text: String,
    checked: bool,
    group: bool,
    on_toggle: Option<fn(i32) -> Msg>,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> RadioButton<Msg> {
    pub fn new(text: &str, checked: bool) -> Self {
        Self {
            text: text.to_string(),
            checked,
            group: false,
            on_toggle: None,
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn group(mut self) -> Self {
        self.group = true;
        self
    }

    pub fn on_toggle(mut self, f: fn(i32) -> Msg) -> Self {
        self.on_toggle = Some(f);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<RadioButton<Msg>> for Widget<Msg> {
    fn from(rb: RadioButton<Msg>) -> Self {
        Widget::RadioButton {
            text: rb.text,
            checked: rb.checked,
            group: rb.group,
            on_toggle: rb.on_toggle,
            bounds: rb.style.bounds,
            layout: rb.style.layout,
        }
    }
}

pub struct GroupBox<Msg> {
    text: String,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> GroupBox<Msg> {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<GroupBox<Msg>> for Widget<Msg> {
    fn from(gb: GroupBox<Msg>) -> Self {
        Widget::GroupBox {
            text: gb.text,
            bounds: gb.style.bounds,
            layout: gb.style.layout,
        }
    }
}

pub struct ComboBoxEx<Msg> {
    items: Vec<String>,
    selected: Option<usize>,
    on_select: Option<fn(usize) -> Msg>,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> ComboBoxEx<Msg> {
    pub fn new(items: &[&str]) -> Self {
        Self {
            items: items.iter().map(|s| s.to_string()).collect(),
            selected: None,
            on_select: None,
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn selected(mut self, idx: usize) -> Self {
        self.selected = Some(idx);
        self
    }

    pub fn on_select(mut self, f: fn(usize) -> Msg) -> Self {
        self.on_select = Some(f);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<ComboBoxEx<Msg>> for Widget<Msg> {
    fn from(cb: ComboBoxEx<Msg>) -> Self {
        Widget::ComboBoxEx {
            items: cb.items,
            selected: cb.selected,
            on_select: cb.on_select,
            bounds: cb.style.bounds,
            layout: cb.style.layout,
        }
    }
}

pub struct DateTime<Msg> {
    #[allow(clippy::type_complexity)]
    on_change: Option<fn(u16, u16, u16, u16, u16, u16) -> Msg>,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> DateTime<Msg> {
    pub fn new() -> Self {
        Self {
            on_change: None,
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn on_change(mut self, f: fn(u16, u16, u16, u16, u16, u16) -> Msg) -> Self {
        self.on_change = Some(f);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> Default for DateTime<Msg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Msg> From<DateTime<Msg>> for Widget<Msg> {
    fn from(dt: DateTime<Msg>) -> Self {
        Widget::DateTime {
            format: dt.style.datetime_format.win32_style(),
            on_change: dt.on_change,
            bounds: dt.style.bounds,
            layout: dt.style.layout,
        }
    }
}

pub struct Header<Msg> {
    columns: Vec<(String, f32)>,
    header_style: u32,
    on_column_click: Option<fn(usize) -> Msg>,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> Header<Msg> {
    pub fn new(columns: &[(&str, f32)]) -> Self {
        Self {
            columns: columns.iter().map(|(t, w)| (t.to_string(), *w)).collect(),
            header_style: 0,
            on_column_click: None,
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }

    pub fn header_style(mut self, s: u32) -> Self {
        self.header_style = s;
        self
    }

    pub fn on_column_click(mut self, f: fn(usize) -> Msg) -> Self {
        self.on_column_click = Some(f);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }
}

impl<Msg> From<Header<Msg>> for Widget<Msg> {
    fn from(h: Header<Msg>) -> Self {
        Widget::Header {
            columns: h.columns,
            header_style: h.header_style,
            on_column_click: h.on_column_click,
            bounds: h.style.bounds,
            layout: h.style.layout,
        }
    }
}

pub struct TabControl<Msg> {
    tabs: Vec<String>,
    selected: Option<usize>,
    tab_style: u32,
    on_tab_change: Option<fn(usize) -> Msg>,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> TabControl<Msg> {
    pub fn new(tabs: &[&str]) -> Self {
        Self {
            tabs: tabs.iter().map(|s| s.to_string()).collect(),
            selected: None,
            tab_style: 0,
            on_tab_change: None,
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn selected(mut self, idx: usize) -> Self {
        self.selected = Some(idx);
        self
    }

    pub fn tab_style(mut self, s: u32) -> Self {
        self.tab_style = s;
        self
    }

    pub fn on_tab_change(mut self, f: fn(usize) -> Msg) -> Self {
        self.on_tab_change = Some(f);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<TabControl<Msg>> for Widget<Msg> {
    fn from(tc: TabControl<Msg>) -> Self {
        Widget::TabControl {
            tabs: tc.tabs,
            selected: tc.selected,
            tab_style: tc.tab_style,
            on_tab_change: tc.on_tab_change,
            bounds: tc.style.bounds,
            layout: tc.style.layout,
        }
    }
}

pub struct Toolbar<Msg> {
    buttons: Vec<String>,
    toolbar_style: u32,
    on_button_click: Option<fn(usize) -> Msg>,
    style: Style,
    _phantom: std::marker::PhantomData<fn(Msg)>,
}

impl<Msg> Toolbar<Msg> {
    pub fn new(buttons: &[&str]) -> Self {
        Self {
            buttons: buttons.iter().map(|s| s.to_string()).collect(),
            toolbar_style: 0,
            on_button_click: None,
            style: Style::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn toolbar_style(mut self, s: u32) -> Self {
        self.toolbar_style = s;
        self
    }

    pub fn on_button_click(mut self, f: fn(usize) -> Msg) -> Self {
        self.on_button_click = Some(f);
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        self.style = self.style.width(w);
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        self.style = self.style.height(h);
        self
    }

    pub fn style(mut self, f: impl FnOnce(Style) -> Style) -> Self {
        self.style = f(self.style);
        self
    }
}

impl<Msg> From<Toolbar<Msg>> for Widget<Msg> {
    fn from(tb: Toolbar<Msg>) -> Self {
        Widget::Toolbar {
            buttons: tb.buttons,
            toolbar_style: tb.toolbar_style,
            on_button_click: tb.on_button_click,
            bounds: tb.style.bounds,
            layout: tb.style.layout,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    enum TestMsg {
        Click,
    }

    #[test]
    fn test_column_builder() {
        let w: Widget<TestMsg> = Column::new()
            .push(Label::new("Hello"))
            .push(Button::new("Click").on_click(TestMsg::Click))
            .into();
        assert!(w.is_container());
        assert_eq!(w.children().len(), 2);
    }

    #[test]
    fn test_variant_eq() {
        let a: Widget<TestMsg> = Label::new("A").into();
        let b: Widget<TestMsg> = Label::new("B").into();
        let c: Widget<TestMsg> = Button::new("B").into();
        assert!(a.variant_eq(&b));
        assert!(!a.variant_eq(&c));
    }

    #[test]
    fn test_button_on_click() {
        let w: Widget<TestMsg> = Button::new("Go").on_click(TestMsg::Click).into();
        match w {
            Widget::Button { on_click, .. } => {
                assert_eq!(on_click, Some(TestMsg::Click));
            }
            _ => panic!("expected Button"),
        }
    }

    #[test]
    fn test_bounds() {
        let w: Widget<TestMsg> = Button::new("B").style(|s| s.pos(10.0, 20.0)).into();
        let b = w.bounds();
        assert_eq!(b.x, 10.0);
        assert_eq!(b.y, 20.0);
    }

    #[test]
    fn test_style_width_height_bounds_sync() {
        let s = Style::new().width(100.0).height(50.0);
        assert_eq!(s.bounds.w, 100.0);
        assert_eq!(s.bounds.h, 50.0);
        assert_eq!(s.layout.width, Length::Px(100.0));
        assert_eq!(s.layout.height, Length::Px(50.0));

        let s2 = s.width(Length::Fill).height(Length::Percent(50.0));
        assert_eq!(s2.bounds.w, 0.0);
        assert_eq!(s2.bounds.h, 0.0);
        assert_eq!(s2.layout.width, Length::Fill);
        assert_eq!(s2.layout.height, Length::Percent(50.0));
    }
}
