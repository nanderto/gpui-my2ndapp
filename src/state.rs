use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use directories::ProjectDirs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WindowBounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_maximized: bool,
}

impl Default for WindowBounds {
    fn default() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            width: 1200.0,
            height: 800.0,
            is_maximized: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PanelState {
    pub left_panel_width: f32,
    pub bottom_panel_height: f32,
    pub selected_left_tab: usize,
    pub selected_main_tab: usize,
    pub selected_bottom_tab: usize,
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            left_panel_width: 300.0,
            bottom_panel_height: 200.0,
            selected_left_tab: 0,
            selected_main_tab: 0,
            selected_bottom_tab: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TabState {
    pub name: String,
    pub file_path: Option<String>,
    pub is_dirty: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
    pub window_bounds: WindowBounds,
    pub panel_state: PanelState,
    pub open_tabs: Vec<TabState>,
    pub version: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            window_bounds: WindowBounds::default(),
            panel_state: PanelState::default(),
            open_tabs: vec![
                TabState {
                    name: "main.rs".to_string(),
                    file_path: Some("src/main.rs".to_string()),
                    is_dirty: false,
                },
                TabState {
                    name: "Cargo.toml".to_string(),
                    file_path: Some("Cargo.toml".to_string()),
                    is_dirty: false,
                },
                TabState {
                    name: "README.md".to_string(),
                    file_path: Some("README.md".to_string()),
                    is_dirty: false,
                },
            ],
            version: "0.1.0".to_string(),
        }
    }
}

impl AppState {
    const APP_NAME: &'static str = "gpui-my2ndapp";
    const STATE_FILE: &'static str = "app_state.json";

    pub fn load() -> Self {
        match Self::get_state_file_path() {
            Some(path) => {
                if path.exists() {
                    match fs::read_to_string(&path) {
                        Ok(contents) => {
                            match serde_json::from_str::<AppState>(&contents) {
                                Ok(state) => {
                                    println!("Loaded app state from: {:?}", path);
                                    return state;
                                }
                                Err(e) => {
                                    eprintln!("Failed to parse state file: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read state file: {}", e);
                        }
                    }
                }
            }
            None => {
                eprintln!("Could not determine state file path");
            }
        }
        
        println!("Using default app state");
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_state_file_path()
            .ok_or("Could not determine state file path")?;
        
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json)?;
        
        println!("Saved app state to: {:?}", path);
        Ok(())
    }

    fn get_state_file_path() -> Option<PathBuf> {
        ProjectDirs::from("", "", Self::APP_NAME)
            .map(|proj_dirs| proj_dirs.config_dir().join(Self::STATE_FILE))
    }

    pub fn update_window_bounds(&mut self, x: f32, y: f32, width: f32, height: f32, is_maximized: bool) {
        self.window_bounds.x = x;
        self.window_bounds.y = y;
        self.window_bounds.width = width;
        self.window_bounds.height = height;
        self.window_bounds.is_maximized = is_maximized;
    }

    pub fn update_panel_state(&mut self, left_width: f32, bottom_height: f32, left_tab: usize, main_tab: usize, bottom_tab: usize) {
        self.panel_state.left_panel_width = left_width;
        self.panel_state.bottom_panel_height = bottom_height;
        self.panel_state.selected_left_tab = left_tab;
        self.panel_state.selected_main_tab = main_tab;
        self.panel_state.selected_bottom_tab = bottom_tab;
    }
}