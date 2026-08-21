pub use crate::layout::{Align, Edges, LayoutStyle, Length};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        w: 0.0,
        h: 0.0,
    };
}

impl Default for Rect {
    fn default() -> Self {
        Self::ZERO
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonStyle {
    #[default]
    Normal,
    Flat,
    Default,
}

impl ButtonStyle {
    pub(crate) fn win32_style(self) -> u32 {
        match self {
            ButtonStyle::Normal => 0,
            ButtonStyle::Flat => 0x00008000,    // BS_FLAT
            ButtonStyle::Default => 0x00000001, // BS_DEFPUSHBUTTON
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CheckBoxStyle {
    #[default]
    Auto, // BS_AUTOCHECKBOX
    Manual,     // BS_CHECKBOX
    ThreeState, // BS_3STATE
    Auto3State, // BS_AUTO3STATE
}

impl CheckBoxStyle {
    pub(crate) fn win32_style(self) -> u32 {
        match self {
            CheckBoxStyle::Auto => 0x0003,
            CheckBoxStyle::Manual => 0x0002,
            CheckBoxStyle::ThreeState => 0x0005,
            CheckBoxStyle::Auto3State => 0x0006,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ComboBoxStyle {
    #[default]
    DropdownList,
    Dropdown,
    Simple,
}

impl ComboBoxStyle {
    pub(crate) fn win32_style(self) -> u32 {
        match self {
            ComboBoxStyle::Simple => 0x0001,
            ComboBoxStyle::Dropdown => 0x0002,
            ComboBoxStyle::DropdownList => 0x0003,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DateTimeFormat {
    #[default]
    ShortDate,
    LongDate,
    Time,
    ShortDateCentury,
}

impl DateTimeFormat {
    pub(crate) fn win32_style(self) -> u32 {
        match self {
            DateTimeFormat::ShortDate => 0x0000,
            DateTimeFormat::Time => 0x0009,
            DateTimeFormat::LongDate => 0x0004,
            DateTimeFormat::ShortDateCentury => 0x000C,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EditStyle {
    #[default]
    SingleLine,
    MultiLine,
    ReadOnly,
    Password,
    Number,
}

impl EditStyle {
    pub(crate) fn win32_style(self) -> u32 {
        match self {
            EditStyle::SingleLine => 0,
            EditStyle::MultiLine => 0x0004, // ES_MULTILINE
            EditStyle::ReadOnly => 0x0800,  // ES_READONLY
            EditStyle::Password => 0x0020,  // ES_PASSWORD
            EditStyle::Number => 0x2000,    // ES_NUMBER
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(u32);

impl Color {
    pub const WHITE: Self = Self(0x00FFFFFF);
    pub const BLACK: Self = Self(0x00000000);
    pub const GRAY: Self = Self(0x00808080);
    pub const LIGHT_GRAY: Self = Self(0x00E0E0E0);

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self((r as u32) | ((g as u32) << 8) | ((b as u32) << 16))
    }

    pub(crate) fn to_colorref(self) -> u32 {
        self.0
    }
}

impl From<u32> for Color {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Style {
    pub bounds: Rect,
    pub layout: LayoutStyle,
    pub button_style: ButtonStyle,
    pub checkbox_style: CheckBoxStyle,
    pub combobox_style: ComboBoxStyle,
    pub datetime_format: DateTimeFormat,
    pub edit_style: EditStyle,
    pub text_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.bounds.x = x;
        self.bounds.y = y;
        self
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.bounds.w = w;
        self.bounds.h = h;
        self
    }

    pub fn width(mut self, w: impl Into<Length>) -> Self {
        let l = w.into();
        self.bounds.w = match l {
            Length::Px(px) => px,
            _ => 0.0,
        };
        self.layout.width = l;
        self
    }

    pub fn height(mut self, h: impl Into<Length>) -> Self {
        let l = h.into();
        self.bounds.h = match l {
            Length::Px(px) => px,
            _ => 0.0,
        };
        self.layout.height = l;
        self
    }

    pub fn min_width(mut self, w: impl Into<Length>) -> Self {
        self.layout.min_width = w.into();
        self
    }

    pub fn min_height(mut self, h: impl Into<Length>) -> Self {
        self.layout.min_height = h.into();
        self
    }

    pub fn max_width(mut self, w: impl Into<Length>) -> Self {
        self.layout.max_width = w.into();
        self
    }

    pub fn max_height(mut self, h: impl Into<Length>) -> Self {
        self.layout.max_height = h.into();
        self
    }

    pub fn padding(mut self, p: Edges) -> Self {
        self.layout.padding = p;
        self
    }

    pub fn padding_all(mut self, p: impl Into<Length>) -> Self {
        self.layout.padding = Edges::all(p);
        self
    }

    pub fn margin(mut self, m: Edges) -> Self {
        self.layout.margin = m;
        self
    }

    pub fn margin_all(mut self, m: impl Into<Length>) -> Self {
        self.layout.margin = Edges::all(m);
        self
    }

    pub fn gap(mut self, g: impl Into<Length>) -> Self {
        let val = g.into();
        self.layout.gap_row = val;
        self.layout.gap_col = val;
        self
    }

    pub fn spacing(self, s: impl Into<Length>) -> Self {
        self.gap(s)
    }

    pub fn flex_grow(mut self, g: f32) -> Self {
        self.layout.flex_grow = g;
        self
    }

    pub fn flex_shrink(mut self, s: f32) -> Self {
        self.layout.flex_shrink = s;
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

    pub fn align_self(mut self, a: Align) -> Self {
        self.layout.align_self = Some(a);
        self
    }

    pub fn layout(mut self, l: LayoutStyle) -> Self {
        self.layout = l;
        self
    }
    pub fn button_style(mut self, s: ButtonStyle) -> Self {
        self.button_style = s;
        self
    }

    pub fn checkbox_style(mut self, s: CheckBoxStyle) -> Self {
        self.checkbox_style = s;
        self
    }

    pub fn combobox_style(mut self, s: ComboBoxStyle) -> Self {
        self.combobox_style = s;
        self
    }

    pub fn datetime_format(mut self, f: DateTimeFormat) -> Self {
        self.datetime_format = f;
        self
    }

    pub fn edit_style(mut self, s: EditStyle) -> Self {
        self.edit_style = s;
        self
    }

    pub fn text_color(mut self, c: Color) -> Self {
        self.text_color = Some(c);
        self
    }

    pub fn bg_color(mut self, c: Color) -> Self {
        self.bg_color = Some(c);
        self
    }
}
