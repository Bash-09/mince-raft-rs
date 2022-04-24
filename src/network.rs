use log::debug;
use log::{error, info, warn};
use mcnetwork::packets;
use miniz_oxide::{deflate::compress_to_vec_zlib, inflate::decompress_to_vec_zlib};

use mcnetwork::packets::*;
use mcnetwork::types::*;

use std::io::Cursor;
use std::{
    io::{Error, Read, Write},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::server::*;

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
                            state: ServerState::Status,
                            count: 0,
                        });

                        // Send all clear to other thread
                        nm.send_message(NetworkCommand::Ok);

                        // nm.stream
                        //     .set_read_timeout(Some(Duration::from_millis(10)))
                        //     .expect("Failed to set timeout duration for socket");

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
                Ok(PacketData::Empty) => {
                    break;
                }
                Ok(packet) => {
                    self.handle_packet(packet);
                }
                Err(e) => {
                    panic!("Error handling packet: {:?}", e);
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
    fn next_packet(&mut self) -> Result<PacketData, Box<dyn std::error::Error>> {
        let mut check = [0u8];
        match self.stream.peek(&mut check) {
            Ok(0) => {
                panic!("TcpStream ded???");
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    return Ok(PacketData::Empty);
                }
                return Err(Box::new(e));
            }
            _ => {}
        }

        self.stream
            .set_nonblocking(false)
            .expect("Failed to set TcpStream to blocking mode");
        let VarInt(len) = VarInt::read(&mut self.stream)?;

        let mut buf = vec![0u8; len as usize];
        self.stream.read_exact(&mut buf)?;
        self.stream
            .set_nonblocking(true)
            .expect("Failed to set TcpStream to nonblocking mode");

        if self.compress {
            let mut cur = Cursor::new(&buf);
            let VarInt(data_len) = VarInt::read(&mut cur)?;

            if data_len == 0 {
                return Ok(decode_packet_clientbound(
                    &buf[cur.position() as usize..],
                    &self.state,
                )?);
            }

            match decompress_to_vec_zlib(&buf[cur.position() as usize..]) {
                Ok(uncompressed) => {
                    return Ok(decode_packet_clientbound(&uncompressed, &self.state)?)
                }
                Err(e) => {
                    todo!("Properly decompression error handling");
                }
            }
        }

        Ok(decode_packet_clientbound(&buf, &self.state)?)
    }

    /// Attempts to login to the server
    ///
    /// # Returns
    ///
    /// * `Some(())` if it successfully logs in, `None` if it fails
    fn login(&mut self, protocol: VarInt, port: u16, name: String) -> Option<()> {
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
        let handshake = Handshake {
            protocol_version: protocol,
            address: local_addr,
            port,
            next: VarInt(2),
        };

        let login = LoginStart { name };

        self.send_packet(&encode(handshake))
            .expect("Failed to send handshake");
        self.state = ServerState::Login;
        self.send_packet(&encode(login))
            .expect("Failed to send login request");

        // Handle all incoming packets until success or failure
        loop {
            match self.next_packet() {
                Ok(PacketData::Empty) => {}
                Ok(packet) => {
                    match &packet {
                        // Please no
                        PacketData::EncryptionRequest(_) => {
                            panic!("I ain't implemented this shit yet");
                        }
                        PacketData::SetCompression(pack) => {
                            if pack.threshold.0 <= 0 {
                                self.compress = false;
                                info!("Disabled Compression");
                            } else {
                                self.compress = true;
                                self.threshold = pack.threshold.0 as usize;
                                info!("Set compression: {}", pack.threshold.0);
                            }
                        }
                        PacketData::Disconnect(_) => {
                            self.send_message(NetworkCommand::ReceivePacket(packet));
                            self.close = true;
                            return None;
                        }
                        PacketData::LoginPluginRequest(_) => {
                            panic!("I don't want to think about LoginPlugin");
                        }
                        PacketData::LoginSuccess(_) => {
                            warn!("Connecting to server with no authentication!");

                            self.state = ServerState::Play;
                            self.send_message(NetworkCommand::ReceivePacket(packet));

                            return Some(());
                        }
                        _ => {
                            warn!("Got unexpected packet during login: {:?}", packet);
                        }
                    }
                }
                Err(e) => {
                    panic!("Error reading packet: {:?}", e);
                }
            }
        }
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
                VarInt(packet.len() as i32).write(&mut data_length)?;
                let compressed = compress_to_vec_zlib(packet, 0);
                let mut packet_length = Vec::new();
                VarInt((data_length.len() + compressed.len()) as i32).write(&mut packet_length)?;

                s.write(&packet_length)?;
                s.write(&data_length)?;
                s.write(&compressed)?;
            } else {
                VarInt((packet.len() + 1) as i32).write(s)?;
                s.write(&[0u8])?;
                s.write(packet)?;
            }
            return Ok(());
        }

        VarInt(packet.len() as i32).write(s)?;
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
                self.send_packet(&encode(Disconnect {
                    reason: String::from("Player Disconnected"),
                }))
                .expect("Failed to send packet");
                self.close = true;
            }
            NetworkCommand::SendPacket(dp) => {
                self.send_packet(&dp).expect("Failed to send packet");
            }
            _ => {}
        }
    }

    /// Handles an incoming packet
    fn handle_packet(&mut self, packet: PacketData) {
        use PacketData::*;

        match &packet {
            Unknown(_buf) => {
                // println!("Got unknown packet: {:02x}", buf[0]);
            }
            KeepAliveClientbound(pack) => {
                self.send_packet(&encode(packets::KeepAliveServerbound {
                    keep_alive_id: pack.keep_alive_id.clone(),
                }))
                .expect("Failed to send heartbeat");
            }

            SetCompression(pack) => {
                if pack.threshold.0 <= 0 {
                    self.compress = false;
                    info!("Disabled Packet Compression.");
                } else {
                    info!("Set Packet Compression: {}", pack.threshold.0);
                    self.compress = true;
                    self.threshold = pack.threshold.0 as usize;
                }
            }

            // Forward other packets to the main thread
            _ => {
                self.send_message(NetworkCommand::ReceivePacket(packet));
            }
        }
    }

    fn send_message(&mut self, comm: NetworkCommand) {
        if let Err(_) = self.channel.send.send(comm) {
            error!("Couldn't communicated with main thread, assuming connection was closed and disconnecting from server.");
            self.close = true;
            self.send_packet(&encode(Disconnect {
                reason: String::from("Player Disconnected"),
            })).expect("Failed to send Disconnect packet");
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
    Login(VarInt, u16, String),

    SendPacket(Vec<u8>),
    ReceivePacket(PacketData),

    Spawn,
}
