#[derive(Debug, Clone, Copy)]
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
        Self {
            x: 0.0,
            y: 0.0,
            w: 100.0,
            h: 24.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
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

#[derive(Debug, Clone, Copy, Default)]
pub enum CheckBoxStyle {
    #[default]
    Auto,       // BS_AUTOCHECKBOX
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

#[derive(Debug, Clone, Copy, Default)]
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

#[derive(Debug, Clone, Copy, Default)]
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

#[derive(Default)]
pub struct Style {
    pub bounds: Rect,
    pub button_style: ButtonStyle,
    pub checkbox_style: CheckBoxStyle,
    pub combobox_style: ComboBoxStyle,
    pub datetime_format: DateTimeFormat,
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

    pub fn width(mut self, w: f32) -> Self {
        self.bounds.w = w;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.bounds.h = h;
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
}
