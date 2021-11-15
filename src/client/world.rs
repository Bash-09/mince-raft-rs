use std::collections::HashMap;

use glam::{IVec2, IVec3, Vec3, Vec3Swizzles};
use resources::blocks::BlockState;

use self::chunks::Chunk;

pub mod chunks;

pub struct World {
    chunks: HashMap<IVec2, Chunk>,
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

    pub fn get_chunks(&self) -> &HashMap<IVec2, Chunk> {
        &self.chunks
    }

    pub fn get_chunks_mut(&mut self) -> &mut HashMap<IVec2, Chunk> {
        &mut self.chunks
    }

    pub fn get_highest_block(&self, coords: &IVec2) -> i32 {
        match self.chunks.get(&chunk_at_coords(coords)) {
            Some(chunk) => chunk.get_highest_block(
                chunk_coords(&IVec3::new(coords.x, 0, coords.y)).xz()),
            None => 0,
        }
    }

    pub fn get_block_at(&self, coords: &IVec3) -> Option<&BlockState> {
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
pub fn chunk_coords(coords: &IVec3) -> IVec3 {
    IVec3::new(
        coords.x.rem_euclid(16),
        coords.y,
        coords.z.rem_euclid(16)
    )
}

/// Converts a given world x/z coordinate into the local chunk's x/y/z coordinate
pub fn chunk_section_coords(coords: &IVec3) -> IVec3 {
    IVec3::new(
        coords.x.rem_euclid(16),
        coords.y.rem_euclid(16),
        coords.z.rem_euclid(16),
    )
}


/// Returns the coordinates used to identify the chunk at the given location
pub fn chunk_at_coords(coords: &IVec2) -> IVec2 {
    IVec2::new(
        (coords.x as f32 / 16.0).floor() as i32,
        (coords.y as f32 / 16.0).floor() as i32,
    )
}

/// Returns the coordinates used to identify the chunk at the given location
pub fn chunk_section_at_coords(coords: &IVec3) -> IVec3 {
    IVec3::new(
        (coords.x as f32 / 16.0).floor() as i32, 
        (coords.y as f32 / 16.0).floor() as i32, 
        (coords.z as f32 / 16.0).floor() as i32, 
    )
}

pub fn block_coords(pos: &Vec3) -> IVec3 {
    IVec3::new(
        pos.x.floor() as i32,
        pos.y.floor() as i32,
        pos.z.floor() as i32,
    )
}