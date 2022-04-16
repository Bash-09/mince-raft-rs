use egui::Context;
use log::{debug, error};

use crate::{
    network::{NetworkCommand, NetworkManager, PROTOCOL_1_17_1},
    server::Server,
    settings::Settings,
};

pub fn render(gui_ctx: &Context, settings: &mut Settings) -> Option<Server> {
    let mut serv = None;

    egui::CentralPanel::default().show(gui_ctx, |ui| {
        ui.heading("Main Menu");

        ui.label("Server");
        ui.text_edit_singleline(&mut settings.direct_connection);
        if ui.button("Connect!").clicked() {
            match NetworkManager::connect(&settings.direct_connection) {
                Ok(server) => {
                    debug!("Connected to server.");
                    server
                        .send_command(NetworkCommand::Login(
                            PROTOCOL_1_17_1,
                            25565,
                            "Harry".to_string(),
                        ))
                        .expect("Failed to login");

                    serv = Some(server);
                }
                Err(e) => {
                    error!("Error connecting: {}", e);
                }
            }
        }
    });

    serv
}
