use std::{collections::HashMap, ops::AddAssign};

use egui_winit::winit::event::VirtualKeyCode;
use glam::{IVec2, IVec3, Vec3, Vec3Swizzles};
use glium_app::context::Context;
use lazy_static::__Deref;
use log::{debug, error, info, warn};
use mcproto_rs::{v1_16_3::{Difficulty, PlayClientSettingsSpec, PlayClientStatusSpec, PlayClientChatMessageSpec, ClientStatusAction, PlayTeleportConfirmSpec, PlayClientPlayerPositionAndRotationSpec, PlayerInfoAction, GameMode}, types::{self, EntityLocation, VarInt}, uuid::UUID4};

use crate::{
    network::{NetworkChannel, NetworkCommand, encode, PacketType},
    settings::Settings,
    world::{
        self,
        chunks::{self, Chunk, ChunkSection},
    }, WindowManager, gui::{pause_windows, info_windows, chat_windows}, resources::PLAYER_INDEX,
};

use self::remote_player::RemotePlayer;

use super::{chat::Chat, entities::Entity, player::Player, world::World};

pub mod remote_player;

pub struct Server {
    network_destination: String,
    pub network: NetworkChannel,

    input_state: InputState,

    world_time: i64,
    day_time: i64,

    player: Player,
    chat: Chat,

    world: World,

    entities: HashMap<i32, Entity>,
    players: HashMap<UUID4, RemotePlayer>,

    difficulty: Difficulty,
    difficulty_locked: bool,

    pub client_disconnect: bool,
    pub server_disconnect: bool,
    pub disconnect_reason: Option<String>,
}

/// The input state of the player.
/// `Playing` - Normal fps input where the mouse and keyboard control the player
/// `Paused` - Paused menu is visible, mouse and keyboard are visible and interact with ui
/// `ShowingInfo` - Similar to Playing but also showing a handful of debug and other useful info,
/// clicking will transition to `InteractingInfo`
/// `InteractingInfo` - Debug and other useful info is visible, mouse is visible and can interact
/// with the info windows
/// `ChatOpen` - Chat is visible and interactable, mouse is visible and can scroll through the chat
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum InputState {
    Playing,
    Paused,
    ShowingInfo,
    InteractingInfo,
    ChatOpen
}

impl Server {
    pub fn new(network_destination: String, network: NetworkChannel) -> Server {
        Server {
            network_destination,
            network,

            input_state: InputState::Playing,

            world_time: 0,
            day_time: 0,

            player: Player::new(),
            chat: Chat::new(),

            world: World::new(),

            entities: HashMap::new(),
            players: HashMap::new(),

            difficulty: Difficulty::Easy,
            difficulty_locked: false,

            client_disconnect: false,
            server_disconnect: false,
            disconnect_reason: None,
        }
    }

    pub fn get_network_destination(&self) -> &str {
        &self.network_destination
    }

