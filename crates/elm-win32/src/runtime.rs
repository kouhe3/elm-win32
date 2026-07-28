use crate::reconciler::NodeTree;
use crate::style::CheckBoxStyle;
use crate::widget::Widget;
use crate::widgets;
use std::collections::HashMap;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HFONT;
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct HwndKey(isize);

impl From<HWND> for HwndKey {
    fn from(h: HWND) -> Self {
        HwndKey(h.0 as isize)
    }
}

enum WidgetAction<Msg> {
    Click(Msg),
    Change(fn(String) -> Msg),
    SelectListBox(fn(usize) -> Msg),
    SelectComboBox(fn(usize) -> Msg),
    Toggle {
        f: fn(i32) -> Msg,
        auto: bool,
        three_state: bool,
    },
}

pub(crate) struct Runtime<Msg: Clone> {
    pub node_tree: NodeTree,
    pub root_hwnd: HWND,
    pub pending_msgs: Vec<Msg>,
    pub available_size: (f32, f32),
    pub dpi_factor: f32,
    pub needs_render: bool,
    prev_tree: Widget<Msg>,
    hwnd_to_action: HashMap<HwndKey, WidgetAction<Msg>>,
    pub(crate) font: Option<HFONT>,
}

impl<Msg: Clone + 'static> Runtime<Msg> {
    pub fn new(
        root_hwnd: HWND,
        available_size: (f32, f32),
        dpi_factor: f32,
        font: Option<HFONT>,
    ) -> Self {
        Self {
            node_tree: NodeTree::new(),
            root_hwnd,
            pending_msgs: Vec::new(),
            available_size,
            dpi_factor,
            needs_render: false,
            prev_tree: Widget::None,
            hwnd_to_action: HashMap::new(),
            font,
        }
    }

    pub fn render(&mut self, new_tree: &Widget<Msg>) {
        self.node_tree
            .reconcile(self.root_hwnd, Some(&self.prev_tree), new_tree, self.font);
        self.apply_bounds(new_tree);
        self.build_action_map(new_tree);
        self.prev_tree = new_tree.clone();
        self.needs_render = false;
    }

    fn apply_bounds(&mut self, widget: &Widget<Msg>) {
        let mut pos = 0usize;
        self.apply_bounds_recursive(widget, &mut pos);
    }

    fn apply_bounds_recursive(&mut self, widget: &Widget<Msg>, pos: &mut usize) {
        let current_pos = *pos;
        *pos += 1;

        if !widget.is_container()
            && !matches!(widget, Widget::None)
            && let Some(node) = self.node_tree.get_by_position(current_pos)
        {
            let bounds = widget.bounds();
            let _ = unsafe {
                SetWindowPos(
                    node.hwnd,
                    Some(HWND_BOTTOM),
                    (bounds.x * self.dpi_factor) as i32,
                    (bounds.y * self.dpi_factor) as i32,
                    (bounds.w * self.dpi_factor) as i32,
                    (bounds.h * self.dpi_factor) as i32,
                    SWP_NOACTIVATE,
                )
            };
        }

        if widget.is_container() {
            for child in widget.children() {
                self.apply_bounds_recursive(child, pos);
            }
        }
    }

    fn build_action_map(&mut self, widget: &Widget<Msg>) {
        self.hwnd_to_action.clear();
        let mut pos = 0usize;
        self.build_action_map_recursive(widget, &mut pos);
    }

    fn build_action_map_recursive(&mut self, widget: &Widget<Msg>, pos: &mut usize) {
        let current_pos = *pos;
        *pos += 1;

        if let Some(node) = self.node_tree.get_by_position(current_pos) {
            let key = HwndKey::from(node.hwnd);

            if let Widget::Button {
                on_click: Some(msg),
                ..
            } = widget
            {
                self.hwnd_to_action
                    .insert(key, WidgetAction::Click(msg.clone()));
            }

            if let Widget::TextEdit {
                on_change: Some(f), ..
            } = widget
            {
                self.hwnd_to_action.insert(key, WidgetAction::Change(*f));
            }

            if let Widget::ListBox {
                on_select: Some(f), ..
            } = widget
            {
                self.hwnd_to_action
                    .insert(key, WidgetAction::SelectListBox(*f));
            }

            if let Widget::ComboBox {
                on_select: Some(f), ..
            } = widget
            {
                self.hwnd_to_action
                    .insert(key, WidgetAction::SelectComboBox(*f));
            }

            if let Widget::CheckBox {
                on_toggle: Some(f),
                checkbox_style,
                ..
            } = widget
            {
                let auto = matches!(
                    *checkbox_style,
                    s if s == CheckBoxStyle::Auto.win32_style()
                        || s == CheckBoxStyle::Auto3State.win32_style()
                );
                let three_state = matches!(
                    *checkbox_style,
                    s if s == CheckBoxStyle::ThreeState.win32_style()
                        || s == CheckBoxStyle::Auto3State.win32_style()
                );
                self.hwnd_to_action.insert(
                    key,
                    WidgetAction::Toggle {
                        f: *f,
                        auto,
                        three_state,
                    },
                );
            }

            if let Widget::RadioButton {
                on_toggle: Some(f), ..
            } = widget
            {
                self.hwnd_to_action.insert(
                    key,
                    WidgetAction::Toggle {
                        f: *f,
                        auto: true,
                        three_state: false,
                    },
                );
            }
        }

        if widget.is_container() {
            for child in widget.children() {
                self.build_action_map_recursive(child, pos);
            }
        }
    }

    pub fn handle_command(&mut self, child_hwnd: HWND, code: u32) {
        if let Some(action) = self.hwnd_to_action.get(&HwndKey::from(child_hwnd)) {
            match action {
                WidgetAction::Click(msg) => self.pending_msgs.push(msg.clone()),
                WidgetAction::Change(f) => {
                    if code == 0x0300 {
                        // EN_CHANGE
                        let text = widgets::edit::get_edit_text(child_hwnd);
                        self.pending_msgs.push(f(text));
                    }
                }
                WidgetAction::SelectListBox(f) => {
                    if code == 1 {
                        // LBN_SELCHANGE
                        let idx = widgets::listbox::get_selected_index(child_hwnd);
                        self.pending_msgs.push(f(idx));
                    }
                }
                WidgetAction::SelectComboBox(f) => {
                    if code == 1 {
                        // CBN_SELCHANGE
                        let idx = widgets::combobox::get_selected_index(child_hwnd);
                        self.pending_msgs.push(f(idx));
                    }
                }
                WidgetAction::Toggle {
                    f,
                    auto,
                    three_state,
                } => {
                    if code != 0 {
                        return;
                    }
                    let state = widgets::checkbox::get_checked(child_hwnd);
                    let new_state = if *auto {
                        state
                    } else if *three_state {
                        (state + 1) % 3
                    } else {
                        1 - state
                    };
                    self.pending_msgs.push(f(new_state));
                }
            }
        }
    }

    pub fn handle_size(&mut self, width: i32, height: i32) {
        self.available_size = (width as f32, height as f32);
        self.needs_render = true;
    }

    pub fn handle_dpi_changed(&mut self, new_dpi: u32) {
        self.dpi_factor = new_dpi as f32 / 96.0;
        self.apply_bounds(&self.prev_tree.clone());
        self.needs_render = true;
    }
}

