use crate::ui::{MenuBar, StatusBar, Sidebar, MainContent, BottomPanel};
use gpui::{Context, Window, div, prelude::*, rgb, px, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent};

pub struct IDEApplication {
    pub selected_left_tab: usize,
    pub selected_main_tab: usize,
    pub selected_bottom_tab: usize,
    pub left_panel_width: f32,
    pub bottom_panel_height: f32,
    pub is_dragging_left_divider: bool,
    pub is_dragging_bottom_divider: bool,
    pub drag_start_x: Option<f32>,
    pub drag_start_y: Option<f32>,
}

impl IDEApplication {
    pub const MENU_BAR_HEIGHT: f32 = 30.0;
    pub const STATUS_BAR_HEIGHT: f32 = 22.0;

    pub fn new() -> Self {
        Self {
            selected_left_tab: 0,
            selected_main_tab: 0,
            selected_bottom_tab: 0,
            left_panel_width: 300.0,
            bottom_panel_height: 200.0,
            is_dragging_left_divider: false,
            is_dragging_bottom_divider: false,
            drag_start_x: None,
            drag_start_y: None,
        }
    }

    pub fn start_left_divider_drag(&mut self, x: f32) {
        self.is_dragging_left_divider = true;
        self.drag_start_x = Some(x);
    }

    pub fn start_bottom_divider_drag(&mut self, y: f32) {
        self.is_dragging_bottom_divider = true;
        self.drag_start_y = Some(y);
    }

    pub fn update_left_panel_width(&mut self, x: f32) {
        if self.is_dragging_left_divider {
            let delta = x - self.drag_start_x.unwrap_or(x);
            let new_width = (self.left_panel_width + delta)
                .max(150.0)   // Minimum width - enough to show icons
                .min(800.0);  // Maximum width - don't take over the whole screen
            self.left_panel_width = new_width;
            self.drag_start_x = Some(x);
        }
    }

    pub fn update_bottom_panel_height(&mut self, y: f32) {
        if self.is_dragging_bottom_divider {
            let delta = self.drag_start_y.unwrap_or(y) - y; // Inverted because dragging up decreases y
            let new_height = (self.bottom_panel_height + delta)
                .max(80.0)    // Minimum height - enough to show tabs
                .min(500.0);  // Maximum height - don't take over the whole screen
            self.bottom_panel_height = new_height;
            self.drag_start_y = Some(y);
        }
    }

    pub fn stop_dragging(&mut self) {
        self.is_dragging_left_divider = false;
        self.is_dragging_bottom_divider = false;
        self.drag_start_x = None;
        self.drag_start_y = None;
    }

    fn render_vertical_divider(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_dragging = self.is_dragging_left_divider;
        div()
            .w(px(4.0))
            .h_full()
            .bg(if is_dragging { rgb(0x007acc) } else { rgb(0x3e3e42) })
            .hover(|style| style.bg(rgb(0x007acc)))
            .cursor_col_resize()
            .flex()
            .justify_center()
            .items_center()
            .on_mouse_down(MouseButton::Left, cx.listener(|app, event: &MouseDownEvent, _window, _cx| {
                app.start_left_divider_drag(event.position.x.into());
            }))
            .child(
                // Add a visual grip indicator
                div()
                    .w(px(1.0))
                    .h(px(20.0))
                    .bg(if is_dragging { rgb(0xffffff) } else { rgb(0x666666) })
                    .rounded_sm()
            )
    }

    fn render_horizontal_divider(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_dragging = self.is_dragging_bottom_divider;
        div()
            .w_full()
            .h(px(4.0))
            .bg(if is_dragging { rgb(0x007acc) } else { rgb(0x3e3e42) })
            .hover(|style| style.bg(rgb(0x007acc)))
            .cursor_row_resize()
            .flex()
            .justify_center()
            .items_center()
            .on_mouse_down(MouseButton::Left, cx.listener(|app, event: &MouseDownEvent, _window, _cx| {
                app.start_bottom_divider_drag(event.position.y.into());
            }))
            .child(
                // Add a visual grip indicator
                div()
                    .w(px(20.0))
                    .h(px(1.0))
                    .bg(if is_dragging { rgb(0xffffff) } else { rgb(0x666666) })
                    .rounded_sm()
            )
    }

}

impl Render for IDEApplication {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .on_mouse_move(cx.listener(|app, event: &MouseMoveEvent, _window, _cx| {
                if app.is_dragging_left_divider {
                    app.update_left_panel_width(event.position.x.into());
                }
                if app.is_dragging_bottom_divider {
                    app.update_bottom_panel_height(event.position.y.into());
                }
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|app, _event: &MouseUpEvent, _window, _cx| {
                app.stop_dragging();
            }))
            .child(MenuBar::new(self).render(window))
            .child(
                div()
                    .flex()
                    .flex_1()
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .child(Sidebar::new(self).render())
                            .child(self.render_vertical_divider(cx))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .child(MainContent::new(self).render())
                                    .child(self.render_horizontal_divider(cx))
                                    .child(BottomPanel::new(self).render())
                            )
                    )
            )
            .child(StatusBar::new(self).render())
    }
}