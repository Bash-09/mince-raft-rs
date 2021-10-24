
use std::{collections::hash_set::Difference, thread, time::Duration};

use crate::{app::client::{entities::{ENTITIES, Entity}, server::Difficulty}, network::{*, packets::DecodedPacket, types::*}, timer::*};

use glium::*;
use glutin::event::VirtualKeyCode;
use winit::os::unix::x11::ffi::Bool;

use crate::io::{mouse::*, keyboard::*};

pub mod gui;
use gui::*;

pub mod client;
use client::{*, chat::Chat};

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






    pub fn init(&mut self) {
        
        match NetworkManager::connect("192.168.1.139:25565") {
            Ok((channel, server)) => {
                println!("Connected to server");

                channel.send.send(NetworkCommand::Login(PROTOCOL_1_17_1, Short(25565), MCString("Harry".to_string()))).expect("Failed to login");

                self.server = Some(server);
                self.network = Some(channel);
            },
            Err(e) => {
                println!("Error connecting: {}", e);
            }
        }

    }







    pub fn update(&mut self, t: &Timer) {
        let delta = t.delta();
        let time = t.absolute_time();

        let modulus = time % self.period;
        if modulus < self.last_mod {
            match &self.server  {
                Some(serv) => {

                    if serv.player.id != 0 {

                        send_packet(&self.network, DecodedPacket::PlayerPositionAndRotation(
                            Double(serv.player.position.get_x()),
                            Double(serv.player.position.get_y()),
                            Double(serv.player.position.get_z()),
                            Float(serv.player.orientation.get_yaw() as f32),
                            Float(serv.player.orientation.get_head_pitch() as f32),
                            Boolean(true),
                        ));
                    }

                },
                None => {}
            }
        }
        self.last_mod = modulus;



        match &mut self.server  {
            Some(serv) => {

                // Send chat message
                if serv.chat.send {
                    let text = serv.chat.get_message_and_clear();
                    serv.chat.send = false;

                    send_packet(&self.network, DecodedPacket::ChatOutgoing(MCString(text)));
                }

                if self.mouse.is_pressed(0) {
                    let vel = 2.0 * delta as f64;
                    let (x, y, z) = serv.player.orientation.get_look_vector();
                    serv.player.position.translate(x as f64 * vel, y as f64 * vel, z as f64 * vel);
                }

                if self.mouse.is_pressed(2) {
                    let vel = -2.0 * delta as f64;
                    let (x, y, z) = serv.player.orientation.get_look_vector();
                    serv.player.position.translate(x as f64 * vel, y as f64 * vel, z as f64 * vel);
                }

                // if self.mouse.is_pressed(0) {
                //     serv.player.position.translate(8.0*delta as f64, 0.0,8.0*delta as f64);
                // }

                // if self.mouse.is_pressed(2) {
                //     serv.player.position.translate(-8.0*delta as f64, 0.0, -8.0*delta as f64);
                // }

                if self.mouse.is_pressed(1) {

                    serv.player.orientation.rotate(
                        self.mouse.get_delta().0 as f64,
                        self.mouse.get_delta().1 as f64
                    );

                }                


            },
            None => {}
        }


        let mut command_queue: Vec<NetworkCommand> = Vec::new();
        match &self.network {
            Some(channel) => {
                loop {
                    match channel.recv.try_recv() {
                        Ok(comm) => {
                            command_queue.push(comm);
                        },
                        Err(_) => {break;}
                    }
                }
            },
            None => {}
        }

        for comm in command_queue {
            self.handle_message(comm);
        }

    }


    fn handle_message(&mut self, comm: NetworkCommand) {
        use NetworkCommand::*;

        let server: &mut Server;
        match &mut self.server  {
            Some(serv) => server = serv,
            None => {
                println!("Receiving network commands but there is no server");
                return;
            }
        }
        

        match comm {
            ReceivePacket(packet) => {
                use DecodedPacket::*;
                match &packet {
                    ServerDifficulty(diff, locked) => {
                        server.difficulty = match diff.0 {
                            1 => Difficulty::Easy,
                            2 => Difficulty::Medium,
                            3 => Difficulty::Hard,
                            _ => Difficulty::Peaceful,
                        };
                        server.difficulty_locked = locked.0;
                    },

                    TimeUpdate(world, day) => {
                        server.world_time = world.0;
                        server.day_time = day.0;
                    }

                    UpdateHealth(health, food, sat) => {
                        server.player.health = health.0;
                        server.player.food = food.0;
                        server.player.saturation = sat.0;
                    },

                    Disconnect(reason) => {
                        panic!("Disconnected: {}", reason.0);
                    },
                    LoginSuccess() => {
                        println!("Login success.");
                        self.log.log_info("Successfully Logged in!");
                    },
                    JoinGame(id) => {
                        println!("Setting player id to {}", id.0);
                        server.join_game(id.0);
                        send_packet(&self.network, ClientSettings(
                            MCString(server.player.locale.clone()),
                            Byte(server.player.view_distance),
                            VarInt(server.player.chat_mode),
                            Boolean(false),
                            UByte(server.player.displayed_skin_parts),
                            VarInt(server.player.main_hand),
                            Boolean(server.player.disable_text_filtering),
                        ));
                        send_packet(&self.network, ClientStatusRespawn);
                    },

                    SpawnLivingEntity(id, uuid, ent_type, x, y, z, yaw, pitch, head, vx, vy, vz) => {
                        match server.entities.insert(id.0, Entity::new_with_values(
                            id.0, uuid.clone(), ent_type.0, 0,
                            x.0, y.0, z.0, 
                            (yaw.0 as f64)/255.0, (pitch.0 as f64)/255.0, (head.0 as f64)/255.0, 
                            (vx.0 as f64)/8000.0, (vy.0 as f64)/8000.0, (vz.0 as f64)/8000.0
                        )) {
                            Some(_) => {},
                            None => {}
                        }
                    },

                    SpawnEntity(id, uuid, ent_type, x, y, z, pitch, yaw, data, vx, vy, vz) => {
                        match server.entities.insert(id.0, Entity::new_with_values(
                            id.0, uuid.clone(), ent_type.0, data.0,
                            x.0, y.0, z.0, 
                            (yaw.0 as f64)/255.0, (pitch.0 as f64)/255.0, 0.0, 
                            (vx.0 as f64)/8000.0, (vy.0 as f64)/8000.0, (vz.0 as f64)/8000.0
                        )) {
                            Some(_) => {},
                            None => {}
                        }
                    },

                    DestroyEntities(_, ids) => {
                        for eid in ids {
                            server.entities.remove(&eid.0);
                        }
                    },

                    EntityPosition(id, dx, dy, dz, on_ground) => {
                        match server.entities.get_mut(&id.0) {
                            Some(ent) => {
                                ent.pos.translate((dx.0 as f64)/4096.0, (dy.0 as f64)/4096.0, (dz.0 as f64)/4096.0);
                            },
                            None => {}
                        }
                    },

                    EntityPositionAndRotation(id, dx, dy, dz, yaw, pitch, on_ground) => {
                        match server.entities.get_mut(&id.0) {
                            Some(ent) => {
                                ent.pos.translate((dx.0 as f64)/4096.0, (dy.0 as f64)/4096.0, (dz.0 as f64)/4096.0);
                                ent.ori.set(yaw.0 as f64 / 256.0, pitch.0 as f64 / 256.0);
                                ent.on_ground = on_ground.0;
                            },
                            None => {}
                        }
                    },

                    EntityRotation(id, yaw, pitch, on_ground) => {
                        match server.entities.get_mut(&id.0) {
                            Some(ent) => {
                                ent.ori.set(yaw.0 as f64 / 256.0, pitch.0 as f64 / 256.0);
                                ent.on_ground = on_ground.0;
                            },
                            None => {}
                        }                    
                    },

                    EntityHeadLook(id, head_yaw) => {
                        match server.entities.get_mut(&id.0) {
                            Some(ent) => {
                                ent.ori_head.set(head_yaw.0 as f64 / 256.0, ent.ori_head.get_head_pitch());
                            },
                            None => {}
                        }
                    },

                    EntityVelocity(id, vx, vy, vz) => {
                        match server.entities.get_mut(&id.0) {
                            Some(ent) => {
                                ent.vel.set(vx.0 as f64/8000.0, vy.0 as f64/8000.0, vz.0 as f64/8000.0);
                            },
                            None => {},
                        }
                    },

                    EntityTeleport(id, x, y, z, yaw, pitch, on_ground) => {
                        match server.entities.get_mut(&id.0) {
                            Some(ent) => {
                                ent.pos.set(x.0, y.0, z.0);
                                ent.ori.set(yaw.0 as f64/256.0, pitch.0 as f64/256.0);
                                ent.on_ground = on_ground.0;
                            },
                            None => {},
                        }
                    },

                    PlayerPositionAndLook(x, y, z, yaw, pitch, flags, teleport_id, dismount) => {
                        server.player.position.set(x.0, y.0, z.0);
                        server.player.orientation.set(yaw.0 as f64, pitch.0 as f64);

                        send_packet(&self.network, TeleportConfirm(teleport_id.clone()));

                        let px = server.player.position.get_x();
                        let py = server.player.position.get_y();
                        let pz = server.player.position.get_z();

                        send_packet(&self.network,
                            PlayerPositionAndRotation(
                                Double(px),
                                Double(py),
                                Double(pz),
                                Float(yaw.0),
                                Float(pitch.0),
                                Boolean(true),
                        ));
                        self.log.log_incoming_packet(packet);
                    },
                    
                    ChatIncoming(_, _, _) => {
                        server.chat.add_message(&packet);
                        self.log.log_incoming_packet(packet);
                    },

                    // SoundEffect() | EntityMetadata() | BlockChange(_, _) => {
                    //     self.log.log_incoming_packet(packet);
                    // }

                    _ => {println!("Unhandled incoming packet: {:?}", packet);}
                }
                // self.log.log(Log::new(&thread::current(), LogType::PacketReceived(packet)));
            },

            Log(log) => {
                self.log.log(log);
            }

            _ => {
                println!("Unhandled message: {:?}", comm);
            }
        }

    }

    pub fn render(&mut self) {
        let mut target = self.dis.draw();

        if self.mouse.is_pressed(0) {
            target.clear_color(0.8, 0.5, 0.5, 1.0);
        } else if self.mouse.is_pressed(2) {
            target.clear_color(1.0, 1.0, 1.0, 1.0);
        } else {
            target.clear_color(0.0, 0.5, 0.8, 1.0);
        }


        

        self.gui.render(&self.dis, &mut target, &self.log, &mut self.server);

        target.finish().unwrap();
    }


}


fn send_packet(network: &Option<NetworkChannel>, packet: DecodedPacket) -> Option<()> {
    match network {
        Some(channel) => {
            match channel.send.send(NetworkCommand::SendPacket(packet)) {
                Ok(_) => {
                    Some(())
                },
                Err(_) => None
            }
        },
        None => None
    }
}