use serde_json::Value;

use crate::network::{packets::DecodedPacket, types::UUID};

pub struct Chat {
    history: Vec<ChatMessage>,

    input: String,
    pub send: bool,
}

impl Chat {
    pub fn new() -> Chat {
        Chat {
            history: Vec::with_capacity(255),
            input: String::with_capacity(255),
            send: false,
        }
    }

    pub fn get_history(&self) -> &Vec<ChatMessage> {
        &self.history
    }

    pub fn add_message(&mut self, message: &DecodedPacket) {
        match message {
            DecodedPacket::ChatIncoming(json, pos, uuid) => {
                let value: Value =
                    serde_json::from_str(&json.0).expect("Failed to unwrap JSON from chat message");

                self.history.push(ChatMessage {
                    sender: UUID(uuid.0.clone()),
                    message_type: ChatMessageType::Temp,
                    text: value["extra"][0]["text"].to_string().replace("\"", ""),
                });
            }
            _ => {}
        }
    }

    pub fn get_message(&mut self) -> &String {
        &self.input
    }

    pub fn get_message_and_clear(&mut self) -> String {
        let out = self.input.clone();
        self.input.clear();

        out
    }

    pub fn set_message(&mut self, text: String) {
        self.input = text;
    }
}

#[derive(Debug)]
pub struct ChatMessage {
    pub sender: UUID,
    pub message_type: ChatMessageType,
    pub text: String,
}

#[derive(Debug)]
pub enum ChatMessageType {
    Temp,
}
