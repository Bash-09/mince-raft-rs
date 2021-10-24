#![allow(dead_code)]

use std::io::Error;

use crate::app::client::server::ServerState;

use super::types::*;



#[derive(Debug)]
pub enum HandshakeMode {
    Status,
    Login,
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
    // Clientbound ************************************************

    // JSON Data, Position, Sender
    // Position - 0: Chat, 1: System Message, 2: Game Info
    ChatIncoming(MCString, Byte, UUID), 
    // Health, Food, Saturation
    UpdateHealth(Float, VarInt, Float), 

    // Login stuff
    Disconnect(MCString), // TODO - Chat disconnect reason
    Status(MCString),

    EncryptionRequest(), // TODO - I'll leave this for a while
    LoginSuccess(), // TODO - UUID and Name
    SetCompression(VarInt), // Theshold
    LoginPluginRequest(), // Not implemented

    JoinGame(Int), // TODO = Got a looooooot of stuff to go here

    // Difficulty (0: peaceful, 1: easy, 2: medium, 3: hard), Difficulty Locked
    ServerDifficulty(UByte, Boolean), 

    // Entity ID, dX, dY, dZ, onGround
    EntityPosition(VarInt, Short, Short, Short, Boolean),

    // Entity ID, dX, dY, dZ, yaw, pitch, onGround
    EntityPositionAndRotation(VarInt, Short, Short, Short, Angle, Angle, Boolean),

    // Entity ID, yaw, pitch
    EntityRotation(VarInt, Angle, Angle, Boolean),

    // Entity ID, Head Yaw
    EntityHeadLook(VarInt, Angle),

    // Entity ID, VX, VY, VZ
    EntityVelocity(VarInt, Short, Short, Short),

    // Entity ID, X, Y, Z, Yaw, Pitch, OnGround
    EntityTeleport(VarInt, Double, Double, Double, Angle, Angle, Boolean),

    // TODO
    EntityMetadata(), // TODO

    // Block Coordinates, New Block State ID
    BlockChange(Position, VarInt),

    // X, Y, Z, Yaw, Pitch, Flags, Teleport ID, Dismount Vehicle
    PlayerPositionAndLook(Double, Double, Double, Float, Float, Byte, VarInt, Boolean),

    // Number of elements in following array, Array of Entity IDs to destroy
    DestroyEntities(VarInt, Vec<VarInt>),

    // Entity ID, UUID, Entity Type, X, Y, Z, Pitch, Yaw, Data, VX, VY, VZ
    SpawnEntity(VarInt, UUID, VarInt, Double, Double, Double, Angle, Angle, Int, Short, Short, Short),

    // Entity ID, UUID, Entity Type, X, Y, Z, Yaw, Pitch, Head Pitch, Vel X, Vel Y, Vel Z
    SpawnLivingEntity(VarInt, UUID, VarInt, Double, Double, Double, Angle, Angle, Angle, Short, Short, Short),

    // World Age, Time of Day
    TimeUpdate(Long, Long), 

    SoundEffect(), // TODO - probably never lol

