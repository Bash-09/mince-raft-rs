use std::collections::HashMap;

use self::chunks::Chunk;


pub mod chunks;


pub struct World {

    chunks: HashMap<(i32, i32), Chunk>,

}


impl World {

    pub fn new() -> World {

        World {  
            chunks: HashMap::new(),

        }
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {

        self.chunks.insert(chunk.get_coords(), chunk);

    }

    pub fn get_chunks(&self) -> &HashMap<(i32, i32), Chunk> {
        &self.chunks
    }

    pub fn get_chunks_mut(&mut self) -> &mut HashMap<(i32, i32), Chunk> {
        &mut self.chunks
    }

    pub fn get_highest_block(&self, coords: (i32, i32)) -> i32 {
        match self.chunks.get(&chunk_at_coords(coords)) {
            Some(chunk) => {
                chunk.get_highest_block(chunk_coords(coords))
            },
            None => 0
        }
    }

}

pub fn chunk_coords(coords: (i32, i32)) -> (i32, i32) {
    (coords.0.rem_euclid(16), coords.1.rem_euclid(16))
}

pub fn chunk_at_coords(coords: (i32, i32)) -> (i32, i32) {
    (((coords.0 as f32) / 16.0).floor() as i32, 
    ((coords.1 as f32) / 16.0).floor() as i32)
}