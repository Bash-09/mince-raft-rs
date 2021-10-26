use std::{
    fmt,
    thread::{self, Thread},
};

use crate::network::packets::DecodedPacket;

use chrono::{offset::Local, DateTime};

pub struct Logger {
    log: Vec<Log>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger { log: Vec::new() }
    }

    pub fn get_log(&self) -> &Vec<Log> {
        &self.log
    }

    pub fn log(&mut self, log: Log) {
        self.log.push(log);
    }

    pub fn log_info(&mut self, info: &str) {
        self.log(Log::new(LogType::Info(info.to_string())));
    }

    pub fn log_incoming_packet(&mut self, packet: DecodedPacket) {
        self.log(Log::new(LogType::PacketReceived(packet)))
    }
}

#[derive(Debug)]
pub struct Log {
    thread: String,
    time: DateTime<Local>,
    log: LogType,
}

impl Log {
    pub fn new(log: LogType) -> Log {
        let name: String;
        match &thread::current().name() {
            Some(nam) => name = nam.to_string(),
            None => name = String::from("Unnamed"),
        }

        Log {
            thread: name,
            time: Local::now(),
            log,
        }
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] [{}]: {:?}",
            self.time.format("%H:%M:%S"),
            self.thread,
            self.log
        )
    }
}

#[derive(Debug)]
pub enum LogType {
    PacketSent(DecodedPacket),
    PacketReceived(DecodedPacket),
    Info(String),
}
