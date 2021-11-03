extern crate serde_json;
extern crate inflector;

use std::{fs::{File, OpenOptions}, io::{BufReader, Write}};
use std::collections::HashMap;

use inflector::Inflector;
use serde_json::Value;

pub mod entities;
pub mod blocks;

pub fn read_json(filename: &str) -> std::io::Result<Value> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    let val: Value = serde_json::from_reader(reader)?;

    Ok(val)
}

pub fn export_file(filename: &str, contents: &str) -> std::io::Result<()> {

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(filename)?;

    file.write(contents.as_bytes())?;

    Ok(())
}




