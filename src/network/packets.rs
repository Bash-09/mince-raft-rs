#![allow(dead_code)]

use std::{borrow::Borrow, io::{Cursor, Error}};


use quartz_nbt::io;

use crate::{app::client::server::ServerState, network::packets};

use super::types::{*, self};

#[derive(Debug)]
pub enum HandshakeMode {
    Status,
    Login,
}

pub trait ClientboundPacket {
    fn decode(packet: &Vec<u8>, pd: &PacketDecoder, state: ServerState) -> Self;
    const ID: u8;
}

pub trait ServerboundPacket {
    fn encode(&self) -> Packet;
    const ID: u8;
}


// **************** CLIENTBOUND PACKETS ******************

// ********* LOGIN MODE ***********

#[derive(Debug)]
pub struct Status{ // 0x00
    pub response: MCString,
}

// ********* PLAYER MODE***********

#[derive(Debug)]
pub struct SpawnEntity{ // 0x00
    pub entity_id: VarInt, // Entity ID
    pub uuid: UUID,   // UUID
    pub entity_type: VarInt, // Entity Type
    pub x: Double, // X
    pub y: Double, // Y
    pub z: Double, // Z
    pub pitch: Angle,  // Pitch
    pub yaw: Angle,  // Yaw
    pub data: Int,    // Data
    pub vx: Short,  // VX
    pub vy: Short,  // VY
    pub vz: Short,  // VZ
}

#[derive(Debug)]
pub struct SpawnExperienceOrb{ // 0x01
    pub entity_id: VarInt, // Entity ID
    pub x: Double, // X,
    pub y: Double, // Y
    pub z: Double, // Z
    pub amount: Short,  // XP Amount
}

#[derive(Debug)]
pub struct SpawnLivingEntity{ // 0x02
    pub entity_id: VarInt, // Entity ID
    pub uuid: UUID,   // UUID
    pub entity_type: VarInt, // Entity Type
    pub x: Double, // X
    pub y: Double, // Y
    pub z: Double, // Z
    pub yaw: Angle,  // Yaw
    pub pitch: Angle,  // Pitch
    pub head_pitch: Angle,  // Head Pitch
    pub vx: Short,  // Vel X
    pub vy: Short,  // Vel Y
    pub vz: Short,  // Vel Z
}

#[derive(Debug)]
pub struct SpawnPainting{ // 0x03
    pub entity_id: VarInt, // Entity ID
    pub uuid: UUID,   // Entity UUID
    pub painting_id: VarInt, // Motive, Painting's ID
    pub center_coords: Position, // Center Coordinates
    pub direction: Byte,   // Enum, Painting direction (North = 2, South = 0, West = 1, East = 3)
}

#[derive(Debug)]
pub struct SpawnPlayer{ // 0x04
    pub entity_id: VarInt, // Entity ID
    pub uuid: UUID,   // Player UUID
    pub x: Double, // X
    pub y: Double, // Y
    pub z: Double, // Z
    pub yaw: Angle,  // Yaw
    pub pitch: Angle,  // Pitch
}

#[derive(Debug)]
pub struct SculkVibrationSignal{ // 0x05
    // TODO
}

#[derive(Debug)]
pub struct EntityAnimation{ // 0x06
    pub player_id: VarInt, // Player ID
    pub animation_id: UByte,  // Animation ID (0 = Swing Main Arm, 1 = Take Damage, 2 = Leave Bed, 3 = Swing Offhand, 4 = Critical Effect, 5 = Magic Critical Effect)
}

#[derive(Debug)]
pub struct Statistics{ // 0x07
    pub stats_len: VarInt, // Count of next array
    pub stats: Vec<(
        VarInt, // Enum, Category ID
        VarInt, // Enum, Statistic ID
        VarInt, // Amount to set it to
    )>,
}

#[derive(Debug)]
pub struct AcknowledgePlayerDigging{ // 0x08
    pub location: Position,   // Location
    pub block_state_id: VarInt,     // Block state ID
    pub player_digging_state: VarInt,     // Enum, Player Digging state
    pub success: Boolean,    // Success
}

#[derive(Debug)]
pub struct BlockBreakAnimation{ // 0x09
    pub breaker_entity_id: VarInt,     // Entity ID of Entity breaking the block
    pub block_pos: Position,   // Block Position
    pub destroy_stage: Byte,       // Destroy Stage {0-9 to set, any other value removes)
}

