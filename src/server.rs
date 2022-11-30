use std::{collections::HashMap, ops::AddAssign};

use egui_winit::winit::event::VirtualKeyCode;
use glam::{IVec2, IVec3, Vec3, Vec3Swizzles};
use glium_app::context::Context;
use lazy_static::__Deref;
use log::{debug, error, info, warn};
use mcproto_rs::{v1_16_3::{Difficulty, PlayClientSettingsSpec, PlayClientStatusSpec, PlayClientChatMessageSpec, ClientStatusAction, PlayTeleportConfirmSpec, PlayClientPlayerPositionAndRotationSpec}, types::{self, EntityLocation}};

use crate::{
    network::{NetworkChannel, NetworkCommand, encode, PacketType},
    settings::Settings,
    world::{
        self,
        chunks::{self, Chunk, ChunkSection},
    },
};

use super::{chat::Chat, entities::Entity, player::Player, world::World};

pub struct Server {
    network_destination: String,
    pub network: NetworkChannel,

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
    pub disconnect_reason: Option<String>,
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
            disconnect_reason: None,
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
        self.difficulty.clone()
    }

    pub fn is_difficulty_locked(&self) -> bool {
        self.difficulty_locked
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn join_game(&mut self, player_id: i32) {
        self.player.id = player_id;
    }

    /// Attempts to send a packet over the provided (possible) network channel
    pub fn send_packet(&self, packet: Vec<u8>) -> Option<()> {
        match self.network.send.send(NetworkCommand::SendPacket(packet)) {
            Ok(_) => Some(()),
            Err(e) => {
                error!("Failed to communicate with network commander: {:?}", e);
                panic!("Disconnected");
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
            }
        }
    }

    pub fn update(&mut self, ctx: &Context, delta: f32, settings: &mut Settings) {
        for ent in self.entities.values_mut() {
            ent.update(delta);
        }

        if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::Escape) {
            self.set_paused(!self.paused);
        }

        // Send chat message
        if self.chat.send {
            let text = self.chat.get_message_and_clear();
            self.chat.send = false;

            self.send_packet(encode(PacketType::PlayClientChatMessage(PlayClientChatMessageSpec{ message: text })));
        }

        self.handle_movement(ctx, delta, settings);

        // Handle messages from the NetworkManager
        while let Ok(comm) = self.network.recv.try_recv() {
            self.handle_message(comm, ctx);
        }
    }

    pub fn handle_movement(&mut self, ctx: &Context, delta: f32, settings: &mut Settings) {
        let vel = 14.0 * delta;

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
                dir.y = dir.x; // Just using this value as temp to swap x and z
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
                dir.y = dir.x; // Just using this value as temp to swap x and z
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
            self.player.get_orientation_mut().rotate(
                off.0 as f32 * 0.05 * settings.mouse_sensitivity,
                off.1 as f32 * 0.05 * settings.mouse_sensitivity,
            );
        }
    }


    pub fn disconnect(&mut self) {
        info!("Disconnecting from server.");
        self.network
            .send
            .send(NetworkCommand::Disconnect)
            .expect("Failed to send message to network thread.");
    }

    /// Handles a message from the NetworkManager
    fn handle_message(&mut self, comm: NetworkCommand, ctx: &Context) {
        use NetworkCommand::*;

        match comm {
            // Handles any incoming packets
            ReceivePacket(packet) => {
                match packet {
                    PacketType::PlayServerDifficulty(pack) => {
                        self.difficulty = pack.difficulty;
                        self.difficulty_locked = pack.locked;
                        info!("Changed difficulty: {}", pack.locked);
                    }

                    PacketType::PlayTimeUpdate(pack) => {
                        self.world_time = pack.world_age;
                        self.day_time = pack.time_of_day;
                    }

                    PacketType::PlayUpdatehealth(pack) => {
                        self.player.health = pack.health;
                        self.player.food = pack.food.0;
                        self.player.saturation = pack.saturation;
                    }

                    PacketType::PlayDisconnect(pack) => {
                        self.disconnect_reason = pack.reason.to_traditional();
                        info!("Disconnected from server: {:?}", self.disconnect_reason);
                        self.disconnect = true;
                    }

                    PacketType::LoginSuccess(pack) => {
                        info!("Successfully Logged in!");
                    }

                    PacketType::PlayJoinGame(id) => {
                        self.join_game(id.entity_id);
                        self.send_packet(encode(PacketType::PlayClientSettings(PlayClientSettingsSpec {
                            locale: self.player.locale.clone(),
                            view_distance: (self.player.view_distance),
                            chat_mode: self.player.chat_mode.clone(),
                            chat_colors: (false),
                            displayed_skin_parts: self.player.displayed_skin_parts,
                            main_hand: self.player.main_hand.clone(),
                        })));
                        self.send_packet(encode(PacketType::PlayClientStatus(PlayClientStatusSpec { action: ClientStatusAction::PerformRespawn })));
                    }

                    PacketType::PlaySpawnLivingEntity(pack) => {
                        match self.entities.insert(
                            pack.entity_id.0,
                            Entity::new_with_values(
                                pack.entity_id.0,
                                pack.entity_uuid.clone(),
                                pack.entity_type.0 as u32,
                                0,
                                pack.location.position.x as f32,
                                pack.location.position.y as f32,
                                pack.location.position.z as f32,
                                pack.location.rotation.yaw.value as f32 / 255.0,
                                pack.location.rotation.pitch.value as f32 / 255.0,
                                pack.head_pitch.value as f32 / 255.0,
                                pack.velocity.x as f32 / 400.0,
                                pack.velocity.y as f32 / 400.0,
                                pack.velocity.z as f32 / 400.0,
                            ),
                        ) {
                            Some(_) => {}
                            None => {}
                        }
                    }

                    PacketType::PlaySpawnEntity(pack) => {
                        self.entities.insert(
                            pack.entity_id.0,
                            Entity::new_with_values(
                                pack.entity_id.0,
                                pack.object_uuid.clone(),
                                pack.entity_type.0 as u32,
                                pack.data,
                                pack.position.x as f32,
                                pack.position.y as f32,
                                pack.position.z as f32,
                                pack.yaw.value as f32 / 255.0,
                                pack.pitch.value as f32 / 255.0,
                                0.0,
                                pack.velocity.x as f32 / 400.0,
                                pack.velocity.y as f32 / 400.0,
                                pack.velocity.z as f32 / 400.0,
                            ),
                        );
                    }

                    PacketType::PlayDestroyEntities(pack) => {
                        for eid in pack.entity_ids.deref() {
                            self.entities.remove(&eid.0);
                        }
                    }

                    PacketType::PlayEntityPosition(pack) => match self.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            let new_pos = ent.last_pos
                                + Vec3::new(
                                    (pack.delta.x as f32) / 4096.0,
                                    (pack.delta.y as f32) / 4096.0,
                                    (pack.delta.z as f32) / 4096.0,
                                );
                            ent.pos = new_pos;
                            ent.last_pos = new_pos;
                        }
                        None => {}
                    },

                    PacketType::PlayEntityPositionAndRotation(pack) => {
                        match self.entities.get_mut(&pack.entity_id.0) {
                            Some(ent) => {
                                let new_pos = ent.last_pos
                                    + Vec3::new(
                                        (pack.delta.position.x as f32) / 4096.0,
                                        (pack.delta.position.y as f32) / 4096.0,
                                        (pack.delta.position.z as f32) / 4096.0,
                                    );
                                ent.pos = new_pos;
                                ent.last_pos = new_pos;
                                ent.ori
                                    .set(pack.delta.rotation.yaw.value as f32 / 256.0, pack.delta.rotation.pitch.value as f32 / 256.0);
                                ent.on_ground = pack.on_ground;
                            }
                            None => {}
                        }
                    }

                    PacketType::PlayEntityRotation(pack) => match self.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.ori
                                .set(pack.rotation.yaw.value as f32 / 256.0, pack.rotation.pitch.value as f32 / 256.0);
                            ent.on_ground = pack.on_ground;
                        }
                        None => {}
                    },

                    PacketType::PlayEntityHeadLook(pack) => match self.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.ori_head
                                .set(pack.head_yaw.value as f32 / 256.0, ent.ori_head.get_head_pitch());
                        }
                        None => {}
                    },

                    PacketType::PlayEntityVelocity(pack) => match self.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.vel = Vec3::new(
                                pack.velocity.x as f32 / 400.0,
                                pack.velocity.y as f32 / 400.0,
                                pack.velocity.z as f32 / 400.0,
                            );
                        }
                        None => {}
                    },

                    PacketType::PlayEntityTeleport(pack) => match self.entities.get_mut(&pack.entity_id.0) {
                        Some(ent) => {
                            ent.pos = Vec3::new(pack.location.position.x as f32, pack.location.position.y as f32, pack.location.position.z as f32);
                            ent.ori
                                .set(pack.location.rotation.yaw.value as f32 / 256.0, pack.location.rotation.pitch.value as f32 / 256.0);
                            ent.on_ground = pack.on_ground;
                        }
                        None => {}
                    },

                    PacketType::PlayServerPlayerPositionAndLook(pack) => {
                        debug!("Player position updated!");

                        self.player.set_position(Vec3::new(
                            pack.location.position.x as f32,
                            pack.location.position.y as f32,
                            pack.location.position.z as f32,
                        ));
                        self.player
                            .get_orientation_mut()
                            .set(pack.location.rotation.yaw, pack.location.rotation.pitch);

                        self.send_packet(encode(PacketType::PlayTeleportConfirm(PlayTeleportConfirmSpec {
                            teleport_id: pack.teleport_id.clone(),
                        })));

                        let px = self.player.get_position().x;
                        let py = self.player.get_position().y;
                        let pz = self.player.get_position().z;

                        self.send_packet(encode(PacketType::PlayClientPlayerPositionAndRotation(PlayClientPlayerPositionAndRotationSpec {
                            on_ground: (true),
                            feet_location: EntityLocation {
                                position: types::Vec3{x: px as f64, y: py as f64, z: pz as f64},
                                rotation: pack.location.rotation,
                            },
                        })));
                    }

                    PacketType::PlayServerChatMessage(chat) => {
                        self.chat.add_message(chat);
                    }

                    PacketType::PlayChunkData(cd) => {
                        self.world.insert_chunk(&ctx.dis, Chunk::new(&ctx.dis, &cd.data));
                    }

                    PacketType::PlayUnloadChunk(pack) => {
                        self.world
                            .get_chunks_mut()
                            .remove(&IVec2::new(pack.position.x, pack.position.z));
                    }

                    PacketType::PlayBlockChange(pack) => {
                        let coords =
                            IVec3::new(pack.location.x, pack.location.y as i32, pack.location.z);
                        let local_coords = world::local_chunk_section_coords(&coords);
                        let chunk_coords = world::chunk_section_at_coords(&coords);

                        if let Some(chunk) = self.world.get_chunks_mut().get_mut(&chunk_coords.xz())
                        {
                            if chunk.sections[chunk_coords.y as usize].is_none() {
                                chunk.sections[chunk_coords.y as usize] =
                                    Some(ChunkSection::new(chunk_coords.y, [0; 4096]));
                            }

                            if let Some(chunk_section) =
                                &mut chunk.sections[chunk_coords.y as usize]
                            {
                                chunk_section.blocks[chunks::vec_to_index(&local_coords)] =
                                    pack.block_id.0 as u16;
                                self.world.regenerate_chunk_section(&ctx.dis, chunk_coords);

                                if local_coords.x == 0 {
                                    self.world.regenerate_chunk_section(
                                        &ctx.dis,
                                        IVec3::new(
                                            chunk_coords.x - 1,
                                            chunk_coords.y,
                                            chunk_coords.z,
                                        ),
                                    );
                                }
                                if local_coords.x == 15 {
                                    self.world.regenerate_chunk_section(
                                        &ctx.dis,
                                        IVec3::new(
                                            chunk_coords.x + 1,
                                            chunk_coords.y,
                                            chunk_coords.z,
                                        ),
                                    );
                                }
                                if local_coords.y == 0 {
                                    self.world.regenerate_chunk_section(
                                        &ctx.dis,
                                        IVec3::new(
                                            chunk_coords.x,
                                            chunk_coords.y - 1,
                                            chunk_coords.z,
                                        ),
                                    );
                                }
                                if local_coords.y == 15 {
                                    self.world.regenerate_chunk_section(
                                        &ctx.dis,
                                        IVec3::new(
                                            chunk_coords.x,
                                            chunk_coords.y + 1,
                                            chunk_coords.z,
                                        ),
                                    );
                                }
                                if local_coords.z == 0 {
                                    self.world.regenerate_chunk_section(
                                        &ctx.dis,
                                        IVec3::new(
                                            chunk_coords.x,
                                            chunk_coords.y,
                                            chunk_coords.z - 1,
                                        ),
                                    );
                                }
                                if local_coords.z == 15 {
                                    self.world.regenerate_chunk_section(
                                        &ctx.dis,
                                        IVec3::new(
                                            chunk_coords.x,
                                            chunk_coords.y,
                                            chunk_coords.z + 1,
                                        ),
                                    );
                                }
                            } else {
                                error!("Block update in empty chunk section");
                            }
                        } else {
                            warn!("Block update in unloaded chunk");
                        }
                    }

                    // Currently ignoring these packets
                    PacketType::PlayEntityMetadata(_) | PacketType::PlayEntityProperties(_) | PacketType::PlayEntityStatus(_)
                    | PacketType::PlayEntityAnimation(_) => {}

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
