use crate::ui::{MenuBar, StatusBar, Sidebar, MainContent, BottomPanel};
use gpui::{Context, Window, div, prelude::*, rgb};

pub struct IDEApplication {
    pub selected_left_tab: usize,
    pub selected_main_tab: usize,
    pub selected_bottom_tab: usize,
    pub left_panel_width: f32,
    pub bottom_panel_height: f32,
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
        }
    }


}

impl Render for IDEApplication {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(MenuBar::new(self).render(window))
            .child(
                div()
                    .flex()
                    .flex_1()
                    .child(Sidebar::new(self).render())
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .child(MainContent::new(self).render())
                            .child(BottomPanel::new(self).render())
                    )
            )
            .child(StatusBar::new(self).render())
    }
}