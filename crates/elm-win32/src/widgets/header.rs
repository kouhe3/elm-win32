use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

pub(crate) fn create_header_hwnd(
    parent: HWND,
    columns: &[(String, f32)],
    style_flags: u32,
) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let default_ccs = (CCS_NORESIZE | CCS_NOPARENTALIGN | CCS_NODIVIDER) as u32;
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            WC_HEADER,
            w!(""),
            WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | default_ccs | style_flags),
            0,
            0,
            300,
            24,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )?
    };

    for (text, width) in columns {
        let mut wide: Vec<u16> = text.encode_utf16().collect();
        wide.push(0);
        let hdi = HDITEMW {
            mask: HDI_WIDTH | HDI_TEXT | HDI_FORMAT,
            cxy: (*width) as i32,
            pszText: PWSTR(wide.as_mut_ptr()),
            cchTextMax: (wide.len() - 1) as i32,
            fmt: HEADER_CONTROL_FORMAT_FLAGS(HDF_LEFT.0 | HDF_STRING.0),
            ..Default::default()
        };
        unsafe {
            SendMessageW(
                hwnd,
                HDM_INSERTITEMW,
                Some(WPARAM(0)),
                Some(LPARAM(std::ptr::from_ref(&hdi) as isize)),
            );
        }
    }

    Ok(hwnd)
}

pub(crate) fn update_header_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_columns, new_columns) = match (old, new) {
        (Widget::Header { columns: a, .. }, Widget::Header { columns: b, .. }) => (a, b),
        _ => return,
    };
    if old_columns != new_columns {
        let count = unsafe { SendMessageW(hwnd, HDM_GETITEMCOUNT, None, None) };
        for i in (0..count.0 as usize).rev() {
            unsafe {
                let _ = SendMessageW(hwnd, HDM_DELETEITEM, Some(WPARAM(i)), None);
            }
        }
        for (text, width) in new_columns {
            let mut wide: Vec<u16> = text.encode_utf16().collect();
            wide.push(0);
            let hdi = HDITEMW {
                mask: HDI_WIDTH | HDI_TEXT | HDI_FORMAT,
                cxy: (*width) as i32,
                pszText: PWSTR(wide.as_mut_ptr()),
                cchTextMax: (wide.len() - 1) as i32,
                fmt: HEADER_CONTROL_FORMAT_FLAGS(HDF_LEFT.0 | HDF_STRING.0),
                ..Default::default()
            };
            unsafe {
                SendMessageW(
                    hwnd,
                    HDM_INSERTITEMW,
                    Some(WPARAM(0)),
                    Some(LPARAM(std::ptr::from_ref(&hdi) as isize)),
                );
            }
        }
    }
}