    // KeepAlive ID, must be responded to with KeepAliceServerbound containing the same ID
    KeepAliveClientbound(Long),


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
                out.add_byte(match mode {HandshakeMode::Status => 0x01, HandshakeMode::Login => 0x02});
            },

            // Login Request
            LoginStart(name) => {
                out = Packet::new_with_id(0x00);
                out.add(&name.to_bytes());
        
                if out.size() > 18 {
                    panic!("Name is too long for packet!");
                }
            },

            PlayerPositionAndRotation(x, y, z, yaw, pitch, on_ground) => {
                out = Packet::new_with_id(0x12);
                out.add(&x.to_bytes());
                out.add(&y.to_bytes());
                out.add(&z.to_bytes());
                out.add(&yaw.to_bytes());
                out.add(&pitch.to_bytes());
                out.add(&on_ground.to_bytes());
            },

            PlayerPosition(x, y, z, on_ground) => {
                out = Packet::new_with_id(0x11);
                out.add(&x.to_bytes());
                out.add(&y.to_bytes());
                out.add(&z.to_bytes());
                out.add(&on_ground.to_bytes());
            },

            ClientStatusRespawn => {
                out = Packet::new_with_id(0x04);
                out.add_byte(0x00);
            },

            KeepAliveServerbound(keep_alive_id) => {
                out = Packet::new_with_id(0x0f);
                out.add(&keep_alive_id.to_bytes());
            },

            ChatOutgoing(message) => {
                out = Packet::new_with_id(0x03);
                out.add(&message.to_bytes());
            },

            ClientSettings(locale, view, chat, cols, skin, hand, filtering) => {
                out = Packet::new_with_id(0x05);
                out.add(&locale.to_bytes());
                out.add(&view.to_bytes());
                out.add(&chat.to_bytes());
                out.add(&cols.to_bytes());
                out.add(&skin.to_bytes());
                out.add(&hand.to_bytes());
                out.add(&filtering.to_bytes());
            },
            
            TeleportConfirm(id) => {
                out = Packet::new_with_id(0x00);
                out.add(&id.to_bytes());
            },

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
        Packet {  
            bytes: Vec::new(),
        }
    }

    /// Create a new packet with a specified hex ID
    pub fn new_with_id(id: u8) -> Packet {
        Packet { 
            bytes: vec![id],
        }
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

    if packet.len() == 0 {return Empty}

    let out: DecodedPacket;

    // Packet decoder, makes decoding packets much easier than what I was doing before :P
    let mut pd = PacketDecoder::new(&packet);

    match packet[0] {

        0x00 => {
            match state {
                ServerState::Login => {
                    out = Disconnect(pd.next_string());
                },
                ServerState::Play => {
                    out = SpawnEntity(
                        pd.next_varint(),
                        pd.next_uuid(),
                        pd.next_varint(),
                        pd.next_double(),
                        pd.next_double(),
                        pd.next_double(),
                        pd.next_angle(),
                        pd.next_angle(),
                        pd.next_int(),
                        pd.next_short(),
                        pd.next_short(),
                        pd.next_short(),
                    );
                },
                ServerState::Status => {
                    out = Status(pd.next_string());
                }
            }
        },

        0x02 => {
            match state {
                ServerState::Login => {
                    out = LoginSuccess();
                },
                ServerState::Play => {
                    out = SpawnLivingEntity(
                        pd.next_varint(),
                        pd.next_uuid(),
                        pd.next_varint(),
                        pd.next_double(),
                        pd.next_double(),
                        pd.next_double(),
                        pd.next_angle(),
                        pd.next_angle(),
                        pd.next_angle(),
                        pd.next_short(),
                        pd.next_short(),
                        pd.next_short()
                    );
                },
                ServerState::Status => {
                    out = Unknown(packet);
                }
            }
        },

        0x03 => {
            match state {
                ServerState::Login => {
                    out = SetCompression(pd.next_varint());
                },
                _ => {
                    out = Unknown(packet);
                }
            }
        }

        0x0c => {
            out = BlockChange(pd.next_position(), pd.next_varint());
        },

        0x0e => {
            match state {
                ServerState::Play => {
                    out = ServerDifficulty(
                        pd.next_ubyte(),
                        pd.next_bool(),
                    );
                },
                _ => out = Unknown(packet),
            }
        },

        0x0f => {
            match state {
                ServerState::Play => {
                    out = ChatIncoming(
                        pd.next_string(),
                        pd.next_byte(),
                        pd.next_uuid(),
                    );
                },
                _ => out = Unknown(packet),
            }
        }

        0x1a => {
            out = Disconnect(pd.next_string());
        }

        0x21 => {
            out = KeepAliveClientbound(pd.next_long());
        },

        0x26 => {
            out = JoinGame(pd.next_int());
        },

        0x29 => {
            out = EntityPosition(
                pd.next_varint(),
                pd.next_short(),
                pd.next_short(),
                pd.next_short(),
                pd.next_bool(),
            );
        },

        0x2a => {
            out = EntityPositionAndRotation(
                pd.next_varint(),
                pd.next_short(),
                pd.next_short(),
                pd.next_short(),
                pd.next_angle(),
                pd.next_angle(),
                pd.next_bool(),
            );
        },

        0x2b => {
            out = EntityRotation(
                pd.next_varint(),
                pd.next_angle(),
                pd.next_angle(),
                pd.next_bool(),
            );
        },


        0x38 => {
            out = PlayerPositionAndLook(
                pd.next_double(),
                pd.next_double(),
                pd.next_double(),
                pd.next_float(),
                pd.next_float(),
                pd.next_byte(),
                pd.next_varint(),
                pd.next_bool(),
            );
        },

        0x3a => {
            let vi_num = pd.next_varint();
            let mut ids: Vec<VarInt> = Vec::new();

            for _ in 0..vi_num.0 as usize {
                ids.push(pd.next_varint());
            }
            out = DestroyEntities(vi_num, ids);
        },


        0x3e => {
            out = EntityHeadLook(
                pd.next_varint(),
                pd.next_angle(),
            );
        },

        0x4d => {
            out = EntityMetadata();
        },

        0x4f => {
            out = EntityVelocity(
                pd.next_varint(),
                pd.next_short(),
                pd.next_short(),
                pd.next_short(),
            );
        }

        0x52 => {
            out = UpdateHealth(
                pd.next_float(),
                pd.next_varint(),
                pd.next_float(),
            );
        },


        0x58 => {
            out = TimeUpdate(
                pd.next_long(),
                pd.next_long(),
            );
        },

        0x5c => {
            out = SoundEffect();
        }

        0x61 => {
            out = EntityTeleport(
                pd.next_varint(),
                pd.next_double(),
                pd.next_double(),
                pd.next_double(),
                pd.next_angle(),
                pd.next_angle(),
                pd.next_bool(),
            );
        }

        _ => {
            out = Unknown(packet);
        }
    }

    match &out {
        #[allow(unused_variables)]
        Unknown(pack) => {
            // println!("Unknow packet: {:02x}", pack[0]);
        },
        _ => {}
    }

    out
}


