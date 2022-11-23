
use log::debug;
use log::{error, info, warn};
use mcproto_rs::protocol::{RawPacket, Id, PacketErr};
use mcproto_rs::types::{BytesSerializer, self, VarInt, TextComponent, BaseComponent};
use mcproto_rs::{v1_16_3::*, status};
use mcproto_rs::{protocol, v1_16_3, Serialize};
use miniz_oxide::{deflate::compress_to_vec_zlib, inflate::decompress_to_vec_zlib};

use std::io::{Cursor, self, ErrorKind};
use std::time::Instant;
use std::{
    io::{Error, Read, Write},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::server::*;

pub type PacketType = v1_16_3::Packet753;
pub type RawPacketType<'a> = v1_16_3::RawPacket753<'a>;

pub struct NetworkManager {
    pub stream: TcpStream,
    pub close: bool,
    pub channel: NetworkChannel,

    compress: bool,
    threshold: usize,

    state: protocol::State,
    pub count: u32,
}

#[derive(Debug)]
pub struct ServerStatus {
    pub icon: Option<Vec<u8>>,
    pub motd: String,
    pub version: String,
    pub num_players: u32,
    pub max_players: u32,
    pub online_players: Vec<String>,
    pub ping: u32,
}

impl NetworkManager {
    /// Attempts to connect to a server, returning a NetworkChannel to communicate with the NetworkManager and receive packets from
    ///
    /// # Arguments
    ///
    /// * `destination` - The target server to connect to
    ///
    /// # Returns
    ///
    /// * `Result<Server, Error>` - Ok holding a Server which can communicate with the new network thread
    ///     Or errors if the TcpStream could not be established.
    ///
    pub fn connect(destination: &str) -> Result<Server, Error> {
        let (tx, ri) = mpsc::channel::<NetworkCommand>();
        let (ti, rx) = mpsc::channel::<NetworkCommand>();

        let mut dest: String = destination.to_string();

        // Check for port included in address
        if !dest.contains(":") {
            debug!("Server address didn't contain port, appending :25565");
            dest.push_str(":25565");
        }

        //Start new thread to be the network manager
        thread::Builder::new()
            .name("NetworkManager".to_string())
            .spawn(move || {
                match TcpStream::connect(dest) {
                    Ok(stream) => {
                        let mut nm = Box::new(NetworkManager {
                            stream,
                            compress: false,
                            threshold: 0,
                            close: false,
                            channel: NetworkChannel { send: ti, recv: ri },
                            state: protocol::State::Status,
                            count: 0,
                        });

                        nm.stream
                            .set_nonblocking(true)
                            .expect("Failed to set TcpStream nonblocking");

                        // Loop until stopped
                        while !nm.close {
                            nm.update();
                        }
                        info!("Closing network connection.");

                        nm.stream
                            .shutdown(std::net::Shutdown::Both)
                            .expect("Couldn't shutdown TCPStream");
                    }
                    Err(e) => {
                        error!("Cum");
                        ti.send(NetworkCommand::Error(e))
                            .expect("NetworkChannel Receiver cannot be reached");
                    }
                }
            })?;

        Ok(Server::new(
            destination.to_string(),
            NetworkChannel { send: tx, recv: rx },
        ))
    }

    /// Manages any incoming packets or messages from other threads
    fn update(&mut self) {
        // Handles all queued messages from other threads
        let mut maybe_msg = self.channel.recv.try_recv();
        while maybe_msg.is_ok() {
            self.handle_message(maybe_msg.unwrap());
            maybe_msg = self.channel.recv.try_recv();
        }

        // Handles incoming packets
        while !self.close {
            match self.next_packet() {
                Ok(packet_result) => {
                    match packet_result {
                        Ok(packet) => self.handle_packet(packet),
                        Err(e) => {
                            log::error!("Couldn't deserialize packet: {}", e);
                        },
                    }
                }
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        return;
                    } else {
                        panic!("Error handling packet: {:?}", e);
                    }
                }
            }
        }
    }

    /// Attempts to get the next packet in the TcpStream
    /// Panics if the TcpStream could not read the next data to prevent correupted packets and unexpected behaviour
    /// # Returns
    ///
    /// Returns a Decoded Packet ready for processing, or Error if it failed.
    ///
    fn next_packet(&mut self) -> io::Result<Result<PacketType, PacketErr>> {
        let mut check = [0u8];
        match self.stream.peek(&mut check) {
            Ok(0) => {
                panic!("TcpStream ded???");
            }
            Err(e) => {
                return Err(e);
            }
            _ => {}
        }

        self.stream
            .set_nonblocking(false)
            .expect("Failed to set TcpStream to blocking mode");
        let len = read_varint(&mut self.stream)?;

        let mut buf = vec![0u8; len as usize];
        self.stream.read_exact(&mut buf)?;
        self.stream
            .set_nonblocking(true)
            .expect("Failed to set TcpStream to nonblocking mode");

        if self.compress {
            let mut cur = Cursor::new(&buf);
            let data_len = read_varint(&mut cur)?;

            if data_len == 0 {
                let id = read_varint(&mut cur)?;
                let id = Id {
                    id,
                    state: self.state,
                    direction: protocol::PacketDirection::ClientBound,
                };
                return Ok(
                    match RawPacketType::create(id, &buf[cur.position() as usize..]) {
                        Ok(raw_packet) => raw_packet.deserialize(),
                        Err(e) => Err(e),
                    }
                );
            }

            match decompress_to_vec_zlib(&buf[cur.position() as usize..]) {
                Ok(uncompressed) => {
                    let mut cur = Cursor::new(&mut uncompressed);
                    let id = read_varint(&mut cur)?;
                    let id = Id {
                        id,
                        state: self.state,
                        direction: protocol::PacketDirection::ClientBound,
                    };
                    return Ok(
                        match RawPacketType::create(id, &uncompressed[cur.position() as usize..]) {
                            Ok(raw_packet) => raw_packet.deserialize(),
                            Err(e) => Err(e),
                        }
                    );
                }
                Err(e) => {
                    todo!("Properly decompression error handling");
                }
            }
        }


        let mut cur = Cursor::new(&mut buf);
        let id = read_varint(&mut cur)?;
        let id = Id {
            id,
            state: self.state,
            direction: protocol::PacketDirection::ClientBound,
        };
        return Ok(
            match RawPacketType::create(id, &buf[cur.position() as usize..]) {
                Ok(raw_packet) => raw_packet.deserialize(),
                Err(e) => Err(e),
            }
        );
    }

    /// Attempts to login to the server
    ///
    /// # Returns
    ///
    /// * `Some(())` if it successfully logs in, `None` if it fails
    fn login(&mut self, protocol: i32, port: u16, name: String) -> Option<()> {
        use std::net::SocketAddr;

        // Extracts local address from TcpStream
        let local_addr = match self.stream.local_addr() {
            Err(e) => {
                panic!("Failed to get local adress from TcpStream: {}", e);
            }
            Ok(addr) => match addr {
                SocketAddr::V4(local) => local.ip().to_string(),
                SocketAddr::V6(local) => local.ip().to_string(),
            },
        };

        // Construct and send handshake and login packets
        let handshake = HandshakeSpec {
            version: VarInt(protocol),
            server_address: local_addr,
            server_port: port,
            next_state: HandshakeNextState::Login,
        };

        let login = LoginStartSpec {
            name,
        };

        self.send_packet(&encode(handshake))
            .expect("Failed to send handshake");
        self.state = protocol::State::Login;
        self.send_packet(&encode(login))
            .expect("Failed to send login request");

        // Handle all incoming packets until success or failure
        loop {
            match self.next_packet() {
                Ok(packet) => {
                    match packet {
                        Ok(packet) => {
                            match packet {
                                // Please no
                                PacketType::LoginEncryptionRequest(_) => {
                                    panic!("I ain't implemented this shit yet");
                                }
                                PacketType::LoginSetCompression(pack) => {
                                    if pack.threshold.0 <= 0 {
                                        self.compress = false;
                                        info!("Disabled Compression");
                                    } else {
                                        self.compress = true;
                                        self.threshold = pack.threshold.0 as usize;
                                        info!("Set compression: {}", pack.threshold.0);
                                    }
                                }
                                PacketType::LoginDisconnect(_) => {
                                    self.send_message(NetworkCommand::ReceivePacket(packet));
                                    self.close = true;
                                    return None;
                                }
                                PacketType::LoginPluginRequest(_) => {
                                    panic!("I don't want to think about LoginPlugin");
                                }
                                PacketType::LoginSuccess(_) => {
                                    warn!("Connecting to server with no authentication!");

                                    self.state = protocol::State::Play;
                                    self.send_message(NetworkCommand::ReceivePacket(packet));

                                    return Some(());
                                }
                                _ => {
                                    warn!("Got unexpected packet during login: {:?}", packet);
                                }
                            };
                        },
                        Err(e) => {
                            panic!("Error decoding packet: {}", e);
                        }
                    }
                }
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        continue;
                    } else {
                        panic!("Error reading packet: {:?}", e);
                    }
                }
            }
        }
    }

    fn status(&mut self) -> Option<status::StatusSpec> {
        use std::net::SocketAddr;

        // Extracts local address from TcpStream
        let local_addr = match self.stream.local_addr() {
            Err(e) => {
                panic!("Failed to get local adress from TcpStream: {}", e);
            }
            Ok(addr) => match addr {
                SocketAddr::V4(local) => local.ip().to_string(),
                SocketAddr::V6(local) => local.ip().to_string(),
            },
        };

        // Construct and send handshake and login packets
        let handshake = HandshakeSpec {
            version: VarInt(0),
            server_address: local_addr,
            server_port: 0,
            next_state: HandshakeNextState::Status,
        };

        let now = Instant::now();
        self.send_packet(&encode(handshake))
            .expect("Failed to send handshake");
        self.send_packet(&encode(StatusRequestSpec{}))
            .expect("Failed to send status request");
        self.send_packet(&encode(StatusPingSpec{ payload: 0 }))
            .expect("Failed to send Status Ping");

        let ping;
        let mut status: status::StatusSpec;

        loop {
            match self.next_packet() {
                Ok(pack) => match pack {
                    Ok(pack) => match pack {
                        PacketType::StatusResponse(pack) => {
                            ping = (Instant::now() - now).as_millis() as u32;
                            status = pack.response;
                            break;
                        }
                        _ => {
                            warn!(
                                "Got unexpected packet waiting for status response: {:?}",
                                pack
                            );
                        }
                    },
                    Err(e) => {
                        panic!("Error decoding packet: {}", e);
                    }
                },
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        continue;
                    } else {
                        error!("Couldn't get response from server status request: {:?}", e);
                        return None;
                    }
                }
            }
        }

        Some(status)
    }

    /// Sends a packet to the server
    /// This should just be the packet contents signed with it's ID, not the packet length.
    /// Sent packets will have their length signed inside this function to handle compression
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the packet is successfully sent
    /// * `Err` if it is not
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let s = &mut self.stream;
        s.set_nonblocking(false)
            .expect("Failed to set Stream to blocking mode");

        // Handle zlib compression
        if self.compress {
            if packet.len() >= self.threshold {
                let mut data_length = Vec::new();
                write_varint(&mut data_length, packet.len() as i32)?;
                let compressed = compress_to_vec_zlib(packet, 0);
                let mut packet_length = Vec::new();
                write_varint(&mut packet_length, (data_length.len() + compressed.len()) as i32)?;

                s.write(&packet_length)?;
                s.write(&data_length)?;
                s.write(&compressed)?;
            } else {
                write_varint(s, (packet.len() + 1) as i32)?;
                s.write(&[0u8])?;
                s.write(packet)?;
            }
            return Ok(());
        }

        write_varint(s, packet.len() as i32)?;
        s.write(packet)?;
        s.set_nonblocking(true)
            .expect("Failed to set TcpStream nonblocking");
        Ok(())
    }

    /// Handles a message (from another thread)
    fn handle_message(&mut self, msg: NetworkCommand) {
        match msg {
            NetworkCommand::Login(protocol, port, name) => {
                info!("Attempting to login to server");
                self.login(protocol, port, name);
            }
            NetworkCommand::Disconnect => {
                self.send_packet(&encode(PlayDisconnectSpec {
                    reason: types::Chat::Text(TextComponent {
                        text: String::from("Player Disconnected"),
                        base: BaseComponent::default(),
                    }),
                }))
                .expect("Failed to send packet");
                self.close = true;
            }
            NetworkCommand::SendPacket(dp) => {
                self.send_packet(&dp).expect("Failed to send packet");
            }
            NetworkCommand::RequestStatus => {
                match self.status() {
                    Some(status) => {
                        self.send_message(NetworkCommand::ReceiveStatus(status));
                    }
                    None => {}
                }
                self.close = true;
            }
            _ => {}
        }
    }

    /// Handles an incoming packet
    fn handle_packet(&mut self, packet: PacketType) {
        match &packet {
            PacketType::PlayClientKeepAlive(pack) => {
                self.send_packet(&encode(PlayClientKeepAliveSpec{id: pack.id})).expect("Failed to send heartbeat.");
            },
            PacketType::LoginSetCompression(pack) => {
                if pack.threshold.0 <= 0 {
                    self.compress = false;
                    info!("Disabled packet compression.");
                } else {
                    info!("Set Packet Compression: {}", pack.threshold);
                    self.compress = true;
                    self.threshold = pack.threshold.0 as usize;
                }
            }
            _ => {
                self.send_message(NetworkCommand::ReceivePacket(packet));
            }
        }
    }

    fn send_message(&mut self, comm: NetworkCommand) {
        if let Err(_) = self.channel.send.send(comm) {
            error!("Couldn't communicated with main thread, assuming connection was closed and disconnecting from server.");
            self.close = true;
            self.send_packet(&encode(PlayDisconnectSpec {
                reason: types::Chat::Text(types::TextComponent {
                    text: String::from("Player Disconnected"),
                    base: types::BaseComponent::default(),
                })
            }))
            .expect("Failed to send Disconnect packet");
        }
    }
}

