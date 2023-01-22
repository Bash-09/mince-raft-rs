use mcproto_rs::{uuid::UUID4, v1_16_3::GameMode};

pub struct RemotePlayer {
    pub uuid: UUID4,
    pub name: String,
    pub gamemode: GameMode,
    pub ping: i32,
    pub display_name: Option<String>,
}