#[derive(Debug)]
pub struct BlockEntityData{ // 0x0a
    pub block_pos: Position,   //
    pub update_type: UByte,      // Enum, Type of update
    pub data: NBTTag,     // Data to set {May be TAG_END{0) which means the block is removed)
}

#[derive(Debug)]
pub struct BlockAction{ // 0x0b
    pub block_pos: Position,   // Block Coords
    pub action_id: UByte,      // Action ID {Varies by block)
    pub action_param: UByte,      // Action Param
    pub block_type_id: VarInt,     // Block type {Block type ID, not block state)
}

#[derive(Debug)]
pub struct BlockChange{ // 0x0c
    pub block_pos: Position,   // Block Coords
    pub block_state_id: VarInt,     // Block ID, new block state ID as given in the global palette
}

#[derive(Debug)]
pub struct BossBar{ // 0x0d
    pub uuid: UUID,       // UUID for this bar
    pub action: VarInt,     // Enum, determines the layout for the rest of the packet
    // TODO
}

#[derive(Debug)]
pub struct ServerDifficulty{ // 0x0e
    pub difficulty: UByte,      // Difficulty, {0: PEaceful, 1: Easy, 2: Normal, 3: Hard)
    pub locked: Boolean,    // Difficulty Locked?
}

#[derive(Debug)]
pub struct ChatIncoming{ // 0x0f
    pub json: MCString,   // JSON Data of chat message
    pub position: Byte,       // Position, {0: Chat, 1: System Message, 2: Game Info)
    pub sender: UUID,       // Sender
}

#[derive(Debug)]
pub struct ClearTitles{ // 0x10
    pub reset: Boolean, // Reset
}

#[derive(Debug)]
pub struct TabComplete{ // 0x11
    pub transaction_id: VarInt,     // Transaction ID
    pub start: VarInt,     // Start of text to replace
    pub len: VarInt,     // Length of text to replace
    pub matches_len: VarInt,     // Count of next array
    pub matches: Vec<(
        MCString,   // An elligible value to insert
        Boolean,    // Has Tooltip
        Option<Chat>, // Tooltip
    )>,
}

#[derive(Debug)]
pub struct DeclareCommands{ // 0x12
    // TODO
}

#[derive(Debug)]
pub struct CloseWindowClientbound{ // 0x13
    pub window_id: UByte,  // ID of window to that was closed. 0 for inventory
}

#[derive(Debug)]
pub struct WindowItems{ // 0x14
    pub window_id: UByte,  // Window ID
    pub state_id: VarInt, // State ID
    pub slots_len: VarInt, // Count of next array
    pub slots: Vec<Slot>,  // List of slots
    pub carried: Slot,   // Carried Item / Item held by player
}

#[derive(Debug)]
pub struct WindowProperty{ // 0x15
    pub window_id: UByte,  // Window ID
    pub property: Short,  // Enum, property to be updated
    pub value: Short,  // New value of property
}

#[derive(Debug)]
pub struct SetSlot{ // 0x16
    pub window_id: Byte,   // Window ID
    pub state_id: VarInt, // State ID
    pub slot_id: Short,  // Which slot to be updated
    pub slot_data: Slot,   // Slot Data
}

#[derive(Debug)]
pub struct SetCooldown{ // 0x17
    pub item_id: VarInt, // ID of item to apply cooldown to
    pub cooldown_ticks: VarInt, // Num of ticks to apply cooldown for, or 0 to clear the cooldown
}

#[derive(Debug)]
pub struct PluginMessage{ // 0x18
    pub channel: Identifier, // Name of Plugin Channel used
    pub data: Vec<Byte>,  // Data for that channel
}

#[derive(Debug)]
pub struct NamedSoundEffect{ // 0x19
    pub sound_name: Identifier, // Sound Name
    pub category: VarInt,     // Enum, category to play sound from
    pub x: Int,    // Effect Pos X,
    pub y: Int,    // Effect Pos Y,
    pub z: Int,    // Effect Pos Z,
    pub vol: Float,  // Volume, {1 = 100% but can be louder)
    pub pitch: Float,  // Pitch
}

#[derive(Debug)]
pub struct Disconnect{ // 0x1a
    pub reason: MCString,   // Disconnect reason
}

#[derive(Debug)]
pub struct EntityStatus{ // 0x1b
    pub entity_id: Int,    // Entity ID
    pub status: Byte,   // Enum, Entity Status
}

