use crate::app::IDEApplication;
use crate::panels::{FileTree, DataPanels, InputPanel};
use gpui::{div, prelude::*, px, rgb};

pub struct Sidebar<'a> {
    app: &'a IDEApplication,
}

impl<'a> Sidebar<'a> {
    pub fn new(app: &'a IDEApplication) -> Self {
        Self { app }
    }

    pub fn render(&self) -> impl IntoElement {
        div()
            .flex()
            .w(px(self.app.left_panel_width))
            .h_full()
            .bg(rgb(0x2d2d30))
            .border_r_1()
            .border_color(rgb(0x3e3e42))
            .child(
                // Vertical tab bar (fixed width)
                div()
                    .flex()
                    .flex_col()
                    .w(px(72.0))
                    .h_full()
                    .bg(rgb(0x252526))
                    .child(self.render_vertical_tab("📁", 0))
                    .child(self.render_vertical_tab("📊", 1))
                    .child(self.render_vertical_tab("⌨️", 2))
            )
            .child(
                // Expandable content panel
                div()
                    .flex_1()
                    .h_full()
                    .child(match self.app.selected_left_tab {
                        0 => div().child(FileTree::new(self.app).render()),
                        1 => div().child(DataPanels::new(self.app).render()),
                        2 => div().child(InputPanel::new(self.app).render()),
                        _ => div().child("Unknown tab"),
                    })
            )
    }

    fn render_vertical_tab(&self, icon: &str, index: usize) -> impl IntoElement {
        let is_active = self.app.selected_left_tab == index;
        let icon = icon.to_string();
        div()
            .flex()
            .justify_center()
            .items_center()
            .w_full()
            .h(px(48.0))
            .bg(if is_active { rgb(0x1e1e1e) } else { rgb(0x252526) })
            .border_r_2()
            .border_color(if is_active { rgb(0x007acc) } else { rgb(0x252526) })
            .hover(|style| {
                if !is_active {
                    style.bg(rgb(0x2a2d2e))
                } else {
                    style
                }
            })
            .text_lg()
            .text_color(if is_active { rgb(0xffffff) } else { rgb(0x999999) })
            .child(icon)
    }
}