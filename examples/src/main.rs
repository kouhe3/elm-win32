use elm_win32::*;

#[derive(Debug, Clone)]
enum Msg {
    CounterIncrement,
    CounterDecrement,
    TextChanged(String),
    ToggleCheck(i32),
    RadioSelect(usize),
    ListSelected(usize),
    ComboSelected(usize),
}

#[derive(Default)]
struct Model {
    count: i32,
    text: String,
    checked: i32,
    radio_idx: usize,
    list_idx: usize,
    combo_idx: usize,
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
            Msg::ToggleCheck(v) => model.checked = v,
            Msg::RadioSelect(i) => model.radio_idx = i,
            Msg::ListSelected(i) => model.list_idx = i,
            Msg::ComboSelected(i) => model.combo_idx = i,
        }
        Cmd::none()
    }

    fn view(&self, model: &Self::Model) -> Widget<Self::Msg> {
        let list_items: &[&str] = &["Alpha", "Beta", "Gamma", "Delta"];
        let combo_items: &[&str] = &["Red", "Green", "Blue", "Yellow"];

        let col1_x = 20.0;
        let col2_x = 270.0;
        let section_gap = 40.0;
        let label_gap = 28.0;
        let mut y = 20.0;

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
            .push(Label::new("ListBox").style(|s| s.pos(col2_x, 20.0).size(80.0, 22.0)))
            .push(
                ListBox::new(list_items)
                    .on_select(Msg::ListSelected)
                    .style(|s| s.pos(col2_x, 48.0).size(180.0, 100.0)),
            )
            .push(
                Label::new(&format!("List selected: [{}]", model.list_idx))
                    .style(|s| s.pos(col2_x, 154.0).size(180.0, 22.0)),
            )
            .push(Label::new("ComboBox").style(|s| s.pos(col2_x, 190.0).size(80.0, 22.0)))
            .push(
                ComboBox::new(combo_items)
                    .on_select(Msg::ComboSelected)
                    .style(|s| s.pos(col2_x, 218.0).size(180.0, 26.0)),
            )
            .push(
                Label::new(&format!("Combo selected: [{}]", model.combo_idx))
                    .style(|s| s.pos(col2_x, 252.0).size(180.0, 22.0)),
            )
            .push(
                GroupBox::new("GroupBox").style(|s| {
                    s.pos(col2_x, 290.0).size(180.0, 60.0)
                }),
            )
            // ---- Status bar ----
            .push(
                Label::new(&format!(
                    "Counter: {} | CheckState: {} | Text: {}",
                    model.count, model.checked, model.text
                ))
                .style(|s| s.pos(20.0, 400.0).size(440.0, 24.0)),
            )
            .into()
    }
}

fn make_y(y: &mut f32, dy: f32) -> Widget<Msg> {
    *y += dy;
    Widget::None
}

fn main() {
    App.run(WindowConfig::new("Widget Gallery", 600.0, 500.0));
}
