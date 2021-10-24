
pub mod packets;
use packets::*;

pub mod types;

use std::{io::{Error, Read, Write}, net::TcpStream, sync::mpsc::{self, Receiver, Sender}, thread, time::Duration};

use crate::app::{client::server::*, logger::{Log, LogType}};

use self::types::*;

pub const PROTOCOL_1_17_1: VarInt = VarInt(756);

pub struct NetworkManager {
    pub stream: TcpStream,
    pub compress: bool,
    pub close: bool,
    pub channel: NetworkChannel,


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
    pub fn connect(destination: &str) -> Result<(NetworkChannel, Server), Error> {

        let (tx, ri) = mpsc::channel::<NetworkCommand>();
        let (ti, rx) = mpsc::channel::<NetworkCommand>();


        let dest: String = destination.to_string();
        thread::Builder::new().name("NetworkManager".to_string()).spawn(move || {
            match TcpStream::connect(dest) {
                Ok(stream) => {

                    let mut nm = Box::new(NetworkManager {
                        stream,
                        compress: false,
                        close: false,
                        channel: NetworkChannel {send: ti, recv: ri},
                        state: ServerState::Status,
                        count: 0,
                    });
                    
                    nm.channel.send.send(NetworkCommand::Ok).expect("NetworkChannel Receiver cannot be reached");

                    // nm.stream.set_nonblocking(true).expect("Failed to set TCPStream to nonblocking");
                    nm.stream.set_read_timeout(Some(Duration::from_millis(10))).expect("Failed to set timeout duration for socket");

                    while !nm.close {
                        nm.update();
                    }

                    nm.stream.shutdown(std::net::Shutdown::Both).expect("Couldn't shutdown TCPStream");

                },
                Err(e) => {
                    ti.send(NetworkCommand::Error(e)).expect("NetworkChannel Receiver cannot be reached");
                }
            }

        }).expect("Failed to start NetworkManager thread");

        match rx.recv().expect("Somehow the channel to the network manager is already lost?") {
            NetworkCommand::Error(e) => {
                return Err(e)
            },
            _ => {}
        }

        Ok((
        NetworkChannel {  
            send: tx,
            recv: rx,
        },
        Server::new(destination.to_string()),
        ))
    }



    fn update(&mut self) {

        let mut maybe_msg = self.channel.recv.try_recv();
        while maybe_msg.is_ok() {
            self.handle_message(maybe_msg.unwrap());
            maybe_msg = self.channel.recv.try_recv();
        }


        loop {

            match self.next_packet() {
                None => {break;},
                Some(packet) => {
                    self.handle_packet(packet);
                }
            }

        }

    }

