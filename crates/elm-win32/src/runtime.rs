use crate::layout::LayoutEngine;
use crate::reconciler::NodeTree;
use crate::style::{CheckBoxStyle, Rect};
use crate::widget::Widget;
use crate::widgets;
use std::collections::HashMap;
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{CreateSolidBrush, SetBkColor, SetTextColor, HFONT};
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
    EditComboBox(fn(String) -> Msg),
    SelectComboBoxEx(fn(usize) -> Msg),
    DateTimeChange(fn(u16, u16, u16, u16, u16, u16) -> Msg),
    HeaderColumnClick(fn(usize) -> Msg),
    TabChange(fn(usize) -> Msg),
    ToolbarButtonClick(fn(usize) -> Msg),
    TrackbarMove(fn(i32) -> Msg),
    Toggle {
        f: fn(i32) -> Msg,
        auto: bool,
        three_state: bool,
    },
}

#[derive(Clone)]
struct EditColors {
    text_color: COLORREF,
    bg_color: COLORREF,
    bg_brush: Option<isize>,
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
    hwnd_to_colors: HashMap<HwndKey, EditColors>,
    pub(crate) font: Option<HFONT>,
    layout_engine: LayoutEngine,
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
            hwnd_to_colors: HashMap::new(),
            font,
            layout_engine: LayoutEngine::new(),
        }
    }

    pub fn render(&mut self, new_tree: &Widget<Msg>) {
        self.node_tree
            .reconcile(self.root_hwnd, Some(&self.prev_tree), new_tree, self.font);
        self.apply_layout(new_tree);
        self.build_action_map(new_tree);
        self.prev_tree = new_tree.clone();
        self.needs_render = false;
    }

    fn apply_layout(&mut self, tree: &Widget<Msg>) {
        Self::layout_and_position(
            &mut self.layout_engine,
            &self.node_tree,
            self.available_size,
            self.dpi_factor,
            self.font,
            tree,
        );
    }

    /// Recompute and re-apply layout for the previous tree (resize, DPI change).
    fn reapply_prev_layout(&mut self) {
        Self::layout_and_position(
            &mut self.layout_engine,
            &self.node_tree,
            self.available_size,
            self.dpi_factor,
            self.font,
            &self.prev_tree,
        );
    }

    fn layout_and_position(
        layout_engine: &mut LayoutEngine,
        node_tree: &NodeTree,
        available_size: (f32, f32),
        dpi_factor: f32,
        font: Option<HFONT>,
        tree: &Widget<Msg>,
    ) {
        let rects = layout_engine.compute(
            tree,
            available_size.0 / dpi_factor,
            available_size.1 / dpi_factor,
            font,
        );
        let mut pos = 0usize;
        Self::apply_bounds_recursive(tree, node_tree, dpi_factor, &rects, &mut pos);
    }

    fn apply_bounds_recursive(
        widget: &Widget<Msg>,
        node_tree: &crate::reconciler::NodeTree,
        dpi_factor: f32,
        rects: &[Rect],
        pos: &mut usize,
    ) {
        let current_pos = *pos;
        *pos += 1;
        if !widget.is_container()
            && !matches!(widget, Widget::None)
            && let Some(node) = node_tree.get_by_position(current_pos)
        {
            let bounds = if current_pos < rects.len() {
                rects[current_pos]
            } else {
                widget.bounds()
            };
            let _ = unsafe {
                SetWindowPos(
                    node.hwnd,
                    None,
                    (bounds.x * dpi_factor) as i32,
                    (bounds.y * dpi_factor) as i32,
                    (bounds.w * dpi_factor) as i32,
                    (bounds.h * dpi_factor) as i32,
                    SWP_NOACTIVATE | SWP_NOZORDER,
                )
            };
        }

        if widget.is_container() {
            for child in widget.children() {
                Self::apply_bounds_recursive(child, node_tree, dpi_factor, rects, pos);
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

            if let Widget::TextEdit {
                text_color,
                bg_color,
                ..
            } = widget
            {
                if let (Some(tc), Some(bc)) = (text_color, bg_color) {
                    self.hwnd_to_colors.insert(
                        key,
                        EditColors {
                            text_color: COLORREF(*tc),
                            bg_color: COLORREF(*bc),
                            bg_brush: None,
                        },
                    );
                }
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

            if let Widget::ComboBox {
                on_edit_change: Some(f),
                ..
            } = widget
            {
                self.hwnd_to_action
                    .insert(key, WidgetAction::EditComboBox(*f));
            }

            if let Widget::ComboBoxEx {
                on_select: Some(f), ..
            } = widget
            {
                self.hwnd_to_action
                    .insert(key, WidgetAction::SelectComboBoxEx(*f));
            }

            if let Widget::DateTime {
                on_change: Some(f), ..
            } = widget
            {
                self.hwnd_to_action
                    .insert(key, WidgetAction::DateTimeChange(*f));
            }

            if let Widget::Header {
                on_column_click: Some(f),
                ..
            } = widget
            {
                self.hwnd_to_action
                    .insert(key, WidgetAction::HeaderColumnClick(*f));
            }

            if let Widget::TabControl {
                on_tab_change: Some(f),
                ..
            } = widget
            {
                self.hwnd_to_action
                    .insert(key, WidgetAction::TabChange(*f));
            }

            if let Widget::Toolbar {
                on_button_click: Some(f),
                ..
            } = widget
            {
                self.hwnd_to_action
                    .insert(key, WidgetAction::ToolbarButtonClick(*f));
            }
            if let Widget::Trackbar {
                on_change: Some(f), ..
            } = widget
            {
                self.hwnd_to_action
                    .insert(key, WidgetAction::TrackbarMove(*f));
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

    pub fn handle_ctlcolor_edit(&mut self, child_hwnd: HWND, hdc_wparam: WPARAM) -> LRESULT {
        if let Some(colors) = self.hwnd_to_colors.get_mut(&HwndKey::from(child_hwnd)) {
            unsafe {
                let hdc = windows::Win32::Graphics::Gdi::HDC(hdc_wparam.0 as *mut _);
                let _ = SetTextColor(hdc, colors.text_color);
                let _ = SetBkColor(hdc, colors.bg_color);
                if colors.bg_brush.is_none() {
                    let brush = CreateSolidBrush(colors.bg_color);
                    colors.bg_brush = Some(brush.0 as isize);
                }
                LRESULT(colors.bg_brush.unwrap_or(0))
            }
        } else {
            LRESULT(0)
        }
    }

    pub fn handle_command(&mut self, child_hwnd: HWND, code: u32, ctrl_id: u32) {
        if let Some(action) = self.hwnd_to_action.get(&HwndKey::from(child_hwnd)) {
            match action {
                // Trackbar moves arrive via WM_HSCROLL/WM_VSCROLL -> handle_scroll
                WidgetAction::TrackbarMove(_f) => {}
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
                WidgetAction::EditComboBox(f) => {
                    if code == 5 {
                        // CBN_EDITCHANGE
                        let text = widgets::combobox::get_edit_text(child_hwnd);
                        self.pending_msgs.push(f(text));
                    }
                }
                WidgetAction::SelectComboBoxEx(f) => {
                    if code == 1 {
                        // CBN_SELCHANGE
                        let idx = widgets::comboex::get_selected_index(child_hwnd);
                        self.pending_msgs.push(f(idx));
                    }
                }
                WidgetAction::DateTimeChange(_f) => {
                    // DateTime changes are handled via WM_NOTIFY -> handle_notify
                }
                WidgetAction::HeaderColumnClick(_f) => {
                    // Header column clicks are handled via WM_NOTIFY -> handle_notify
                }
                WidgetAction::TabChange(_f) => {
                    // Tab selection is handled via WM_NOTIFY -> handle_notify
                }
                WidgetAction::ToolbarButtonClick(f) => {
                    self.pending_msgs.push(f(ctrl_id as usize));
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

    /// Trackbar moves arrive as WM_HSCROLL / WM_VSCROLL with lParam = child HWND.
    pub fn handle_scroll(&mut self, child_hwnd: HWND) {
        if let Some(WidgetAction::TrackbarMove(f)) =
            self.hwnd_to_action.get(&HwndKey::from(child_hwnd))
        {
            let pos = widgets::trackbar::get_position(child_hwnd);
            self.pending_msgs.push(f(pos));
        }
    }

    pub fn handle_size(&mut self, width: i32, height: i32) {
        self.available_size = (width as f32, height as f32);
        self.reapply_prev_layout();
        self.needs_render = true;
    }

    pub fn handle_dpi_changed(&mut self, new_dpi: u32) {
        self.dpi_factor = new_dpi as f32 / 96.0;
        self.reapply_prev_layout();
        self.needs_render = true;
    }

    pub fn handle_notify(&mut self, child_hwnd: HWND, code: u32, lparam: LPARAM) {
        use windows::Win32::UI::Controls::DTN_DATETIMECHANGE;
        use windows::Win32::UI::Controls::HDN_ITEMCLICK;
        use windows::Win32::UI::Controls::TCN_SELCHANGE;
        if let Some(action) = self.hwnd_to_action.get(&HwndKey::from(child_hwnd)) {
            if code == DTN_DATETIMECHANGE
                && let WidgetAction::DateTimeChange(f) = action
            {
                let (y, mo, d, h, mi, s) = widgets::datetime::get_systemtime(child_hwnd);
                self.pending_msgs.push(f(y, mo, d, h, mi, s));
            }
            if code == HDN_ITEMCLICK
                && let WidgetAction::HeaderColumnClick(f) = action
            {
                let nm_header =
                    unsafe { &*(lparam.0 as *const windows::Win32::UI::Controls::NMHEADERW) };
                self.pending_msgs.push(f(nm_header.iItem as usize));
            }
            if code == TCN_SELCHANGE
                && let WidgetAction::TabChange(f) = action
            {
                let idx = widgets::tabcontrol::get_selected_index(child_hwnd);
                self.pending_msgs.push(f(idx));
            }
        }
    }
}

// ---- FFI-safe handle for WndProc ----

#[repr(C)]
pub(crate) struct RuntimeHandle {
    pub on_command: unsafe fn(data: *mut std::ffi::c_void, child: HWND, code: u32, ctrl_id: u32),
    pub on_scroll: unsafe fn(data: *mut std::ffi::c_void, child: HWND),
    pub on_notify: unsafe fn(data: *mut std::ffi::c_void, child: HWND, code: u32, lparam: LPARAM),
    pub on_ctlcolor_edit:
        unsafe fn(data: *mut std::ffi::c_void, child: HWND, wparam: WPARAM) -> LRESULT,
    pub on_size: unsafe fn(data: *mut std::ffi::c_void, w: i32, h: i32),
    pub on_dpi_changed: unsafe fn(data: *mut std::ffi::c_void, dpi: u32),
    pub on_destroy: unsafe fn(data: *mut std::ffi::c_void),
    pub data: *mut std::ffi::c_void,
}

impl RuntimeHandle {
    pub fn new<Msg: Clone + 'static>(runtime_ptr: *mut Runtime<Msg>) -> Box<Self> {
        Box::new(RuntimeHandle {
            on_command: Self::on_command_thunk::<Msg>,
            on_scroll: Self::on_scroll_thunk::<Msg>,
            on_notify: Self::on_notify_thunk::<Msg>,
            on_ctlcolor_edit: Self::on_ctlcolor_edit_thunk::<Msg>,
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
        ctrl_id: u32,
    ) {
        unsafe {
            let rt = &mut *(data as *mut Runtime<Msg>);
            rt.handle_command(child, code, ctrl_id);
        }
    }

    unsafe fn on_notify_thunk<Msg: Clone + 'static>(
        data: *mut std::ffi::c_void,
        child: HWND,
        code: u32,
        lparam: LPARAM,
    ) {
        unsafe {
            let rt = &mut *(data as *mut Runtime<Msg>);
            rt.handle_notify(child, code, lparam);
        }
    }

    unsafe fn on_scroll_thunk<Msg: Clone + 'static>(data: *mut std::ffi::c_void, child: HWND) {
        unsafe {
            let rt = &mut *(data as *mut Runtime<Msg>);
            rt.handle_scroll(child);
        }
    }

    unsafe fn on_ctlcolor_edit_thunk<Msg: Clone + 'static>(
        data: *mut std::ffi::c_void,
        child: HWND,
        wparam: WPARAM,
    ) -> LRESULT {
        unsafe {
            let rt = &mut *(data as *mut Runtime<Msg>);
            rt.handle_ctlcolor_edit(child, wparam)
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