#[derive(Debug)]
pub struct Explosion{ // 0x1c
    pub x: Float,  // X
    pub y: Float,  // Y
    pub z: Float,  // Z
    pub strength: Float,  // Strength
    pub blocks_len: VarInt, // Count of next array
    pub block_offsets: Vec<(   // X/Y/Z offsets of affected blocks
        Byte,   // Blocks in this array are set to Air
        Byte,
        Byte
    )>,
    pub vx: Float,  // Vel X // Velocity of player being pushed by the explosion
    pub vy: Float,  // Vel Y
    pub vz: Float,  // Vel Z
}

#[derive(Debug)]
pub struct UnloadChunk{ // 0x1d
    pub x: Int,    // Chunk X
    pub z: Int,    // Chunk Z
}  

#[derive(Debug)]
pub struct ChangeGameState{ // 0x1e
    pub reason: UByte,
    pub value: Float,
}

#[derive(Debug)]
pub struct OpenHorseWindow{ // 0x1f
    pub window_id: Byte,
    pub num_slots: VarInt,
    pub entity_id: Int,
}

#[derive(Debug)]
pub struct InitializeWorldBorder{ // 0x20
    pub x: Double,
    pub z: Double,
    pub old_diameter: Double,
    pub new_diameter: Double,
    pub speed: VarLong, // Number of millis until new diameter is reached
    pub portal_teleport_boundary: VarInt,
    pub warning_blocks: VarInt,
    pub warning_time: VarInt
}

#[derive(Debug)]
pub struct KeepAliveClientbound{ // 0x21
    pub keep_alive_id: Long
}

#[derive(Debug)]
pub struct ChunkData{ // 0x22
    pub x: Int,
    pub z: Int,
    pub bit_mask_len: VarInt,
    pub bit_mask: Vec<Long>,
    pub heightmaps: NBTTag,
    pub biomes_len: VarInt,
    pub biomes: Vec<VarInt>,
    pub data_len: VarInt,
    pub data: Vec<Byte>,
    pub block_entities_len: VarInt,
    pub block_entities: Vec<NBTTag>,
}

#[derive(Debug)]
pub struct Effect{ // 0x23
    pub effect_id: Int,
    pub location: Position,
    pub data: Int,
    pub disable_relative_volume: Boolean,
}

#[derive(Debug)]
pub struct Particle{ // 0x24
    pub particle_id: Int,
    pub long_distance: Boolean,
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub off_x: Float,
    pub off_y: Float,
    pub off_z: Float,
    pub particle_data: Float,
    pub particle_count: Int,
    // TODO - Data
}

#[derive(Debug)]
pub struct UpdateLight{ // 0x25
    pub chunk_x: VarInt,
    pub chunk_z: VarInt,
    pub trust_edges: Boolean,
    pub sky_light_mask_len: VarInt,
    pub sky_light_mask: Vec<Long>,
    pub block_light_mask_len: VarInt,
    pub block_light_mask: Vec<Long>,
    pub empty_sky_light_mask_len: VarInt,
    pub empty_sky_light_mask: Vec<Long>,
    pub empty_block_light_mask_len: VarInt,
    pub empty_block_light_mask: Vec<Long>,
    pub sky_lights_len: VarInt,
    pub sky_lights: Vec<(VarInt, [Byte; 2048])>,
    pub block_lights_len: VarInt,
    pub block_lights: Vec<(VarInt, [Byte; 2048])>,
}

