use egui::Context;

use crate::server::Server;

pub fn render(gui_ctx: &Context, server: &Server) {
    egui::Window::new("Players").show(gui_ctx, |ui| {
        egui::Grid::new("Players").striped(true).show(ui, |ui| {
            for player in server.get_players().values() {
                ui.label(&player.name);
                ui.label(&format!("{}ms", &player.ping));
                ui.end_row();
            }
        });
    });
}
