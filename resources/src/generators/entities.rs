use std::collections::HashMap;

use inflector::Inflector;
use serde_json::Value;

use super::{export_file, read_json};



#[derive(Debug)]
pub struct EntityType {
    id: i32,
    name: String,
}

/// Reads a minecraft entities json file and creates a module to include those entities in the binary
/// 
/// # Arguments
/// * `filename: &str` - The file to read from (usually entities.json or something)
/// 
/// # Returns
/// * `String` - The sourcecode for a Rust module which contains all the entities to be compiled
/// 
pub fn get_entities_string(filename: &str) -> std::io::Result<Option<String>> {

    let json = read_json(filename)?;

    let mut out: HashMap<i32, EntityType> = HashMap::new();

    match json {
        Value::Object(map) => {
            use serde_json::Value::*;

            for (name, val) in map.iter() {

                let id: i32;

                match val.get("id") {
                    Some(Number(num)) => {
                        id = num.as_i64().expect("Non-Integer ID") as i32;
                    },
                    _ => continue
                }

                let e = EntityType {
                    id,
                    name: name.replace("minecraft:", "").replace("_", " ").to_title_case(),
                };
                out.insert(id, e);

            }

        },
        _ => {
            return Ok(None)
        }
    }


    let mut file = String::new();
    file +="
use phf::phf_map;

#[derive(Debug)]
pub struct EntityType {
    pub id: i32,
    pub name: &'static str,
}

pub static ENTITIES: phf::Map<i32, EntityType> = phf_map! {
";

    let mut ents_vec: Vec<EntityType> = Vec::with_capacity(out.len());
    for e in out.into_values() {
        ents_vec.push(e);
    }
    ents_vec.sort_by(|a, b| {
        a.id.cmp(&b.id)
    });


    for e in ents_vec.iter() {
        file += format!("\t{0}i32 => EntityType{{ id: {0}, name: \"{1}\" }},\n", e.id, e.name).as_str();
    }

    file += "};";


    Ok(Some(file))
}

pub fn export_entities(filename: &str) -> std::io::Result<bool> {
    return match get_entities_string(filename)? {
        Some(data) => {
            export_file("./src/entities.rs", &data)?;
            Ok(true)
        },
        None => {
            Ok(false)
        }
    }
}
