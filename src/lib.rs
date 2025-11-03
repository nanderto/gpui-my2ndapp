pub mod app;
pub mod ui;
pub mod panels;

pub use app::IDEApplication;
pub use gpui::*;

// Re-export common types for convenience
pub use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size,
};