#[derive(Debug)]
pub struct JoinGame{ // 0x26
    pub player_id: Int,
    pub is_hardcore: Boolean,
    pub gamemode: UByte,
    pub prev_gamemode: Byte,
    pub world_names_len: VarInt,
    pub world_names: Vec<Identifier>,
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

#[derive(Debug)]
pub struct MapData{ // 0x27
    // TODO
}

#[derive(Debug)]
pub struct TradeList{ // 0x28
    // TODO
}

#[derive(Debug)]
pub struct EntityPosition{ // 0x29
    pub entity_id: VarInt,
    pub dx: Short,
    pub dy: Short,
    pub dz: Short,
    pub on_ground: Boolean,
}

#[derive(Debug)]
pub struct EntityPositionAndRotation{ // 0x2a
    pub entity_id: VarInt,
    pub dx: Short,
    pub dy: Short,
    pub dz: Short,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: Boolean,
}

#[derive(Debug)]
pub struct EntityRotation{ // 0x2b
    pub entity_id: VarInt,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: Boolean,
}




#[derive(Debug)]
pub struct UpdateHealth{ // 0x52
    pub health: Float,
    pub food: VarInt,
    pub saturation: Float,
}

#[derive(Debug)]
pub struct EntityHeadLook{ // 0x3e
    pub entity_id: VarInt,
    pub head_yaw: Angle,
}

#[derive(Debug)]
pub struct EntityVelocity{ // 0x4f
    pub entity_id: VarInt,
    pub vx: Short,
    pub vy: Short,
    pub vz: Short,
}

#[derive(Debug)]
pub struct EntityTeleport{ // 0x61
    pub entity_id: VarInt,
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: Boolean,
}

#[derive(Debug)]
pub struct EntityMetadata{ // 0x4d
    pub entity_id: VarInt,
    // TODO - Metadata
}

#[derive(Debug)]
pub struct PlayerPositionAndLook{ // 0x38
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub yaw: Float,
    pub pitch: Float,
    pub flags: Byte,
    pub teleport_id: VarInt,
    pub dismount: Boolean,
}

#[derive(Debug)]
pub struct DestroyEntities{ // 0x3a
    pub entities_len: VarInt,
    pub entities: Vec<VarInt>,
}


#[derive(Debug)]
pub struct TimeUpdate{ // 0x58
    pub world_age: Long,
    pub day_time: Long,
}

#[derive(Debug)]
pub struct SoundEffect{ // 0x5c
    // TODO - Sound Effect
}


/// All the different types of packets that will be going back and forth
#[derive(Debug)]
pub enum DecodedPacket {
    Ok,
    Close,
    Error(Error),

    // ********************************************************
    // Serverbound ********************************************

    // Yes, I am a little lacking in these ones so far, but for now I just want a client that can see shit, not actually do shit
    Handshake(VarInt, MCString, Short, HandshakeMode),
    LoginStart(MCString),
    ClientStatusRespawn,

    // X, Y, Z, onGround
    PlayerPosition(Double, Double, Double, Boolean),

    // X, Y, Z, Yaw, Pitch, OnGround
    PlayerPositionAndRotation(Double, Double, Double, Float, Float, Boolean),

    KeepAliveServerbound(Long),

    ChatOutgoing(MCString),

    // Locale, View Distance, Chat Mode (0: Enabled, 1: Commands Only, 2: Hidden), Chat Colours, Visible Skin parts, Main Hand (0: Left, 1:Right), Disable Text Filtering
    ClientSettings(MCString, Byte, VarInt, Boolean, UByte, VarInt, Boolean),

    TeleportConfirm(VarInt), // Teleport ID as given by PlayerPositionAndLook

    // ************************************************************
    // CLIENTBOUND *************************************CLIENTBOUND

    // ********************** LOGIN
    Status(Status),

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
    Effect(Effect),
    Particle(Particle),
    UpdateLight(UpdateLight),
    JoinGame(JoinGame),
    MapData(MapData),
    TradeList(TradeList),
    EntityPosition(EntityPosition),
    EntityPositionAndRotation(EntityPositionAndRotation),
    EntityRotation(EntityRotation),


    UpdateHealth(UpdateHealth),
    EntityHeadLook(EntityHeadLook),
    EntityVelocity(EntityVelocity),
    EntityTeleport(EntityTeleport),
    EntityMetadata(EntityMetadata),
    PlayerPositionAndLook(PlayerPositionAndLook),
    DestroyEntities(DestroyEntities),
    TimeUpdate(TimeUpdate),
    SoundEffect(SoundEffect),

    // Login stuff

    EncryptionRequest(),    // TODO - I'll leave this for a while
    LoginSuccess(),         // TODO - UUID and Name
    SetCompression(VarInt), // Theshold
    LoginPluginRequest(),   // Not implemented


