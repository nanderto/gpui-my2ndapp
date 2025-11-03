use crate::app::IDEApplication;
use gpui::{div, prelude::*, px, rgb};

pub struct BottomPanel<'a> {
    app: &'a IDEApplication,
}

impl<'a> BottomPanel<'a> {
    pub fn new(app: &'a IDEApplication) -> Self {
        Self { app }
    }

    pub fn render(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .h(px(self.app.bottom_panel_height))
            .bg(rgb(0x252526))
            .border_t_1()
            .border_color(rgb(0x3e3e42))
            .child(
                // Bottom tab bar
                div()
                    .flex()
                    .h(px(35.0))
                    .bg(rgb(0x2d2d30))
                    .child(self.render_bottom_tab("Terminal", 0))
                    .child(self.render_bottom_tab("Problems", 1))
                    .child(self.render_bottom_tab("Output", 2))
            )
            .child(
                // Bottom content
                div()
                    .flex_1()
                    .child(match self.app.selected_bottom_tab {
                        0 => div().child(self.render_terminal()),
                        1 => div().child(self.render_problems_panel()),
                        2 => div().child(self.render_output_panel()),
                        _ => div(),
                    })
            )
    }

    fn render_bottom_tab(&self, title: &str, index: usize) -> impl IntoElement {
        let is_active = self.app.selected_bottom_tab == index;
        let title = title.to_string();
        div()
            .flex()
            .items_center()
            .px_3()
            .h_full()
            .bg(if is_active { rgb(0x1e1e1e) } else { rgb(0x2d2d30) })
            .border_r_1()
            .border_color(rgb(0x3e3e42))
            .hover(|style| {
                if !is_active {
                    style.bg(rgb(0x323233))
                } else {
                    style
                }
            })
            .text_sm()
            .text_color(if is_active { rgb(0xffffff) } else { rgb(0xcccccc) })
            .child(title)
    }

    fn render_terminal(&self) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .bg(rgb(0x0c0c0c))
            .p_2()
            .text_sm()
            .text_color(rgb(0x00ff00))
            .child("PS C:\\dev\\tmp\\gpui-my2ndapp> cargo run\n   Compiling gpui-my2ndapp v0.1.0\n    Finished dev [unoptimized + debuginfo] target(s)")
    }

    fn render_problems_panel(&self) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .p_2()
            .text_sm()
            .text_color(rgb(0xcccccc))
            .child("No problems detected")
    }

    fn render_output_panel(&self) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .p_2()
            .text_sm()
            .text_color(rgb(0xcccccc))
            .child("Build output will appear here")
    }
}