use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

const BASE_STYLE: u32 = WS_CHILD.0 | WS_VISIBLE.0 | 0x0009; // BS_AUTORADIOBUTTON
const GROUP_STYLE: u32 = 0x20000; // WS_GROUP

pub(crate) fn create_radiobutton_hwnd(
    parent: HWND,
    text: &str,
    checked: bool,
    group: bool,
) -> Result<HWND> {
    let mut style = BASE_STYLE;
    if group {
        style |= GROUP_STYLE;
    }
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let text_h = HSTRING::from(text);
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            &text_h,
            WINDOW_STYLE(style),
            0,
            0,
            100,
            24,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )?
    };
    unsafe {
        SendMessageW(
            hwnd,
            BM_SETCHECK,
            Some(WPARAM(checked as usize)),
            Some(LPARAM(0)),
        );
    }
    Ok(hwnd)
}

#[allow(dead_code)]
pub(crate) fn get_checked(hwnd: HWND) -> bool {
    let ret = unsafe { SendMessageW(hwnd, BM_GETCHECK, Some(WPARAM(0)), Some(LPARAM(0))) };
    ret.0 != 0
}

pub(crate) fn update_radiobutton_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_checked, new_checked, old_text, new_text) = match (old, new) {
        (
            Widget::RadioButton {
                checked: oc,
                text: ot,
                ..
            },
            Widget::RadioButton {
                checked: nc,
                text: nt,
                ..
            },
        ) => (oc, nc, ot, nt),
        _ => return,
    };
    if old_checked != new_checked {
        unsafe {
            SendMessageW(
                hwnd,
                BM_SETCHECK,
                Some(WPARAM(*new_checked as usize)),
                Some(LPARAM(0)),
            );
        }
    }
    if old_text != new_text {
        let _ = unsafe { SetWindowTextW(hwnd, &HSTRING::from(new_text)) };
    }
}
