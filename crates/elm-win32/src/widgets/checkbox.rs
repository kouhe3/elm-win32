use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

pub(crate) fn create_checkbox_hwnd(
    parent: HWND,
    text: &str,
    check_state: i32,
    cb_style: u32,
) -> Result<HWND> {
    let style = WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | cb_style);
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let text_h = HSTRING::from(text);
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            &text_h,
            style,
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
    if check_state != 0 {
        unsafe {
            SendMessageW(
                hwnd,
                BM_SETCHECK,
                Some(WPARAM(check_state as usize)),
                Some(LPARAM(0)),
            );
        }
    }
    Ok(hwnd)
}

pub(crate) fn get_checked(hwnd: HWND) -> i32 {
    let ret = unsafe { SendMessageW(hwnd, BM_GETCHECK, Some(WPARAM(0)), Some(LPARAM(0))) };
    ret.0 as i32
}

pub(crate) fn update_checkbox_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_state, new_state, old_text, new_text) = match (old, new) {
        (
            Widget::CheckBox {
                check_state: oc,
                text: ot,
                ..
            },
            Widget::CheckBox {
                check_state: nc,
                text: nt,
                ..
            },
        ) => (oc, nc, ot, nt),
        _ => return,
    };
    if old_state != new_state {
        unsafe {
            SendMessageW(
                hwnd,
                BM_SETCHECK,
                Some(WPARAM(*new_state as usize)),
                Some(LPARAM(0)),
            );
        }
    }
    if old_text != new_text {
        let _ = unsafe { SetWindowTextW(hwnd, &HSTRING::from(new_text)) };
    }
}
