use crate::app::IDEApplication;
use gpui::{div, prelude::*, rgb};

pub struct DataPanels<'a> {
    app: &'a IDEApplication,
}

impl<'a> DataPanels<'a> {
    pub fn new(app: &'a IDEApplication) -> Self {
        Self { app }
    }

    pub fn render(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .gap_2()
            .p_2()
            .child(self.render_data_panel("Database", vec!["Users", "Products", "Orders"]))
            .child(self.render_data_panel("API Endpoints", vec!["/api/users", "/api/products"]))
            .child(self.render_data_panel("Environment", vec!["Development", "Staging", "Production"]))
    }

    fn render_data_panel(&self, title: &str, items: Vec<&str>) -> impl IntoElement {
        let title = title.to_string();
        let items: Vec<String> = items.into_iter().map(|s| s.to_string()).collect();
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x1e1e1e))
            .border_1()
            .border_color(rgb(0x3e3e42))
            .rounded_md()
            .child(
                div()
                    .p_2()
                    .bg(rgb(0x2d2d30))
                    .text_sm()
                    .text_color(rgb(0xffffff))
                    .child(title)
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_2()
                    .children(items.into_iter().map(|item| {
                        div()
                            .py_1()
                            .px_2()
                            .hover(|style| style.bg(rgb(0x2a2d2e)))
                            .text_sm()
                            .text_color(rgb(0xcccccc))
                            .child(item)
                    }))
            )
    }
}