use elm_win32::*;

#[derive(Debug, Clone)]
enum Msg {
    CounterIncrement,
    CounterDecrement,
    TextChanged(String),
    NumTextChanged(String),
    ToggleCheck(i32),
    RadioSelect(usize),
    ListSelected(usize),
    ComboSelected(usize),
    ComboExSelected(usize),
    DateTimeChanged(u16, u16, u16, u16, u16, u16),
    HeaderColumnClicked(usize),
    TabChanged(usize),
    ToolbarButtonClicked(usize),
}

#[derive(Default)]
struct Model {
    count: i32,
    text: String,
    num_text: String,
    checked: i32,
    radio_idx: usize,
    list_idx: usize,
    combo_idx: usize,
    comboex_idx: usize,
    dt_year: u16,
    dt_month: u16,
    dt_day: u16,
    header_col: usize,
    tab_idx: usize,
    toolbar_btn: usize,
}

struct App;

impl Program for App {
    type Model = Model;
    type Msg = Msg;

    fn init(&self) -> (Self::Model, Cmd<Self::Msg>) {
        (Model::default(), Cmd::none())
    }

    fn update(&self, msg: Self::Msg, model: &mut Self::Model) -> Cmd<Self::Msg> {
        match msg {
            Msg::CounterIncrement => model.count += 1,
            Msg::CounterDecrement => model.count -= 1,
            Msg::TextChanged(s) => model.text = s,
            Msg::NumTextChanged(s) => model.num_text = s,
            Msg::ToggleCheck(v) => model.checked = v,
            Msg::RadioSelect(i) => model.radio_idx = i,
            Msg::ListSelected(i) => model.list_idx = i,
            Msg::ComboSelected(i) => model.combo_idx = i,
            Msg::ComboExSelected(i) => model.comboex_idx = i,
            Msg::DateTimeChanged(y, mo, d, _h, _mi, _s) => {
                model.dt_year = y;
                model.dt_month = mo;
                model.dt_day = d;
            }
            Msg::HeaderColumnClicked(i) => model.header_col = i,
            Msg::TabChanged(i) => model.tab_idx = i,
            Msg::ToolbarButtonClicked(i) => model.toolbar_btn = i,
        }
        Cmd::none()
    }

