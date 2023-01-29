use egui::{Color32, Context, RichText};

use crate::{
    server::Server,
    world::{
        block_coords,
        chunks::{Chunk, ChunkSection},
    },
};

pub fn render(gui_ctx: &Context, server: &Server) {
    egui::Window::new("Info").show(gui_ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Server: "));
            ui.label(
                RichText::new(format!("{}", server.get_network_destination()))
                    .color(Color32::LIGHT_GRAY),
            );
        });

        let difficulty_locked: &str;
        if server.is_difficulty_locked() {
            difficulty_locked = "(Locked)";
        } else {
            difficulty_locked = "";
        }
        ui.horizontal(|ui| {
            ui.label(RichText::new("Difficulty: "));
            ui.label(
                RichText::new(format!(
                    "{:?} {}",
                    server.get_difficulty(),
                    difficulty_locked
                ))
                .color(Color32::LIGHT_GRAY),
            );
        });

        let period: &str;
        let day_time = server.get_day_time() % 24000;
        if day_time < 6000 {
            period = "Sunrise";
        } else if day_time < 12000 {
            period = "Noon";
        } else if day_time < 18000 {
            period = "Sunset";
        } else {
            period = "Midnight"
        }

        ui.horizontal(|ui| {
            ui.label(RichText::new("Day: "));
            ui.label(
                RichText::new(format!("{}", server.get_world_time() / 24000))
                    .color(Color32::LIGHT_GRAY),
            );
        });

        ui.horizontal(|ui| {
            ui.label(RichText::new("Time: "));
            ui.label(
                RichText::new(format!("{} ({})", day_time, period)).color(Color32::LIGHT_GRAY),
            );
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Health"));
                ui.label(
                    egui::RichText::new(format!("{:.2}", server.get_player().health))
                        .color(Color32::LIGHT_GRAY),
                );
            });

            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Food"));
                ui.label(
                    egui::RichText::new(format!("{:.2}", server.get_player().food))
                        .color(Color32::LIGHT_GRAY),
                );
            });

            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Saturation"));
                ui.label(
                    egui::RichText::new(format!("{:.2}", server.get_player().saturation))
                        .color(Color32::LIGHT_GRAY),
                );
            });
        });

        ui.separator();

        ui.horizontal(|ui| {
            let pos = server.get_player().get_position();
            ui.label("Pos: ");
            ui.label(
                RichText::new(format!("{:.2} / {:.2} / {:.2}", pos.x, pos.y, pos.z))
                    .color(Color32::LIGHT_GRAY),
            );
        });

        ui.horizontal(|ui| {
            let look = server.get_player().get_orientation().get_look_vector();
            ui.label("Look: ");
            ui.label(
                RichText::new(format!("{:.2} / {:.2} / {:.2} ", look.x, look.y, look.z))
                    .color(Color32::LIGHT_GRAY),
            );
            ui.label(
                RichText::new(format!(
                    "Y: {:.2} / P: {:.2}",
                    server.get_player().get_orientation().get_yaw(),
                    server.get_player().get_orientation().get_pitch()
                ))
                .color(Color32::LIGHT_GRAY),
            );
        });

        ui.separator();

        let pos = block_coords(&server.get_player().get_position());
        let chunk = ChunkSection::section_containing(&pos);
        let chunk_coords = Chunk::map_from_world_coords(&pos);

        ui.horizontal(|ui| {
            ui.label("Chunk: ");
            ui.label(
                RichText::new(format!("{} / {} / {}", chunk.x, chunk.y, chunk.z))
                    .color(Color32::LIGHT_GRAY),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Block: ");
            ui.label(
                RichText::new(format!(
                    "{} / {} / {}",
                    chunk_coords.x, chunk_coords.y, chunk_coords.z
                ))
                .color(Color32::LIGHT_GRAY),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Block beneath: ");

            let mut pos = block_coords(server.get_player().get_position());
            loop {
                if pos.y <= 0 {
                    break;
                }
                match server.get_world().block_at(&pos) {
                    Some(b) => {
                        if b.id == 0 {
                            pos.y -= 1;
                            continue;
                        }

                        ui.label(
                            RichText::new(format!("{} - {}", pos.y, b.name))
                                .color(Color32::LIGHT_GRAY),
                        );
                        break;
                    }
                    None => {
                        pos.y -= 1;
                        continue;
                    }
                }
            }
        })

        // let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
        // ui.new_line();
        // ui.new_line();
        // ui.same_line_with_pos(40.0);
        // ui.label("Chunk");
        // ui.same_line_with_pos(140.0);
        // ui.label("Block in Chunk");
        // stack.pop();

        // let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
        // ui.label("X:");
        // stack.pop();
        // ui.same_line_with_pos(40.0);
        // ui.label(format!("{}", chunk.x));
        // ui.same_line_with_pos(140.0);
        // ui.label(format!("{}", chunk_coords.x));

        // let stack = ui.push_style_color(StyleColor::Text, [0.6, 0.6, 0.6, 1.0]);
        // ui.label("Z:");
        // stack.pop();
        // ui.same_line_with_pos(40.0);
        // ui.label(format!("{}", chunk.y));
        // ui.same_line_with_pos(140.0);
        // ui.label(format!("{}", chunk_coords.z));

        // let mut pos = block_coords(server.get_player().get_position());

        // loop {
        //     if pos.y <= 0 {break}
        //     match server.world.get_block_at(&pos) {
        //         Some(b) => {
        //             if b.state_id == 0 {
        //                 pos.y -= 1;
        //                 continue;
        //             }

        //             ui.label(format!("Block beneath: {} - {}", pos.y, b.name));
        //             break;
        //         },
        //         None => {
        //             pos.y -= 1;
        //             continue;
        //         }
        //     }
        // }
    });
}
