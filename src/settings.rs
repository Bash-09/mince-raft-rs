

pub struct Settings {
    pub direct_connection: String,
    pub mouse_visible: bool,
    pub show_fps: bool,
}


impl Settings {

    pub fn default() -> Settings {
        Settings {
            direct_connection: String::from("192.168.20.9:25565"),
            mouse_visible: true,
            show_fps: true,
        }
    }

}