

use std::io::Error;

use crate::app::client::server::ServerState;

use super::types::*;



#[derive(Debug)]
pub enum HandshakeMode {
    Status,
    Login,
}

#[derive(Debug)]
pub enum DecodedPacket {
    Ok,
    Close,
    Error(Error),





    // Serverbound ********************************************
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




    // Clientbound ************************************************

    // Position - 0: Chat, 1: System Message, 2: Game Info
    ChatIncoming(MCString, Byte, UUID), // JSON Data, Position, Sender
    UpdateHealth(Float, VarInt, Float), // Health, Food, Saturation

    // Login stuff
    Disconnect(MCString), // TODO - Chat disconnect reason
    Status(MCString),

    EncryptionRequest(), // TODO - I'll leave this for a while
    LoginSuccess(), // TODO - UUID and Name
    SetCompression(VarInt), // TODO - Theshold
    LoginPluginRequest(), // Not implemented

    JoinGame(Int), // TODO = Got a looooooot of stuff to go here

    ServerDifficulty(UByte, Boolean), // Difficulty (0: peaceful, 1: easy, 2: medium, 3: hard), Difficulty Locked

    // Entity ID, dX, dY, dZ, onGround
    EntityPosition(VarInt, Short, Short, Short, Boolean),

    // Entity ID, dX, dY, dZ, yaw, pitch, onGround
    EntityPositionAndRotation(VarInt, Short, Short, Short, Angle, Angle, Boolean),

    // Entity ID, yaw, pitch
    EntityRotation(VarInt, Angle, Angle, Boolean),

    // X, Y, Z, Yaw, Pitch, Flags, Teleport ID, Dismount Vehicle
    PlayerPositionAndLook(Double, Double, Double, Float, Float, Byte, VarInt, Boolean),

    // Number of elements in following array, Array of Entity IDs to destroy
    DestroyEntities(VarInt, Vec<VarInt>),
    SpawnEntity(),

    //Entity ID, UUID, Entity Type, X, Y, Z, Yaw, Pitch, Head Pitch, Vel X, Vel Y, Vel Z
    SpawnLivingEntity(VarInt, UUID, VarInt, Double, Double, Double, Angle, Angle, Angle, Short, Short, Short),

    TimeUpdate(Long, Long), // World Age, Time of Day


    KeepAliveClientbound(Long),


    Unknown(Vec<u8>),
    Empty,
}

impl DecodedPacket {
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
            }    ,
            
            TeleportConfirm(id) => {
                out = Packet::new_with_id(0x00);
                out.add(&id.to_bytes());
            }
            // Packets we don't care to encode
            _ => {
                print!("Unknown Packet to encode: {:?}", self);
                return None;
            }
        }
        Some(out)
    }
}


pub struct Packet {
    bytes: Vec<u8>,
}

impl Packet {

    pub fn new() -> Packet {
        Packet {  
            bytes: vec![0x00],
        }
    }

    pub fn new_with_id(id: u8) -> Packet {
        Packet { 
            bytes: vec![0x01, id],
        }
    }


    pub fn add(&mut self, bytes: &Vec<u8>) {
        for b in bytes.iter() {
            self.bytes.push(*b);
        }
        self.bytes[0] += bytes.len() as u8;
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.bytes[0] += 1;
        self.bytes.push(byte);
    }

    pub fn get_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn size(&self) -> usize {
        self.bytes[0] as usize - 1
    }

}

