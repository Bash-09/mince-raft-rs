use std::{collections::HashMap, ops::AddAssign};

use egui_winit::winit::event::VirtualKeyCode;
use glam::Vec3;
use glium_app::context::Context;
use log::{error, info, debug};

use crate::{network::{NetworkChannel, packets::DecodedPacket, NetworkCommand, types::*}, world::chunks::Chunk, settings::Settings, state::State};

use super::{
    chat::Chat,
    entities::Entity,
    player::Player,
    world::World,
};

pub enum ServerState {
    Status,
    Login,
    Play,
}

pub struct Server {
    network_destination: String,
    network: NetworkChannel,

    world_time: i64,
    day_time: i64,

    player: Player,
    chat: Chat,

    world: World,

    entities: HashMap<i32, Entity>,

    difficulty: Difficulty,
    difficulty_locked: bool,

    paused: bool,
    pub disconnect: bool,
}

impl Server {
    pub fn new(network_destination: String, network: NetworkChannel) -> Server {
        Server {
            network_destination,
            network,

            world_time: 0,
            day_time: 0,

            player: Player::new(),
            chat: Chat::new(),

            world: World::new(),

            entities: HashMap::new(),

            difficulty: Difficulty::Easy,
            difficulty_locked: false,

            paused: false,
            disconnect: false,
        }
    }

    pub fn get_network_destination(&self) -> &str {
        &self.network_destination
    }

    pub fn get_world_time(&self) -> i64 {
        self.world_time
    }

