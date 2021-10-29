use std::{collections::hash_set::Difference, thread, time::Duration};

use crate::{app::client::{entities::{Entity, ENTITIES}, server::Difficulty, world::chunks::Chunk}, network::{packets::DecodedPacket, types::*, *}, timer::*};

use glium::*;
use glutin::event::VirtualKeyCode;
use winit::os::unix::x11::ffi::Bool;

use crate::io::{keyboard::*, mouse::*};

pub mod gui;
use gui::*;

pub mod client;
use client::{chat::Chat, *};

pub mod logger;
use logger::*;

use self::client::server::Server;

pub struct App {
    pub dis: Display,
    pub gui: Gui,
    pub log: Logger,

    pub mouse: Mouse,
    pub keyboard: Keyboard,

    network: Option<NetworkChannel>,
    server: Option<Server>,

    period: f32,
    last_mod: f32,
}

impl App {
    pub fn new(dis: Display, gui: Gui) -> App {
        App {
            dis,
            gui,
            log: Logger::new(),

            mouse: Mouse::new(),
            keyboard: Keyboard::new(),

            network: None,
            server: None,

            period: 0.05,
            last_mod: 0.0,
        }
    }

    /// Initiailises the App struct
    pub fn init(&mut self) {
        // Creates a network manager and attempts to connect to and login to a server
        match NetworkManager::connect("192.168.1.139:25565") {
            Ok((channel, server)) => {
                println!("Connected to server");

                channel
                    .send
                    .send(NetworkCommand::Login(
                        PROTOCOL_1_17_1,
                        Short(25565),
                        MCString("Harry".to_string()),
                    ))
                    .expect("Failed to login");

                self.server = Some(server);
                self.network = Some(channel);
            }
            Err(e) => {
                println!("Error connecting: {}", e);
            }
        }
    }

    pub fn update(&mut self, t: &Timer) {
        let delta = t.delta();
        let time = t.absolute_time();

        // Runs some code only once every self.period seconds
        let modulus = time % self.period;
        if modulus < self.last_mod {
            match &self.server {
                Some(serv) => {
                    // Send player position update packets
                    if serv.player.id != 0 {
                        send_packet(
                            &self.network,
                            DecodedPacket::PlayerPositionAndRotation(
                                Double(serv.player.position.get_x()),
                                Double(serv.player.position.get_y()),
                                Double(serv.player.position.get_z()),
                                Float(serv.player.orientation.get_yaw() as f32),
                                Float(serv.player.orientation.get_head_pitch() as f32),
                                Boolean(true),
                            ),
                        );
                    }
                }
                None => {}
            }
        }
        self.last_mod = modulus;

        // Runs some code while the server is valid
        match &mut self.server {
            Some(serv) => {
                // Send chat message
                if serv.chat.send {
                    let text = serv.chat.get_message_and_clear();
                    serv.chat.send = false;

                    send_packet(&self.network, DecodedPacket::ChatOutgoing(MCString(text)));
                }

                // Move player
                if self.mouse.is_pressed(0) {
                    let vel = 2.0 * delta as f64;
                    let (x, y, z) = serv.player.orientation.get_look_vector();
                    serv.player
                        .position
                        .translate(x as f64 * vel, y as f64 * vel, z as f64 * vel);
                }

                if self.mouse.is_pressed(2) {
                    let vel = -2.0 * delta as f64;
                    let (x, y, z) = serv.player.orientation.get_look_vector();
                    serv.player
                        .position
                        .translate(x as f64 * vel, y as f64 * vel, z as f64 * vel);
                }

                // Change player direction
                if self.mouse.is_pressed(1) {
                    serv.player.orientation.rotate(
                        self.mouse.get_delta().0 as f64 * 0.5,
                        self.mouse.get_delta().1 as f64 * 0.2,
                    );
                }
            }
            None => {}
        }

        // Collect messages from the NetworkManager
        let mut command_queue: Vec<NetworkCommand> = Vec::new();
        match &self.network {
            Some(channel) => loop {
                match channel.recv.try_recv() {
                    Ok(comm) => {
                        command_queue.push(comm);
                    }
                    Err(_) => {
                        break;
                    }
                }
            },
            None => {}
        }
        // Handle messages from the network manager
        for comm in command_queue {
            self.handle_message(comm);
        }
    }

