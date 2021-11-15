use glam::Vec3Swizzles;
use imgui::*;

use crate::client::{server::Server, world::{self, block_coords}};

pub struct DebugWindow {}

impl DebugWindow {
    pub fn new() -> DebugWindow {
        DebugWindow {}
    }

    pub fn render(&mut self, ui: &Ui, server: &Server) {
        Window::new("Debug")
            .size([275.0, 300.0], Condition::FirstUseEver)
            .position([700.0, 25.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.text(format!("Server: {}", server.network_destination));

                let difficulty_locked: &str;
                if server.difficulty_locked {
                    difficulty_locked = "(Locked)";
                } else {
                    difficulty_locked = "";
                }
                ui.text(format!(
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
                ui.text(format!(
                    "Day: {}    Time: {} ({})",
                    server.world_time / 24000,
                    day_time,
                    period
                ));

                // ui.new_line();
                // ui.text(format!("Player: {}", server.player.id));

                ui.new_line();
                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Health");
                stack.pop();
                ui.same_line_with_pos(75.0);
                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Food");
                stack.pop();
                ui.same_line_with_pos(150.0);
                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Saturation");
                stack.pop();

                ui.text(format!("{}", server.player.health));
                ui.same_line_with_pos(75.0);
                ui.text(format!("{}", server.player.food));
                ui.same_line_with_pos(150.0);
                ui.text(format!("{}", server.player.saturation));

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.new_line();
                ui.new_line();
                ui.same_line_with_pos(40.0);
                ui.text("Position");
                ui.same_line_with_pos(140.0);
                ui.text("Looking");
                stack.pop();

                let look = server.player.get_orientation().get_look_vector();

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("X:");
                stack.pop();
                ui.same_line_with_pos(50.0);
                ui.text(format!("{:.2}", server.player.get_position().x));
                ui.same_line_with_pos(150.0);
                ui.text(format!("{:.2}", look.x));

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Y:");
                stack.pop();
                ui.same_line_with_pos(50.0);
                ui.text(format!("{:.2}", server.player.get_position().y));
                ui.same_line_with_pos(150.0);
                ui.text(format!("{:.2}", look.y));

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Z:");
                stack.pop();
                ui.same_line_with_pos(50.0);
                ui.text(format!("{:.2}", server.player.get_position().z));
                ui.same_line_with_pos(150.0);
                ui.text(format!("{:.2}", look.z));

                let pos = block_coords(&server.player.get_position());
                let chunk = world::chunk_at_coords(&pos.xz());
                let chunk_coords = world::chunk_coords(&pos);

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.new_line();
                ui.new_line();
                ui.same_line_with_pos(40.0);
                ui.text("Chunk");
                ui.same_line_with_pos(140.0);
                ui.text("Block in Chunk");
                stack.pop();

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("X:");
                stack.pop();
                ui.same_line_with_pos(40.0);
                ui.text(format!("{}", chunk.x));
                ui.same_line_with_pos(140.0);
                ui.text(format!("{}", chunk_coords.x));

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Z:");
                stack.pop();
                ui.same_line_with_pos(40.0);
                ui.text(format!("{}", chunk.y));
                ui.same_line_with_pos(140.0);
                ui.text(format!("{}", chunk_coords.z));

                let mut pos = block_coords(server.player.get_position());

                loop {
                    if pos.y <= 0 {break}
                    match server.world.get_block_at(&pos) {
                        Some(b) => {
                            if b.state_id == 0 {
                                pos.y -= 1;
                                continue;
                            }

                            ui.text(format!("Block beneath: {} - {}", pos.y, b.name));
                            break;
                        },
                        None => {
                            pos.y -= 1;
                            continue;
                        }
                    }
                }
            });
    }
}