    Unknown(Vec<u8>),
    Empty,
}

impl DecodedPacket {
    /// Encodes a DecodedPacket to be sent to the server
    pub fn encode(&self) -> Option<Packet> {
        use DecodedPacket::*;

        let mut out: Packet;

        match self {
            // Handshake
            Handshake(protocol, origin, port, mode) => {
                out = Packet::new_with_id(0x00);
                out.add(&protocol.to_bytes());
                out.add(&origin.to_bytes());
                out.add(&port.to_bytes());
                out.add_byte(match mode {
                    HandshakeMode::Status => 0x01,
                    HandshakeMode::Login => 0x02,
                });
            }

            // Login Request
            LoginStart(name) => {
                out = Packet::new_with_id(0x00);
                out.add(&name.to_bytes());

                if out.size() > 18 {
                    panic!("Name is too long for packet!");
                }
            }

            PlayerPositionAndRotation(x, y, z, yaw, pitch, on_ground) => {
                out = Packet::new_with_id(0x12);
                out.add(&x.to_bytes());
                out.add(&y.to_bytes());
                out.add(&z.to_bytes());
                out.add(&yaw.to_bytes());
                out.add(&pitch.to_bytes());
                out.add(&on_ground.to_bytes());
            }

            PlayerPosition(x, y, z, on_ground) => {
                out = Packet::new_with_id(0x11);
                out.add(&x.to_bytes());
                out.add(&y.to_bytes());
                out.add(&z.to_bytes());
                out.add(&on_ground.to_bytes());
            }

            ClientStatusRespawn => {
                out = Packet::new_with_id(0x04);
                out.add_byte(0x00);
            }

            KeepAliveServerbound(keep_alive_id) => {
                out = Packet::new_with_id(0x0f);
                out.add(&keep_alive_id.to_bytes());
            }

            ChatOutgoing(message) => {
                out = Packet::new_with_id(0x03);
                out.add(&message.to_bytes());
            }

            ClientSettings(locale, view, chat, cols, skin, hand, filtering) => {
                out = Packet::new_with_id(0x05);
                out.add(&locale.to_bytes());
                out.add(&view.to_bytes());
                out.add(&chat.to_bytes());
                out.add(&cols.to_bytes());
                out.add(&skin.to_bytes());
                out.add(&hand.to_bytes());
                out.add(&filtering.to_bytes());
            }

            TeleportConfirm(id) => {
                out = Packet::new_with_id(0x00);
                out.add(&id.to_bytes());
            }

            // Packets we don't care to encode (like all the clientbound ones)
            _ => {
                print!("Unknown Packet to encode: {:?}", self);
                return None;
            }
        }
        Some(out)
    }
}

// A raw collection of bytes used to contruct a packet
pub struct Packet {
    bytes: Vec<u8>,
}

impl Packet {
    /// Create a new packet with no ID
    pub fn new() -> Packet {
        Packet { bytes: Vec::new() }
    }

    /// Create a new packet with a specified hex ID
    pub fn new_with_id(id: u8) -> Packet {
        Packet { bytes: vec![id] }
    }

    /// Push a vector of bytes to the packet
    pub fn add(&mut self, bytes: &Vec<u8>) {
        for b in bytes.iter() {
            self.bytes.push(*b);
        }
    }

    /// Push a single byte to the packet
    pub fn add_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    /// Returns the packet as a vector of bytes with the length signed as a VarInt at the start
    pub fn get_bytes_with_length(&self) -> Vec<u8> {
        let len = VarInt(self.bytes.len() as i32);
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut len.to_bytes());
        bytes.append(&mut self.bytes.clone());
        bytes
    }

    /// Returns the packet as a vector of bytes with no size signature at the start
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.bytes.clone());
        bytes
    }

    pub fn size(&self) -> usize {
        self.bytes.len()
    }
}

