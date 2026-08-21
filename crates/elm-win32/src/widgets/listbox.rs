use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

const LBS_NOTIFY: u32 = 1;
const LISTBOX_STYLE: u32 = WS_CHILD.0 | WS_VISIBLE.0 | WS_BORDER.0 | WS_VSCROLL.0 | LBS_NOTIFY;

fn add_items(hwnd: HWND, items: &[String]) {
    for item in items {
        let text = HSTRING::from(item.as_str());
        unsafe {
            SendMessageW(
                hwnd,
                LB_ADDSTRING,
                Some(WPARAM(0)),
                Some(LPARAM(text.as_ptr() as isize)),
            );
        }
    }
}

pub(crate) fn create_listbox_hwnd(parent: HWND, items: &[String]) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("LISTBOX"),
            w!(""),
            WINDOW_STYLE(LISTBOX_STYLE),
            0,
            0,
            100,
            100,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )?
    };

    add_items(hwnd, items);

    Ok(hwnd)
}

pub(crate) fn get_selected_index(hwnd: HWND) -> usize {
    let ret = unsafe { SendMessageW(hwnd, LB_GETCURSEL, Some(WPARAM(0)), Some(LPARAM(0))) };
    if ret.0 >= 0 { ret.0 as usize } else { 0 }
}

pub(crate) fn update_listbox_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_items, new_items) = match (old, new) {
        (Widget::ListBox { items: a, .. }, Widget::ListBox { items: b, .. }) => (a, b),
        _ => return,
    };
    if old_items != new_items {
        unsafe {
            let _ = SendMessageW(hwnd, LB_RESETCONTENT, Some(WPARAM(0)), Some(LPARAM(0)));
        }
        add_items(hwnd, new_items);
    }
}
