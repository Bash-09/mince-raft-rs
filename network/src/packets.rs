use std::{io::{Read, Write, Cursor}, error::Error};

use derive_packet::derive_packet;

use crate::types::*;

pub enum ServerState {
    Handshake,
    Status,
    Login,
    Play,
}

pub trait Packet {
    const ID: VarInt;
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>>;
}


#[derive(Debug, Clone)]
pub struct Unknown {
    pub bytes: Vec<u8>
}

impl Packet for Unknown {
    const ID: VarInt = VarInt(0);

    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let bytes: Vec<u8> = r.bytes().map(|f| f.unwrap()).collect();
        Ok(Unknown {bytes})
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        panic!("Shouldn't be writing an unknown packet!")
    }
}

#[derive(Debug, Clone)]
#[derive_packet(0x00)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub address: MCString,
    pub port: UShort,
    pub next: VarInt,
}


// ********** STATUS MODE ***********

#[derive(Debug, Clone)]
#[derive_packet(0x00)]
pub struct Request {} // Status request

#[derive(Debug, Clone)]
#[derive_packet(0x00)]
pub struct Response { // Status response
    pub json: MCString,
}

#[derive(Debug, Clone)]
#[derive_packet(0x00)]
pub struct TeleportConfirm { // Status response
    pub teleport_id: VarInt,
}

#[derive(Debug, Clone)]
#[derive_packet(0x01)]
pub struct Ping {
    pub payload: Long,
}

#[derive(Debug, Clone)]
#[derive_packet(0x01)]
pub struct ClientPong {
    pub payload: Long
}

// ************* LOGIN MODE **************

#[derive(Debug, Clone)]
#[derive_packet(0x00)]
pub struct LoginStart {
    pub name: MCString
}

#[derive(Debug, Clone)]
#[derive_packet(0x01)]
pub struct EncryptionRequest {
    pub server_id: MCString,
    pub public_key: Array<Byte>,
    pub verify_token: Array<Byte>,
}

#[derive(Debug, Clone)]
#[derive_packet(0x02)]
pub struct LoginSuccess {
    pub uuid: UUID,
    pub username: MCString,
}

#[derive(Debug, Clone)]
#[derive_packet(0x03)]
pub struct SetCompression {
    pub threshold: VarInt,
}

#[derive(Debug, Clone)]
#[derive_packet(0x04)]
pub struct LoginPluginRequest {
    pub message_id: VarInt,
    pub channel: Identifier,
    pub data: Array<Byte>,
}

// ****************** PLAY MODE ***********

#[derive(Debug, Clone)]
#[derive_packet(0x00)]
pub struct SpawnEntity {
    pub entity_id: VarInt,   // Entity ID
    pub uuid: UUID,          // UUID
    pub entity_type: VarInt, // Entity Type
    pub x: Double,           // X
    pub y: Double,           // Y
    pub z: Double,           // Z
    pub pitch: Angle,        // Pitch
    pub yaw: Angle,          // Yaw
    pub data: Int,           // Data
    pub vx: Short,           // VX
    pub vy: Short,           // VY
    pub vz: Short,           // VZ
}

#[derive(Debug, Clone)]
#[derive_packet(0x01)]
pub struct SpawnExperienceOrb {
    pub entity_id: VarInt, // Entity ID
    pub x: Double,         // X,
    pub y: Double,         // Y
    pub z: Double,         // Z
    pub amount: Short,     // XP Amount
}

#[derive(Debug, Clone)]
#[derive_packet(0x02)]
pub struct SpawnLivingEntity {
    pub entity_id: VarInt,   // Entity ID
    pub uuid: UUID,          // UUID
    pub entity_type: VarInt, // Entity Type
    pub x: Double,           // X
    pub y: Double,           // Y
    pub z: Double,           // Z
    pub yaw: Angle,          // Yaw
    pub pitch: Angle,        // Pitch
    pub head_pitch: Angle,   // Head Pitch
    pub vx: Short,           // Vel X
    pub vy: Short,           // Vel Y
    pub vz: Short,           // Vel Z
}

