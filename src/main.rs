use gpui_my2ndapp::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: None,
                window_decorations: Some(gpui::WindowDecorations::Client),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| IDEApplication::new())
            },
        )
        .unwrap();
        cx.activate(true);
    });
}