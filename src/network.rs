pub mod packets;
use miniz_oxide::{
    deflate::{compress_to_vec, compress_to_vec_zlib},
    inflate::{decompress_to_vec, decompress_to_vec_zlib},
};
use packets::*;

pub mod types;

use std::{
    io::{Error, Read, Write},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use crate::app::{
    client::server::*,
    logger::{Log, LogType},
};

use self::types::*;

pub const PROTOCOL_1_17_1: VarInt = VarInt(756);

pub struct NetworkManager {
    pub stream: TcpStream,
    pub close: bool,
    pub channel: NetworkChannel,

    compress: bool,
    threshold: usize,

    state: ServerState,
    pub count: u32,
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
    /// * `Result<(NetworkChannel, Server), Error>` - Ok holding a channel to communicate with the new network thread, and a `Server` struct
    ///     Or errors if the TcpStream could not be established.
    ///
    pub fn connect(destination: &str) -> Result<(NetworkChannel, Server), Error> {
        let (tx, ri) = mpsc::channel::<NetworkCommand>();
        let (ti, rx) = mpsc::channel::<NetworkCommand>();

        let dest: String = destination.to_string();

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
                            state: ServerState::Status,
                            count: 0,
                        });

                        // Send all clear to other thread
                        nm.channel
                            .send
                            .send(NetworkCommand::Ok)
                            .expect("NetworkChannel Receiver cannot be reached");

                        nm.stream
                            .set_read_timeout(Some(Duration::from_millis(10)))
                            .expect("Failed to set timeout duration for socket");

                        // Loop until stopped
                        while !nm.close {
                            nm.update();
                        }

                        nm.stream
                            .shutdown(std::net::Shutdown::Both)
                            .expect("Couldn't shutdown TCPStream");
                    }
                    Err(e) => {
                        ti.send(NetworkCommand::Error(e))
                            .expect("NetworkChannel Receiver cannot be reached");
                    }
                }
            })
            .expect("Failed to start NetworkManager thread");

        // Ensure the thread is running properly
        match rx
            .recv()
            .expect("Somehow the channel to the network manager is already lost?")
        {
            NetworkCommand::Error(e) => return Err(e),
            _ => {}
        }

        Ok((
            NetworkChannel { send: tx, recv: rx },
            Server::new(destination.to_string()),
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
        loop {
            match self.next_packet() {
                None => {
                    break;
                }
                Some(packet) => {
                    self.handle_packet(packet);
                }
            }
        }
    }

    /// Attempts to get the next packet in the TcpStream
    /// Panics if the TcpStream could not read the next data to prevent correupted packets and unexpected behaviour
    /// # Returns
    ///
    /// Returns a Decoded Packet ready for processing, or None if there was no packet to receive.
    ///
    fn next_packet(&mut self) -> Option<DecodedPacket> {
        // Check there is packet and get size of it
        match VarInt::from_stream(&mut self.stream) {
            Ok(Some(VarInt(0))) => {
                return None;
            }
            Ok(Some(VarInt(len))) => {
                let mut buf = vec![0; len as usize];

                match self.stream.read_exact(&mut buf) {
                    Ok(_) => {
                        if self.compress {
                            let (data_length, vi_len) = VarInt::from_vec(&buf, 0);

                            if data_length.0 == 0 {
                                // Return packet without decompressing
                                buf.remove(0);
                                return Some(decode_packet(buf, &self.state));
                            } else {
                                // Return packet after decompressing

                                match decompress_to_vec_zlib(&buf[vi_len..]) {
                                    Ok(uncompressed) => {
                                        return Some(decode_packet(uncompressed, &self.state));
                                    }
                                    Err(e) => {
                                        panic!("Failed to decompress packet: {:?}", e);
                                    }
                                }
                            }
                        }

                        // Return packet without decompressing
                        return Some(decode_packet(buf, &self.state));
                    }
                    Err(e) => {
                        println!("Error reading packet from stream: {}", e);
                        panic!("Force stopped to prevent unexpected behaviour.");
                    }
                }
            }
            Ok(None) => {
                println!("Failed reading packet!");
                return None;
            }
            Err(_) => {
                return None;
            }
        }
    }

    /// Attempts to login to the server
    ///
    /// # Returns
    ///
    /// * `Some(())` if it successfully logs in, `None` if it fails
    fn login(&mut self, protocol: VarInt, port: Short, name: MCString) -> Option<()> {
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
        let handshake =
            DecodedPacket::Handshake(protocol, MCString(local_addr), port, HandshakeMode::Login);

        let login = DecodedPacket::LoginStart(name);

        self.send_packet(handshake)
            .expect("Failed to send handshake");
        self.state = ServerState::Login;
        self.send_packet(login)
            .expect("Failed to send login request");

        // Handle all incoming packets until success or failure
        loop {
            match self.next_packet() {
                Some(packet) => {
                    match packet {
                        // Please no
                        DecodedPacket::EncryptionRequest() => {
                            panic!("I ain't implemented this shit yet");
                        }
                        DecodedPacket::SetCompression(threshold) => {
                            if threshold.0 <= 0 {
                                self.compress = false;
                                self.channel
                                    .send
                                    .send(NetworkCommand::Log(Log::new(LogType::Info(
                                        "Disabled Compression".to_string(),
                                    ))))
                                    .expect("Failed to send log to main thread.");
                            } else {
                                self.compress = true;
                                self.threshold = threshold.0 as usize;
                                self.channel
                                    .send
                                    .send(NetworkCommand::Log(Log::new(LogType::Info(
                                        format!("Set Compression: {}", threshold.0).to_string(),
                                    ))))
                                    .expect("Failed to send log to main thread.");
                            }
                        }
                        DecodedPacket::Disconnect(_) => {
                            self.channel
                                .send
                                .send(NetworkCommand::ReceivePacket(packet))
                                .unwrap();
                            return None;
                        }
                        DecodedPacket::LoginPluginRequest() => {
                            panic!("I don't want to think about LoginPlugin");
                        }
                        DecodedPacket::LoginSuccess() => {
                            println!("Connecting to server with no authentication!");
                            self.state = ServerState::Play;

                            self.channel
                                .send
                                .send(NetworkCommand::ReceivePacket(packet))
                                .unwrap();
                            return Some(());
                        }
                        _ => {
                            println!("Got unexpected packet during login: {:?}", packet);
                        }
                    }
                }
                None => {}
            }
        }
    }

    /// Sends a packet to the server
    ///
    /// # Returns
    ///
    /// * `Some(())` if the packet is successfully sent
    /// * `None` if it is not
    fn send_packet(&mut self, packet: DecodedPacket) -> Option<()> {
        // Attempt to encode packet
        match packet.encode() {
            Some(pack) => {
                // Compress packet
                if self.compress {
                    let mut bytes = pack.get_bytes();

                    if bytes.len() >= self.threshold {
                        // Send with compression
                        let data_length = VarInt(bytes.len() as i32);
                        let mut compressed = compress_to_vec_zlib(&bytes, 0);
                        let packet_length =
                            VarInt((compressed.len() + data_length.num_bytes()) as i32);

                        // Build packet bytes from packet_length, data_length and the compressed packet
                        let mut new_bytes = Vec::new();
                        new_bytes.append(&mut packet_length.to_bytes());
                        new_bytes.append(&mut data_length.to_bytes());
                        new_bytes.append(&mut compressed);

                        return match self.stream.write(new_bytes.as_slice()) {
                            Ok(_) => Some(()),
                            Err(_) => None,
                        };
                    } else {
                        // Send without compression while compression is enabled
                        let mut new_bytes = Vec::new();
                        new_bytes.append(&mut VarInt(bytes.len() as i32 + 1).to_bytes());
                        new_bytes.push(0);
                        new_bytes.append(&mut bytes);

                        return match self.stream.write(new_bytes.as_slice()) {
                            Ok(_) => Some(()),
                            Err(_) => None,
                        };
                    }

                    None
                } else {
                    // Send without compression
                    let bytes = pack.get_bytes_with_length();
                    match self.stream.write(bytes.as_slice()) {
                        Ok(_) => Some(()),
                        Err(_) => None,
                    }
                }
            }
            // Packet encode failure
            None => {
                println!("Failed to encode packet: {:02x?}", packet);
                return None;
            }
        }
    }

    /// Handles a message (from another thread)
    fn handle_message(&mut self, msg: NetworkCommand) {
        match msg {
            NetworkCommand::Login(protocol, port, name) => {
                println!("Logging in");
                self.login(protocol, port, name);
                println!("Finished login");
            }
            NetworkCommand::Disconnect => {
                self.close = true;
            }
            NetworkCommand::SendPacket(dp) => {
                self.send_packet(dp);
            }
            _ => {}
        }
    }

    /// Handles an incoming packet
    fn handle_packet(&mut self, packet: DecodedPacket) {
        use DecodedPacket::*;

        match &packet {
            Unknown(buf) => {
                // println!("Got unknown packet: {:02x}", buf[0]);
            }
            KeepAliveClientbound(keep_alive_id) => {
                self.send_packet(KeepAliveServerbound(keep_alive_id.clone()))
                    .expect("Failed to send heartbeat");
            }

            Disconnect(reason) => {
                self.close = true;
                println!("Disconnected from server: {}", &reason.0);
                self.channel
                    .send
                    .send(NetworkCommand::ReceivePacket(packet))
                    .expect("Failed to send message back to client");
            }

            SetCompression(threshold) => {
                if threshold.0 <= 0 {
                    self.compress = false;
                } else {
                    self.compress = true;
                    self.threshold = threshold.0 as usize;
                }
            }

            // Forwards these packets to the main thread
            TimeUpdate(_, _)
            | UpdateHealth(_, _, _)
            | ServerDifficulty(_, _)
            | ChatIncoming(_, _, _)
            | JoinGame(_)
            | EntityPosition(_, _, _, _, _)
            | EntityPositionAndRotation(_, _, _, _, _, _, _)
            | EntityRotation(_, _, _, _)
            | SpawnLivingEntity(_, _, _, _, _, _, _, _, _, _, _, _)
            | DestroyEntities(_, _)
            | EntityMetadata()
            | EntityVelocity(_, _, _, _)
            | EntityTeleport(_, _, _, _, _, _, _)
            | EntityHeadLook(_, _)
            | SoundEffect()
            | PlayerPositionAndLook(_, _, _, _, _, _, _, _) => {
                self.channel
                    .send
                    .send(NetworkCommand::ReceivePacket(packet))
                    .expect("Failed to send message back to client");
            }

            _ => {
                println!("Got packet {:?}", packet);
            }
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
    Login(VarInt, Short, MCString),

    SendPacket(DecodedPacket),
    ReceivePacket(DecodedPacket),

    Log(Log),

    Spawn,
}
