use crate::runtime::RuntimeHandle;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::{FillRect, GetStockObject, SetBkMode, TRANSPARENT, WHITE_BRUSH, HBRUSH, HDC};
use windows::Win32::UI::Controls::NMHDR;
use windows::Win32::UI::WindowsAndMessaging::*;

pub(crate) unsafe extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let handle_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *const RuntimeHandle;
    if handle_ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    }
    let handle = unsafe { &*handle_ptr };

    match msg {
        WM_COMMAND => {
            let child_hwnd = HWND(lparam.0 as *mut _);
            let code = (wparam.0 as u32 >> 16) & 0xFFFF;
            let ctrl_id = (wparam.0 as u32) & 0xFFFF;
            unsafe { (handle.on_command)(handle.data, child_hwnd, code, ctrl_id) };
            LRESULT(0)
        }
        WM_NOTIFY => {
            let nmhdr = unsafe { &*(lparam.0 as *const NMHDR) };
            unsafe { (handle.on_notify)(handle.data, nmhdr.hwndFrom, nmhdr.code, lparam) };
            LRESULT(0)
        }
        WM_SIZE => {
            let width = (lparam.0 as u32 & 0xFFFF) as i32;
            let height = ((lparam.0 as u32) >> 16) as i32;
            unsafe { (handle.on_size)(handle.data, width, height) };
            LRESULT(0)
        }
        WM_DPICHANGED => {
            let new_dpi = wparam.0 as u32 & 0xFFFF;
            unsafe { (handle.on_dpi_changed)(handle.data, new_dpi) };
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe {
                (handle.on_destroy)(handle.data);
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        WM_CLOSE => {
            unsafe {
                let _ = DestroyWindow(hwnd);
            };
            LRESULT(0)
        }
        WM_ERASEBKGND => {
            unsafe {
                let hdc = HDC(wparam.0 as *mut _);
                let mut rect = RECT::default();
                let _ = GetClientRect(hwnd, &mut rect);
                let brush = HBRUSH(GetStockObject(WHITE_BRUSH).0);
                let _ = FillRect(hdc, &rect, brush);
            }
            LRESULT(1)
        }
        WM_CTLCOLOREDIT => {
            unsafe { (handle.on_ctlcolor_edit)(handle.data, HWND(lparam.0 as *mut _), wparam) }
        }
        WM_CTLCOLORSTATIC => {
            unsafe {
                let hdc_static = HDC(wparam.0 as *mut _);
                let _ = SetBkMode(hdc_static, TRANSPARENT);
                LRESULT(GetStockObject(WHITE_BRUSH).0 as isize)
            }
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}
