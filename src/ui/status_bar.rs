use crate::app::IDEApplication;
use gpui::{div, prelude::*, px, rgb};

pub struct StatusBar<'a> {
    app: &'a IDEApplication,
}

impl<'a> StatusBar<'a> {
    pub fn new(app: &'a IDEApplication) -> Self {
        Self { app }
    }

    pub fn render(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(IDEApplication::STATUS_BAR_HEIGHT))
            .bg(rgb(0x007acc))
            .px_3()
            .child(
                // Left side status items
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child("main.rs")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child("Ln 1, Col 1")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child("UTF-8")
                    )
            )
            .child(
                // Right side status items
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child("Rust")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child("✓ Ready")
                    )
            )
    }
}