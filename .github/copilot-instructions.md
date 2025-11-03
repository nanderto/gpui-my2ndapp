# GPUI Application Development Guide

## Project Overview
This is a GPUI (GPU-accelerated UI) desktop application built with Rust. GPUI is Zed's high-performance, reactive UI framework that renders using GPU acceleration and follows declarative UI patterns similar to React.

## Key Architecture Patterns

### Component Structure
- **State Components**: Implement `Render` trait for UI components that hold state
- **Render Method**: Always returns `impl IntoElement` - the core rendering contract
- **Context Parameters**: `_window: &mut Window, _cx: &mut Context<Self>` are standard

```rust
struct MyComponent {
    state: SharedString,
}

impl Render for MyComponent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("content")
    }
}
```

### UI Layout Patterns
- **Flexbox-first**: Use `flex()`, `flex_col()`, `gap_*()` for layouts
- **Method chaining**: All styling uses fluent builder pattern: `.bg().size().border_1()`
- **Pixel units**: Use `px()` for explicit sizing: `size(px(500.0))`
- **Color system**: Use `rgb()` for hex colors or `gpui::red()` for built-in colors
- **Responsive spacing**: `gap_2()`, `gap_3()` for consistent spacing

### Application Lifecycle
```rust
Application::new().run(|cx: &mut App| {
    let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
    cx.open_window(WindowOptions { /* ... */ }, |_, cx| {
        cx.new(|_| MyComponent::new())
    });
    cx.activate(true);
});
```

## Development Workflow

### Running the Application
```bash
cargo run  # Launches windowed desktop application
```

### Dependencies
- **Core**: `gpui = "0.2.2"` - Main UI framework
- **Edition**: Uses Rust 2024 edition
- **Build target**: Desktop application (Windows/macOS/Linux)

### Project Structure
```
src/
  main.rs           # Application entry point
  lib.rs            # Library exports and module declarations
  app.rs            # Core IDEApplication struct and main render logic
  ui/               # UI component modules
    mod.rs          # UI module exports
    menu_bar.rs     # Top menu bar with File, Edit, View menus
    status_bar.rs   # Bottom status bar with file info
    sidebar.rs      # Left sidebar with vertical tabs
    main_content.rs # Main content area with horizontal tabs
    bottom_panel.rs # Bottom panel with terminal/problems/output
  panels/           # Content panel modules
    mod.rs          # Panel module exports
    file_tree.rs    # File explorer tree view
    data_panels.rs  # Database/API/Environment data panels
    input_panel.rs  # User input forms and settings
Cargo.toml          # Standard Rust project configuration
target/             # Build artifacts (gitignored)
```

## GPUI-Specific Conventions

### Element Creation
- Use `div()` as the primary container element
- Chain styling methods directly: `div().bg(rgb(0x505050)).size(px(500.0))`
- Add children with `.child()` method - accepts strings, other elements, or format! macros

### Styling Patterns
- **Colors**: `rgb(0x505050)` for hex colors, `gpui::blue()` for named colors
- **Borders**: `border_1().border_color().border_dashed()` for styled borders
- **Shadows**: `shadow_lg()` for elevation effects
- **Rounded corners**: `rounded_md()` for corner radius
- **Text**: `text_xl().text_color()` for typography

### Common UI Patterns
- **Centered layouts**: `justify_center().items_center()`
- **Color palettes**: Create visual systems with consistent color squares/swatches
- **Responsive containers**: Use `size()` with `px()` for explicit dimensions

### Window Management Patterns
- **Window Controls**: Use `window.minimize_window()`, `window.zoom_window()`, `cx.quit()` for window operations
- **Window State Awareness**: Use `window.is_maximized()` to check current state and update UI accordingly
- **Context-Aware UI**: Change button icons/behavior based on window state (□ for maximize, ⧉ for restore)
- **Draggable Areas**: Use `on_mouse_down(MouseButton::Left, |_, window, _| window.start_window_move())` for title bar dragging
- **Frameless Windows**: Set `titlebar: None, window_decorations: Client` in WindowOptions for custom title bars
- **Event Handling**: Use `.on_mouse_down(MouseButton::Left, closure)` for interactive elements