// ---- FFI-safe handle for WndProc ----

#[repr(C)]
pub(crate) struct RuntimeHandle {
    pub on_command: unsafe fn(data: *mut std::ffi::c_void, child: HWND, code: u32),
    pub on_size: unsafe fn(data: *mut std::ffi::c_void, w: i32, h: i32),
    pub on_dpi_changed: unsafe fn(data: *mut std::ffi::c_void, dpi: u32),
    pub on_destroy: unsafe fn(data: *mut std::ffi::c_void),
    pub data: *mut std::ffi::c_void,
}

impl RuntimeHandle {
    pub fn new<Msg: Clone + 'static>(runtime_ptr: *mut Runtime<Msg>) -> Box<Self> {
        Box::new(RuntimeHandle {
            on_command: Self::on_command_thunk::<Msg>,
            on_size: Self::on_size_thunk::<Msg>,
            on_dpi_changed: Self::on_dpi_changed_thunk::<Msg>,
            on_destroy: Self::on_destroy_thunk::<Msg>,
            data: runtime_ptr as *mut std::ffi::c_void,
        })
    }

    unsafe fn on_command_thunk<Msg: Clone + 'static>(
        data: *mut std::ffi::c_void,
        child: HWND,
        code: u32,
    ) {
        unsafe {
            let rt = &mut *(data as *mut Runtime<Msg>);
            rt.handle_command(child, code);
        }
    }

    unsafe fn on_size_thunk<Msg: Clone + 'static>(data: *mut std::ffi::c_void, w: i32, h: i32) {
        unsafe {
            let rt = &mut *(data as *mut Runtime<Msg>);
            rt.handle_size(w, h);
        }
    }

    unsafe fn on_dpi_changed_thunk<Msg: Clone + 'static>(data: *mut std::ffi::c_void, dpi: u32) {
        unsafe {
            let rt = &mut *(data as *mut Runtime<Msg>);
            rt.handle_dpi_changed(dpi);
        }
    }

    unsafe fn on_destroy_thunk<Msg: Clone + 'static>(_data: *mut std::ffi::c_void) {}
}
