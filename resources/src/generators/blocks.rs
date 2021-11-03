use std::collections::HashMap;

use inflector::Inflector;
use serde_json::{Value};

use super::{export_file, read_json};

#[derive(Debug)]
pub struct BlockType {
    id: i32,
    name: String,
}



pub fn get_blocks_map(filename: &str) -> std::io::Result<Option<HashMap<i32, BlockType>>> {
    let json = read_json(filename)?;

    let mut blocks: HashMap<i32, BlockType> = HashMap::new();

    match json {
        Value::Object(map) => {

            for (name, val) in map.iter() {

                let id: i32;

                match val.get("id") {
                    Some(Value::Number(num)) => {
                        id = num.as_i64().expect("Non-Integer ID") as i32;
                    },
                    _ => continue
                }

                let e = BlockType {
                    id,
                    name: name.replace("minecraft:", "").replace("_", " ").to_title_case(),
                };
                blocks.insert(id, e);

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

    let mut out = String::new();
    out += "
use phf::phf_map;

#[derive(Debug)]
pub struct BlockType {
    id: i32,
    name: &'static str,
}

pub static BLOCKS: phf::Map<i32, BlockType> = phf_map! {
";
    let mut blocks_vec: Vec<BlockType> = Vec::with_capacity(blocks.len());
    for b in blocks.into_values() {
        blocks_vec.push(b);
    }
    blocks_vec.sort_by(|a, b| {
        a.id.cmp(&b.id)
    });

    for b in blocks_vec.iter() {
        out += &format!("\t{0}i32 => BlockType{{ id: {0}, name: \"{1}\" }}, \n", b.id, b.name);
    }
    out += "};";

    Ok(out)
}


pub fn export_blocks(filename: &str) -> std::io::Result<bool> {
    let data = get_blocks_string(filename)?;
    if data.is_empty() {return Ok(false)}

    export_file("./src/blocks.rs", &data)?;
    Ok(true)
}