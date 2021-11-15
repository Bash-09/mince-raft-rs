use std::ops::{AddAssign, Mul};

use crate::{client::{{
        entities::Entity,
        server::Difficulty,
        world::chunks::Chunk,
    }, network::{types::*, *}}, timer::*};

mod network;


use glam::{Vec2, Vec3};
use glium::{Display, Surface, glutin::event::VirtualKeyCode};
use log::{debug, error, info};

use crate::io::{keyboard::*, mouse::*};

pub mod gui;
use gui::*;

pub mod chat;
pub mod entities;
pub mod player;
pub mod server;
pub mod world;
pub mod renderer;


use self::{network::{NetworkChannel, NetworkManager, packets::DecodedPacket}, renderer::Renderer, server::Server};





pub struct Client {
    pub dis: Display,
    pub gui: Gui,
    pub rend: Renderer,

    pub mouse: Mouse,
    pub keyboard: Keyboard,

    network: Option<NetworkChannel>,
    server: Option<Server>,

    period: f32,
    last_mod: f32,
}

impl Client {
    pub fn new(dis: Display, gui: Gui) -> Client {
        let rend = Renderer::new(&dis);

        Client {
            dis,
            gui,
            rend,

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
                debug!("Connected to server.");
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
                error!("Error connecting: {}", e);
            }
        }
    }

    pub fn update(&mut self, t: &Timer) {
        let delta = t.delta();
        let time = t.absolute_time();

        println!("Delta: {}", delta);

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
                                Double(serv.player.get_position().x as f64),
                                Double(serv.player.get_position().y as f64),
                                Double(serv.player.get_position().z as f64),
                                Float(serv.player.get_orientation().get_yaw() as f32),
                                Float(serv.player.get_orientation().get_head_pitch() as f32),
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

                // Update camera
                self.rend.cam.set_pos(serv.player.get_position().clone());
                self.rend.cam.translate(Vec3::new(0.0, 1.7, 0.0));
                self.rend.cam.set_rot(serv.player.get_orientation().get_rotations() * -1.0);

                if self.keyboard.pressed_this_frame(&VirtualKeyCode::Escape) {
                    self.gui.show_gui = !self.gui.show_gui;
                }

                // Send chat message
                if serv.chat.send {
                    let text = serv.chat.get_message_and_clear();
                    serv.chat.send = false;

                    send_packet(&self.network, DecodedPacket::ChatOutgoing(MCString(text)));
                }


                if !self.gui.show_gui {
                    let vel = 8.0 * delta;

                    if self.keyboard.is_pressed(&VirtualKeyCode::W) {
                        let mut dir = serv.player.get_orientation().get_look_vector();
                        dir.y = 0.0;
                        dir = dir.normalize();
                        dir *= vel;
                        serv.player.get_position_mut().add_assign(dir);
                    }

                    if self.keyboard.is_pressed(&VirtualKeyCode::S) {
                        let mut dir = serv.player.get_orientation().get_look_vector();
                        dir.y = 0.0;
                        dir = dir.normalize();
                        dir *= -vel;
                        serv.player.get_position_mut().add_assign(dir);
                    }

                    if self.keyboard.is_pressed(&VirtualKeyCode::A) {
                        let mut dir = serv.player.get_orientation().get_look_vector();
                        dir.y = 0.0;
                        dir = dir.normalize();
                        dir *= -vel;
                        dir.y = dir.x;
                        dir.x = -dir.z;
                        dir.z = dir.y;
                        dir.y = 0.0;
                        serv.player.get_position_mut().add_assign(dir);
                    }

                    if self.keyboard.is_pressed(&VirtualKeyCode::D) {
                        let mut dir = serv.player.get_orientation().get_look_vector();
                        dir.y = 0.0;
                        dir = dir.normalize();
                        dir *= vel;
                        dir.y = dir.x;
                        dir.x = -dir.z;
                        dir.z = dir.y;
                        dir.y = 0.0;
                        serv.player.get_position_mut().add_assign(dir);
                    }

                    if self.keyboard.is_pressed(&VirtualKeyCode::Space) {
                        serv.player.get_position_mut().add_assign(Vec3::new(0.0, vel, 0.0));
                    }

                    if self.keyboard.is_pressed(&VirtualKeyCode::LShift) {
                        serv.player.get_position_mut().add_assign(Vec3::new(0.0, -vel, 0.0));
                    }

                    if self.mouse.is_pressed(0) {
                        let off = self.mouse.get_delta();
                        serv.player.get_orientation_mut().rotate(
                            off.0 as f32 * 0.1,
                            off.1 as f32 * 0.1,
                        );
                    }

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
                error!("Receiving network commands but there is no server!");
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
                        info!("Changed difficulty: {}", pack.locked.0);
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
                        info!("Disconnected from server: {}", pack.reason.0);
                        self.network = None;
                        self.server = None;
                    }

                    LoginSuccess(pack) => {
                        info!("Successfully Logged in!");
                    }

                    JoinGame(id) => {
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
                                pack.x.0 as f32,
                                pack.y.0 as f32,
                                pack.z.0 as f32,
                                (pack.yaw.0 as f32) / 255.0,
                                (pack.pitch.0 as f32) / 255.0,
                                (pack.head_pitch.0 as f32) / 255.0,
                                (pack.vx.0 as f32) / 8000.0,
                                (pack.vy.0 as f32) / 8000.0,
                                (pack.vz.0 as f32) / 8000.0,
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
                                pack.x.0 as f32,
                                pack.y.0 as f32,
                                pack.z.0 as f32,
                                (pack.yaw.0 as f32) / 255.0,
                                (pack.pitch.0 as f32) / 255.0,
                                0.0,
                                (pack.vx.0 as f32) / 8000.0,
                                (pack.vy.0 as f32) / 8000.0,
                                (pack.vz.0 as f32) / 8000.0,
                            ),
                        );
                    }

                    DestroyEntities(pack) => {
                        for eid in &pack.entities {
                            server.entities.remove(&eid.0);
                        }
                    }

                    EntityPosition(pack) => match server.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.pos.add_assign(Vec3::new(
                                (pack.dx.0 as f32) / 4096.0,
                                (pack.dy.0 as f32) / 4096.0,
                                (pack.dz.0 as f32) / 4096.0,
                            ));
                        }
                        None => {}
                    },

                    EntityPositionAndRotation(pack) => {
                        match server.entities.get_mut(&pack.entity_id.0) {
                            Some(ent) => {
                                ent.pos.add_assign(Vec3::new(
                                    (pack.dx.0 as f32) / 4096.0,
                                    (pack.dy.0 as f32) / 4096.0,
                                    (pack.dz.0 as f32) / 4096.0,
                                ));
                                ent.ori
                                    .set(pack.yaw.0 as f32 / 256.0, pack.pitch.0 as f32 / 256.0);
                                ent.on_ground = pack.on_ground.0;
                            }
                            None => {}
                        }
                    }

                    EntityRotation(pack) => match server.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.ori
                                .set(pack.yaw.0 as f32 / 256.0, pack.pitch.0 as f32 / 256.0);
                            ent.on_ground = pack.on_ground.0;
                        }
                        None => {}
                    },

                    EntityHeadLook(pack) => match server.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.ori_head.set(
                                pack.head_yaw.0 as f32 / 256.0,
                                ent.ori_head.get_head_pitch(),
                            );
                        }
                        None => {}
                    },

                    EntityVelocity(pack) => match server.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.vel.add_assign(Vec3::new(
                                pack.vx.0 as f32 / 8000.0,
                                pack.vy.0 as f32 / 8000.0,
                                pack.vz.0 as f32 / 8000.0,
                            ));
                        }
                        None => {}
                    },

                    EntityTeleport(pack) => match server.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.pos = Vec3::new(pack.x.0 as f32, pack.y.0 as f32, pack.z.0 as f32);
                            ent.ori
                                .set(pack.yaw.0 as f32 / 256.0, pack.pitch.0 as f32 / 256.0);
                            ent.on_ground = pack.on_ground.0;
                        }
                        None => {}
                    },

                    PlayerPositionAndLook(pack) => {
                        debug!("Player position updated!");

                        server.player.set_position(Vec3::new(pack.x.0 as f32, pack.y.0 as f32, pack.z.0 as f32));
                        server
                            .player
                            .get_orientation_mut()
                            .set(pack.yaw.0 as f32, pack.pitch.0 as f32);

                        send_packet(&self.network, TeleportConfirm(pack.teleport_id.clone()));

                        let px = server.player.get_position().x;
                        let py = server.player.get_position().y;
                        let pz = server.player.get_position().z;

                        send_packet(
                            &self.network,
                            PlayerPositionAndRotation(
                                Double(px as f64),
                                Double(py as f64),
                                Double(pz as f64),
                                Float(pack.yaw.0),
                                Float(pack.pitch.0),
                                Boolean(true),
                            ),
                        );
                    }

                    ChatIncoming(chat) => {
                        server.chat.add_message(&chat);
                    }

                    ChunkData(cd) => {
                        server.world.insert_chunk(Chunk::new(&self.dis, &cd));
                    }

                    // Currently ignoring these packets
                    SoundEffect(_) 
                    | EntityMetadata(_) 
                    | EntityProperties(_)
                    | EntityStatus(_)
                    | Effect(_)
                    | EntityAnimation(_)
                    => {
                        // self.log.log_incoming_packet(packet);
                    }

                    // Packets that have been forwarded but not handled properly
                    _ => {
                        debug!("Got Packet: {:?}", packet);
                    }
                }
            }

            // What do with these messages ay??
            _ => {
                debug!("Unhandled message: {:?}", comm);
            }
        }
    }

    pub fn close(&mut self) {
        info!("Closing client.");
        match &self.network {
            Some(nc) => {
                nc.send.send(NetworkCommand::Disconnect).expect("Failed to send disconnect command to network commander.");
            },
            None => {}
        }
    }

    /// Renders the screen
    pub fn render(&mut self) {
        let mut target = self.dis.draw();

        // Render world if it exists
        match &self.server {
            Some(s) => {
                self.rend.render_server(&mut target, s);
            },
            None => {}
        }

        // GUI
        self.gui.render(&self.dis, &mut target, &mut self.server);

        target.finish().unwrap();
    }
}

/// Attempts to send a packet over the provided (possible) network channel
fn send_packet(network: &Option<NetworkChannel>, packet: DecodedPacket) -> Option<()> {
    match network {
        Some(channel) => match channel.send.send(NetworkCommand::SendPacket(packet)) {
            Ok(_) => Some(()),
            Err(e) => {
                error!("Failed to communicate with network commander: {:?}", e);
                panic!("Disconnected");
                None
            },
        },
        None => None,
    }
}
