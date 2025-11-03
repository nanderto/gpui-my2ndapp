use crate::app::IDEApplication;
use gpui::{div, prelude::*, px, rgb};

pub struct MainContent<'a> {
    app: &'a IDEApplication,
}

impl<'a> MainContent<'a> {
    pub fn new(app: &'a IDEApplication) -> Self {
        Self { app }
    }

    pub fn render(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .child(
                // Horizontal tab bar
                div()
                    .flex()
                    .h(px(35.0))
                    .bg(rgb(0x2d2d30))
                    .border_b_1()
                    .border_color(rgb(0x3e3e42))
                    .children(
                        self.app.open_tabs.iter().enumerate().map(|(index, tab)| {
                            self.render_horizontal_tab(
                                &tab.name,
                                index,
                                index == self.app.selected_main_tab
                            )
                        })
                    )
            )
            .child(
                // Content area
                div()
                    .flex_1()
                    .bg(rgb(0x1e1e1e))
                    .child(self.render_text_editor())
            )
    }

    fn render_horizontal_tab(&self, title: &str, _index: usize, is_active: bool) -> impl IntoElement {
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
            .child(
                div()
                    .text_sm()
                    .text_color(if is_active { rgb(0xffffff) } else { rgb(0xcccccc) })
                    .child(title)
            )
            .child(
                div()
                    .ml_2()
                    .w(px(16.0))
                    .h(px(16.0))
                    .hover(|style| style.bg(rgb(0x424242)))
                    .text_xs()
                    .text_color(rgb(0x999999))
                    .child("×")
            )
    }

    fn render_text_editor(&self) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .p_4()
            .text_sm()
            .text_color(rgb(0xcccccc))
            .child("// Welcome to the IDE!\n// This is where code editing will happen\n\nfn main() {\n    println!(\"Hello, IDE!\");\n}")
    }
}