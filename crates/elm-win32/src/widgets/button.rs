use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

pub(crate) fn create_button_hwnd(
    parent: HWND,
    text: &str,
    bs_style: u32,
) -> Result<HWND> {
    let style = WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | bs_style);
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let text_h = HSTRING::from(text);
    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            &text_h,
            style,
            0,
            0,
            100,
            30,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )
    }
}

pub(crate) fn update_button_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_text, new_text) = match (old, new) {
        (Widget::Button { text: old_t, .. }, Widget::Button { text: new_t, .. }) => (old_t, new_t),
        _ => return,
    };
    if old_text != new_text {
        let _ = unsafe { SetWindowTextW(hwnd, &HSTRING::from(new_text)) };
    }
}