pub fn decode_packet(packet: Vec<u8>, state: &ServerState) -> DecodedPacket {
    use DecodedPacket::*;

    if packet.len() == 0 {return Empty}

    let len = packet.len();
    let out: DecodedPacket;

    let mut pd = PacketDecoder::new(&packet);

    match packet[0] {

        0x00 => {
            match state {
                ServerState::Login => {
                    out = Disconnect(pd.next_string());
                    // out = Disconnect(MCString::from_bytes(&packet[1..]).expect("Failed to decode message from disconnect packet."));
                },
                ServerState::Play => {
                    out = SpawnEntity(/* TODO */);
                },
                ServerState::Status => {
                    out = Status(pd.next_string());
                    // out = Status(MCString::from_bytes(&packet[1..]).expect("Failed to decode message from Status packet"));
                }
            }
        }

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

        0x0e => {
            match state {
                ServerState::Play => {
                    out = ServerDifficulty(
                        pd.next_ubyte(),
                        pd.next_bool(),
                    );
                    // out = ServerDifficulty(
                    //     UByte::from_bytes(&[packet[1]]),
                    //     Boolean::from_bytes(&extract_8(&packet, 2)).expect("Invalid bool in Server Difficulty packet")
                    // );
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
                    // out = ChatIncoming(
                    //     MCString::from_bytes(&packet[1..]).expect("Failed to extract chat message"),
                    //     Byte::from_bytes(&extract_8(&packet, len-17)),
                    //     UUID::from_bytes(&extract_128(&packet, len-16)),
                    // );
                },
                _ => out = Unknown(packet),
            }
        }

        0x1a => {
            out = Disconnect(pd.next_string());
            // out = Disconnect(MCString::from_bytes(&packet[1..]).expect("Couldn't decode disconnect reason"));
        }

        0x21 => {
            out = KeepAliveClientbound(pd.next_long());
            // out = KeepAliveClientbound(Long::from_bytes(&extract_64(&packet, 1)));
        },

        0x26 => {
            out = JoinGame(pd.next_int());
            // out = JoinGame(Int::from_bytes(&extract_32(&packet, 1)));
        },

        0x29 => {
            out = EntityPosition(
                pd.next_varint(),
                pd.next_short(),
                pd.next_short(),
                pd.next_short(),
                pd.next_bool(),
            );
            // let (var_int, var_int_size) = VarInt::from_vec(&packet, 1);
            // out = EntityPosition(
            //     var_int,
            //     Short::from_bytes(&extract_16(&packet, var_int_size+1)),
            //     Short::from_bytes(&extract_16(&packet, var_int_size+3)),
            //     Short::from_bytes(&extract_16(&packet, var_int_size+5)),
            //     Boolean::from_bytes(&extract_8(&packet, var_int_size+7)).expect("EntityPosition didn't end in Boolean value")
            // );
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
            // let (var_int, var_int_size) = VarInt::from_vec(&packet, 1);
            // out = EntityPositionAndRotation(
            //     var_int,
            //     Short::from_bytes(&extract_16(&packet, var_int_size+1)),
            //     Short::from_bytes(&extract_16(&packet, var_int_size+3)),
            //     Short::from_bytes(&extract_16(&packet, var_int_size+5)),
            //     Angle::from_bytes(&extract_8(&packet, var_int_size+7)),
            //     Angle::from_bytes(&extract_8(&packet, var_int_size+8)),
            //     Boolean::from_bytes(&extract_8(&packet, var_int_size+9)).expect("EntityPosition didn't end in Boolean value")
            // );
        },

        0x2b => {
            out = EntityRotation(
                pd.next_varint(),
                pd.next_angle(),
                pd.next_angle(),
                pd.next_bool(),
            );
            // let (var_int, var_int_size) = VarInt::from_vec(&packet, 1);
            // out = EntityRotation(
            //     var_int,                
            //     Angle::from_bytes(&extract_8(&packet, var_int_size+1)),
            //     Angle::from_bytes(&extract_8(&packet, var_int_size+2)),
            //     Boolean::from_bytes(&extract_8(&packet, var_int_size+3)).expect("EntityPosition didn't end in Boolean value")
            // );
        },


        0x3a => {
            let vi_num = pd.next_varint();
            let mut ids: Vec<VarInt> = Vec::new();

            for _ in 0..vi_num.0 as usize {
                ids.push(pd.next_varint());
            }
            out = DestroyEntities(vi_num, ids);

            // let (vi_num, vi_len) = VarInt::from_vec(&packet, 1);
            // let mut ids: Vec<VarInt> = Vec::new();

            // let mut acc = vi_len + 1;
            // for i in 0..vi_num.0 as usize {
            //     let (vi_id, vi_id_len) = VarInt::from_vec(&packet, acc);
            //     ids.push(vi_id);
            //     acc += vi_id_len;
            // }
            // out = DestroyEntities(vi_num, ids);
        }


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

            // let (var_int, var_int_size) = VarInt::from_vec(&packet, 34);
            // out = PlayerPositionAndLook(
            //     Double::from_bytes(&extract_64(&packet, 1)),
            //     Double::from_bytes(&extract_64(&packet, 9)),
            //     Double::from_bytes(&extract_64(&packet, 17)),
            //     Float::from_bytes(&extract_32(&packet, 25)),
            //     Float::from_bytes(&extract_32(&packet, 29)),
            //     Byte::from_bytes(&extract_8(&packet, 33)),
            //     var_int,
            //     Boolean::from_bytes(&extract_8(&packet, 34+var_int_size)).expect("Invalid Boolean in PlayerPositionAndLook packet"),
            // );
        },

        0x52 => {
            out = UpdateHealth(
                pd.next_float(),
                pd.next_varint(),
                pd.next_float(),
            );
            // let (varint, varint_size) = VarInt::from_vec(&packet, 5);
            // out = UpdateHealth(
            //     Float::from_bytes(&extract_32(&packet, 1)),
            //     varint,
            //     Float::from_bytes(&extract_32(&packet, 5+varint_size)),
            // );
        },


        0x58 => {
            out = TimeUpdate(
                pd.next_long(),
                pd.next_long(),
            );
            // out = TimeUpdate(
            //     Long::from_bytes(&extract_64(&packet, 1)),
            //     Long::from_bytes(&extract_64(&packet, 9)),
            // );
        }



        _ => {
            // println!("Couldn't decode clientbound packet: {:02x}", packet[0]);
            out = Unknown(packet);
        }
    }

    match &out {
        Unknown(pack) => {
            // println!("Unknow packet: {:02x}", pack[0]);
        },
        _ => {}
    }

    out
}

struct PacketDecoder<'a>  {
    buf: &'a Vec<u8>,
    ind: usize,
}

impl PacketDecoder<'_> {

    pub fn new<'a>(buf: &'a Vec<u8>) -> PacketDecoder<'a> {
        PacketDecoder {
            buf,
            ind: 1
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
        todo!()
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