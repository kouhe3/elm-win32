use crate::runtime::{Runtime, RuntimeHandle};
use crate::widget::Widget;
use crate::wndproc::wndproc;
use std::mem;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::{CreateFontIndirectW, DeleteObject, LOGFONTW, CLEARTYPE_QUALITY, HGDIOBJ};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::HiDpi::{
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext,
};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::*;

pub struct Cmd<Msg>(pub(crate) Vec<Msg>);

impl<Msg> Cmd<Msg> {
    pub fn none() -> Self {
        Cmd(Vec::new())
    }

    pub fn batch(msgs: Vec<Msg>) -> Self {
        Cmd(msgs)
    }

    pub fn into_vec(self) -> Vec<Msg> {
        self.0
    }
}

pub struct WindowConfig {
    pub title: String,
    pub width: f32,
    pub height: f32,
}

impl WindowConfig {
    pub fn new(title: &str, width: f32, height: f32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
        }
    }
}

pub trait Program: Sized {
    type Model;
    type Msg: Clone + 'static;

    fn init(&self) -> (Self::Model, Cmd<Self::Msg>);
    fn update(&self, msg: Self::Msg, model: &mut Self::Model) -> Cmd<Self::Msg>;
    fn view(&self, model: &Self::Model) -> Widget<Self::Msg>;

    fn run(self, config: WindowConfig) {
        launch(self, config);
    }
}

fn launch<P: Program>(program: P, config: WindowConfig) {
    unsafe {
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }

    let hinstance =
        unsafe { HINSTANCE(GetModuleHandleW(None).expect("GetModuleHandleW failed").0) };

    let class_name = w!("ELM_WIN32_WINDOW");

    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        hInstance: hinstance,
        lpszClassName: PCWSTR::from_raw(class_name.as_ptr()),
        ..Default::default()
    };

    if unsafe { RegisterClassW(&wc) } == 0 {
        panic!("RegisterClassW failed");
    }

    let title_h = HSTRING::from(config.title.as_str());
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            &title_h,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            config.width as i32,
            config.height as i32,
            None,
            None,
            Some(hinstance),
            None,
        )
    }
    .expect("CreateWindowExW failed");

    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOW);
    };

    // Create default font
    let font = unsafe {
        let face = w!("Segoe UI");
        let face_slice = face.as_wide();
        let len = face_slice.len().min(31);
        let mut lf = LOGFONTW {
            lfHeight: -14,
            lfWeight: 400,
            lfQuality: CLEARTYPE_QUALITY,
            ..Default::default()
        };
        lf.lfFaceName[..len].copy_from_slice(&face_slice[..len]);
        Some(CreateFontIndirectW(&lf))
    };

    // Create Runtime with default DPI factor (WM_DPICHANGED will update it)
    let mut runtime = Box::new(Runtime::<P::Msg>::new(
        hwnd,
        (config.width, config.height),
        1.0,
        font,
    ));
    let runtime_ptr: *mut Runtime<P::Msg> = &mut *runtime;
    let handle = RuntimeHandle::new::<P::Msg>(runtime_ptr);
    let handle_ptr = Box::into_raw(handle);

    unsafe {
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, handle_ptr as isize);
    }

    // Init model
    let (mut model, _cmd) = program.init();

    // Initial render
    let tree = program.view(&model);
    runtime.render(&tree);

    // Message loop
    let mut msg = MSG::default();
    loop {
        let ret = unsafe { GetMessageW(&mut msg, None, 0, 0) };
        if ret.0 == 0 {
            break;
        }

        unsafe {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Process pending messages
        let pending: Vec<P::Msg> = mem::take(&mut runtime.pending_msgs);
        let had_pending = !pending.is_empty();
        for m in pending {
            let cmd = program.update(m, &mut model);
            for batch_msg in cmd.into_vec() {
                runtime.pending_msgs.push(batch_msg);
            }
        }

        // Only re-render if model may have changed or system event (resize/dpi),
        // avoiding unnecessary SetWindowPos calls that can close ComboBox
        // dropdowns mid-interaction.
        if had_pending || runtime.needs_render {
            let new_tree = program.view(&model);
            runtime.render(&new_tree);
        }
    }

    // Cleanup
    unsafe {
        if let Some(f) = runtime.font {
            let _ = DeleteObject(HGDIOBJ(f.0));
        }
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        let _ = Box::from_raw(handle_ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    enum Msg {
        Click,
    }

    struct TestModel {
        count: i32,
    }

    struct TestApp;

    impl Program for TestApp {
        type Model = TestModel;
        type Msg = Msg;

        fn init(&self) -> (Self::Model, Cmd<Self::Msg>) {
            (TestModel { count: 0 }, Cmd::none())
        }

        fn update(&self, msg: Self::Msg, model: &mut Self::Model) -> Cmd<Self::Msg> {
            match msg {
                Msg::Click => model.count += 1,
            }
            Cmd::none()
        }

        fn view(&self, _model: &Self::Model) -> Widget<Self::Msg> {
            Widget::None
        }
    }

    #[test]
    fn test_cmd_none() {
        let cmd: Cmd<Msg> = Cmd::none();
        assert!(cmd.into_vec().is_empty());
    }

    #[test]
    fn test_program_init() {
        let app = TestApp;
        let (model, cmd) = app.init();
        assert_eq!(model.count, 0);
        assert!(cmd.into_vec().is_empty());
    }

    #[test]
    fn test_program_update() {
        let app = TestApp;
        let mut model = TestModel { count: 0 };
        let cmd = app.update(Msg::Click, &mut model);
        assert_eq!(model.count, 1);
        assert!(cmd.into_vec().is_empty());
    }
}