    /// Handles a message from the NetworkManager
    fn handle_message(&mut self, comm: NetworkCommand) {
        use NetworkCommand::*;

        // Checks the server exists
        let server: &mut Server;
        match &mut self.server {
            Some(serv) => server = serv,
            None => {
                println!("Receiving network commands but there is no server");
                return;
            }
        }

        match comm {
            // Handles any incoming packets
            ReceivePacket(packet) => {
                use DecodedPacket::*;
                match &packet {
                    ServerDifficulty(pack) => {
                        server.difficulty = match pack.difficulty.0 {
                            1 => Difficulty::Easy,
                            2 => Difficulty::Medium,
                            3 => Difficulty::Hard,
                            _ => Difficulty::Peaceful,
                        };
                        server.difficulty_locked = pack.locked.0;
                    }

                    TimeUpdate(pack) => {
                        server.world_time = pack.world_age.0;
                        server.day_time = pack.day_time.0;
                    }

                    UpdateHealth(pack) => {
                        server.player.health = pack.health.0;
                        server.player.food = pack.food.0;
                        server.player.saturation = pack.saturation.0;
                    }

                    Disconnect(pack) => {
                        panic!("Disconnected: {}", pack.reason.0);
                    }

                    LoginSuccess() => {
                        println!("Login success.");
                        self.log.log_info("Successfully Logged in!");
                    }

                    JoinGame(id) => {
                        println!("Setting player id to {}", id.player_id.0);
                        server.join_game(id.player_id.0);
                        send_packet(
                            &self.network,
                            ClientSettings(
                                MCString(server.player.locale.clone()),
                                Byte(server.player.view_distance),
                                VarInt(server.player.chat_mode),
                                Boolean(false),
                                UByte(server.player.displayed_skin_parts),
                                VarInt(server.player.main_hand),
                                Boolean(server.player.disable_text_filtering),
                            ),
                        );
                        send_packet(&self.network, ClientStatusRespawn);
                    }

                    SpawnLivingEntity(pack) => {
                        match server.entities.insert(
                            pack.entity_id.0,
                            Entity::new_with_values(
                                pack.entity_id.0,
                                pack.uuid.clone(),
                                pack.entity_type.0,
                                0,
                                pack.x.0,
                                pack.y.0,
                                pack.z.0,
                                (pack.yaw.0 as f64) / 255.0,
                                (pack.pitch.0 as f64) / 255.0,
                                (pack.head_pitch.0 as f64) / 255.0,
                                (pack.vx.0 as f64) / 8000.0,
                                (pack.vy.0 as f64) / 8000.0,
                                (pack.vz.0 as f64) / 8000.0,
                            ),
                        ) {
                            Some(_) => {}
                            None => {}
                        }
                    }

                    SpawnEntity(pack) => {
                        server.entities.insert(
                            pack.entity_id.0,
                            Entity::new_with_values(
                                pack.entity_id.0,
                                pack.uuid.clone(),
                                pack.entity_type.0,
                                pack.data.0,
                                pack.x.0,
                                pack.y.0,
                                pack.z.0,
                                (pack.yaw.0 as f64) / 255.0,
                                (pack.pitch.0 as f64) / 255.0,
                                0.0,
                                (pack.vx.0 as f64) / 8000.0,
                                (pack.vy.0 as f64) / 8000.0,
                                (pack.vz.0 as f64) / 8000.0,
                            ),
                        );
                    }

                    DestroyEntities(pack) => {
                        for eid in &pack.entities {
                            server.entities.remove(&eid.0);
                        }
                    }

                    EntityPosition(pack) => {
                        match server.entities.get_mut(&pack.entity_id.0) {
                            Some(ent) => {
                                ent.pos.translate(
                                    (pack.dx.0 as f64) / 4096.0,
                                    (pack.dy.0 as f64) / 4096.0,
                                    (pack.dz.0 as f64) / 4096.0,
                                );
                            }
                            None => {}
                        }
                    }

                    EntityPositionAndRotation(pack) => {
                        match server.entities.get_mut(&pack.entity_id.0) {
                            Some(ent) => {
                                ent.pos.translate(
                                    (pack.dx.0 as f64) / 4096.0,
                                    (pack.dy.0 as f64) / 4096.0,
                                    (pack.dz.0 as f64) / 4096.0,
                                );
                                ent.ori.set(pack.yaw.0 as f64 / 256.0, pack.pitch.0 as f64 / 256.0);
                                ent.on_ground = pack.on_ground.0;
                            }
                            None => {}
                        }
                    }

                    EntityRotation(pack) => {
                        match server.entities.get_mut(&pack.entity_id.0) {
                            Some(ent) => {
                                ent.ori.set(pack.yaw.0 as f64 / 256.0, pack.pitch.0 as f64 / 256.0);
                                ent.on_ground = pack.on_ground.0;
                            }
                            None => {}
                        }
                    }

                    EntityHeadLook(pack) => match server.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.ori_head
                                .set(pack.head_yaw.0 as f64 / 256.0, ent.ori_head.get_head_pitch());
                        }
                        None => {}
                    },

                    EntityVelocity(pack) => match server.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.vel.set(
                                pack.vx.0 as f64 / 8000.0,
                                pack.vy.0 as f64 / 8000.0,
                                pack.vz.0 as f64 / 8000.0,
                            );
                        }
                        None => {}
                    },