## Complex UI Component Patterns

### VS Code-Style Application Layout
```rust
struct IDEApplication {
    selected_left_tab: usize,
    selected_main_tab: usize,
    selected_bottom_tab: usize,
    left_panel_width: f32,
    bottom_panel_height: f32,
}

impl Render for IDEApplication {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .child(self.render_left_sidebar())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .child(self.render_main_content())
                    .child(self.render_bottom_panel())
            )
    }
}
```

### Fixed-Width Vertical Sidebar with Tabs
```rust
fn render_left_sidebar(&self) -> impl IntoElement {
    div()
        .flex()
        .w(px(self.left_panel_width))
        .h_full()
        .bg(rgb(0x2d2d30))
        .border_r_1()
        .border_color(rgb(0x3e3e42))
        .child(
            // Vertical tab bar (wider for large icons)
            div()
                .flex()
                .flex_col()
                .w(px(72.0))  // 50% wider for large icons
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
                .child(match self.selected_left_tab {
                    0 => self.render_file_tree(),
                    1 => self.render_data_panels(),
                    2 => self.render_input_panel(),
                    _ => div().child("Unknown tab"),
                })
        )
}
```

### Tabbed File Explorer Component
```rust
fn render(&self) -> impl IntoElement {
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
            // Tab content with Explorer files and Connections
            div()
                .flex_1()
                .p_2()
                .child(match selected_tab {
                    0 => div().child(self.render_explorer_content()),
                    1 => div().child(self.render_connections_content()),
                    _ => div().child("Unknown tab"),
                })
        )
}

fn render_tree_node(&self, node: &FileTreeNode, depth: usize) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .child(
            div()
                .flex()
                .items_center()
                .h(px(22.0))
                .pl(px(depth as f32 * 16.0))
                .hover(|style| style.bg(rgb(0x2a2d2e)))
                .child(
                    if node.is_directory {
                        div()
                            .w(px(16.0))
                            .h(px(16.0))
                            .child(if node.is_expanded { "▼" } else { "▶" })
                    } else {
                        div().w(px(16.0))
                    }
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0xcccccc))
                        .child(node.name.clone())
                )
        )
        .children(
            if node.is_expanded {
                node.children.iter().map(|child| {
                    self.render_tree_node(child, depth + 1)
                }).collect()
            } else {
                vec![]
            }
        )
}
```

### Horizontal Tab System with Content Panels
```rust
fn render_main_content(&self) -> impl IntoElement {
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
                .child(self.render_horizontal_tab("main.rs", 0, true))
                .child(self.render_horizontal_tab("Cargo.toml", 1, false))
                .child(self.render_horizontal_tab("README.md", 2, false))
        )
        .child(
            // Content area
            div()
                .flex_1()
                .bg(rgb(0x1e1e1e))
                .child(self.render_text_editor())
        )
}

fn render_horizontal_tab(&self, title: &str, index: usize, is_active: bool) -> impl IntoElement {
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
                .child("×")
        )
}
```

### Split Panel Layout with Bottom Terminal
```rust
fn render_bottom_panel(&self) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .h(px(self.bottom_panel_height))
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
                .child(match self.selected_bottom_tab {
                    0 => self.render_terminal(),
                    1 => self.render_problems_panel(),
                    2 => self.render_output_panel(),
                    _ => div(),
                })
        )
}
```

### Data Panels with Vertical Arrangement
```rust
fn render_data_panels(&self) -> impl IntoElement {
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
                .font_weight(600)
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
```

