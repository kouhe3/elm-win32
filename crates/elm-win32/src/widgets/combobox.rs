use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

fn add_items(hwnd: HWND, items: &[String]) {
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
}

pub(crate) fn create_combobox_hwnd(
    parent: HWND,
    items: &[String],
    combo_style: u32,
    selected: Option<usize>,
) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("COMBOBOX"),
            w!(""),
            WINDOW_STYLE(
                WS_CHILD.0 | WS_VISIBLE.0 | combo_style | 0x0200, /* CBS_HASSTRINGS */
            ),
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

    add_items(hwnd, items);

    if let Some(idx) = selected {
        unsafe {
            SendMessageW(hwnd, CB_SETCURSEL, Some(WPARAM(idx)), Some(LPARAM(0)));
        }
    }

    Ok(hwnd)
}

pub(crate) fn get_selected_index(hwnd: HWND) -> usize {
    let ret = unsafe { SendMessageW(hwnd, CB_GETCURSEL, Some(WPARAM(0)), Some(LPARAM(0))) };
    if ret.0 >= 0 { ret.0 as usize } else { 0 }
}

pub(crate) fn get_edit_text(hwnd: HWND) -> String {
    let len = unsafe { SendMessageW(hwnd, WM_GETTEXTLENGTH, Some(WPARAM(0)), Some(LPARAM(0))) };
    if len.0 <= 0 {
        return String::new();
    }
    let cap = (len.0 + 1) as usize;
    let mut buf = vec![0u16; cap];
    unsafe {
        SendMessageW(
            hwnd,
            WM_GETTEXT,
            Some(WPARAM(cap)),
            Some(LPARAM(buf.as_mut_ptr() as isize)),
        );
    }
    if let Some(null_pos) = buf.iter().position(|&c| c == 0) {
        buf.truncate(null_pos);
    }
    String::from_utf16_lossy(&buf)
}

pub(crate) fn update_combobox_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_items, old_selected, new_items, new_selected) = match (old, new) {
        (
            Widget::ComboBox {
                items: a,
                selected: os,
                ..
            },
            Widget::ComboBox {
                items: b,
                selected: ns,
                ..
            },
        ) => (a, *os, b, ns),
        _ => return,
    };
    if old_items != new_items {
        unsafe {
            let _ = SendMessageW(hwnd, CB_RESETCONTENT, Some(WPARAM(0)), Some(LPARAM(0)));
        }
        add_items(hwnd, new_items);
    }
    if old_selected != *new_selected
        && let Some(selected) = new_selected
    {
        unsafe {
            SendMessageW(hwnd, CB_SETCURSEL, Some(WPARAM(*selected)), Some(LPARAM(0)));
        }
    }
}