                    EntityTeleport(pack) => {
                        match server.entities.get_mut(&pack.entity_id.0) {
                            Some(ent) => {
                                ent.pos.set(pack.x.0, pack.y.0, pack.z.0);
                                ent.ori.set(pack.yaw.0 as f64 / 256.0, pack.pitch.0 as f64 / 256.0);
                                ent.on_ground = pack.on_ground.0;
                            }
                            None => {}
                        }
                    }

                    PlayerPositionAndLook(pack) => {
                        server.player.position.set(pack.x.0, pack.y.0, pack.z.0);
                        server.player.orientation.set(pack.yaw.0 as f64, pack.pitch.0 as f64);

                        send_packet(&self.network, TeleportConfirm(pack.teleport_id.clone()));

                        let px = server.player.position.get_x();
                        let py = server.player.position.get_y();
                        let pz = server.player.position.get_z();

                        send_packet(
                            &self.network,
                            PlayerPositionAndRotation(
                                Double(px),
                                Double(py),
                                Double(pz),
                                Float(pack.yaw.0),
                                Float(pack.pitch.0),
                                Boolean(true),
                            ),
                        );
                        self.log.log_incoming_packet(packet);
                    }

                    ChatIncoming(chat) => {
                        server.chat.add_message(&chat);
                        self.log.log_incoming_packet(packet);
                    }

                    ChunkData(cd) => {
                        server.world.insert_chunk(Chunk::new(&cd));
                    }

                    // Currently ignoring these packets
                    SoundEffect(_) | EntityMetadata(_) | BlockChange(_) => {
                        // self.log.log_incoming_packet(packet);
                    }

                    // Packets that have been forwarded but not handled properly
                    _ => {
                        println!("Incoming packet: {:?}", packet);
                    }
                }
                // self.log.log(Log::new(&thread::current(), LogType::PacketReceived(packet)));
            }

            // Log incoming logs
            Log(log) => {
                self.log.log(log);
            }

            // What do with these messages ay??
            _ => {
                println!("Unhandled message: {:?}", comm);
            }
        }
    }

    /// Renders the screen
    pub fn render(&mut self) {
        let mut target = self.dis.draw();

        // Change background colour on certain mouse clicks, idek why I do this lmao
        if self.mouse.is_pressed(0) {
            target.clear_color(0.8, 0.5, 0.5, 1.0);
        } else if self.mouse.is_pressed(2) {
            target.clear_color(1.0, 1.0, 1.0, 1.0);
        } else {
            target.clear_color(0.0, 0.5, 0.8, 1.0);
        }

        // GUI
        self.gui
            .render(&self.dis, &mut target, &self.log, &mut self.server);

        target.finish().unwrap();
    }
}

/// Attempts to send a packet over the provided (possible) network channel
fn send_packet(network: &Option<NetworkChannel>, packet: DecodedPacket) -> Option<()> {
    match network {
        Some(channel) => match channel.send.send(NetworkCommand::SendPacket(packet)) {
            Ok(_) => Some(()),
            Err(_) => None,
        },
        None => None,
    }
}
