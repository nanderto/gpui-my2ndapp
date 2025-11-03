use crate::app::IDEApplication;
use gpui::{div, prelude::*, px, rgb, Window, WindowControlArea};

pub struct MenuBar<'a> {
    app: &'a IDEApplication,
}

impl<'a> MenuBar<'a> {
    pub fn new(app: &'a IDEApplication) -> Self {
        Self { app }
    }

    pub fn render(&self, window: &mut Window) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .w_full()
            .h(px(IDEApplication::MENU_BAR_HEIGHT))
            .bg(rgb(0x2d2d30))
            .border_b_1()
            .border_color(rgb(0x3e3e42))
            .px_2()
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(self.render_menu_item("File"))
                    .child(self.render_menu_item("Edit"))
                    .child(self.render_menu_item("View"))
                    .child(self.render_menu_item("Go"))
                    .child(self.render_menu_item("Run"))
                    .child(self.render_menu_item("Terminal"))
                    .child(self.render_menu_item("Help"))
            )
            .child(
                // Draggable area in the middle (using window_control_area)
                div()
                    .flex_1()
                    .h_full()
                    .items_center()
                    .justify_center()
                    .bg(rgb(0xffffff)) // White background to make it obvious
                    .window_control_area(WindowControlArea::Drag) // Register as drag area
                    .hover(|style| {
                        style.bg(rgb(0xf0f0f0)) // Light gray hover effect
                    })
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x000000)) // Black text on white background
                            .child("GPUI IDE - Drag to Move (Window Control Area)")
                    )
            )
            .child(
                // Window controls on the right
                div()
                    .flex()
                    .gap_1()
                    .child(self.render_minimize_button())
                    .child(self.render_maximize_button(window))
                    .child(self.render_close_button())
            )
    }

    fn render_menu_item(&self, label: &str) -> impl IntoElement {
        let label = label.to_string();
        div()
            .px_3()
            .py_1()
            .hover(|style| style.bg(rgb(0x3e3e42)))
            .text_sm()
            .text_color(rgb(0xcccccc))
            .child(label)
    }

    fn render_minimize_button(&self) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .w(px(30.0))
            .h(px(30.0))
            .window_control_area(WindowControlArea::Min) // Min control area - Windows handles this
            .hover(|style| style.bg(rgb(0x3e3e42)))
            // Windows handles minimize automatically with WindowControlArea::Min
            .text_sm()
            .text_color(rgb(0xcccccc))
            .child("−")
    }

    fn render_maximize_button(&self, window: &mut Window) -> impl IntoElement {
        let is_maximized = window.is_maximized();
        let icon = if is_maximized { "⊡" } else { "□" };
        
        div()
            .flex()
            .justify_center()
            .items_center()
            .w(px(30.0))
            .h(px(30.0))
            .window_control_area(WindowControlArea::Max) // Max control area like Zed - Windows handles this automatically!
            .hover(|style| style.bg(rgb(0x3e3e42)))
            // No manual click handling needed - WindowControlArea::Max handles maximize/restore automatically!
            .text_sm()
            .text_color(rgb(0xcccccc))
            .child(icon)
    }

    fn render_close_button(&self) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .w(px(30.0))
            .h(px(30.0))
            .window_control_area(WindowControlArea::Close) // Close control area - Windows handles this
            .hover(|style| style.bg(rgb(0xe74c3c)))
            // Windows handles close automatically with WindowControlArea::Close
            .text_sm()
            .text_color(rgb(0xcccccc))
            .child("×")
    }
}