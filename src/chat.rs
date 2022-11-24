use mcproto_rs::v1_16_3::PlayServerChatMessageSpec;

pub struct Chat {
    history: Vec<PlayServerChatMessageSpec>,

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

    pub fn get_history(&self) -> &Vec<PlayServerChatMessageSpec> {
        &self.history
    }

    pub fn add_message(&mut self, chat: PlayServerChatMessageSpec) {
        self.history.push(chat);
    }

    pub fn get_message(&self) -> &String {
        &self.input
    }

    pub fn get_message_mut(&mut self) -> &mut String {
        &mut self.input
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