    pub fn get_day_time(&self) -> i64 {
        self.day_time
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_chat(&self) -> &Chat {
        &self.chat
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn get_entities(&self) -> &HashMap<i32, Entity> {
        &self.entities
    }

    pub fn get_difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn is_difficulty_locked(&self) -> bool {
        self.difficulty_locked
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool, state: &mut State) {
        self.paused = paused;
        state.mouse_visible = self.paused;
        state.mouse_grabbed = !self.paused;

        if !paused {
            state.options_visible = false;
        }
    }


    pub fn join_game(&mut self, player_id: i32) {
        self.player.id = player_id;
    }


    /// Attempts to send a packet over the provided (possible) network channel
    pub fn send_packet(&self, packet: DecodedPacket) -> Option<()> {
        match self.network.send.send(NetworkCommand::SendPacket(packet)) {
            Ok(_) => Some(()),
            Err(e) => {
                error!("Failed to communicate with network commander: {:?}", e);
                panic!("Disconnected");
                None
            }
        }
    }

    /// Attempts to send a packet over the provided (possible) network channel
    pub fn send_command(&self, command: NetworkCommand) -> Option<()> {
        match self.network.send.send(command) {
            Ok(_) => Some(()),
            Err(e) => {
                error!("Failed to communicate with network commander: {:?}", e);
                panic!("Disconnected");
                None
            }
        }
    }

    pub fn update(&mut self, ctx: &Context, state: &mut State, settings: &Settings, delta: f32) {

        if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::Escape) {

            self.set_paused(!self.paused, state);

        }

        // Send chat message
        if self.chat.send {
            let text = self.chat.get_message_and_clear();
            self.chat.send = false;

            self.send_packet(DecodedPacket::ChatOutgoing(MCString(text)));
        }

        // if !self.gui.show_gui {
        let vel = 8.0 * delta;

        if !self.paused {

            if ctx.keyboard.is_pressed(&VirtualKeyCode::W) {
                let mut dir = self.player.get_orientation().get_look_vector();
                dir.y = 0.0;
                dir = dir.normalize();
                dir *= vel;
                self.player.get_position_mut().add_assign(dir);
            }

            if ctx.keyboard.is_pressed(&VirtualKeyCode::S) {
                let mut dir = self.player.get_orientation().get_look_vector();
                dir.y = 0.0;
                dir = dir.normalize();
                dir *= -vel;
                self.player.get_position_mut().add_assign(dir);
            }

            if ctx.keyboard.is_pressed(&VirtualKeyCode::A) {
                let mut dir = self.player.get_orientation().get_look_vector();
                dir.y = 0.0;
                dir = dir.normalize();
                dir *= -vel;
                dir.y = dir.x;
                dir.x = -dir.z;
                dir.z = dir.y;
                dir.y = 0.0;
                self.player.get_position_mut().add_assign(dir);
            }

            if ctx.keyboard.is_pressed(&VirtualKeyCode::D) {
                let mut dir = self.player.get_orientation().get_look_vector();
                dir.y = 0.0;
                dir = dir.normalize();
                dir *= vel;
                dir.y = dir.x;
                dir.x = -dir.z;
                dir.z = dir.y;
                dir.y = 0.0;
                self.player.get_position_mut().add_assign(dir);
            }

            if ctx.keyboard.is_pressed(&VirtualKeyCode::Space) {
                self.player
                    .get_position_mut()
                    .add_assign(Vec3::new(0.0, vel, 0.0));
            }

            if ctx.keyboard.is_pressed(&VirtualKeyCode::LShift) {
                self.player
                    .get_position_mut()
                    .add_assign(Vec3::new(0.0, -vel, 0.0));
            }

            if ctx.mouse.pressed_this_frame(0) {
                
            }

            let off = ctx.mouse.get_delta();
            self.player
                .get_orientation_mut()
                .rotate(off.0 as f32 * 0.05 * settings.mouse_sensitivity, off.1 as f32 * 0.05 * settings.mouse_sensitivity);
        }



        // }

                // Collect messages from the NetworkManager
        let mut command_queue: Vec<NetworkCommand> = Vec::new();
        loop {
            match self.network.recv.try_recv() {
                Ok(comm) => {
                    command_queue.push(comm);
                }
                Err(_) => {
                    break;
                }
            }
        }
        // Handle messages from the network manager
        for comm in command_queue {
            self.handle_message(comm, ctx);
        }

    }

    pub fn disconnect(&mut self) {
        info!("Disconnecting from server.");
        self.network.send.send(NetworkCommand::Disconnect).expect("Failed to send message to network thread.");
    }


    /// Handles a message from the NetworkManager
    fn handle_message(&mut self, comm: NetworkCommand, ctx: &Context) {
        use NetworkCommand::*;

        match comm {
            // Handles any incoming packets
            ReceivePacket(packet) => {
                use DecodedPacket::*;
                match &packet {
                    ServerDifficulty(pack) => {
                        self.difficulty = match pack.difficulty.0 {
                            1 => Difficulty::Easy,
                            2 => Difficulty::Medium,
                            3 => Difficulty::Hard,
                            _ => Difficulty::Peaceful,
                        };
                        self.difficulty_locked = pack.locked.0;
                        info!("Changed difficulty: {}", pack.locked.0);
                    }

                    TimeUpdate(pack) => {
                        self.world_time = pack.world_age.0;
                        self.day_time = pack.day_time.0;
                    }

                    UpdateHealth(pack) => {
                        self.player.health = pack.health.0;
                        self.player.food = pack.food.0;
                        self.player.saturation = pack.saturation.0;
                    }

                    Disconnect(pack) => {
                        info!("Disconnected from server: {}", pack.reason.0);
                        self.disconnect = true;
                    }

                    LoginSuccess(pack) => {
                        info!("Successfully Logged in!");
                    }

                    JoinGame(id) => {
                        self.join_game(id.player_id.0);
                        self.send_packet(
                            ClientSettings(
                                MCString(self.player.locale.clone()),
                                Byte(self.player.view_distance),
                                VarInt(self.player.chat_mode),
                                Boolean(false),
                                UByte(self.player.displayed_skin_parts),
                                VarInt(self.player.main_hand),
                                Boolean(self.player.disable_text_filtering),
                            ),
                        );
                        self.send_packet(ClientStatusRespawn);
                    }

                    SpawnLivingEntity(pack) => {
                        match self.entities.insert(
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
                        self.entities.insert(
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
                            self.entities.remove(&eid.0);
                        }
                    }

                    EntityPosition(pack) => match self.entities.get_mut(&pack.entity_id.0) {
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
                        match self.entities.get_mut(&pack.entity_id.0) {
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

                    EntityRotation(pack) => match self.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.ori
                                .set(pack.yaw.0 as f32 / 256.0, pack.pitch.0 as f32 / 256.0);
                            ent.on_ground = pack.on_ground.0;
                        }
                        None => {}
                    },

                    EntityHeadLook(pack) => match self.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.ori_head.set(
                                pack.head_yaw.0 as f32 / 256.0,
                                ent.ori_head.get_head_pitch(),
                            );
                        }
                        None => {}
                    },

                    EntityVelocity(pack) => match self.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.vel.add_assign(Vec3::new(
                                pack.vx.0 as f32 / 8000.0,
                                pack.vy.0 as f32 / 8000.0,
                                pack.vz.0 as f32 / 8000.0,
                            ));
                        }
                        None => {}
                    },

                    EntityTeleport(pack) => match self.entities.get_mut(&pack.entity_id.0) {
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

                        self.player.set_position(Vec3::new(
                            pack.x.0 as f32,
                            pack.y.0 as f32,
                            pack.z.0 as f32,
                        ));
                        self
                            .player
                            .get_orientation_mut()
                            .set(pack.yaw.0 as f32, pack.pitch.0 as f32);

                        self.send_packet(TeleportConfirm(pack.teleport_id.clone()));

                        let px = self.player.get_position().x;
                        let py = self.player.get_position().y;
                        let pz = self.player.get_position().z;

                        self.send_packet(
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
                        self.chat.add_message(&chat);
                    }

                    ChunkData(cd) => {
                        self.world.insert_chunk(Chunk::new(&ctx.dis, &cd));
                    }

                    // Currently ignoring these packets
                    SoundEffect(_) | EntityMetadata(_) | EntityProperties(_) | EntityStatus(_)
                    | Effect(_) | EntityAnimation(_) => {
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

}

#[derive(Debug, Copy, Clone)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Medium,
    Hard,
}