    fn next_packet(&mut self) -> Option<DecodedPacket> {

        match VarInt::from_stream(&mut self.stream) {
            Ok(Some(VarInt(0))) => {return None;}
            Ok(Some(VarInt(len))) => {
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(0);
                }
                
                match self.stream.read_exact(&mut buf) {
                    Ok(_) => {

                        // if self.count >= 20 {panic!("Force stopped");}
                        // self.count += 1;

                        // println!("Got packet: {:02x} - {:02x}", len, buf[0]);
                        // println!("Packet: {:02x?}", buf);
                        // println!("Buffer length: {:02x}", buf.len());


                        return Some(decode_packet(buf, &self.state));
                        // return None;

                    },
                    Err(e) => {
                        println!("Error reading packet from stream: {}", e);
                        panic!("Force stopped to prevent unexpected behaviour.");
                        return None;
                    }
                }
            },
            Ok(None) => {
                println!("Failed reading packet!");
                return None;
            },
            Err(_) => {
                return None;
            }
        }

    }


    fn login(&mut self, protocol: VarInt, port: Short, name: MCString) -> Option<()> {
        use std::net::SocketAddr;

        let local_addr = match self.stream.local_addr() {
            Err(e) => {panic!("Failed to get local adress from TcpStream: {}", e);},
            Ok(addr) => {
                match addr {
                    SocketAddr::V4(local) => {
                        local.ip().to_string()
                    },
                    SocketAddr::V6(local) => {
                        local.ip().to_string()
                    }
                }
            }
        };

        // let local_addr = "localhost".to_string();

        let handshake = DecodedPacket::Handshake(
            protocol, 
            MCString(local_addr), 
            port, 
            HandshakeMode::Login);

        let login = DecodedPacket::LoginStart(name);

        self.send_packet(handshake).expect("Failed to send handshake");
        self.state = ServerState::Login;
        self.send_packet(login).expect("Failed to send login request");


        loop {

            match self.next_packet() {
                Some(packet) => {
                    match packet {
                        DecodedPacket::EncryptionRequest() => {
                            panic!("I ain't implemented this shit yet");
                        },
                        DecodedPacket::SetCompression(_threshold) => {
                            panic!("Nope, no compression just yet!");
                        },
                        DecodedPacket::Disconnect(_) => {
                            // println!("Disconnected: {}", reason.0.clone());
                            self.channel.send.send(NetworkCommand::ReceivePacket(packet)).unwrap();
                            return None;
                        },
                        DecodedPacket::LoginPluginRequest() => {
                            panic!("God no, I don't even want to think about loginplugin");
                        },
                        DecodedPacket::LoginSuccess() => {
                            println!("Connecting to server with no authentication!");
                            self.state = ServerState::Play;

                            self.channel.send.send(NetworkCommand::ReceivePacket(packet)).unwrap();
                            return Some(());
                        }

                        _ => {}

                    }
                },
                None => {},
            }

        }

        // None
    }

    fn send_packet(&mut self, packet: DecodedPacket) -> Option<()> {
        match packet.encode() {
            Some(pack) => {
                let bytes = pack.get_bytes();
                match self.stream.write(bytes) {
                    Ok(_) => {
                        // self.channel.send.send(NetworkCommand::Log(Log::new(LogType::PacketSent(packet)))).expect("Failed to send packet sent log to main thread");
                        Some(())
                    },
                    Err(_) => None
                }
            },
            None => {
                println!("Failed to encode packet: {:02x?}", packet);
                return None;
            }
        }
    }


    fn handle_message(&mut self, msg: NetworkCommand) {

        match msg {
            NetworkCommand::Login(protocol, port, name) => {
                println!("Logging in");
                self.login(protocol, port, name);
                println!("Finished login");
            },

            NetworkCommand::Disconnect => {
                self.close = true;
            },
            NetworkCommand::SendPacket(dp) => {
                self.send_packet(dp);
            }
            _ => {
                
            }
        }

    }



    fn handle_packet(&mut self, packet: DecodedPacket) {
        use DecodedPacket::*;

        match &packet {
            DecodedPacket::Unknown(buf) => {
                // println!("Got unknown packet: {:02x}", buf[0]);
            },
            DecodedPacket::KeepAliveClientbound(keep_alive_id) => {
                self.send_packet(DecodedPacket::KeepAliveServerbound(keep_alive_id.clone())).expect("Failed to send heartbeat");
            },

            Disconnect(reason) => {
                self.close = true;
                println!("Disconnected from server: {}", &reason.0);
                self.channel.send.send(NetworkCommand::ReceivePacket(packet)).expect("Failed to send message back to client");
            },

            


            // Packets to be forwarded to the client
            TimeUpdate(_, _) |
            UpdateHealth(_, _, _) |
            ServerDifficulty(_, _) |
            ChatIncoming(_, _, _) |
            JoinGame(_) |
            EntityPosition(_, _, _, _, _) |
            EntityPositionAndRotation(_, _, _, _, _, _, _) |
            EntityRotation(_, _, _, _) |
            SpawnLivingEntity(_, _, _, _, _, _, _, _, _, _, _, _) |
            DestroyEntities(_, _) |
            PlayerPositionAndLook(_, _, _, _, _, _, _, _) => {
                self.channel.send.send(NetworkCommand::ReceivePacket(packet)).expect("Failed to send message back to client");
            }

            _ => {
                println!("Got packet {:?}", packet);
            }
        }



    }



}


pub struct NetworkChannel {
    pub send: Sender<NetworkCommand>,
    pub recv: Receiver<NetworkCommand>,
}


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