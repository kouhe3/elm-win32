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

        Column::new()
            .padding_all(20.0)
            .spacing(16.0)
            // ---- Top 3-Column Layout ----
            .push(
                Row::new()
                    .spacing(24.0)
                    // ---- Column 1: Buttons, Edits, Checks, Radios ----
                    .push(
                        Column::new()
                            .flex_grow(1.0)
                            .spacing(8.0)
                            .push(Label::new("Button"))
                            .push(
                                Row::new()
                                    .spacing(6.0)
                                    .push(Button::new("+1").on_click(Msg::CounterIncrement).width(56.0))
                                    .push(Button::new("-1").on_click(Msg::CounterDecrement).width(56.0))
                                    .push(
                                        Button::new("Flat")
                                            .width(56.0)
                                            .style(|s| s.button_style(ButtonStyle::Flat)),
                                    ),
                            )
                            .push(Label::new("TextEdit"))
                            .push(
                                TextEdit::new(&model.text)
                                    .on_change(Msg::TextChanged)
                                    .width(200.0),
                            )
                            .push(
                                TextEdit::new("password")
                                    .width(200.0)
                                    .style(|s| s.edit_style(EditStyle::Password)),
                            )
                            .push(
                                TextEdit::new(&model.num_text)
                                    .on_change(Msg::NumTextChanged)
                                    .width(80.0)
                                    .style(|s| s.edit_style(EditStyle::Number)),
                            )
                            .push(
                                TextEdit::new("Multi-line edit")
                                    .width(200.0)
                                    .height(60.0)
                                    .style(|s| s.edit_style(EditStyle::MultiLine)),
                            )
                            .push(
                                TextEdit::new("Read only")
                                    .width(200.0)
                                    .style(|s| s.edit_style(EditStyle::ReadOnly)),
                            )
                            .push(Label::new("CheckBox"))
                            .push(
                                CheckBox::new("Auto (2-state)")
                                    .check_state(model.checked)
                                    .on_toggle(Msg::ToggleCheck),
                            )
                            .push(
                                CheckBox::new("Manual (2-state)")
                                    .check_state(model.checked)
                                    .checkbox_style(CheckBoxStyle::Manual)
                                    .on_toggle(Msg::ToggleCheck),
                            )
                            .push(
                                CheckBox::new("Auto (3-state)")
                                    .check_state(model.checked)
                                    .checkbox_style(CheckBoxStyle::Auto3State)
                                    .on_toggle(Msg::ToggleCheck),
                            )
                            .push(
                                CheckBox::new("Manual (3-state)")
                                    .check_state(model.checked)
                                    .checkbox_style(CheckBoxStyle::ThreeState)
                                    .on_toggle(Msg::ToggleCheck),
                            )
                            .push(Label::new("RadioButton"))
                            .push(
                                RadioButton::new("Option A", model.radio_idx == 0)
                                    .on_toggle(move |_| Msg::RadioSelect(0))
                                    .group(),
                            )
                            .push(
                                RadioButton::new("Option B", model.radio_idx == 1)
                                    .on_toggle(move |_| Msg::RadioSelect(1)),
                            ),
                    )
                    // ---- Column 2: ListBox, ComboBox, GroupBox, Toolbar ----
                    .push(
                        Column::new()
                            .flex_grow(1.0)
                            .spacing(8.0)
                            .push(Label::new("ListBox"))
                            .push(
                                ListBox::new(list_items)
                                    .on_select(Msg::ListSelected)
                                    .width(180.0)
                                    .height(90.0),
                            )
                            .push(Label::new(&format!("List selected: [{}]", model.list_idx)))
                            .push(Label::new("ComboBox"))
                            .push(
                                ComboBox::new(combo_items)
                                    .on_select(Msg::ComboSelected)
                                    .width(180.0),
                            )
                            .push(Label::new(&format!("Combo selected: [{}]", model.combo_idx)))
                            .push(Label::new("ComboBoxEx"))
                            .push(
                                ComboBoxEx::new(combo_items)
                                    .on_select(Msg::ComboExSelected)
                                    .width(180.0),
                            )
                            .push(Label::new(&format!("ComboEx selected: [{}]", model.comboex_idx)))
                            .push(
                                GroupBox::new("GroupBox")
                                    .width(180.0)
                                    .height(60.0),
                            )
                            .push(Label::new("Toolbar"))
                            .push(
                                Toolbar::new(&["新建", "打开", "保存", "剪切", "复制", "粘贴"])
                                    .toolbar_style(0x0800 | 0x1000)
                                    .on_button_click(Msg::ToolbarButtonClicked)
                                    .width(360.0)
                                    .height(28.0),
                            )
                            .push(Label::new(&format!("Clicked btn: {}", model.toolbar_btn))),
                    )
                    // ---- Column 3: DateTime, Header, TabControl ----
                    .push(
                        Column::new()
                            .flex_grow(1.0)
                            .spacing(8.0)
                            .push(Label::new("DateTime"))
                            .push(
                                DateTime::new()
                                    .on_change(Msg::DateTimeChanged)
                                    .width(160.0),
                            )
                            .push(Label::new("Time"))
                            .push(
                                DateTime::new()
                                    .width(160.0)
                                    .style(|s| s.datetime_format(DateTimeFormat::Time)),
                            )
                            .push(Label::new(&format!(
                                "Date: {}/{}/{}",
                                model.dt_month, model.dt_day, model.dt_year
                            )))
                            .push(Label::new("Header"))
                            .push(
                                Header::new(&[
                                    ("名称", 70.0),
                                    ("大小", 55.0),
                                    ("类型", 55.0),
                                    ("日期", 70.0),
                                ])
                                .header_style(0x0002)
                                .on_column_click(Msg::HeaderColumnClicked)
                                .width(250.0),
                            )
                            .push(Label::new(&format!("Clicked column: {}", model.header_col)))
                            .push(Label::new("TabControl"))
                            .push(Label::new(&format!("Selected tab: {}", model.tab_idx)))
                            .push(
                                TabControl::new(&["标签一", "标签二", "标签三", "标签四"])
                                    .selected(model.tab_idx)
                                    .on_tab_change(Msg::TabChanged)
                                    .width(260.0)
                                    .height(120.0),
                            ),
                    ),
            )
            // ---- Status Bar at Bottom ----
            .push(
                Label::new(&format!(
                    "Counter: {} | CheckState: {} | Text: {}",
                    model.count, model.checked, model.text
                ))
                .width(Length::Fill)
                .height(24.0),
            )
            .into()
    }
}

fn main() {
    App.run(WindowConfig::new("Widget Gallery (Taffy Powered)", 920.0, 780.0));
}
