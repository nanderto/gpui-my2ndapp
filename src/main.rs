use gpui_my2ndapp::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        // Create app instance to get saved window bounds
        let app = IDEApplication::new();
        let (x, y, width, height, is_maximized) = app.get_window_bounds();
        
        // Create bounds from saved state
        let bounds = Bounds::new(
            point(px(x), px(y)),
            size(px(width), px(height))
        );

        cx.open_window(
            WindowOptions {
                window_bounds: Some(if is_maximized {
                    WindowBounds::Maximized(bounds)
                } else {
                    WindowBounds::Windowed(bounds)
                }),
                titlebar: None,
                window_decorations: Some(gpui::WindowDecorations::Client),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| app)
            },
        )
        .unwrap();
        
        cx.activate(true);
    });
}