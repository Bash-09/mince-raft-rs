use std::collections::HashMap;

use glam::{IVec2, IVec3, Vec3, Vec3Swizzles};
use glium::Display;

use crate::resources::BlockState;

use self::{chunks::Chunk, chunk_builder::ChunkBuilder};

pub mod chunks;
pub mod chunk_builder;
pub mod models;

pub struct World {
    chunks: HashMap<IVec2, Chunk>,
}

impl World {
    pub fn new() -> World {
        World {
            chunks: HashMap::new(),
        }
    }

    pub fn insert_chunk(&mut self, dis: &Display, chunk: Chunk) {
        let chunk_coords = chunk.get_coords().clone();
        self.chunks.insert(*chunk.get_coords(), chunk);

        self.regenerate_chunk(dis, IVec2::new(chunk_coords.x, chunk_coords.y));
        self.regenerate_chunk(dis, IVec2::new(chunk_coords.x+1, chunk_coords.y+1));
        self.regenerate_chunk(dis, IVec2::new(chunk_coords.x+1, chunk_coords.y-1));
        self.regenerate_chunk(dis, IVec2::new(chunk_coords.x-1, chunk_coords.y+1));
        self.regenerate_chunk(dis, IVec2::new(chunk_coords.x-1, chunk_coords.y-1));
    }

    pub fn get_chunks(&self) -> &HashMap<IVec2, Chunk> {
        &self.chunks
    }

    pub fn get_chunks_mut(&mut self) -> &mut HashMap<IVec2, Chunk> {
        &mut self.chunks
    }

    pub fn get_highest_block(&self, coords: &IVec2) -> i32 {
        match self.chunks.get(&chunk_at_coords(coords)) {
            Some(chunk) => {
                chunk.get_highest_block(local_chunk_coords(&IVec3::new(coords.x, 0, coords.y)).xz())
            }
            None => 0,
        }
    }

    pub fn get_block_at(&self, coords: &IVec3) -> Option<&BlockState> {
        let chunk = chunk_at_coords(&coords.xz());
        return match self.chunks.get(&chunk) {
            Some(c) => Some(c.block_at(local_chunk_coords(&coords))),
            None => None,
        };
    }

    pub fn regenerate_chunk_section(&mut self, dis: &Display, cs: IVec3) {
        if let Some(chunk) = self.chunks.get(&cs.xz()) {
            if chunk.sections[cs.y as usize].is_none() {return}

            let north = self.chunks.get(&IVec2::new(cs.x, cs.z-1));
            let east = self.chunks.get(&IVec2::new(cs.x+1, cs.z));
            let south = self.chunks.get(&IVec2::new(cs.x, cs.z+1));
            let west = self.chunks.get(&IVec2::new(cs.x-1, cs.z));

            if north.is_none() || east.is_none() || south.is_none() || west.is_none() {
                return;
            }

            let mesh = ChunkBuilder::generate_mesh(
                &chunk.sections[cs.y as usize].as_ref().unwrap(),
                if cs.y == 15 {&None} else {&chunk.sections[cs.y as usize + 1]},
                if cs.y == 0 {&None} else {&chunk.sections[cs.y as usize - 1]},
                &north.unwrap().sections[cs.y as usize], 
                &east.unwrap().sections[cs.y as usize], 
                &south.unwrap().sections[cs.y as usize], 
                &west.unwrap().sections[cs.y as usize]
            );

            self.chunks.get_mut(&cs.xz()).unwrap()
                .sections[cs.y as usize].as_mut().unwrap()
                .load_mesh(dis, mesh);
        }   
    }

    pub fn regenerate_chunk(&mut self, dis: &Display, cs: IVec2) {
        for y in 0..16 {
            self.regenerate_chunk_section(dis, IVec3::new(cs.x, y, cs.y));
        }
    }
}

/// Converts a given world coordinate into coordinates within the chunk
pub fn local_chunk_coords(coords: &IVec3) -> IVec3 {
    IVec3::new(coords.x.rem_euclid(16), coords.y, coords.z.rem_euclid(16))
}

/// Converts a given world coordinate into coordinates within the chunk section
pub fn local_chunk_section_coords(coords: &IVec3) -> IVec3 {
    IVec3::new(
        coords.x.rem_euclid(16),
        coords.y.rem_euclid(16),
        coords.z.rem_euclid(16),
    )
}

/// Returns the coordinates of the chunk containing the given position
pub fn chunk_at_coords(coords: &IVec2) -> IVec2 {
    IVec2::new(
        (coords.x as f32 / 16.0).floor() as i32,
        (coords.y as f32 / 16.0).floor() as i32,
    )
}

/// Returns the coordinates of the chunk section containing the given position
pub fn chunk_section_at_coords(coords: &IVec3) -> IVec3 {
    IVec3::new(
        (coords.x as f32 / 16.0).floor() as i32,
        (coords.y as f32 / 16.0).floor() as i32,
        (coords.z as f32 / 16.0).floor() as i32,
    )
}

/// Returns the block containing the given position
pub fn block_coords(pos: &Vec3) -> IVec3 {
    IVec3::new(
        pos.x.floor() as i32,
        pos.y.floor() as i32,
        pos.z.floor() as i32,
    )
}
