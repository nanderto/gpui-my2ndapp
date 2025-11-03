use crate::app::IDEApplication;
use gpui::{div, prelude::*, px, rgb};

pub struct FileTree<'a> {
    app: &'a IDEApplication,
}

impl<'a> FileTree<'a> {
    pub fn new(app: &'a IDEApplication) -> Self {
        Self { app }
    }

    pub fn render(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x1e1e1e))
            .child(
                // Horizontal tab bar for Explorer/Connections
                div()
                    .flex()
                    .h(px(35.0))
                    .bg(rgb(0x2d2d30))
                    .border_b_1()
                    .border_color(rgb(0x3e3e42))
                    .child(self.render_explorer_tab("Explorer", 0, true))
                    .child(self.render_explorer_tab("Connections", 1, false))
            )
            .child(
                // Tab content
                div()
                    .flex_1()
                    .p_2()
                    .child(match 0 { // For now, always show Explorer (index 0)
                        0 => div().child(self.render_explorer_content()),
                        1 => div().child(self.render_connections_content()),
                        _ => div().child("Unknown tab"),
                    })
            )
    }

    fn render_explorer_tab(&self, title: &str, _index: usize, is_active: bool) -> impl IntoElement {
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

    fn render_explorer_content(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xcccccc))
                    .mb_2()
                    .child("FILES")
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x999999))
                    .child("📁 src/\n  📄 main.rs\n  📄 lib.rs\n  📄 app.rs\n  📁 ui/\n  📁 panels/\n📄 Cargo.toml\n📄 .gitignore")
            )
    }

    fn render_connections_content(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xcccccc))
                    .mb_2()
                    .child("CONNECTIONS")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .p_2()
                            .bg(rgb(0x2d2d30))
                            .rounded_md()
                            .hover(|style| style.bg(rgb(0x3e3e42)))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xcccccc))
                                    .child("🔗 Database Connection")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .p_2()
                            .bg(rgb(0x2d2d30))
                            .rounded_md()
                            .hover(|style| style.bg(rgb(0x3e3e42)))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xcccccc))
                                    .child("🌐 API Endpoint")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .p_2()
                            .bg(rgb(0x2d2d30))
                            .rounded_md()
                            .hover(|style| style.bg(rgb(0x3e3e42)))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xcccccc))
                                    .child("☁️ Cloud Service")
                            )
                    )
            )
    }
}