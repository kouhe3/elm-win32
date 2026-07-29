use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

pub(crate) fn create_tabcontrol_hwnd(
    parent: HWND,
    tabs: &[String],
    selected: Option<usize>,
    style_flags: u32,
) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            WC_TABCONTROLW,
            w!(""),
            WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | style_flags),
            0,
            0,
            300,
            200,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )?
    };

    for tab_text in tabs {
        let mut wide: Vec<u16> = tab_text.encode_utf16().collect();
        wide.push(0);
        let tc = TCITEMW {
            mask: TCIF_TEXT,
            pszText: PWSTR(wide.as_mut_ptr()),
            cchTextMax: (wide.len() - 1) as i32,
            ..Default::default()
        };
        unsafe {
            SendMessageW(
                hwnd,
                TCM_INSERTITEMW,
                Some(WPARAM(0)),
                Some(LPARAM(std::ptr::from_ref(&tc) as isize)),
            );
        }
    }

    if let Some(idx) = selected {
        unsafe {
            SendMessageW(hwnd, TCM_SETCURSEL, Some(WPARAM(idx)), Some(LPARAM(0)));
        }
    }

    Ok(hwnd)
}

pub(crate) fn get_selected_index(hwnd: HWND) -> usize {
    let ret = unsafe { SendMessageW(hwnd, TCM_GETCURSEL, None, None) };
    ret.0 as usize
}

pub(crate) fn update_tabcontrol_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_tabs, old_selected, new_tabs, new_selected) = match (old, new) {
        (
            Widget::TabControl {
                tabs: a,
                selected: os,
                ..
            },
            Widget::TabControl {
                tabs: b,
                selected: ns,
                ..
            },
        ) => (a, *os, b, *ns),
        _ => return,
    };

    if old_tabs != new_tabs {
        unsafe {
            let _ = SendMessageW(hwnd, TCM_DELETEALLITEMS, None, None);
        }
        for tab_text in new_tabs {
            let mut wide: Vec<u16> = tab_text.encode_utf16().collect();
            wide.push(0);
            let tc = TCITEMW {
                mask: TCIF_TEXT,
                pszText: PWSTR(wide.as_mut_ptr()),
                cchTextMax: (wide.len() - 1) as i32,
                ..Default::default()
            };
            unsafe {
                SendMessageW(
                    hwnd,
                    TCM_INSERTITEMW,
                    Some(WPARAM(0)),
                    Some(LPARAM(std::ptr::from_ref(&tc) as isize)),
                );
            }
        }
    }

    if old_selected != new_selected {
        if let Some(idx) = new_selected {
            unsafe {
                SendMessageW(hwnd, TCM_SETCURSEL, Some(WPARAM(idx)), Some(LPARAM(0)));
            }
        }
    }
}