### Input Panel with Form Components
```rust
fn render_input_panel(&self) -> impl IntoElement {
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
                        .font_weight(600)
                        .text_color(rgb(0xffffff))
                        .child("Project Settings")
                )
                .child(self.render_input_field("Project Name", "my-project"))
                .child(self.render_input_field("Version", "0.1.0"))
                .child(self.render_textarea("Description", "Enter project description..."))
        )
        .child(
            div()
                .flex()
                .gap_2()
                .child(self.render_button("Save", true))
                .child(self.render_button("Cancel", false))
        )
}
```

### Window Management with Custom Title Bar
```rust
fn render_window_controls(&self) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_between()
        .h(px(30.0))
        .w_full()
        .bg(rgb(0x2d2d30))
        .border_b_1()
        .border_color(rgb(0x3e3e42))
        .child(
            // Left side: Menu items
            div()
                .flex()
                .items_center()
                .gap_4()
                .pl_3()
                .child(self.render_menu_item("File"))
                .child(self.render_menu_item("Edit"))
                .child(self.render_menu_item("View"))
        )
        .child(
            // Center: Draggable title area
            div()
                .flex_1()
                .h_full()
                .items_center()
                .justify_center()
                .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                    window.start_window_move();
                })
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0xcccccc))
                        .child("GPUI IDE")
                )
        )
        .child(
            // Right side: Window controls
            div()
                .flex()
                .gap_1()
                .child(self.render_minimize_button())
                .child(self.render_maximize_button())
                .child(self.render_close_button())
        )
}

fn render_minimize_button(&self) -> impl IntoElement {
    div()
        .flex()
        .justify_center()
        .items_center()
        .w(px(30.0))
        .h(px(30.0))
        .hover(|style| style.bg(rgb(0x3e3e42)))
        .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
            window.minimize_window();
        })
        .text_sm()
        .text_color(rgb(0xcccccc))
        .child("−")
}

fn render_maximize_button(&self, window: &mut Window) -> impl IntoElement {
    let is_maximized = window.is_maximized();
    let icon = if is_maximized { "⧉" } else { "□" };
    
    div()
        .flex()
        .justify_center()
        .items_center()
        .w(px(30.0))
        .h(px(30.0))
        .hover(|style| style.bg(rgb(0x3e3e42)))
        .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
            window.zoom_window();
        })
        .text_sm()
        .text_color(rgb(0xcccccc))
        .child(icon)
}

fn render_close_button(&self) -> impl IntoElement {
    div()
        .flex()
        .justify_center()
        .items_center()
        .w(px(30.0))
        .h(px(30.0))
        .hover(|style| style.bg(rgb(0xe74c3c)))
        .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
            cx.quit();
        })
        .text_sm()
        .text_color(rgb(0xcccccc))
        .child("×")
}
```

## Integration Points
- **GPUI Framework**: Direct dependency on Zed's UI framework
- **Desktop Integration**: Native windowing via GPUI's application lifecycle
- **GPU Rendering**: All rendering is hardware-accelerated through GPUI

## Modular Architecture

### Component Structure
- **UI Components**: Located in `src/ui/` - self-contained rendering components
- **Content Panels**: Located in `src/panels/` - specialized content areas
- **Main Application**: `src/app.rs` - orchestrates all components
- **Component Pattern**: Each component takes `&IDEApplication` and has a `render()` method

```rust
pub struct MenuBar<'a> {
    app: &'a IDEApplication,
}

impl<'a> MenuBar<'a> {
    pub fn new(app: &'a IDEApplication) -> Self { Self { app } }
    pub fn render(&self) -> impl IntoElement { /* ... */ }
}
```

### Module Organization
- **lib.rs**: Central module declarations and re-exports
- **main.rs**: Minimal entry point using `use gpui_my2ndapp::*;`
- **Component Composition**: Main app renders components via `MenuBar::new(self).render()`
- **State Access**: All components access shared state through the main app reference

## Development Notes
- GPUI applications are desktop-native and require GPU support
- The framework uses immediate-mode rendering with retained-mode optimizations
- State management follows Rust ownership patterns with `SharedString` for text data
- Window management is declarative through `WindowOptions` and `Bounds`
- Modular design allows easy addition of new UI components and content panels