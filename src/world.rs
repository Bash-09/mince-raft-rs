use std::{collections::HashMap, sync::RwLockReadGuard};

use glam::{IVec2, IVec3, Vec3, Vec3Swizzles};
use glium::Display;
use mcproto_rs::v1_16_3::PlayBlockChangeSpec;

use crate::resources::{BlockState, BLOCKS};

use self::{
    chunk_builder::ChunkBuilder,
    chunks::{block_pos_to_index, BlockIndex, Chunk, ChunkSection, MAX_SECTION, MIN_SECTION},
};

pub mod chunk_builder;
pub mod chunks;

pub type WorldCoords = IVec3;
pub type ChunkCoords = IVec3;
pub type SectionCoords = IVec3;
pub type ChunkLocation = IVec2;
pub type SectionLocation = IVec3;

pub struct World {
    chunks: HashMap<IVec2, Chunk>,
    chunks_to_generate: Vec<ChunkLocation>,
    sections_to_generate: Vec<SectionLocation>,
}

impl World {
    pub fn new() -> World {
        World {
            chunks: HashMap::new(),
            chunks_to_generate: Vec::new(),
            sections_to_generate: Vec::new(),
        }
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {
        let chunk_coords = *chunk.get_coords();
        self.chunks.insert(chunk_coords, chunk);
        self.queue_chunk_mesh(chunk_coords);
    }

    pub fn queue_chunk_section_mesh(&mut self, location: SectionLocation) {
        self.sections_to_generate.push(location);
    }

    pub fn queue_chunk_mesh(&mut self, location: ChunkLocation) {
        self.chunks_to_generate.push(location);
    }

    fn are_chunk_neighbours_loaded(&self, loc: &ChunkLocation) -> bool {
        let chunk = self.get_chunk(loc);
        let north = self.get_chunk(&ChunkLocation::new(loc.x, loc.y - 1));
        let east = self.get_chunk(&ChunkLocation::new(loc.x + 1, loc.y));
        let south = self.get_chunk(&ChunkLocation::new(loc.x, loc.y + 1));
        let west = self.get_chunk(&ChunkLocation::new(loc.x - 1, loc.y));
        chunk.is_some() && north.is_some() && east.is_some() && south.is_some() && west.is_some()
    }

    fn generate_section_mesh(&mut self, loc: &SectionLocation, dis: &Display, threaded: bool) {
        let chunk = self.get_section(loc);
        // Discard chunk sections that are empty
        if chunk.is_none() {
            return;
        }
        let chunk = chunk.unwrap();

        // Can unwrap because we checked above that they are all valid
        let north = self.get_section(&SectionLocation::new(loc.x, loc.y, loc.z - 1));
        let east = self.get_section(&SectionLocation::new(loc.x + 1, loc.y, loc.z));
        let south = self.get_section(&SectionLocation::new(loc.x, loc.y, loc.z + 1));
        let west = self.get_section(&SectionLocation::new(loc.x - 1, loc.y, loc.z));
        let above = if loc.y >= MAX_SECTION {
            None
        } else {
            self.get_section(&SectionLocation::new(loc.x, loc.y + 1, loc.z))
        };
        let below = if loc.y <= MIN_SECTION {
            None
        } else {
            self.get_section(&SectionLocation::new(loc.x, loc.y - 1, loc.z))
        };

        if threaded {
            todo!();
        } else {
            let verts = ChunkBuilder::generate_mesh(chunk, above, below, north, east, south, west);
            self.get_chunk_mut(&ChunkLocation::new(loc.x, loc.z))
                .unwrap()
                .load_mesh(dis, verts, loc.y);
        }
    }

    pub fn generate_meshes(&mut self, dis: &Display, threaded: bool) {
        let mut temp = Vec::new();
        std::mem::swap(&mut self.chunks_to_generate, &mut temp);
        let ready_chunks: Vec<_> = temp
            .drain_filter(|loc| self.are_chunk_neighbours_loaded(loc))
            .collect();
        std::mem::swap(&mut self.chunks_to_generate, &mut temp);

        for loc in ready_chunks {
            for y in MIN_SECTION..=MAX_SECTION {
                self.generate_section_mesh(&SectionLocation::new(loc.x, y, loc.y), dis, threaded);
            }
        }

        let mut temp = Vec::new();
        std::mem::swap(&mut temp, &mut self.sections_to_generate);
        temp.retain(|loc| {
            // Retain chunks that don't have all their neighbouring chunks
            if !self.are_chunk_neighbours_loaded(&ChunkLocation::new(loc.x, loc.z)) {
                return true;
            }

            // Discard chunk sections that are empty
            if self.get_section(loc).is_none() {
                return false;
            }

            self.generate_section_mesh(loc, dis, threaded);
            false
        });

        std::mem::swap(&mut temp, &mut self.sections_to_generate);
    }

    pub fn get_chunks(&self) -> &HashMap<IVec2, Chunk> {
        &self.chunks
    }

    pub fn get_chunks_mut(&mut self) -> &mut HashMap<IVec2, Chunk> {
        &mut self.chunks
    }

    pub fn get_chunk(&self, location: &ChunkLocation) -> Option<&Chunk> {
        self.chunks.get(location)
    }

    pub fn get_chunk_mut(&mut self, location: &ChunkLocation) -> Option<&mut Chunk> {
        self.chunks.get_mut(location)
    }

    pub fn get_chunk_containing(&self, coords: &WorldCoords) -> Option<&Chunk> {
        self.get_chunk(&Chunk::chunk_containing(coords))
    }

    pub fn get_chunk_containing_mut(&mut self, coords: &WorldCoords) -> Option<&mut Chunk> {
        self.get_chunk_mut(&Chunk::chunk_containing(coords))
    }

    pub fn get_section(&self, location: &SectionLocation) -> Option<RwLockReadGuard<ChunkSection>> {
        self.get_chunk(&ChunkLocation::new(location.x, location.z))
            .map(|c| c.get_section(location.y))
            .unwrap_or(None)
    }

    pub fn get_section_containing(
        &self,
        coords: &WorldCoords,
    ) -> Option<RwLockReadGuard<ChunkSection>> {
        self.get_chunk_containing(coords)
            .map(|c| c.get_section_containing(coords.y))
            .unwrap_or(None)
    }

    /// Get the height of the highest block at the x/z coordinates provided. Can return None if the
    /// coordinates provided are within an unloaded chunk
    pub fn get_highest_block(&self, coords: &IVec2) -> Option<i32> {
        let coords = IVec3::new(coords.x, 0, coords.y);
        self.get_chunk(&Chunk::chunk_containing(&coords))
            .map(|c| c.get_highest_block(Chunk::map_from_world_coords(&coords).xz()))
    }

    pub fn is_chunk_loaded(&self, location: &ChunkLocation) -> bool {
        self.chunks.get(location).is_some()
    }

    pub fn block_at(&self, coords: &WorldCoords) -> Option<&BlockState> {
        self.chunks
            .get(&Chunk::chunk_containing(coords))
            .map(|c| c.block_at(&Chunk::map_from_world_coords(coords)))
            .unwrap_or(None)
    }

    pub fn handle_block_change(&mut self, pack: PlayBlockChangeSpec) {
        if pack.block_id.0 < 0 || pack.block_id.0 > BLOCKS.len() as i32 {
            log::error!("Got block change with invalid block ID");
            return;
        }

        let coords = IVec3::new(pack.location.x, pack.location.y.into(), pack.location.z);
        let section_loc = ChunkSection::section_containing(&coords);
        let mut sections_to_regenerate = Vec::new();

        if let Some(chunk) = self.get_chunk_containing_mut(&coords) {
            if !chunk.is_section_present(section_loc.y) {
                chunk.put_section(ChunkSection {
                    y: ChunkSection::section_containing_height(coords.y),
                    blocks: [0; 4096],
                });
            }

            if let Some(mut section) = chunk.get_section_mut(section_loc.y) {
                let local_coords = ChunkSection::map_from_world_coords(&coords);

                section.blocks[block_pos_to_index(&local_coords)] = pack.block_id.0 as BlockIndex;
                sections_to_regenerate.push(section_loc);

                if local_coords.x == 0 {
                    sections_to_regenerate.push(IVec3::new(
                        section_loc.x - 1,
                        section_loc.y,
                        section_loc.z,
                    ));
                }
                if local_coords.x == 15 {
                    sections_to_regenerate.push(IVec3::new(
                        section_loc.x + 1,
                        section_loc.y,
                        section_loc.z,
                    ));
                }
                if local_coords.y == 0 {
                    sections_to_regenerate.push(IVec3::new(
                        section_loc.x,
                        section_loc.y - 1,
                        section_loc.z,
                    ));
                }
                if local_coords.y == 15 {
                    sections_to_regenerate.push(IVec3::new(
                        section_loc.x,
                        section_loc.y + 1,
                        section_loc.z,
                    ));
                }
                if local_coords.z == 0 {
                    sections_to_regenerate.push(IVec3::new(
                        section_loc.x,
                        section_loc.y,
                        section_loc.z - 1,
                    ));
                }
                if local_coords.z == 15 {
                    sections_to_regenerate.push(IVec3::new(
                        section_loc.x,
                        section_loc.y,
                        section_loc.z + 1,
                    ));
                }
            } else {
                panic!("New ChunkSection was not emplaced on block update in empty chunk section");
            }
        } else {
            log::warn!("Block update in unloaded chunk");
        }

        for coords in sections_to_regenerate {
            self.queue_chunk_section_mesh(coords);
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns the block coordinates of the given position
pub fn block_coords(pos: &Vec3) -> IVec3 {
    IVec3::new(
        pos.x.floor() as i32,
        pos.y.floor() as i32,
        pos.z.floor() as i32,
    )
}
