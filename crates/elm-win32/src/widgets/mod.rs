pub mod button;
pub mod checkbox;
pub mod combobox;
pub mod comboex;
pub mod container;
pub mod datetime;
pub mod edit;
pub mod groupbox;
pub mod label;
pub mod listbox;
pub mod radiobutton;

use crate::widget::Widget;
use windows::Win32::Foundation::HWND;

pub(crate) fn create_hwnd<Msg>(
    parent: HWND,
    widget: &Widget<Msg>,
) -> Result<HWND, Box<dyn std::error::Error>> {
    match widget {
        Widget::Button {
            text, button_style, ..
        } => Ok(button::create_button_hwnd(parent, text, *button_style)?),
        Widget::Label { text, .. } => Ok(label::create_label_hwnd(parent, text)?),
        Widget::TextEdit { text, edit_style, .. } => Ok(edit::create_edit_hwnd(parent, text, *edit_style)?),
        Widget::ListBox { items, .. } => Ok(listbox::create_listbox_hwnd(parent, items)?),
        Widget::ComboBox {
            items,
            combo_style,
            selected,
            ..
        } => Ok(combobox::create_combobox_hwnd(
            parent, items, *combo_style, *selected,
        )?),
        Widget::ComboBoxEx {
            items, selected, ..
        } => Ok(comboex::create_comboex_hwnd(parent, items, *selected)?),
        Widget::DateTime { format, .. } => Ok(datetime::create_datetime_hwnd(parent, *format)?),
        Widget::CheckBox {
            text,
            check_state,
            checkbox_style,
            ..
        } => Ok(checkbox::create_checkbox_hwnd(
            parent, text, *check_state, *checkbox_style,
        )?),
        Widget::RadioButton {
            text,
            checked,
            group,
            ..
        } => Ok(radiobutton::create_radiobutton_hwnd(
            parent, text, *checked, *group,
        )?),
        Widget::GroupBox { text, .. } => Ok(groupbox::create_groupbox_hwnd(parent, text)?),
        Widget::Column { .. } | Widget::Row { .. } => Err("containers have no HWND".into()),
        Widget::None => Err("Widget::None has no HWND".into()),
    }
}

pub(crate) fn update_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    match (old, new) {
        (Widget::Button { .. }, Widget::Button { .. }) => {
            button::update_button_hwnd(hwnd, old, new);
        }
        (Widget::Label { .. }, Widget::Label { .. }) => {
            label::update_label_hwnd(hwnd, old, new);
        }
        (Widget::TextEdit { .. }, Widget::TextEdit { .. }) => {
            edit::update_edit_hwnd(hwnd, old, new);
        }
        (Widget::ListBox { .. }, Widget::ListBox { .. }) => {
            listbox::update_listbox_hwnd(hwnd, old, new);
        }
        (Widget::ComboBox { .. }, Widget::ComboBox { .. }) => {
            combobox::update_combobox_hwnd(hwnd, old, new);
        }
        (Widget::ComboBoxEx { .. }, Widget::ComboBoxEx { .. }) => {
            comboex::update_comboex_hwnd(hwnd, old, new);
        }
        (Widget::DateTime { .. }, Widget::DateTime { .. }) => {
            datetime::update_datetime_hwnd(hwnd, old, new);
        }
        (Widget::CheckBox { .. }, Widget::CheckBox { .. }) => {
            checkbox::update_checkbox_hwnd(hwnd, old, new);
        }
        (Widget::RadioButton { .. }, Widget::RadioButton { .. }) => {
            radiobutton::update_radiobutton_hwnd(hwnd, old, new);
        }
        (Widget::GroupBox { .. }, Widget::GroupBox { .. }) => {
            groupbox::update_groupbox_hwnd(hwnd, old, new);
        }
        _ => {}
    }
}