// Struct to hold communication channels between network manager and other threads
pub struct NetworkChannel {
    pub send: Sender<NetworkCommand>,
    pub recv: Receiver<NetworkCommand>,
}

// Types of Messages that can be sent
#[derive(Debug)]
pub enum NetworkCommand {
    Ok,
    Error(Error),
    Disconnect,
    // Login(protocol, port, name)
    Login(i32, u16, String),

    SendPacket(Vec<u8>),
    ReceivePacket(PacketType),

    RequestStatus,
    ReceiveStatus(status::StatusSpec),

    Spawn,
}

fn read_varint<R: Read>(r: &mut R) -> io::Result<i32> {
    const PART: u32 = 0x7F;
    let mut size = 0;
    let mut val = 0u32;
    let mut byte: [u8; 1] = [0];

    r.read_exact(&mut byte)?;
    loop {
        val |= (byte[0] as u32 & PART) << (size * 7);
        size += 1;

        if (byte[0] & 0x80) == 0 {
            break;
        }

        r.read_exact(&mut byte)?;
    }

    Ok(val as i32)
}

fn write_varint<W: Write>(w: &mut W, val: i32) -> io::Result<()> {
    let mut buf: Vec<u8> = Vec::new();

    const PART: u32 = 0x7F;
    let mut val = val as u32;
    loop {
        if (val & !PART) == 0 {
            buf.push(val as u8);
            break;
        }
        buf.push(val as u8 | !0x7F);
        val >>= 7;
    }
    w.write(&buf)?;
    Ok(())
}

fn encode<S: Serialize>(packet: S) -> Vec<u8> {
    let mut serializer = BytesSerializer {
        data: Vec::new(),
    };
    packet.mc_serialize(&mut serializer);
    serializer.into_bytes()
}