#[derive(Debug, Clone)]
#[derive_packet(0x03)]
pub struct SpawnPainting {
    pub entity_id: VarInt,       // Entity ID
    pub uuid: UUID,              // Entity UUID
    pub painting_id: VarInt,     // Motive, Painting's ID
    pub center_coords: Position, // Center Coordinates
    pub direction: Byte, // Enum, Painting direction (North = 2, South = 0, West = 1, East = 3)
}

#[derive(Debug, Clone)]
#[derive_packet(0x03)]
pub struct ChatMessageServerbound {
    pub message: MCString
}

#[derive(Debug, Clone)]
#[derive_packet(0x04)]
pub struct SpawnPlayer {
    pub entity_id: VarInt, // Entity ID
    pub uuid: UUID,        // Player UUID
    pub x: Double,         // X
    pub y: Double,         // Y
    pub z: Double,         // Z
    pub yaw: Angle,        // Yaw
    pub pitch: Angle,      // Pitch
}

#[derive(Debug, Clone)]
#[derive_packet(0x04)]
pub struct ClientStatus {
    pub action: VarInt
}

#[derive(Debug, Clone)]
#[derive_packet(0x05)]
pub struct SculkVibrationSignal {
// TODO
}

#[derive(Debug, Clone)]
#[derive_packet(0x05)]
pub struct ClientSettings {
    pub locale: MCString,
    pub view_distance: Byte,
    pub chat_mode: VarInt,
    pub chat_colors: Boolean,
    pub display_skin_parts: UByte,
    pub main_hand: VarInt,
    pub disable_text_filtering: Boolean,
}

#[derive(Debug, Clone)]
#[derive_packet(0x06)]
pub struct EntityAnimation {
    pub player_id: VarInt,   // Player ID
    pub animation_id: UByte, // Animation ID (0 = Swing Main Arm, 1 = Take Damage, 2 = Leave Bed, 3 = Swing Offhand, 4 = Critical Effect, 5 = Magic Critical Effect)
}

#[derive(Debug, Clone)]
pub struct Statistics {
    pub stats_len: VarInt, // Count of next array
    pub stats: Vec<(
        VarInt, // Enum, Category ID
        VarInt, // Enum, Statistic ID
        VarInt, // Amount to set it to
    )>,
}

