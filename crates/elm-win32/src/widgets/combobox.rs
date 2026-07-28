use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

const COMBOBOX_STYLE: u32 =
    WS_CHILD.0 | WS_VISIBLE.0 | 0x0003 /* CBS_DROPDOWNLIST */ | 0x0200 /* CBS_HASSTRINGS */;

pub(crate) fn create_combobox_hwnd(parent: HWND, items: &[String]) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("COMBOBOX"),
            w!(""),
            WINDOW_STYLE(COMBOBOX_STYLE),
            0,
            0,
            200,
            200,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )?
    };

    for item in items {
        let text = HSTRING::from(item.as_str());
        unsafe {
            SendMessageW(
                hwnd,
                CB_ADDSTRING,
                Some(WPARAM(0)),
                Some(LPARAM(text.as_ptr() as isize)),
            );
        }
    }

    Ok(hwnd)
}

pub(crate) fn get_selected_index(hwnd: HWND) -> usize {
    let ret = unsafe { SendMessageW(hwnd, CB_GETCURSEL, Some(WPARAM(0)), Some(LPARAM(0))) };
    if ret.0 >= 0 { ret.0 as usize } else { 0 }
}

pub(crate) fn update_combobox_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_items, new_items) = match (old, new) {
        (Widget::ComboBox { items: a, .. }, Widget::ComboBox { items: b, .. }) => (a, b),
        _ => return,
    };
    if old_items != new_items {
        unsafe {
            let _ = SendMessageW(hwnd, CB_RESETCONTENT, Some(WPARAM(0)), Some(LPARAM(0)));
        }
        for item in new_items {
            let text = HSTRING::from(item.as_str());
            unsafe {
                SendMessageW(
                    hwnd,
                    CB_ADDSTRING,
                    Some(WPARAM(0)),
                    Some(LPARAM(text.as_ptr() as isize)),
                );
            }
        }
    }
}
