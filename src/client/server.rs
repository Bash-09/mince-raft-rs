use std::collections::{btree_set::Difference, HashMap};

use crate::client::network::packets::DecodedPacket;

use super::{
    chat::Chat,
    entities::Entity,
    player::{self, Player},
    world::World,
};

pub enum ServerState {
    Status,
    Login,
    Play,
}

pub struct Server {
    pub network_destination: String,

    pub world_time: i64,
    pub day_time: i64,

    pub player: Player,
    pub chat: Chat,

    pub world: World,

    pub entities: HashMap<i32, Entity>,

    pub difficulty: Difficulty,
    pub difficulty_locked: bool,
}

impl Server {
    pub fn new(network_destination: String) -> Server {
        Server {
            network_destination,

            world_time: 0,
            day_time: 0,

            player: Player::new(),
            chat: Chat::new(),

            world: World::new(),

            entities: HashMap::new(),

            difficulty: Difficulty::Easy,
            difficulty_locked: false,
        }
    }

    pub fn join_game(&mut self, player_id: i32) {
        self.player.id = player_id;
    }
}

#[derive(Debug)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Medium,
    Hard,
}
