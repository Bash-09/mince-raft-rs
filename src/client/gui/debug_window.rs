use imgui::*;

use crate::client::{server::Server, world};

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

                // ui.new_line();
                // ui.text(im_str!("Player: {}", server.player.id));

                ui.new_line();
                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Health");
                stack.pop(ui);
                ui.same_line(75.0);
                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Food");
                stack.pop(ui);
                ui.same_line(150.0);
                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Saturation");
                stack.pop(ui);

                ui.text(im_str!("{}", server.player.health));
                ui.same_line(75.0);
                ui.text(im_str!("{}", server.player.food));
                ui.same_line(150.0);
                ui.text(im_str!("{}", server.player.saturation));

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.new_line();
                ui.new_line();
                ui.same_line(40.0);
                ui.text("Position");
                ui.same_line(140.0);
                ui.text("Looking");
                stack.pop(ui);

                let look = server.player.orientation.get_look_vector();

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("X:");
                stack.pop(ui);
                ui.same_line(50.0);
                ui.text(im_str!("{:.2}", server.player.position.get_x()));
                ui.same_line(150.0);
                ui.text(im_str!("{:.2}", look.0));

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Y:");
                stack.pop(ui);
                ui.same_line(50.0);
                ui.text(im_str!("{:.2}", server.player.position.get_y()));
                ui.same_line(150.0);
                ui.text(im_str!("{:.2}", look.1));

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Z:");
                stack.pop(ui);
                ui.same_line(50.0);
                ui.text(im_str!("{:.2}", server.player.position.get_z()));
                ui.same_line(150.0);
                ui.text(im_str!("{:.2}", look.2));

                let pos = server.player.position.get_block_coords();
                let chunk = world::chunk_at_coords((pos.0, pos.2));
                let chunk_coords = world::chunk_coords((pos.0, pos.2));

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.new_line();
                ui.new_line();
                ui.same_line(40.0);
                ui.text("Chunk");
                ui.same_line(140.0);
                ui.text("Block in Chunk");
                stack.pop(ui);

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("X:");
                stack.pop(ui);
                ui.same_line(40.0);
                ui.text(im_str!("{}", chunk.0));
                ui.same_line(140.0);
                ui.text(im_str!("{}", chunk_coords.0));

                let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
                ui.text("Z:");
                stack.pop(ui);
                ui.same_line(40.0);
                ui.text(im_str!("{}", chunk.1));
                ui.same_line(140.0);
                ui.text(im_str!("{}", chunk_coords.1));

                let mut pos = server.player.position.get_block_coords();


                loop {
                    if pos.1 <= 0 {break}
                    match server.world.block_at(pos) {
                        Some(b) => {
                            if b.state_id == 0 {
                                pos.1 -= 1;
                                continue;
                            }

                            ui.text(im_str!("Block beneath: {} - {}", pos.1, b.name));
                            break;
                        },
                        None => {
                            pos.1 -= 1;
                            continue;
                        }
                    }
                }
            });
    }
}
