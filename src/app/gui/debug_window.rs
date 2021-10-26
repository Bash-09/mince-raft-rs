use imgui::*;

use crate::app::client::server::Server;

pub struct DebugWindow {}

impl DebugWindow {
    pub fn new() -> DebugWindow {
        DebugWindow {}
    }

    pub fn render(&mut self, ui: &Ui, server: &Server) {
        Window::new(im_str!("Debug"))
            .size([275.0, 300.0], Condition::FirstUseEver)
            .position([700.0, 25.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.text(im_str!("Server: {}", server.network_destination));

                let difficulty_locked: &str;
                if server.difficulty_locked {
                    difficulty_locked = "(Locked)";
                } else {
                    difficulty_locked = "";
                }
                ui.text(im_str!(
                    "Difficulty: {:?} {}",
                    server.difficulty,
                    difficulty_locked
                ));

                let period: &str;
                let day_time = server.day_time % 24000;
                if day_time < 6000 {
                    period = "Sunrise";
                } else if day_time < 12000 {
                    period = "Noon";
                } else if day_time < 18000 {
                    period = "Sunset";
                } else {
                    period = "Midnight"
                }
                ui.text(im_str!(
                    "Day: {}    Time: {} ({})",
                    server.world_time / 24000,
                    day_time,
                    period
                ));

                ui.new_line();
                ui.text(im_str!("Player: {}", server.player.id));
                ui.text(im_str!("X: {:.2}", server.player.position.get_x()));
                ui.same_line(120.0);
                ui.text(im_str!("Health: {}", server.player.health));

                ui.text(im_str!("Y: {:.2}", server.player.position.get_y()));
                ui.same_line(120.0);
                ui.text(im_str!("Food:   {}", server.player.food));

                ui.text(im_str!("Z: {:.2}", server.player.position.get_z()));
                ui.same_line(120.0);
                ui.text(im_str!("Sature: {}", server.player.saturation));

                ui.text(im_str!("Yaw:   {}", server.player.orientation.get_yaw()));
                ui.text(im_str!(
                    "Pitch: {}",
                    server.player.orientation.get_head_pitch()
                ));
            });
    }
}
