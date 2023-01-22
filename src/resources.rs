use std::collections::HashMap;

use inflector::Inflector;
use lazy_static::lazy_static;
use serde_json::{self, Value};

pub struct Entity {
    pub name: String,
    pub id: u32,
    pub translation_key: String,
    pub width: f32,
    pub height: f32,
}

pub struct BlockState {
    pub name: String,
    pub id: u32,
    pub model: Option<String>,
    pub collision_shape: Option<u64>,
}

pub const PLAYER_INDEX: usize = 106;

lazy_static! {
    pub static ref ENTITIES: HashMap<u32, Entity> = {
        let mut entities = HashMap::new();

        let json: HashMap<String, Value> =
            serde_json::from_slice(include_bytes!("../assets/entities.min.json"))
                .expect("Failed to interpret entities.json");
        for (name, val) in json.iter() {
            if let Some(id) = val.get("id") {
                entities.insert(
                    id.as_u64().unwrap() as u32,
                    Entity {
                        name: format_name(name),
                        id: id.as_u64().unwrap() as u32,
                        translation_key: val
                            .get("loot_table")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                        width: val.get("width").unwrap().as_f64().unwrap() as f32,
                        height: val.get("height").unwrap().as_f64().unwrap() as f32,
                    },
                );
            }
        }

        entities
    };
    pub static ref BLOCKS: HashMap<u32, BlockState> = {
        let mut blocks = HashMap::new();

        let json: HashMap<String, Value> =
            serde_json::from_slice(include_bytes!("../assets/blocks.min.json"))
                .expect("Failed to interpret blocks.json");
        for (name, val) in json.iter() {
            let name = format_name(name);
            for (id, state) in val.get("states").unwrap().as_object().unwrap().iter() {
                let id = id.parse().unwrap();
                blocks.insert(
                    id,
                    BlockState {
                        name: name.clone(),
                        id,
                        model: {
                            match state.get("model") {
                                Some(model) => model.as_str().map(|model| model.to_string()),
                                None => None,
                            }
                        },
                        collision_shape: {
                            match state.get("collision_shape") {
                                Some(collision_shape) => collision_shape.as_u64(),
                                None => None,
                            }
                        },
                    },
                );
            }
        }

        blocks
    };
    pub static ref MODELS: HashMap<String, Value> =
        serde_json::from_slice(include_bytes!("../assets/models.min.json"))
            .expect("Failed to interpret models.json");
}

pub fn format_name(name: &str) -> String {
    name.replace("minecraft:", "")
        .replace('_', " ")
        .to_title_case()
}
