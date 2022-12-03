use mcproto_rs::v1_16_3::PlayServerChatMessageSpec;

pub struct Chat {
    history: Vec<(PlayServerChatMessageSpec, i64)>,

    input: String,
}

impl Chat {
    pub fn new() -> Chat {
        Chat {
            history: Vec::with_capacity(255),
            input: String::with_capacity(255),
        }
    }

    pub fn get_history(&self) -> &Vec<(PlayServerChatMessageSpec, i64)> {
        &self.history
    }

    pub fn add_message(&mut self, chat: PlayServerChatMessageSpec, time: i64) {
        self.history.push((chat, time));
    }

    pub fn get_current_message(&self) -> &String {
        &self.input
    }

    pub fn get_current_message_mut(&mut self) -> &mut String {
        &mut self.input
    }

    pub fn get_current_message_and_clear(&mut self) -> String {
        let out = self.input.clone();
        self.input.clear();

        out
    }

    pub fn set_current_message(&mut self, text: String) {
        self.input = text;
    }
}