    fn view(&self, model: &Self::Model) -> Widget<Self::Msg> {
        let list_items: &[&str] = &["Alpha", "Beta", "Gamma", "Delta"];
        let combo_items: &[&str] = &["Red", "Green", "Blue", "Yellow"];

        let col1_x = 20.0;
        let col2_x = 270.0;
        let col3_x = 490.0;
        let section_gap = 40.0;
        let label_gap = 28.0;
        let mut y = 50.0;
        let mut y2 = 50.0f32;
        let mut y3 = 50.0f32;

        Column::new()
            // ---- Column 1 ----
            .push(Label::new("Button").style(|s| s.pos(col1_x, y).size(80.0, 22.0)))
            .push(make_y(&mut y, label_gap))
            .push(
                Button::new("+1")
                    .on_click(Msg::CounterIncrement)
                    .style(|s| s.pos(col1_x, y).size(56.0, 30.0)),
            )
            .push(
                Button::new("-1")
                    .on_click(Msg::CounterDecrement)
                    .style(|s| s.pos(col1_x + 62.0, y).size(56.0, 30.0)),
            )
            .push(Button::new("Flat").style(|s| {
                s.pos(col1_x + 124.0, y)
                    .size(56.0, 30.0)
                    .button_style(ButtonStyle::Flat)
            }))
            .push(make_y(&mut y, section_gap))
            .push(Label::new("TextEdit").style(|s| s.pos(col1_x, y).size(80.0, 22.0)))
            .push(make_y(&mut y, label_gap))
            .push(
                TextEdit::new(&model.text)
                    .on_change(Msg::TextChanged)
                    .style(|s| s.pos(col1_x, y).size(200.0, 26.0)),
            )
            .push(make_y(&mut y, 34.0))
            .push(
                TextEdit::new("password")
                    .style(|s| {
                        s.pos(col1_x, y)
                            .size(200.0, 26.0)
                            .edit_style(EditStyle::Password)
                    }),
            )
            .push(make_y(&mut y, 34.0))
            .push(
                TextEdit::new(&model.num_text)
                    .on_change(Msg::NumTextChanged)
                    .style(|s| {
                        s.pos(col1_x, y)
                            .size(80.0, 26.0)
                            .edit_style(EditStyle::Number)
                    }),
            )
            .push(make_y(&mut y, 34.0))
            .push(
                TextEdit::new("Multi-line edit")
                    .style(|s| {
                        s.pos(col1_x, y)
                            .size(200.0, 80.0)
                            .edit_style(EditStyle::MultiLine)
                    }),
            )
            .push(make_y(&mut y, 88.0))
            .push(
                TextEdit::new("Read only")
                    .style(|s| {
                        s.pos(col1_x, y)
                            .size(200.0, 26.0)
                            .edit_style(EditStyle::ReadOnly)
                    }),
            )
            .push(make_y(&mut y, section_gap))
            .push(Label::new("CheckBox").style(|s| s.pos(col1_x, y).size(80.0, 22.0)))
            .push(make_y(&mut y, label_gap))
            .push(
                CheckBox::new("Auto (2-state)")
                    .check_state(model.checked)
                    .on_toggle(Msg::ToggleCheck)
                    .style(|s| s.pos(col1_x, y).size(180.0, 26.0)),
            )
            .push(make_y(&mut y, 28.0))
            .push(
                CheckBox::new("Manual (2-state)")
                    .check_state(model.checked)
                    .checkbox_style(CheckBoxStyle::Manual)
                    .on_toggle(Msg::ToggleCheck)
                    .style(|s| s.pos(col1_x, y).size(180.0, 26.0)),
            )
            .push(make_y(&mut y, 28.0))
            .push(
                CheckBox::new("Auto (3-state)")
                    .check_state(model.checked)
                    .checkbox_style(CheckBoxStyle::Auto3State)
                    .on_toggle(Msg::ToggleCheck)
                    .style(|s| s.pos(col1_x, y).size(180.0, 26.0)),
            )
            .push(make_y(&mut y, 28.0))
            .push(
                CheckBox::new("Manual (3-state)")
                    .check_state(model.checked)
                    .checkbox_style(CheckBoxStyle::ThreeState)
                    .on_toggle(Msg::ToggleCheck)
                    .style(|s| s.pos(col1_x, y).size(180.0, 26.0)),
            )
            .push(make_y(&mut y, section_gap))
            .push(Label::new("RadioButton").style(|s| s.pos(col1_x, y).size(100.0, 22.0)))
            .push(make_y(&mut y, label_gap))
            .push(
                RadioButton::new("Option A", model.radio_idx == 0)
                    .on_toggle(move |_| Msg::RadioSelect(0))
                    .group()
                    .style(|s| s.pos(col1_x, y).size(160.0, 24.0)),
            )
            .push(make_y(&mut y, 26.0))
            .push(
                RadioButton::new("Option B", model.radio_idx == 1)
                    .on_toggle(move |_| Msg::RadioSelect(1))
                    .style(|s| s.pos(col1_x, y).size(160.0, 24.0)),
            )
            // ---- Column 2 ----
            .push(
                Label::new(&format!("Clicked btn: {}", model.toolbar_btn))
                    .style(|s| s.pos(col2_x, y2).size(180.0, 22.0)),
            )
            .push(make_y(&mut y2, 26.0))
            .push(Label::new("ListBox").style(|s| s.pos(col2_x, y2).size(80.0, 22.0)))
            .push(make_y(&mut y2, label_gap))
            .push(
                ListBox::new(list_items)
                    .on_select(Msg::ListSelected)
                    .style(|s| s.pos(col2_x, y2).size(180.0, 100.0)),
            )
            .push(make_y(&mut y2, 106.0))
            .push(
                Label::new(&format!("List selected: [{}]", model.list_idx))
                    .style(|s| s.pos(col2_x, y2).size(180.0, 22.0)),
            )
            .push(make_y(&mut y2, section_gap - 14.0))
            .push(Label::new("ComboBox").style(|s| s.pos(col2_x, y2).size(80.0, 22.0)))
            .push(make_y(&mut y2, label_gap))
            .push(
                ComboBox::new(combo_items)
                    .on_select(Msg::ComboSelected)
                    .style(|s| s.pos(col2_x, y2).size(180.0, 26.0)),
            )
            .push(make_y(&mut y2, 34.0))
            .push(
                Label::new(&format!("Combo selected: [{}]", model.combo_idx))
                    .style(|s| s.pos(col2_x, y2).size(180.0, 22.0)),
            )
            .push(make_y(&mut y2, section_gap - 14.0))
            .push(Label::new("ComboBoxEx").style(|s| s.pos(col2_x, y2).size(100.0, 22.0)))
            .push(make_y(&mut y2, label_gap))
            .push(
                ComboBoxEx::new(combo_items)
                    .on_select(Msg::ComboExSelected)
                    .style(|s| s.pos(col2_x, y2).size(180.0, 26.0)),
            )
            .push(make_y(&mut y2, 34.0))
            .push(
                Label::new(&format!("ComboEx selected: [{}]", model.comboex_idx))
                    .style(|s| s.pos(col2_x, y2).size(180.0, 22.0)),
            )
            .push(make_y(&mut y2, section_gap - 8.0))
            .push(
                GroupBox::new("GroupBox").style(|s| {
                    s.pos(col2_x, y2).size(180.0, 60.0)
                }),
            )
            .push(make_y(&mut y2, 74.0))
            .push(
                Label::new("Toolbar").style(|s| s.pos(col2_x, y2).size(80.0, 22.0)),
            )
            .push(make_y(&mut y2, label_gap))
            .push(
                Toolbar::new(&["新建", "打开", "保存", "剪切", "复制", "粘贴"])
                    .toolbar_style(0x0800 | 0x1000) // TBSTYLE_FLAT | TBSTYLE_LIST
                    .on_button_click(Msg::ToolbarButtonClicked)
                    .style(|s| s.pos(col2_x, y2).size(400.0, 28.0)),
            )
            .push(make_y(&mut y2, 34.0))
            // ---- Column 3 ----
            .push(Label::new("DateTime").style(|s| s.pos(col3_x, y3).size(80.0, 22.0)))
            .push(make_y(&mut y3, label_gap))
            .push(
                DateTime::new()
                    .on_change(Msg::DateTimeChanged)
                    .style(|s| s.pos(col3_x, y3).size(160.0, 24.0)),
            )
            .push(make_y(&mut y3, section_gap))
            .push(
                Label::new("Time").style(|s| s.pos(col3_x, y3).size(80.0, 22.0)),
            )
            .push(make_y(&mut y3, label_gap))
            .push(
                DateTime::new()
                    .style(|s| s.pos(col3_x, y3).size(160.0, 24.0).datetime_format(DateTimeFormat::Time)),
            )
            .push(make_y(&mut y3, section_gap))
            .push(
                Label::new(&format!(
                    "Date: {}/{}/{}",
                    model.dt_month, model.dt_day, model.dt_year
                ))
                .style(|s| s.pos(col3_x, y3).size(180.0, 22.0)),
            )
            .push(make_y(&mut y3, section_gap))
            .push(
                Label::new("Header").style(|s| s.pos(col3_x, y3).size(80.0, 22.0)),
            )
            .push(make_y(&mut y3, label_gap))
            .push(
                Header::new(&[
                    ("名称", 70.0),
                    ("大小", 55.0),
                    ("类型", 55.0),
                    ("日期", 70.0),
                ])
                .header_style(0x0002) // HDS_BUTTONS
                .on_column_click(Msg::HeaderColumnClicked)
                .style(|s| s.pos(col3_x, y3).size(260.0, 24.0)),
            )
            .push(make_y(&mut y3, 30.0))
            .push(
                Label::new(&format!("Clicked column: {}", model.header_col))
                    .style(|s| s.pos(col3_x, y3).size(180.0, 22.0)),
            )
            .push(make_y(&mut y3, section_gap))
            .push(
                Label::new("TabControl").style(|s| s.pos(col3_x, y3).size(120.0, 22.0)),
            )
            .push(make_y(&mut y3, label_gap))
            .push(
                Label::new(&format!("Selected tab: {}", model.tab_idx))
                    .style(|s| s.pos(col3_x, y3).size(180.0, 22.0)),
            )
            .push(make_y(&mut y3, label_gap))
            .push(
                TabControl::new(&["标签一", "标签二", "标签三", "标签四"])
                    .selected(model.tab_idx)
                    .on_tab_change(Msg::TabChanged)
                    .style(|s| s.pos(0.0,0.0).size(260.0, 120.0)),
            )
            .push(make_y(&mut y3, 126.0))
            // ---- Status bar ----
            .push(
                Label::new(&format!(
                    "Counter: {} | CheckState: {} | Text: {}",
                    model.count, model.checked, model.text
                ))
                .style(|s| s.pos(20.0, 460.0).size(560.0, 24.0)),
            )
            .into()
    }
}

fn make_y(y: &mut f32, dy: f32) -> Widget<Msg> {
    *y += dy;
    Widget::None
}

fn main() {
    App.run(WindowConfig::new("Widget Gallery", 800.0, 900.0));
}
