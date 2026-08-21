use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

pub(crate) fn create_toolbar_hwnd(
    parent: HWND,
    buttons: &[String],
    style_flags: u32,
) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let default_ccs = (CCS_NORESIZE | CCS_NOPARENTALIGN | CCS_NODIVIDER) as u32;
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            TOOLBARCLASSNAMEW,
            w!(""),
            WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | default_ccs | style_flags),
            0,
            0,
            300,
            28,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )?
    };

    unsafe {
        SendMessageW(
            hwnd,
            TB_BUTTONSTRUCTSIZE,
            Some(WPARAM(std::mem::size_of::<TBBUTTON>())),
            None,
        );
    }

    for (i, text) in buttons.iter().enumerate() {
        let mut wide: Vec<u16> = text.encode_utf16().collect();
        wide.push(0);
        let tbb = TBBUTTON {
            iBitmap: -1,
            idCommand: i as i32,
            fsState: TBSTATE_ENABLED as u8,
            fsStyle: (BTNS_BUTTON | BTNS_AUTOSIZE | BTNS_SHOWTEXT) as u8,
            dwData: 0,
            iString: wide.as_mut_ptr() as isize,
            ..Default::default()
        };
        unsafe {
            SendMessageW(
                hwnd,
                TB_ADDBUTTONSW,
                Some(WPARAM(1)),
                Some(LPARAM(std::ptr::from_ref(&tbb) as isize)),
            );
        }
    }

    Ok(hwnd)
}

pub(crate) fn update_toolbar_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_buttons, new_buttons) = match (old, new) {
        (
            Widget::Toolbar {
                buttons: a, ..
            },
            Widget::Toolbar {
                buttons: b, ..
            },
        ) => (a, b),
        _ => return,
    };
    if old_buttons != new_buttons {
        let count = unsafe { SendMessageW(hwnd, TB_BUTTONCOUNT, None, None) };
        for _ in 0..count.0 as usize {
            unsafe {
                let _ = SendMessageW(hwnd, TB_DELETEBUTTON, Some(WPARAM(0)), None);
            }
        }
        for (i, text) in new_buttons.iter().enumerate() {
            let mut wide: Vec<u16> = text.encode_utf16().collect();
            wide.push(0);
            let tbb = TBBUTTON {
                iBitmap: -1,
                idCommand: i as i32,
                fsState: TBSTATE_ENABLED as u8,
                fsStyle: (BTNS_BUTTON | BTNS_AUTOSIZE | BTNS_SHOWTEXT) as u8,
                dwData: 0,
                iString: wide.as_mut_ptr() as isize,
                ..Default::default()
            };
            unsafe {
                SendMessageW(
                    hwnd,
                    TB_ADDBUTTONSW,
                    Some(WPARAM(1)),
                    Some(LPARAM(std::ptr::from_ref(&tbb) as isize)),
                );
            }
        }
    }
}
