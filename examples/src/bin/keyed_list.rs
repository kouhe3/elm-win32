use elm_win32::*;

#[derive(Debug, Clone)]
enum Msg {
    InsertFirst,
    Reverse,
    Remove(u32),
}

#[derive(Clone)]
struct Item {
    id: u32,
    label: String,
}

struct Model {
    next_id: u32,
    items: Vec<Item>,
}

struct KeyedList;

impl Program for KeyedList {
    type Model = Model;
    type Msg = Msg;

    fn init(&self) -> (Self::Model, Cmd<Self::Msg>) {
        (
            Model {
                next_id: 4,
                items: vec![
                    Item {
                        id: 1,
                        label: "Alpha".into(),
                    },
                    Item {
                        id: 2,
                        label: "Beta".into(),
                    },
                    Item {
                        id: 3,
                        label: "Gamma".into(),
                    },
                ],
            },
            Cmd::none(),
        )
    }

    fn update(&self, msg: Self::Msg, model: &mut Self::Model) -> Cmd<Self::Msg> {
        match msg {
            Msg::InsertFirst => {
                let id = model.next_id;
                model.next_id += 1;
                model.items.insert(
                    0,
                    Item {
                        id,
                        label: format!("Item {id}"),
                    },
                );
            }
            Msg::Reverse => model.items.reverse(),
            Msg::Remove(id) => model.items.retain(|item| item.id != id),
        }
        Cmd::none()
    }

    fn view(&self, model: &Self::Model) -> Widget<Self::Msg> {
        let mut list = Column::new().spacing(8.0).width(Length::Fill);
        for item in &model.items {
            list = list.push(
                Row::new()
                    .spacing(8.0)
                    .width(Length::Fill)
                    .push(Label::new(&format!("#{}", item.id)).width(48.0))
                    .push(TextEdit::new(&item.label).flex_grow(1.0))
                    .push(Button::new("Remove").on_click(Msg::Remove(item.id)))
                    .key(item.id.to_string()),
            );
        }

        Column::new()
            .padding_all(20.0)
            .spacing(12.0)
            .push(Label::new("Keyed dynamic TextEdit list"))
            .push(
                Label::new(
                    "Focus or edit a row, then insert or reverse. The HWND state follows its key.",
                )
                .width(Length::Fill),
            )
            .push(
                Row::new()
                    .spacing(8.0)
                    .push(Button::new("Insert first").on_click(Msg::InsertFirst))
                    .push(Button::new("Reverse").on_click(Msg::Reverse)),
            )
            .push(list)
            .into()
    }
}

fn main() {
    KeyedList.run(WindowConfig::new("elm-win32 keyed list", 620.0, 440.0));
}
