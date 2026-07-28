use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

const GROUPBOX_STYLE: u32 = WS_CHILD.0 | WS_VISIBLE.0 | 0x0007; // BS_GROUPBOX

pub(crate) fn create_groupbox_hwnd(parent: HWND, text: &str) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let text_h = HSTRING::from(text);
    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            &text_h,
            WINDOW_STYLE(GROUPBOX_STYLE),
            0,
            0,
            100,
            80,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )
    }
}

pub(crate) fn update_groupbox_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_text, new_text) = match (old, new) {
        (Widget::GroupBox { text: ot, .. }, Widget::GroupBox { text: nt, .. }) => (ot, nt),
        _ => return,
    };
    if old_text != new_text {
        let _ = unsafe { SetWindowTextW(hwnd, &HSTRING::from(new_text)) };
    }
}