/// Decodes a packet from a vector of bytes into a DecodedPacket, given the server state
pub fn decode_packet(packet: Vec<u8>, state: &ServerState) -> DecodedPacket {
    use DecodedPacket::*;

    if packet.len() == 0 {
        return Empty;
    }

    let out: DecodedPacket;

    // Packet decoder, makes decoding packets much easier than what I was doing before :P
    let mut pd = PacketDecoder::new(&packet);

    match packet[0] {
        0x00 => match state {
            ServerState::Login => {
                out = Disconnect(packets::Disconnect{reason: pd.next_string()});
            }
            ServerState::Play => {
                out = SpawnEntity(packets::SpawnEntity{
                    entity_id: pd.next_varint(),
                    uuid: pd.next_uuid(),
                    entity_type: pd.next_varint(),
                    x: pd.next_double(),
                    y: pd.next_double(),
                    z: pd.next_double(),
                    pitch: pd.next_angle(),
                    yaw: pd.next_angle(),
                    data: pd.next_int(),
                    vx: pd.next_short(),
                    vy: pd.next_short(),
                    vz: pd.next_short(),
                });
            }
            ServerState::Status => {
                out = Status(packets::Status{response: pd.next_string()});
            }
        },

        0x02 => match state {
            ServerState::Login => {
                out = LoginSuccess();
            }
            ServerState::Play => {
                out = SpawnLivingEntity(packets::SpawnLivingEntity{
                    entity_id: pd.next_varint(),
                    uuid: pd.next_uuid(),
                    entity_type: pd.next_varint(),
                    x: pd.next_double(),
                    y: pd.next_double(),
                    z: pd.next_double(),
                    yaw: pd.next_angle(),
                    pitch: pd.next_angle(),
                    head_pitch: pd.next_angle(),
                    vx: pd.next_short(),
                    vy: pd.next_short(),
                    vz: pd.next_short(),
                });
            }
            ServerState::Status => {
                out = Unknown(packet);
            }
        },

        0x03 => match state {
            ServerState::Login => {
                out = SetCompression(pd.next_varint());
            }
            _ => {
                out = Unknown(packet);
            }
        },

        0x0c => {
            out = BlockChange(packets::BlockChange{
                block_pos: pd.next_position(), 
                block_state_id: pd.next_varint()
            });
        }

        0x0e => match state {
            ServerState::Play => {
                out = ServerDifficulty(packets::ServerDifficulty{
                    difficulty: pd.next_ubyte(), 
                    locked: pd.next_bool()
                });
            }
            _ => out = Unknown(packet),
        },

        0x0f => match state {
            ServerState::Play => {
                out = ChatIncoming(packets::ChatIncoming{
                    json: pd.next_string(), 
                    position: pd.next_byte(), 
                    sender: pd.next_uuid()
                });
            }
            _ => out = Unknown(packet),
        },

        0x1a => {
            out = Disconnect(packets::Disconnect{reason: pd.next_string()});
        }

        0x21 => {
            out = KeepAliveClientbound(packets::KeepAliveClientbound{
                keep_alive_id: pd.next_long()}
            );
        }

        0x22 => {
            let chunk_x = pd.next_int();
            let chunk_z = pd.next_int();
            let bit_mask_len = pd.next_varint();
            let mut bit_mask: Vec<Long> = Vec::new();
            for _ in 0..bit_mask_len.0 as usize {
                bit_mask.push(pd.next_long());
            }
            let heightmaps = pd.next_nbt();
            let biomes_len = pd.next_varint();
            let mut biomes: Vec<VarInt> = Vec::new();
            for _ in 0..biomes_len.0 as usize {
                biomes.push(pd.next_varint());
            }
            let data_len = pd.next_varint();
            let mut data = Vec::new();
            for _ in 0..data_len.0 as usize {
                data.push(pd.next_byte());
            }
            let blocks_len = pd.next_varint();
            let mut block_entities = Vec::new();
            for _ in 0..blocks_len.0 as usize {
                block_entities.push(pd.next_nbt());
            }
            out = ChunkData(packets::ChunkData{
                    x: chunk_x,
                    z: chunk_z,
                    bit_mask_len,
                    bit_mask,
                    heightmaps,
                    biomes_len,
                    biomes,
                    data_len,
                    data,
                    block_entities_len: blocks_len,
                    block_entities,
            });
        }

        0x26 => {
            let player_id = pd.next_int();
            let is_hardcore = pd.next_bool();
            let gamemode = pd.next_ubyte();
            let prev_gamemode = pd.next_byte();
            let world_names_len = pd.next_varint();
            let mut world_names: Vec<Identifier> = Vec::new();
            for _ in 0..world_names_len.0 as usize {
                world_names.push(pd.next_string());
            }
            out = JoinGame(packets::JoinGame{
                player_id,
                is_hardcore,
                gamemode,
                prev_gamemode,
                world_names_len,
                world_names,
                dimension_codec: pd.next_nbt(),
                dimension: pd.next_nbt(),
                world_name: pd.next_string(),
                hashed_seed: pd.next_long(),
                max_players: pd.next_varint(),
                view_distance: pd.next_varint(),
                reduced_debug_info: pd.next_bool(),
                enable_respawn_screen: pd.next_bool(),
                is_debug: pd.next_bool(),
                is_flat: pd.next_bool(),
            });
        }

        0x29 => {
            out = EntityPosition(packets::EntityPosition{
                entity_id: pd.next_varint(),
                dx: pd.next_short(),
                dy: pd.next_short(),
                dz: pd.next_short(),
                on_ground: pd.next_bool(),
            });
        }

        0x2a => {
            out = EntityPositionAndRotation(packets::EntityPositionAndRotation{
                entity_id: pd.next_varint(),
                dx: pd.next_short(),
                dy: pd.next_short(),
                dz: pd.next_short(),
                yaw: pd.next_angle(),
                pitch: pd.next_angle(),
                on_ground: pd.next_bool(),
            });
        }

        0x2b => {
            out = EntityRotation(packets::EntityRotation{
                entity_id: pd.next_varint(),
                yaw: pd.next_angle(),
                pitch: pd.next_angle(),
                on_ground: pd.next_bool(),
            });
        }

        0x38 => {
            out = PlayerPositionAndLook(packets::PlayerPositionAndLook{
                x: pd.next_double(),
                y: pd.next_double(),
                z: pd.next_double(),
                yaw: pd.next_float(),
                pitch: pd.next_float(),
                flags: pd.next_byte(),
                teleport_id: pd.next_varint(),
                dismount: pd.next_bool(),
            });
        }

        0x3a => {
            let vi_num = pd.next_varint();
            let mut ids: Vec<VarInt> = Vec::new();

            for _ in 0..vi_num.0 as usize {
                ids.push(pd.next_varint());
            }
            out = DestroyEntities(packets::DestroyEntities{
                entities_len: vi_num, 
                entities: ids
            });
        }

        0x3e => {
            out = EntityHeadLook(packets::EntityHeadLook{
                entity_id: pd.next_varint(), 
                head_yaw: pd.next_angle()
            });
        }

        0x4d => {
            out = EntityMetadata(packets::EntityMetadata{
                entity_id: pd.next_varint(),
            });
        }

        0x4f => {
            out = EntityVelocity(packets::EntityVelocity{
                entity_id: pd.next_varint(),
                vx: pd.next_short(),
                vy: pd.next_short(),
                vz: pd.next_short(),
            });
        }

        0x52 => {
            out = UpdateHealth(packets::UpdateHealth{
                health: pd.next_float(), 
                food: pd.next_varint(), 
                saturation: pd.next_float()
            });
        }

        0x58 => {
            out = TimeUpdate(packets::TimeUpdate{
                world_age: pd.next_long(), 
                day_time: pd.next_long()
            });
        }

        0x5c => {
            out = SoundEffect(packets::SoundEffect{});
        }

        0x61 => {
            out = EntityTeleport(packets::EntityTeleport{
                entity_id: pd.next_varint(),
                x: pd.next_double(),
                y: pd.next_double(),
                z: pd.next_double(),
                yaw: pd.next_angle(),
                pitch: pd.next_angle(),
                on_ground: pd.next_bool(),
            });
        }

        _ => {
            out = Unknown(packet);
        }
    }

    match &out {
        #[allow(unused_variables)]
        Unknown(pack) => {
            println!("Unknow packet: {:02x}", pack[0]);
        }
        _ => {}
    }

    out
}

