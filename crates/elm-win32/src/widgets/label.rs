use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

const LABEL_STYLE: WINDOW_STYLE = WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0);

pub(crate) fn create_label_hwnd(parent: HWND, text: &str) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let text_h = HSTRING::from(text);
    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("STATIC"),
            &text_h,
            LABEL_STYLE,
            0,
            0,
            100,
            20,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )
    }
}

pub(crate) fn update_label_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_text, new_text) = match (old, new) {
        (Widget::Label { text: old_t, .. }, Widget::Label { text: new_t, .. }) => (old_t, new_t),
        _ => return,
    };
    if old_text != new_text {
        let _ = unsafe { SetWindowTextW(hwnd, &HSTRING::from(new_text)) };
    }
}
