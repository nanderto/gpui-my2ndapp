use crate::app::IDEApplication;
use gpui::{div, prelude::*, rgb};

pub struct InputPanel<'a> {
    #[allow(dead_code)] // Will be used for settings and input forms
    app: &'a IDEApplication,
}

impl<'a> InputPanel<'a> {
    pub fn new(app: &'a IDEApplication) -> Self {
        Self { app }
    }

    pub fn render(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .p_4()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .child("Project Settings")
                    )
                    .child(self.render_input_field("Project Name", "gpui-my2ndapp"))
                    .child(self.render_input_field("Version", "0.1.0"))
            )
    }

    fn render_input_field(&self, label: &str, value: &str) -> impl IntoElement {
        let label = label.to_string();
        let value = value.to_string();
        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xcccccc))
                    .child(label)
            )
            .child(
                div()
                    .px_2()
                    .py_1()
                    .bg(rgb(0x3c3c3c))
                    .border_1()
                    .border_color(rgb(0x5a5a5a))
                    .text_sm()
                    .text_color(rgb(0xffffff))
                    .child(value)
            )
    }
}