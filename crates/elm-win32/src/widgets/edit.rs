use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

const EDIT_STYLE: WINDOW_STYLE = WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | WS_BORDER.0);

pub(crate) fn create_edit_hwnd(parent: HWND, text: &str) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let text_h = HSTRING::from(text);
    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("EDIT"),
            &text_h,
            EDIT_STYLE,
            0,
            0,
            100,
            24,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )
    }
}

pub(crate) fn get_edit_text(hwnd: HWND) -> String {
    unsafe {
        let len = GetWindowTextLengthW(hwnd) as usize;
        if len == 0 {
            return String::new();
        }
        let mut buf = vec![0u16; len + 1];
        GetWindowTextW(hwnd, &mut buf);
        String::from_utf16_lossy(&buf[..len])
    }
}

pub(crate) fn update_edit_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_text, new_text) = match (old, new) {
        (Widget::TextEdit { text: old_t, .. }, Widget::TextEdit { text: new_t, .. }) => {
            (old_t, new_t)
        }
        _ => return,
    };
    if old_text != new_text {
        let _ = unsafe { SetWindowTextW(hwnd, &HSTRING::from(new_text)) };
    }
}
