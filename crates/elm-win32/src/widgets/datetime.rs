use crate::widget::Widget;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

pub(crate) fn create_datetime_hwnd(parent: HWND, format: u32) -> Result<HWND> {
    let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None)?.0) };
    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            DATETIMEPICK_CLASSW,
            w!(""),
            WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | format),
            0,
            0,
            200,
            24,
            Some(parent),
            None,
            Some(hinstance),
            None,
        )
    }
}

pub(crate) fn get_systemtime(hwnd: HWND) -> (u16, u16, u16, u16, u16, u16) {
    let mut st = windows::Win32::Foundation::SYSTEMTIME::default();
    unsafe {
        SendMessageW(
            hwnd,
            DTM_GETSYSTEMTIME,
            Some(WPARAM(0)),
            Some(LPARAM(std::ptr::from_mut(&mut st) as isize)),
        );
    }
    (
        st.wYear,
        st.wMonth,
        st.wDay,
        st.wHour,
        st.wMinute,
        st.wSecond,
    )
}

pub(crate) fn update_datetime_hwnd<Msg>(_hwnd: HWND, _old: &Widget<Msg>, _new: &Widget<Msg>) {}
