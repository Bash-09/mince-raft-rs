use egui::Context;
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::{
    network::{NetworkCommand, NetworkManager, PROTOCOL_1_17_1},
    server::Server,
    settings::Settings, Client,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SavedServer {
    pub ip: String,
    pub name: String,
}

pub struct ServerStatus {
    pub version: String, 
    pub num_players: u32,
    pub max_players: u32,
    pub online_players: Vec<String>,
}


pub fn render(gui_ctx: &Context, cli: &mut Client) -> Option<Server> {
    let mut serv = None;


    egui::SidePanel::left("Server manager")
    .resizable(true)
    .show(gui_ctx, |ui| {

        ui.heading("Account Settings");

        ui.radio_value(&mut cli.settings.online_play, true, "Online mode");
        ui.radio_value(&mut cli.settings.online_play, false, "Offline mode");

        ui.separator();

        if cli.settings.online_play {

            ui.label("Online play is not yet implemented");

        } else {

            ui.horizontal(|ui| {
                ui.label("Player Name: ");
                ui.text_edit_singleline(&mut cli.settings.name);
            });

        }

    });


    egui::CentralPanel::default().show(gui_ctx, |ui| {

        ui.heading("Servers");
        ui.add_space(15.0);

        ui.label("IP Address: ");
        ui.text_edit_singleline(&mut cli.settings.direct_connection);

        ui.horizontal(|ui| {
            if ui.button("Direct Connect").clicked() {
                match connect(&cli.settings.direct_connection, cli.settings.name.clone()) {
                    Ok(s) => serv = Some(s),
                    Err(e) => error!("Failed to connect to server: {:?}", e),
                }
            }

            if ui.button("Save Server").clicked() {
                cli.settings.saved_servers.push(SavedServer { 
                    ip: cli.settings.direct_connection.clone(), 
                    name: format!("Saved Server {}", cli.settings.saved_servers.len() + 1),
                })
            }
        });
        ui.separator();

        ui.add_space(15.0);
        ui.horizontal(|ui| {
            ui.add_space(15.0);

            let mut remove: Option<usize> = None;
            for (i, s) in &mut cli.settings.saved_servers.iter().enumerate() {
                ui.vertical(|ui| {
                    ui.label(&s.name);
                    ui.label(&s.ip);

                    ui.horizontal(|ui| {
                        if ui.button("Connect").clicked() {
                            match connect(&s.ip, cli.settings.name.clone()) {
                                Ok(s) => serv = Some(s),
                                Err(e) => error!("Failed to connect to server: {:?}", e),
                            }
                        }
                        if ui.button("Delete").clicked() {
                            remove = Some(i);
                        }
                    });
                });
            }
            if let Some(i) = remove {
                cli.settings.saved_servers.remove(i);
            }
        });
    });

    serv
}

fn connect(ip: &str, name: String) -> Result<Server, std::io::Error> {
    match NetworkManager::connect(ip) {
        Ok(server) => {
            debug!("Connected to server.");
            server
                .send_command(NetworkCommand::Login(
                    PROTOCOL_1_17_1,
                    25565,
                    name,
                ))
                .expect("Failed to login");

            return Ok(server);
        }
        Err(e) => {
            return Err(e);
        }
    }
}