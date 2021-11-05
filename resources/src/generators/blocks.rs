use std::collections::HashMap;

use inflector::Inflector;
use serde_json::{Value};

use super::{export_file, read_json};

#[derive(Debug)]
pub struct BlockState {
    pub state_id: i32,
    pub name: String,
}



pub fn get_blocks_map(filename: &str) -> std::io::Result<Option<HashMap<i32, BlockState>>> {
    let json = read_json(filename)?;

    let mut blocks: HashMap<i32, BlockState> = HashMap::new();

    // Read json data
    match json {
        // Get it as map of blocks
        Value::Object(map) => {
            // For each block
            for (nam, val) in map.iter() {
                // Get name
                let name = nam.replace("minecraft:", "").replace("_", " ").to_title_case();
                // Get map of states
                match val.get("states") {
                    Some(Value::Object(states)) => {
                        // For each state
                        for (id, v) in states.iter() {
                            // Get state id
                            let id: i32 = id.parse().unwrap_or_else(|_| {-1});
                            if id == -1 {continue}

                            // Voilla
                            let e = BlockState {

                                state_id: id,
                                name: name.clone(),
                            };
            
                            blocks.insert(id, e);
                        }

                    },
                    _ => continue,
                }

            }

        },
        _ => {
            println!("No blocks");
            return Ok(None);
        }
    }

    Ok(Some(blocks))
}

pub fn get_blocks_string(filename: &str) -> std::io::Result<String> {

    let blocks;
    match get_blocks_map(filename)? {
        Some(b) => blocks = b,
        _ => return Ok(String::new()),
    }

    let mut blocks_vec: Vec<BlockState> = Vec::with_capacity(blocks.len());
    for b in blocks.into_values() {
        blocks_vec.push(b);
    }
    blocks_vec.sort_by(|a, b| {
        a.state_id.cmp(&b.state_id)
    });

    let mut out = String::new();
    out += &format!("
    #[derive(Debug)]
pub struct BlockState {{
    pub state_id: i32,
    pub name: &'static str,
}}

pub const BLOCKS: [BlockState; {}] = [
", blocks_vec.len());

    for b in blocks_vec.iter() {
        out += &format!("\tBlockState{{ state_id: {0}, name: \"{1}\" }}, \n", b.state_id, b.name);
    }
    out += "];";

    Ok(out)
}


pub fn export_blocks(filename: &str) -> std::io::Result<bool> {
    let data = get_blocks_string(filename)?;
    if data.is_empty() {return Ok(false)}

    export_file("./src/blocks.rs", &data)?;
    Ok(true)
}