use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::{
    TBM_CLEARSEL, TBM_SETPAGESIZE, TBM_SETPOS, TBM_SETRANGE, TBM_SETSEL, TRACKBAR_CLASSW,
};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

// TBM_GETPOS (WM_USER) is missing from windows 0.62 bindings.
const TBM_GETPOS: u32 = 0x0400;

const fn makelong(lo: i32, hi: i32) -> isize {
    ((lo as u16 as usize) | ((hi as u16 as usize) << 16)) as isize
}

/// Creates and initializes a trackbar, mirroring the canonical Win32
/// "Create a Trackbar" sequence: TBM_SETRANGE, TBM_SETPAGESIZE, TBM_SETSEL,
/// TBM_SETPOS.
pub(crate) fn create_trackbar_hwnd(
    parent: HWND,
    min: i32,
    max: i32,
    position: i32,
    page_size: Option<i32>,
    selection: Option<(i32, i32)>,
    trackbar_style: u32,
) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            TRACKBAR_CLASSW,
            w!("Trackbar Control"),
            WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | trackbar_style),
            0,
            0,
            200,
            30,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )?
    };

    unsafe {
        SendMessageW(
            hwnd,
            TBM_SETRANGE,
            Some(WPARAM(1)), // redraw flag
            Some(LPARAM(makelong(min, max))),
        );

        if let Some(page) = page_size {
            SendMessageW(
                hwnd,
                TBM_SETPAGESIZE,
                Some(WPARAM(0)),
                Some(LPARAM(page as isize)),
            );
        }

        if let Some((sel_min, sel_max)) = selection {
            SendMessageW(
                hwnd,
                TBM_SETSEL,
                Some(WPARAM(0)), // redraw flag
                Some(LPARAM(makelong(sel_min, sel_max))),
            );
        }

        SendMessageW(
            hwnd,
            TBM_SETPOS,
            Some(WPARAM(1)), // redraw flag
            Some(LPARAM(position as isize)),
        );
    }

    Ok(hwnd)
}

pub(crate) fn get_position(hwnd: HWND) -> i32 {
    let ret = unsafe { SendMessageW(hwnd, TBM_GETPOS, Some(WPARAM(0)), Some(LPARAM(0))) };
    ret.0 as i32
}

pub(crate) fn update_trackbar_hwnd<Msg>(hwnd: HWND, old: &Widget<Msg>, new: &Widget<Msg>) {
    let (old_tb, new_tb) = match (old, new) {
        (Widget::Trackbar { .. }, Widget::Trackbar { .. }) => (old, new),
        _ => return,
    };
    let (
        Widget::Trackbar {
            min: old_min,
            max: old_max,
            position: old_pos,
            page_size: old_page,
            selection: old_sel,
            ..
        },
        Widget::Trackbar {
            min: new_min,
            max: new_max,
            position: new_pos,
            page_size: new_page,
            selection: new_sel,
            ..
        },
    ) = (old_tb, new_tb)
    else {
        return;
    };

    unsafe {
        if (old_min, old_max) != (new_min, new_max) {
            SendMessageW(
                hwnd,
                TBM_SETRANGE,
                Some(WPARAM(1)),
                Some(LPARAM(makelong(*new_min, *new_max))),
            );
        }
        if old_page != new_page
            && let Some(page) = new_page
        {
            SendMessageW(
                hwnd,
                TBM_SETPAGESIZE,
                Some(WPARAM(0)),
                Some(LPARAM(*page as isize)),
            );
        }
        if old_sel != new_sel {
            match new_sel {
                Some((sel_min, sel_max)) => {
                    SendMessageW(
                        hwnd,
                        TBM_SETSEL,
                        Some(WPARAM(1)),
                        Some(LPARAM(makelong(*sel_min, *sel_max))),
                    );
                }
                None => {
                    SendMessageW(hwnd, TBM_CLEARSEL, Some(WPARAM(1)), Some(LPARAM(0)));
                }
            }
        }
        if old_pos != new_pos {
            SendMessageW(
                hwnd,
                TBM_SETPOS,
                Some(WPARAM(1)),
                Some(LPARAM(*new_pos as isize)),
            );
        }
    }
}
