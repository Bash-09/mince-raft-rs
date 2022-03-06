use egui::Context;
use log::{debug, error};

use crate::{server::Server, settings::Settings, network::{NetworkManager, NetworkCommand, PROTOCOL_1_17_1, types::*}};

pub fn render(gui_ctx: &Context, settings: &mut Settings) -> Option<Server> {

    let mut serv = None;

    egui::CentralPanel::default()
    .show(gui_ctx, |ui| {

        ui.heading("Main Menu");

        ui.label("Server");
        ui.text_edit_singleline(&mut settings.direct_connection);
        if ui.button("Connect!").clicked() {
            match NetworkManager::connect(&settings.direct_connection) {
                Ok(server) => {
                    debug!("Connected to server.");
                    server.network
                        .send
                        .send(NetworkCommand::Login(
                            PROTOCOL_1_17_1,
                            Short(25565),
                            MCString("Harry".to_string()),
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
