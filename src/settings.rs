use std::path::Path;

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::gui::main_menu::SavedServer;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub direct_connection: String,
    pub show_fps: bool,

    pub mouse_sensitivity: f32,

    pub online_play: bool,
    pub name: String,
    pub saved_servers: Vec<SavedServer>,

    pub day_colour: [f32; 3],
    pub fog_near: f32,
    pub fog_far: f32,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            direct_connection: String::new(),
            show_fps: true,

            mouse_sensitivity: 1.0,

            online_play: false,
            name: String::from("Harry"),
            saved_servers: Vec::new(),

            day_colour: [0.2, 0.5, 0.9],
            fog_near: 5.0,
            fog_far: 320.0,
        }
    }

    /// Save settings in json format to the specified file
    pub fn save<P: AsRef<Path>>(&self, file: P) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_json::to_string(&self)?;
        std::fs::write(file, &contents)?;
        Ok(())
    }

    /// Read settings from json format from the specified file
    /// Saved settings in json must have the same structure as the struct trying to load othewise it will fail
    /// Maybe in the future I will improve this but I can't be bothered for now since I don't have many settings to save yet
    pub fn load<P: AsRef<Path>>(file: P) -> Result<Settings, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(file)?;
        let set = serde_json::from_str::<Settings>(&contents)?;
        Ok(set)
    }
}