/// Packet Decoder walks a provided vector of bytes and extracts variables from them
pub struct PacketDecoder<'a> {
    buf: &'a Vec<u8>,
    ind: usize,
}

impl PacketDecoder<'_> {
    /// Create a packet decoder for a provided Vector
    pub fn new<'a>(buf: &'a Vec<u8>) -> PacketDecoder<'a> {
        PacketDecoder {
            buf,
            ind: 1, // Start at 1 to skip the packet type signature
        }
    }

    pub fn get_index(&self) -> usize {
        self.ind
    }

    pub fn next_bool(&mut self) -> Boolean {
        self.ind += 1;
        match self.buf.get(self.ind - 1) {
            Some(0x00) => Boolean(false),
            Some(0x01) => Boolean(true),
            _ => {
                panic!("Invalid Boolean")
            }
        }
    }

    pub fn next_byte(&mut self) -> Byte {
        self.ind += 1;
        Byte(self.buf[self.ind - 1] as i8)
    }

    pub fn next_ubyte(&mut self) -> UByte {
        self.ind += 1;
        UByte(self.buf[self.ind - 1])
    }

    pub fn next_short(&mut self) -> Short {
        let bytes = extract_16(self.buf, self.ind);
        self.ind += 2;
        Short::from_bytes(&bytes)
    }

    pub fn next_ushort(&mut self) -> UShort {
        let bytes = extract_16(self.buf, self.ind);
        self.ind += 2;
        UShort::from_bytes(&bytes)
    }

    pub fn next_int(&mut self) -> Int {
        let bytes = extract_32(self.buf, self.ind);
        self.ind += 4;
        Int::from_bytes(&bytes)
    }

    pub fn next_long(&mut self) -> Long {
        let bytes = extract_64(self.buf, self.ind);
        self.ind += 8;
        Long::from_bytes(&bytes)
    }

    pub fn next_float(&mut self) -> Float {
        let bytes = extract_32(self.buf, self.ind);
        self.ind += 4;
        Float::from_bytes(&bytes)
    }

    pub fn next_double(&mut self) -> Double {
        let bytes = extract_64(self.buf, self.ind);
        self.ind += 8;
        Double::from_bytes(&bytes)
    }

    pub fn next_string(&mut self) -> MCString {
        let (vi, vi_len) = VarInt::from_vec(self.buf, self.ind);
        let str = MCString::from_bytes(&self.buf[self.ind..]);
        self.ind += vi_len + vi.0 as usize;
        str.expect("Invalid MCString")
    }

    pub fn next_varint(&mut self) -> VarInt {
        let (vi, vi_len) = VarInt::from_vec(self.buf, self.ind);
        self.ind += vi_len;
        vi
    }

    pub fn next_varlong(&mut self) -> VarLong {
        let (vi, vi_len) = VarLong::from_vec(self.buf, self.ind);
        self.ind += vi_len;
        vi
    }

    pub fn next_entity_metadata(&mut self) -> EntityMetadata {
        todo!()
    }

    pub fn next_slot(&mut self) -> Slot {
        todo!()
    }

    pub fn next_nbt(&mut self) -> NBTTag {
        let mut cursor = Cursor::new(self.buf);
        cursor.set_position(self.ind as u64);

        match io::read_nbt(&mut cursor, io::Flavor::Uncompressed) {
            Ok((nbt, name)) => {
                self.ind = cursor.position() as usize;
                return NBTTag(nbt);
            },
            Err(e) => {
                panic!("Failed to decode NBT data: {}", e);
            }
        }
    }

    pub fn next_position(&mut self) -> Position {
        let big = u64::from_be_bytes(extract_64(self.buf, self.ind));

        let x = (big >> 38) as i32;
        let y = (big & 0xfff) as i32;
        let z = (big << 26 >> 38) as i32;

        self.ind += 8;
        Position(x, y, z)
    }

    pub fn next_angle(&mut self) -> Angle {
        self.next_ubyte()
    }

    pub fn next_uuid(&mut self) -> UUID {
        let mut b1 = [0u8; 8];
        let mut b2 = [0u8; 8];
        for i in 0..16 {
            if i < 8 {
                b1[i] = self.buf[i]
            }
            if i >= 8 {
                b2[i - 8] = self.buf[i]
            }
        }
        self.ind += 16;
        UUID([u64::from_be_bytes(b1), u64::from_be_bytes(b2)])
    }

    pub fn print_remaining_bytes(&self) {
        println!("Printing remaining bytes:");
        println!("{:02x?}", &self.buf[self.ind..]);
    }
}