/// Packet Decoder walks a provided vector of bytes and extracts variables from them
struct PacketDecoder<'a>  {
    buf: &'a Vec<u8>,
    ind: usize,
}

impl PacketDecoder<'_> {

    /// Create a packet decoder for a provided Vector
    pub fn new<'a>(buf: &'a Vec<u8>) -> PacketDecoder<'a> {
        PacketDecoder {
            buf,
            ind: 1 // Start at 1 to skip the packet type signature
        }
    }

    pub fn next_bool(&mut self) -> Boolean {
        self.ind += 1;
        match self.buf.get(self.ind-1) {
            Some(0x00) => {Boolean(false)}
            Some(0x01) => {Boolean(true)}
            _ => {panic!("Invalid Boolean")}
        }
    }

    pub fn next_byte(&mut self) -> Byte {
        self.ind += 1;
        Byte(self.buf[self.ind-1] as i8)
    }

    pub fn next_ubyte(&mut self) -> UByte {
        self.ind += 1;
        UByte(self.buf[self.ind-1])
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
            if i < 8 {b1[i] = self.buf[i]}
            if i >= 8 {b2[i-8] = self.buf[i]}
        }
        self.ind += 16;
        UUID([
            u64::from_be_bytes(b1),
            u64::from_be_bytes(b2)
        ])
    }



}

// These functions just extract arrays of specific length from a given buffer at a starting index
// Really, nothing to see here

fn extract_8(buf: &Vec<u8>, start: usize) -> [u8; 1] {
    [buf[start]]
}

fn extract_16(buf: &Vec<u8>, start: usize) -> [u8; 2] {
    [
        buf[start],
        buf[start+1]
    ]
}

fn extract_32(buf: &Vec<u8>, start: usize) -> [u8; 4] {
    [
        buf[start],
        buf[start+1],
        buf[start+2],
        buf[start+3]
    ]
}

fn extract_64(buf: &Vec<u8>, start: usize) -> [u8; 8] {
    [
        buf[start],
        buf[start+1],
        buf[start+2],
        buf[start+3],
        buf[start+4],
        buf[start+5],
        buf[start+6],
        buf[start+7]
    ]
}

fn extract_128(buf: &Vec<u8>, start: usize) -> [u8; 16] {
    [
        buf[start],
        buf[start+1],
        buf[start+2],
        buf[start+3],
        buf[start+4],
        buf[start+5],
        buf[start+6],
        buf[start+7],
        buf[start+8],
        buf[start+9],
        buf[start+10],
        buf[start+11],
        buf[start+12],
        buf[start+13],
        buf[start+14],
        buf[start+15]
    ]
}