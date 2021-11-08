use std::collections::HashMap;

use cgmath::{Vector2, Vector3};
use resources::blocks::BlockState;

use self::chunks::Chunk;

pub mod chunks;

pub struct World {
    chunks: HashMap<Vector2<i32>, Chunk>,
}

impl World {
    pub fn new() -> World {
        World {
            chunks: HashMap::new(),
        }
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(*chunk.get_coords(), chunk);
    }

    pub fn get_chunks(&self) -> &HashMap<Vector2<i32>, Chunk> {
        &self.chunks
    }

    pub fn get_chunks_mut(&mut self) -> &mut HashMap<Vector2<i32>, Chunk> {
        &mut self.chunks
    }

    pub fn get_highest_block(&self, coords: &Vector2<i32>) -> i32 {
        match self.chunks.get(&chunk_at_coords(coords)) {
            Some(chunk) => chunk.get_highest_block(
                chunk_coords(&Vector3::new(coords.x, 0, coords.y)).xz()),
            None => 0,
        }
    }

    pub fn get_block_at(&self, coords: &Vector3<i32>) -> Option<&BlockState> {
        let chunk = chunk_at_coords(&coords.xz());
        return match self.chunks.get(&chunk) {
            Some(c) => {
                Some(c.block_at(chunk_coords(&coords)))
            },
            None => None,
        }
    }
}

/// Converts a given world x/z coordinate into the local chunk's x/y/z coordinate
pub fn chunk_coords(coords: &Vector3<i32>) -> Vector3<i32> {
    Vector3::new(
        coords.x.rem_euclid(16),
        coords.y,
        coords.z.rem_euclid(16)
    )
}

/// Converts a given world x/z coordinate into the local chunk's x/y/z coordinate
pub fn chunk_section_coords(coords: &Vector3<i32>) -> Vector3<i32> {
    coords.map(|v| {v.rem_euclid(16)})
}


/// Returns the coordinates used to identify the chunk at the given location
pub fn chunk_at_coords(coords: &Vector2<i32>) -> Vector2<i32> {
    coords.map(|v| {(v as f32 / 16.0).floor() as i32})
}

/// Returns the coordinates used to identify the chunk at the given location
pub fn chunk_section_at_coords(coords: &Vector3<i32>) -> Vector3<i32> {
    coords.map(|v| {(v as f32 / 16.0).floor() as i32})
}

pub fn block_coords(pos: &Vector3<f32>) -> Vector3<i32> {
    pos.map(|p| {p.floor() as i32})
}