// These functions just extract arrays of specific length from a given buffer at a starting index
// Really, nothing to see here

fn extract_8(buf: &Vec<u8>, start: usize) -> [u8; 1] {
    [buf[start]]
}

fn extract_16(buf: &Vec<u8>, start: usize) -> [u8; 2] {
    [buf[start], buf[start + 1]]
}

fn extract_32(buf: &Vec<u8>, start: usize) -> [u8; 4] {
    [buf[start], buf[start + 1], buf[start + 2], buf[start + 3]]
}

fn extract_64(buf: &Vec<u8>, start: usize) -> [u8; 8] {
    [
        buf[start],
        buf[start + 1],
        buf[start + 2],
        buf[start + 3],
        buf[start + 4],
        buf[start + 5],
        buf[start + 6],
        buf[start + 7],
    ]
}

fn extract_128(buf: &Vec<u8>, start: usize) -> [u8; 16] {
    [
        buf[start],
        buf[start + 1],
        buf[start + 2],
        buf[start + 3],
        buf[start + 4],
        buf[start + 5],
        buf[start + 6],
        buf[start + 7],
        buf[start + 8],
        buf[start + 9],
        buf[start + 10],
        buf[start + 11],
        buf[start + 12],
        buf[start + 13],
        buf[start + 14],
        buf[start + 15],
    ]
}
