use std::collections::HashMap;

use inflector::Inflector;
use lazy_static::lazy_static;
use serde_json::{self, Value};

lazy_static! {
    pub static ref ENTITIES: HashMap<String, Value> = serde_json::from_slice(include_bytes!("../assets/entities.min.json")).expect("Failed to interpret entities.json");
    pub static ref ENTITY_IDS: HashMap<u32, String> = {
        let mut ids = HashMap::new();

        for (name, val) in ENTITIES.iter() {
            if let Some(id) = val.get("id") {
                ids.insert(id.as_u64().unwrap() as u32, name.clone());
            }
        }

        ids
    };

    pub static ref MODELS: HashMap<String, Value> = serde_json::from_slice(include_bytes!("../assets/models.min.json")).expect("Failed to interpret models.json");
    
    pub static ref BLOCKS: HashMap<String, Value> = serde_json::from_slice(include_bytes!("../assets/blocks.min.json")).expect("Failed to interpret blocks.json");
    pub static ref BLOCKSTATE_IDS: HashMap<u32, String> = {
        let mut map: HashMap<u32, String> = HashMap::new();

        for (name, val) in BLOCKS.iter() {
            let states = val.get("states").unwrap().as_object().unwrap();

            for (id, _) in states.iter() {
                map.insert(id.parse::<u32>().unwrap(), name.clone());
            }
        }

        map
    };
}

pub fn format_name(name: &str) -> String {
    name.replace("minecraft:", "").replace("_", " ").to_title_case()
}