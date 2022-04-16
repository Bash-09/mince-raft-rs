pub struct Settings {
    pub direct_connection: String,
    pub show_fps: bool,

    pub mouse_sensitivity: f32,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            direct_connection: String::from("192.168.20.9:25565"),
            show_fps: true,

            mouse_sensitivity: 1.0,
        }
    }
}