    pub fn get_input_state(&self) -> InputState {
        self.input_state
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

    pub fn get_chat_mut(&mut self) -> &mut Chat {
        &mut self.chat
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
        self.input_state == InputState::Paused
    }

    pub fn set_input_state(&mut self, state: InputState) {
        self.input_state = state;
    }

    pub fn join_game(&mut self, player_id: i32) {
        self.player.id = player_id;
    }

    pub fn get_players(&self) -> &HashMap<UUID4, RemotePlayer> {
        &self.players
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

    pub fn should_grab_mouse(&self) -> bool {
        match self.input_state {
            InputState::Playing => true,
            InputState::Paused => false,
            InputState::ShowingInfo => true,
            InputState::InteractingInfo => false,
            InputState::ChatOpen => false,
        }
    }

    pub fn render(&mut self, gui_ctx: &egui::Context, windows: &mut WindowManager) {
        if self.input_state != InputState::ChatOpen {
            chat_windows::render_inactive(self, gui_ctx);
        }

        match self.input_state {
            InputState::Playing => {},
            InputState::Paused => {
                match pause_windows::render(gui_ctx, windows) {
                    pause_windows::PauseAction::Disconnect => self.disconnect(),
                    pause_windows::PauseAction::Unpause => self.set_input_state(InputState::Playing),
                    pause_windows::PauseAction::Nothing => {},
                }
            },
            InputState::ShowingInfo  | InputState::InteractingInfo => info_windows::render(gui_ctx, self),
            InputState::ChatOpen => chat_windows::render_active(self, gui_ctx),
        }
    }

    pub fn update(&mut self, ctx: &Context, delta: f32, settings: &mut Settings) {
        for ent in self.entities.values_mut() {
            ent.update(delta);
        }

        match self.input_state {
            InputState::Playing => self.handle_playing_state(ctx, delta, settings),
            InputState::Paused => self.handle_paused_state(ctx, delta, settings),
            InputState::ShowingInfo => self.handle_show_info_state(ctx, delta, settings),
            InputState::InteractingInfo => self.handle_interact_info_state(ctx, delta, settings),
            InputState::ChatOpen => self.handle_chat_open_state(ctx, delta, settings),
        }

        // Handle messages from the NetworkManager
        loop {
            match self.network.recv.try_recv() {
                Ok(comm) => self.handle_message(comm, ctx),
                Err(e) => match e {
                    std::sync::mpsc::TryRecvError::Empty => break,
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        log::error!("Could not communicate with server. Assuming disconnected.");
                        self.server_disconnect = true;
                        if self.disconnect_reason.is_none() {
                            self.disconnect_reason = Some(String::from("Server forced disconnect. (You were probably sending too many connection requests)"));
                        }
                        return;
                    },
                },
            }
        }
    }

    fn handle_playing_state(&mut self, ctx: &Context, delta: f32, settings: &mut Settings) {
        if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::Escape) {
            self.input_state = InputState::Paused;
        } else if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::T) {
            self.input_state = InputState::ChatOpen;
        } else if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::Slash) {
            self.input_state = InputState::ChatOpen;
            self.chat.set_current_message(String::from("/"));
        } else if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::Tab) {
            self.input_state = InputState::ShowingInfo;
        }

        self.handle_keyboard_movement(ctx, delta, settings);
        self.handle_mouse_movement(ctx, delta, settings);
    }

    fn handle_paused_state(&mut self, ctx: &Context, delta: f32, settings: &mut Settings) {
        if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::Escape) {
            self.input_state = InputState::Playing;
        }

    }

    fn handle_show_info_state(&mut self, ctx: &Context, delta: f32, settings: &mut Settings) {
        if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::Escape) {
            self.input_state = InputState::Paused;
        } else if ctx.mouse.pressed_this_frame(0) {
            self.input_state = InputState::InteractingInfo;
        } else if ctx.keyboard.released_this_frame(&VirtualKeyCode::Tab) {
            self.input_state = InputState::Playing;
        }

        self.handle_keyboard_movement(ctx, delta, settings);
        self.handle_mouse_movement(ctx, delta, settings);
    }

    fn handle_interact_info_state(&mut self, ctx: &Context, delta: f32, settings: &mut Settings) {
        if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::Escape) {
            self.input_state = InputState::Paused;
        } else if ctx.keyboard.released_this_frame(&VirtualKeyCode::Tab) {
            self.input_state = InputState::Playing;
        }

        self.handle_keyboard_movement(ctx, delta, settings);
    }

    fn handle_chat_open_state(&mut self, ctx: &Context, delta: f32, settings: &mut Settings) {
        if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::Escape) {
            self.input_state = InputState::Playing;
        } else if ctx.keyboard.pressed_this_frame(&VirtualKeyCode::Return) {
            let text = self.chat.get_current_message_and_clear();
            if !text.is_empty() {
                self.send_packet(encode(PacketType::PlayClientChatMessage(PlayClientChatMessageSpec{ message: text })));
            }
            self.input_state = InputState::Playing;
        }
    }

    pub fn handle_mouse_movement(&mut self, ctx: &Context, delta: f32, settings: &mut Settings) {
        let off = ctx.mouse.get_delta();
        self.player.get_orientation_mut().rotate(
            off.0 as f32 * 0.05 * settings.mouse_sensitivity,
            off.1 as f32 * 0.05 * settings.mouse_sensitivity,
        );
    }

    pub fn handle_keyboard_movement(&mut self, ctx: &Context, delta: f32, settings: &mut Settings) {
        let vel = 14.0 * delta;

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
    }


    pub fn disconnect(&mut self) {
        info!("Disconnecting from server.");
        self.network
            .send
            .send(NetworkCommand::Disconnect)
            .expect("Failed to send message to network thread.");
        self.client_disconnect = true;
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
                        self.server_disconnect = true;
                    }

                    PacketType::LoginSuccess(pack) => {
                        info!("Successfully Logged in!");
                    }

                    PacketType::LoginDisconnect(pack) => {
                        info!("Disconnected during login");
                        self.server_disconnect = true;
                        self.disconnect_reason = pack.message.to_traditional();
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

                    PacketType::PlaySpawnPlayer(pack) => {
                        self.entities.insert(
                            pack.entity_id.0, 
                            Entity::new_with_values(
                                pack.entity_id.0, 
                                pack.uuid, 
                                PLAYER_INDEX as u32, 
                                0, 
                                pack.location.position.x as f32, 
                                pack.location.position.y as f32, 
                                pack.location.position.z as f32, 
                                pack.location.rotation.yaw.value as f32 / 255.0, 
                                pack.location.rotation.pitch.value  as f32 / 255.0, 
                                pack.location.rotation.pitch.value  as f32 / 255.0, 
                                0.0, 0.0, 0.0)
                        );
                    }

                    PacketType::PlaySpawnLivingEntity(pack) => {
                        match self.entities.insert(
                            pack.entity_id.0,
                            Entity::new_with_values(
                                pack.entity_id.0,
                                pack.entity_uuid,
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
                                pack.object_uuid,
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
                            teleport_id: pack.teleport_id,
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
                        self.chat.add_message(chat, self.world_time);
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

                    PacketType::PlayPlayerInfo(pack) => {
                        use mcproto_rs::v1_16_3::PlayerInfoActionList;
                        match pack.actions {
                            PlayerInfoActionList::Add(players) => {
                                for player in players.iter() {
                                    self.players.insert(player.uuid, RemotePlayer { 
                                        uuid: player.uuid, 
                                        name: player.action.name.clone(), 
                                        gamemode: player.action.game_mode.clone(), 
                                        ping: player.action.ping_ms.0, 
                                        display_name:  player.action.display_name.clone().map(|dn| dn.to_traditional()).unwrap_or(None)
                                    });
                                }
                            },
                            PlayerInfoActionList::UpdateGameMode(players) => {
                                let players: Vec<PlayerInfoAction<GameMode>> = From::from(players);
                                for player in players {
                                    if let Some(p) = self.players.get_mut(&player.uuid) {
                                        p.gamemode = player.action;
                                    }
                                }
                            },
                            PlayerInfoActionList::UpdateLatency(players) => {
                                let players: Vec<PlayerInfoAction<VarInt>> = From::from(players);
                                for player in players {
                                    if let Some(p) = self.players.get_mut(&player.uuid) {
                                        p.ping = player.action.into();
                                    }
                                }
                            },
                            PlayerInfoActionList::UpdateDisplayName(players) => {
                                for player in players.iter() {
                                    if let Some(p) = self.players.get_mut(&player.uuid) {
                                        p.display_name = player.action.clone().map(|chat| chat.to_traditional().unwrap_or_else(|| "Failed to parse name".to_string()));
                                    }
                                }
                            },
                            PlayerInfoActionList::Remove(players) => {
                                for player in players.iter() {
                                    self.players.remove(player);
                                }
                            },
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
