use egui::Context;
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::{
    network::{NetworkCommand, NetworkManager, PROTOCOL_1_17_1},
    server::Server,
    settings::{Settings, SETTINGS}, Client,
};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SavedServer {
    pub ip: String,
    pub name: String,
}

pub fn render(gui_ctx: &Context, cli: &mut Client) -> Option<Server> {
    let mut serv = None;

    let mut settings = SETTINGS.write().expect("Couldn't acquire settings");

    egui::SidePanel::left("Server manager")
    .resizable(true)
    .show(gui_ctx, |ui| {

        ui.heading("Account Settings");

        ui.radio_value(&mut settings.online_play, true, "Online mode");
        ui.radio_value(&mut settings.online_play, false, "Offline mode");

        ui.separator();

        if settings.online_play {

            ui.label("Online play is not yet implemented");

        } else {

            ui.horizontal(|ui| {
                ui.label("Player Name: ");
                ui.text_edit_singleline(&mut settings.name);
            });

        }

    });


    egui::CentralPanel::default().show(gui_ctx, |ui| {

        ui.heading("Servers");
        ui.add_space(15.0);

        ui.label("IP Address: ");
        ui.text_edit_singleline(&mut settings.direct_connection);

        ui.horizontal(|ui| {
            if ui.button("Direct Connect").clicked() {
                match connect(&settings.direct_connection, settings.name.clone()) {
                    Ok(s) => serv = Some(s),
                    Err(e) => error!("Failed to connect to server: {:?}", e),
                }
            }

            if ui.button("Save Server").clicked() {
                let ip = settings.direct_connection.clone();
                let name = format!("Saved Server {}", settings.saved_servers.len() + 1);
                settings.saved_servers.push(SavedServer { ip, name });
            }
        });
        ui.separator();

        ui.add_space(15.0);
        ui.horizontal(|ui| {
            ui.add_space(15.0);

            let mut remove: Option<usize> = None;
            for (i, s) in settings.saved_servers.iter().enumerate() {
                ui.vertical(|ui| {
                    ui.set_max_width(200.0);
                    ui.label(&s.name);
                    ui.label(&s.ip);



                    ui.horizontal(|ui| {
                        if ui.button("Connect").clicked() {
                            match connect(&s.ip, settings.name.clone()) {
                                Ok(s) => serv = Some(s),
                                Err(e) => error!("Failed to connect to server: {:?}", e),
                            }
                        }
                        if ui.button("Refresh").clicked() {
                            match NetworkManager::connect(&s.ip) {
                                Ok(server) => {
                                    server.send_command(NetworkCommand::RequestStatus).unwrap();
                                    cli.outstanding_server_pings.insert(s.clone(), server);
                                },
                                Err(e) => {
                                    error!("Couldn't get status from server: {:?}", e);
                                }
                            };
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Edit").clicked() {
                            todo!();
                        }
                        if ui.button("Delete").clicked() {
                            remove = Some(i);
                        }
                    });

                    ui.separator();

                    match cli.server_pings.get(s) {
                        Some(status) => {
                            ui.label(&status.motd);
                            ui.label(format!("Ping: {}ms", status.ping));
                            ui.label(format!("{} / {} Players online.", status.num_players, status.max_players));

                            if status.num_players > 0 {
                                for p in &status.online_players {
                                    ui.label(p);
                                }
                            }
                        },
                        None => {},
                    }
                });
            }
            if let Some(i) = remove {
                settings.saved_servers.remove(i);
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