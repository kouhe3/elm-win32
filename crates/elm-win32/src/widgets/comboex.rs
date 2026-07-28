use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

pub(crate) fn create_comboex_hwnd(
    parent: HWND,
    items: &[String],
    selected: Option<usize>,
) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            WC_COMBOBOXEXW,
            w!(""),
            WINDOW_STYLE(
                WS_CHILD.0
                    | WS_VISIBLE.0
                    | WS_VSCROLL.0
                    | 0x0003u32, /* CBS_DROPDOWNLIST */
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

    for item in items {
        let mut wide: Vec<u16> = item.encode_utf16().collect();
        wide.push(0);
        let cei = COMBOBOXEXITEMW {
            mask: CBEIF_TEXT,
            iItem: -1,
            pszText: PWSTR(wide.as_mut_ptr()),
            cchTextMax: (wide.len() - 1) as i32,
            iImage: -1,
            iSelectedImage: -1,
            iOverlay: -1,
            iIndent: 0,
            lParam: LPARAM(0),
        };
        unsafe {
            SendMessageW(
                hwnd,
                CBEM_INSERTITEMW,
                Some(WPARAM(0)),
                Some(LPARAM(std::ptr::from_ref(&cei) as isize)),
            );
        }
    }

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

pub(crate) fn update_comboex_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_items, old_selected, new_items, new_selected, new_selected_exists) = match (old, new) {
        (
            Widget::ComboBoxEx { items: a, selected: os, .. },
            Widget::ComboBoxEx { items: b, selected: ns, .. },
        ) => (a, *os, b, ns.unwrap_or(0), ns.is_some()),
        _ => return,
    };
    if old_items != new_items {
        unsafe {
            let _ = SendMessageW(hwnd, CB_RESETCONTENT, Some(WPARAM(0)), Some(LPARAM(0)));
        }
        for item in new_items {
            let mut wide: Vec<u16> = item.encode_utf16().collect();
            wide.push(0);
            let cei = COMBOBOXEXITEMW {
                mask: CBEIF_TEXT,
                iItem: -1,
                pszText: PWSTR(wide.as_mut_ptr()),
                cchTextMax: (wide.len() - 1) as i32,
                iImage: -1,
                iSelectedImage: -1,
                iOverlay: -1,
                iIndent: 0,
                lParam: LPARAM(0),
            };
            unsafe {
                SendMessageW(
                    hwnd,
                    CBEM_INSERTITEMW,
                    Some(WPARAM(0)),
                    Some(LPARAM(std::ptr::from_ref(&cei) as isize)),
                );
            }
        }
    }
    if new_selected_exists && old_selected != Some(new_selected) {
        unsafe {
            SendMessageW(hwnd, CB_SETCURSEL, Some(WPARAM(new_selected)), Some(LPARAM(0)));
        }
    }
}