impl Packet for Statistics {
    const ID: VarInt = VarInt(0x07);

    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        todo!()
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

#[derive(Debug, Clone)]
#[derive_packet(0x08)]
pub struct AcknowledgePlayerDigging {
    pub location: Position,           // Location
    pub block_state_id: VarInt,       // Block state ID
    pub player_digging_state: VarInt, // Enum, Player Digging state
    pub success: Boolean,             // Success
}

#[derive(Debug, Clone)]
#[derive_packet(0x09)]
pub struct BlockBreakAnimation {
    pub breaker_entity_id: VarInt, // Entity ID of Entity breaking the block
    pub block_pos: Position,       // Block Position
    pub destroy_stage: Byte,       // Destroy Stage {0-9 to set, any other value removes)
}

#[derive(Debug, Clone)]
#[derive_packet(0x0a)]
pub struct BlockEntityData {
    pub block_pos: Position, //
    pub update_type: UByte,  // Enum, Type of update
    pub data: NBTTag,        // Data to set {May be TAG_END{0) which means the block is removed)
}

#[derive(Debug, Clone)]
#[derive_packet(0x0b)]
pub struct BlockAction {
    pub block_pos: Position,   // Block Coords
    pub action_id: UByte,      // Action ID {Varies by block)
    pub action_param: UByte,   // Action Param
    pub block_type_id: VarInt, // Block type {Block type ID, not block state)
}

#[derive(Debug, Clone)]
#[derive_packet(0x0c)]
pub struct BlockChange {
    pub block_pos: Position,    // Block Coords
    pub block_state_id: VarInt, // Block ID, new block state ID as given in the global palette
}

#[derive(Debug, Clone)]
#[derive_packet(0x0d)]
pub struct BossBar {
    pub uuid: UUID, // UUID for this bar
    pub action: VarInt, // Enum, determines the layout for the rest of the packet
                    // TODO
}

#[derive(Debug, Clone)]
#[derive_packet(0x0e)]
pub struct ServerDifficulty {
    pub difficulty: UByte, // Difficulty, {0: PEaceful, 1: Easy, 2: Normal, 3: Hard)
    pub locked: Boolean,   // Difficulty Locked?
}

#[derive(Debug, Clone)]
#[derive_packet(0x0f)]
pub struct ChatIncoming {
    pub json: MCString, // JSON Data of chat message
    pub position: Byte, // Position, {0: Chat, 1: System Message, 2: Game Info)
    pub sender: UUID,   // Sender
}

#[derive(Debug, Clone)]
#[derive_packet(0x0f)]
pub struct KeepAliveServerbound {
    pub keep_alive_id: Long
}

#[derive(Debug, Clone)]
#[derive_packet(0x10)]
pub struct ClearTitles {
    pub reset: Boolean, // Reset
}

#[derive(Debug, Clone)]
#[derive_packet(0x10)]
pub struct PlayerPosition {
    pub x: Double,
    pub feet_t: Double, 
    pub z: Double,
    pub on_ground: Boolean
}

#[derive(Debug, Clone)]
pub struct TabComplete {
    pub transaction_id: VarInt, // Transaction ID
    pub start: VarInt,          // Start of text to replace
    pub len: VarInt,            // Length of text to replace
    pub matches_len: VarInt,    // Count of next array
    pub matches: Vec<(
        MCString,     // An elligible value to insert
        Boolean,      // Has Tooltip
        Option<Chat>, // Tooltip
    )>,
}

impl Packet for TabComplete {
    const ID: VarInt = VarInt(0x11);

    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        todo!()
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

#[derive(Debug, Clone)]
#[derive_packet(0x12)]
pub struct PlayerPositionAndRotation {
    pub x: Double,
    pub feet_y: Double,
    pub z: Double,
    pub yaw: Float,
    pub pitch: Float,
    pub on_ground: Boolean
}

#[derive(Debug, Clone)]
#[derive_packet(0x12)]
pub struct DeclareCommands {
// TODO
}

#[derive(Debug, Clone)]
#[derive_packet(0x13)]
pub struct CloseWindowClientbound {
    pub window_id: UByte, // ID of window to that was closed. 0 for inventory
}

#[derive(Debug, Clone)]
#[derive_packet(0x14)]
pub struct WindowItems {
    pub window_id: UByte,  // Window ID
    pub state_id: VarInt,  // State ID
    pub slots: Array<Slot>,  // List of slots
    pub carried: Slot,     // Carried Item / Item held by player
}

#[derive(Debug, Clone)]
#[derive_packet(0x15)]
pub struct WindowProperty {
    pub window_id: UByte, // Window ID
    pub property: Short,  // Enum, property to be updated
    pub value: Short,     // New value of property
}

#[derive(Debug, Clone)]
#[derive_packet(0x16)]
pub struct SetSlot {
    pub window_id: Byte,  // Window ID
    pub state_id: VarInt, // State ID
    pub slot_id: Short,   // Which slot to be updated
    pub slot_data: Slot,  // Slot Data
}

#[derive(Debug, Clone)]
#[derive_packet(0x17)]
pub struct SetCooldown {
    pub item_id: VarInt,        // ID of item to apply cooldown to
    pub cooldown_ticks: VarInt, // Num of ticks to apply cooldown for, or 0 to clear the cooldown
}

#[derive(Debug, Clone)]
pub struct PluginMessage {
    pub channel: Identifier, // Name of Plugin Channel used
    pub data: Vec<Byte>,     // Data for that channel
}

impl Packet for PluginMessage {
    const ID: VarInt = VarInt(0x18);

    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        todo!()
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

#[derive(Debug, Clone)]
#[derive_packet(0x19)]
pub struct NamedSoundEffect {
    pub sound_name: Identifier, // Sound Name
    pub category: VarInt,       // Enum, category to play sound from
    pub x: Int,                 // Effect Pos X,
    pub y: Int,                 // Effect Pos Y,
    pub z: Int,                 // Effect Pos Z,
    pub vol: Float,             // Volume, {1 = 100% but can be louder)
    pub pitch: Float,           // Pitch
}

#[derive(Debug, Clone)]
#[derive_packet(0x1a)]
pub struct Disconnect {
    pub reason: MCString
}

#[derive(Debug, Clone)]
#[derive_packet(0x1b)]
pub struct EntityStatus {
    // 0x1b
    pub entity_id: Int, // Entity ID
    pub status: Byte,   // Enum, Entity Status
}

#[derive(Debug, Clone)]
pub struct Explosion {
    pub x: Float,           // X
    pub y: Float,           // Y
    pub z: Float,           // Z
    pub strength: Float,    // Strength
    pub blocks_len: VarInt, // Count of next array
    pub block_offsets: Vec<(
        // X/Y/Z offsets of affected blocks
        Byte, // Blocks in this array are set to Air
        Byte,
        Byte,
    )>,
    pub vx: Float, // Vel X // Velocity of player being pushed by the explosion
    pub vy: Float, // Vel Y
    pub vz: Float, // Vel Z
}

impl Packet for Explosion {
    const ID: VarInt = VarInt(0x1c);

    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        todo!()
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

#[derive(Debug, Clone)]
#[derive_packet(0x1d)]
pub struct UnloadChunk {
    pub x: Int, // Chunk X
    pub z: Int, // Chunk Z
}

#[derive(Debug, Clone)]
#[derive_packet(0x1e)]
pub struct ChangeGameState {
    pub reason: UByte,
    pub value: Float,
}

#[derive(Debug, Clone)]
#[derive_packet(0x1f)]
pub struct OpenHorseWindow {
    pub window_id: Byte,
    pub num_slots: VarInt,
    pub entity_id: Int,
}

#[derive(Debug, Clone)]
#[derive_packet(0x20)]
pub struct InitializeWorldBorder {
    pub x: Double,
    pub z: Double,
    pub old_diameter: Double,
    pub new_diameter: Double,
    pub speed: VarLong, // Number of millis until new diameter is reached
    pub portal_teleport_boundary: VarInt,
    pub warning_blocks: VarInt,
    pub warning_time: VarInt,
}

#[derive(Debug, Clone)]
#[derive_packet(0x21)]
pub struct KeepAliveClientbound {
    pub keep_alive_id: Long,
}

#[derive(Debug, Clone)]
#[derive_packet(0x22)]
pub struct ChunkData {
    pub x: Int,
    pub z: Int,
    pub bit_mask: Array<Long>,
    pub heightmaps: NBTTag,
    pub biomes: Array<VarInt>,
    pub data: Array<UByte>,
    pub block_entities: Array<NBTTag>,
}

#[derive(Debug, Clone)]
pub struct UpdateLight {
    pub chunk_x: VarInt,
    pub chunk_z: VarInt,
    pub trust_edges: Boolean,
    pub sky_light_mask: Array<Long>,
    pub block_light_mask: Array<Long>,
    pub empty_sky_light_mask: Array<Long>,
    pub empty_block_light_mask: Array<Long>,
    pub sky_lights_len: VarInt,
    pub sky_lights: Vec<(VarInt, [Byte; 2048])>,
    pub block_lights_len: VarInt,
    pub block_lights: Vec<(VarInt, [Byte; 2048])>,
}

impl Packet for UpdateLight {
    const ID: VarInt = VarInt(25);

    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        todo!()
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

#[derive(Debug, Clone)]
#[derive_packet(0x26)]
pub struct JoinGame {
    pub player_id: Int,
    pub is_hardcore: Boolean,
    pub gamemode: UByte,
    pub prev_gamemode: Byte,
    pub world_names: Array<Identifier>,
    pub dimension_codec: NBTTag,
    pub dimension: NBTTag,
    pub world_name: Identifier,
    pub hashed_seed: Long,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub reduced_debug_info: Boolean,
    pub enable_respawn_screen: Boolean,
    pub is_debug: Boolean,
    pub is_flat: Boolean,
}

#[derive(Debug, Clone)]
#[derive_packet(0x29)]
pub struct EntityPosition {
    pub entity_id: VarInt,
    pub dx: Short,
    pub dy: Short,
    pub dz: Short,
    pub on_ground: Boolean,
}

#[derive(Debug, Clone)]
#[derive_packet(0x2a)]
pub struct EntityPositionAndRotation {
    pub entity_id: VarInt,
    pub dx: Short,
    pub dy: Short,
    pub dz: Short,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: Boolean,
}

#[derive(Debug, Clone)]
#[derive_packet(0x2b)]
pub struct EntityRotation {
    pub entity_id: VarInt,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: Boolean,
}

#[derive(Debug, Clone)]
#[derive_packet(0x38)]
pub struct PlayerPositionAndLook {
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub yaw: Float,
    pub pitch: Float,
    pub flags: Byte,
    pub teleport_id: VarInt,
    pub dismount: Boolean,
}

#[derive(Debug, Clone)]
#[derive_packet(0x3a)]
pub struct DestroyEntities {
    pub entities: Array<VarInt>,
}

#[derive(Debug, Clone)]
#[derive_packet(0x3e)]
pub struct EntityHeadLook {
    pub entity_id: VarInt,
    pub head_yaw: Angle,
}

#[derive(Debug, Clone)]
#[derive_packet(0x4d)]
pub struct EntityMetadata {
    pub entity_id: VarInt,
    // TODO - Metadata
}

#[derive(Debug, Clone)]
#[derive_packet(0x4f)]
pub struct EntityVelocity {
    pub entity_id: VarInt,
    pub vx: Short,
    pub vy: Short,
    pub vz: Short,
}

#[derive(Debug, Clone)]
#[derive_packet(0x52)]
pub struct UpdateHealth {
    pub health: Float,
    pub food: VarInt,
    pub saturation: Float,
}

#[derive(Debug, Clone)]
#[derive_packet(0x58)]
pub struct TimeUpdate {
    pub world_age: Long,
    pub day_time: Long,
}

#[derive(Debug, Clone)]
#[derive_packet(0x61)]
pub struct EntityTeleport {
    pub entity_id: VarInt,
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: Boolean,
}

#[derive(Debug, Clone)]
pub struct EntityProperties {
    pub entity_id: VarInt,
    pub num_properties: VarInt,
    pub properties: Vec<(
        // List of properties
        Identifier, // Key
        Double,     // Value
        VarInt,     // Num of Modifiers
        Vec<(
            // List of Modifier Data
            UUID,   // UUID
            Double, // Amount
            Byte,   // Operation
        )>,
    )>,
}

impl Packet for EntityProperties {
    const ID: VarInt = VarInt(0x63);

    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        todo!()
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum PacketData {
    Ok,
    Close,

    // ********************************************************
    // Serverbound ********************************************

    Handshake(Handshake),
    LoginStart(LoginStart),
    ClientStatus(ClientStatus),
    ClientSettings(ClientSettings),

    // X, Y, Z, onGround
    PlayerPosition(PlayerPosition),

    // X, Y, Z, Yaw, Pitch, OnGround
    PlayerPositionAndRotation(PlayerPositionAndRotation),

    KeepAliveServerbound(KeepAliveServerbound),

    ChatMessageServerbound(ChatMessageServerbound),

    TeleportConfirm(TeleportConfirm), // Teleport ID as given by PlayerPositionAndLook

    // ************************************************************
    // CLIENTBOUND *************************************CLIENTBOUND

    // ********************** LOGIN
    Response(Response),
    EncryptionRequest(EncryptionRequest),
    LoginSuccess(LoginSuccess),
    SetCompression(SetCompression),
    LoginPluginRequest(LoginPluginRequest),

    // ********************** STATUS
    Request(Request),
    ClientPong(ClientPong),

    // ********************** PLAY
    SpawnEntity(SpawnEntity),
    SpawnExperienceOrb(SpawnExperienceOrb),
    SpawnLivingEntity(SpawnLivingEntity),
    SpawnPainting(SpawnPainting),
    SpawnPlayer(SpawnPlayer),
    SculkVibrationSignal(SculkVibrationSignal),
    EntityAnimation(EntityAnimation),
    Statistics(Statistics),
    AcknowledgePlayerDigging(AcknowledgePlayerDigging),
    BlockBreakAnimation(BlockBreakAnimation),
    BlockEntityData(BlockEntityData),
    BlockAction(BlockAction),
    BlockChange(BlockChange),
    BossBar(BossBar),
    ServerDifficulty(ServerDifficulty),
    ChatIncoming(ChatIncoming),
    ClearTitles(ClearTitles),
    TabComplete(TabComplete),
    DeclareCommands(DeclareCommands),
    CloseWindowClientbound(CloseWindowClientbound),
    WindowItems(WindowItems),
    WindowProperty(WindowProperty),
    SetSlot(SetSlot),
    SetCooldown(SetCooldown),
    PluginMessage(PluginMessage),
    NamedSoundEffect(NamedSoundEffect),
    Disconnect(Disconnect),
    EntityStatus(EntityStatus),
    Explosion(Explosion),
    UnloadChunk(UnloadChunk),
    ChangeGameState(ChangeGameState),
    OpenHorseWindow(OpenHorseWindow),
    InitializeWorldBorder(InitializeWorldBorder),
    KeepAliveClientbound(KeepAliveClientbound),
    ChunkData(ChunkData),
    UpdateLight(UpdateLight),
    JoinGame(JoinGame),
    EntityPosition(EntityPosition),
    EntityPositionAndRotation(EntityPositionAndRotation),
    EntityRotation(EntityRotation),

    PlayerPositionAndLook(PlayerPositionAndLook),
    DestroyEntities(DestroyEntities),
    EntityHeadLook(EntityHeadLook),
    EntityMetadata(EntityMetadata),
    EntityVelocity(EntityVelocity),
    UpdateHealth(UpdateHealth),
    TimeUpdate(TimeUpdate),
    EntityTeleport(EntityTeleport),
    EntityProperties(EntityProperties),

    Unknown(Vec<u8>),
    Empty,
}

pub fn decode_packet_clientbound(packet: &[u8], state: &ServerState) -> Result<PacketData, Box<dyn std::error::Error>> {
    use crate::packets::*;

    if packet.len() ==0 {
        return Ok(PacketData::Empty);
    }

    let mut cur = Cursor::new(packet);
    
    let VarInt(id) = VarInt::read(&mut cur)?;

    return Ok(match state {
        ServerState::Status => { 
            match id {
                0x00 => PacketData::Response(Response::read(&mut cur)?),
                0x01 => PacketData::ClientPong(ClientPong::read(&mut cur)?),
                _ => PacketData::Unknown(packet.to_vec())
            }
        },
        ServerState::Login => {
            match id {
                0x00 => PacketData::Disconnect(Disconnect::read(&mut cur)?),
                0x01 => PacketData::EncryptionRequest(EncryptionRequest::read(&mut cur)?),
                0x02 => PacketData::LoginSuccess(LoginSuccess::read(&mut cur)?),
                0x03 => PacketData::SetCompression(SetCompression::read(&mut cur)?),
                0x04 => PacketData::LoginPluginRequest(LoginPluginRequest::read(&mut cur)?),
                _ => PacketData::Unknown(packet.to_vec())
            }
        },
        ServerState::Play => {
            match id {
                0x00 => PacketData::SpawnEntity(SpawnEntity::read(&mut cur)?),
                0x01 => PacketData::SpawnExperienceOrb(SpawnExperienceOrb::read(&mut cur)?),
                0x02 => PacketData::SpawnLivingEntity(SpawnLivingEntity::read(&mut cur)?),
                0x03 => PacketData::SpawnPainting(SpawnPainting::read(&mut cur)?),
                0x04 => PacketData::SpawnPlayer(SpawnPlayer::read(&mut cur)?),
                0x05 => PacketData::SculkVibrationSignal(SculkVibrationSignal::read(&mut cur)?),
                0x06 => PacketData::EntityAnimation(EntityAnimation::read(&mut cur)?),
                0x07 => PacketData::Statistics(Statistics::read(&mut cur)?),
                0x08 => PacketData::AcknowledgePlayerDigging(AcknowledgePlayerDigging::read(&mut cur)?),
                0x09 => PacketData::BlockBreakAnimation(BlockBreakAnimation::read(&mut cur)?),
                0x0a => PacketData::BlockEntityData(BlockEntityData::read(&mut cur)?),
                0x0b => PacketData::BlockAction(BlockAction::read(&mut cur)?),
                0x0c => PacketData::BlockChange(BlockChange::read(&mut cur)?),
                0x0e => PacketData::ServerDifficulty(ServerDifficulty::read(&mut cur)?),
                0x0f => PacketData::ChatIncoming(ChatIncoming::read(&mut cur)?),
                0x10 => PacketData::ClearTitles(ClearTitles::read(&mut cur)?),
                0x11 => PacketData::TabComplete(TabComplete::read(&mut cur)?),
                0x13 => PacketData::CloseWindowClientbound(CloseWindowClientbound::read(&mut cur)?),
                0x15 => PacketData::WindowProperty(WindowProperty::read(&mut cur)?),
                0x16 => PacketData::SetSlot(SetSlot::read(&mut cur)?),
                0x17 => PacketData::SetCooldown(SetCooldown::read(&mut cur)?),
                0x19 => PacketData::NamedSoundEffect(NamedSoundEffect::read(&mut cur)?),
                0x1a => PacketData::Disconnect(Disconnect::read(&mut cur)?),
                0x1b => PacketData::EntityStatus(EntityStatus::read(&mut cur)?),
                0x1c => PacketData::Explosion(Explosion::read(&mut cur)?),
                0x1d => PacketData::UnloadChunk(UnloadChunk::read(&mut cur)?),
                0x1e => PacketData::ChangeGameState(ChangeGameState::read(&mut cur)?),
                0x1f => PacketData::OpenHorseWindow(OpenHorseWindow::read(&mut cur)?),
                0x20 => PacketData::InitializeWorldBorder(InitializeWorldBorder::read(&mut cur)?),
                0x21 => PacketData::KeepAliveClientbound(KeepAliveClientbound::read(&mut cur)?),
                0x22 => PacketData::ChunkData(ChunkData::read(&mut cur)?),
                0x26 => PacketData::JoinGame(JoinGame::read(&mut cur)?),
                0x29 => PacketData::EntityPosition(EntityPosition::read(&mut cur)?),
                0x2a => PacketData::EntityPositionAndRotation(EntityPositionAndRotation::read(&mut cur)?),
                0x2b => PacketData::EntityRotation(EntityRotation::read(&mut cur)?),
                0x38 => PacketData::PlayerPositionAndLook(PlayerPositionAndLook::read(&mut cur)?),
                0x3a => PacketData::DestroyEntities(DestroyEntities::read(&mut cur)?),
                0x3e => PacketData::EntityHeadLook(EntityHeadLook::read(&mut cur)?),
                0x4d => PacketData::EntityMetadata(EntityMetadata::read(&mut cur)?),
                0x4f => PacketData::EntityVelocity(EntityVelocity::read(&mut cur)?),
                0x52 => PacketData::UpdateHealth(UpdateHealth::read(&mut cur)?),
                0x58 => PacketData::TimeUpdate(TimeUpdate::read(&mut cur)?),
                0x61 => PacketData::EntityTeleport(EntityTeleport::read(&mut cur)?),
                // 0x63 => PacketData::EntityProperties(EntityProperties::read(&mut cur)?),
                _ => PacketData::Unknown(packet.to_vec()),
            }
        }
        _ => {PacketData::Unknown(packet.to_vec())}
    })
}

pub fn encode<P: Packet>(packet: P) -> Vec<u8> {
    let mut out = Vec::new();

    packet.write(&mut out).unwrap();

    out
}