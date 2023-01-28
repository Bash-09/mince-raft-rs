use std::sync::{
    mpsc::{channel, Receiver, Sender},
    RwLockReadGuard,
};

use glam::IVec3;
use threadpool::ThreadPool;

use crate::{
    renderer::BlockVertex,
    resources::{block_models::BlockModel, BLOCKS, BLOCK_MODELS_PARSED},
};

use super::{
    chunks::{
        block_index_to_pos, block_pos_to_index, Chunk, ChunkSection, WrappedChunkSection,
        MAX_SECTION, MIN_SECTION,
    },
    SectionLocation,
};

pub struct ChunkBuilder {
    incoming: Receiver<(SectionLocation, Vec<BlockVertex>)>,
    outgoing: Sender<(SectionLocation, Vec<BlockVertex>)>,
    pool: ThreadPool,
}

impl ChunkBuilder {
    pub fn new() -> ChunkBuilder {
        let (send, recv) = channel();
        ChunkBuilder {
            incoming: recv,
            outgoing: send,
            pool: ThreadPool::new(std::thread::available_parallelism().unwrap().into()),
        }
    }

    pub fn get_incoming_meshes(&self) -> &Receiver<(SectionLocation, Vec<BlockVertex>)> {
        &self.incoming
    }

    pub fn generate_chunk(
        &self,
        chunk: &Chunk,
        north: &Chunk,
        east: &Chunk,
        south: &Chunk,
        west: &Chunk,
        threaded: bool,
    ) {
        for sec in chunk.get_sections() {
            if sec.is_none() {
                continue;
            }
            let sec = sec.unwrap();
            let sec_read = sec.read().unwrap();
            let loc = IVec3::new(chunk.get_coords().x, sec_read.y, chunk.get_coords().y);

            let above = if loc.y < MAX_SECTION {
                chunk.get_section(loc.y + 1)
            } else {
                None
            };
            let below = if loc.y > MIN_SECTION {
                chunk.get_section(loc.y - 1)
            } else {
                None
            };
            let north = north.get_section(loc.y);
            let south = south.get_section(loc.y);
            let east = east.get_section(loc.y);
            let west = west.get_section(loc.y);

            self.generate_chunk_section(
                sec.clone(),
                loc,
                above,
                below,
                north,
                south,
                east,
                west,
                threaded,
            );
        }
    }

    pub fn generate_chunk_section(
        &self,
        sect: WrappedChunkSection,
        loc: SectionLocation,
        above: Option<WrappedChunkSection>,
        below: Option<WrappedChunkSection>,
        north: Option<WrappedChunkSection>,
        east: Option<WrappedChunkSection>,
        south: Option<WrappedChunkSection>,
        west: Option<WrappedChunkSection>,
        threaded: bool,
    ) {
        let outgoing = self.outgoing.clone();

        let run = move || {
            let above = above.as_ref();
            let below = below.as_ref();
            let north = north.as_ref();
            let south = south.as_ref();
            let east = east.as_ref();
            let west = west.as_ref();
            outgoing
                .send((
                    loc,
                    Self::generate_mesh(
                        sect.read().unwrap(),
                        above.map(|s| s.read().unwrap()),
                        below.map(|s| s.read().unwrap()),
                        north.map(|s| s.read().unwrap()),
                        east.map(|s| s.read().unwrap()),
                        south.map(|s| s.read().unwrap()),
                        west.map(|s| s.read().unwrap()),
                    ),
                ))
                .unwrap();
        };

        if threaded {
            self.pool.execute(move || run());
        } else {
            run();
        }
    }

    pub fn generate_mesh(
        section: RwLockReadGuard<ChunkSection>,
        above: Option<RwLockReadGuard<ChunkSection>>,
        below: Option<RwLockReadGuard<ChunkSection>>,
        north: Option<RwLockReadGuard<ChunkSection>>,
        east: Option<RwLockReadGuard<ChunkSection>>,
        south: Option<RwLockReadGuard<ChunkSection>>,
        west: Option<RwLockReadGuard<ChunkSection>>,
    ) -> Vec<BlockVertex> {
        let mut verts: Vec<BlockVertex> = Vec::new();

        let value = section.blocks;
        for (i, b) in value.iter().enumerate() {
            let block = BLOCKS.get(&((*b).into()));
            if block.is_none() {
                continue;
            }
            let block = block.unwrap();

            if block.models.is_none() {
                continue;
            }
            if block.models.as_ref().unwrap().len() == 0 {
                continue;
            }

            let model: Option<&BlockModel> =
                BLOCK_MODELS_PARSED.get(block.models.as_ref().unwrap().get(0).unwrap());
            if model.is_none() {
                log::error!(
                    "Couldn't find model {}",
                    block.models.as_ref().unwrap().get(0).unwrap()
                );
                continue;
            }
            let model = model.unwrap();

            let pos = block_index_to_pos(i);

            // Top Face
            let mut n = pos;
            n.y = (n.y + 1).rem_euclid(16);
            let ni = block_pos_to_index(&n);
            let b_above = if pos.y == 15 {
                if let Some(above) = &above {
                    above.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            // Bottom Face
            let mut n = pos;
            n.y = (n.y - 1).rem_euclid(16);
            let ni = block_pos_to_index(&n);
            let b_below = if pos.y == 0 {
                if let Some(below) = &below {
                    below.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            // North Face
            let mut n = pos;
            n.z = (n.z - 1).rem_euclid(16);
            let ni = block_pos_to_index(&n);
            let b_north = if pos.z == 0 {
                if let Some(north) = &north {
                    north.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            // South Face
            let mut n = pos;
            n.z = (n.z + 1).rem_euclid(16);
            let ni = block_pos_to_index(&n);
            let b_south = if pos.z == 15 {
                if let Some(south) = &south {
                    south.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            // East Face
            let mut n = pos;
            n.x = (n.x + 1).rem_euclid(16);
            let ni = block_pos_to_index(&n);
            let b_east = if pos.x == 15 {
                if let Some(east) = &east {
                    east.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            // West Face
            let mut n = pos;
            n.x = (n.x - 1).rem_euclid(16);
            let ni = block_pos_to_index(&n);
            let b_west = if pos.x == 0 {
                if let Some(west) = &west {
                    west.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            for mut vert in model.generate_mesh(b_above, b_below, b_north, b_east, b_south, b_west)
            {
                vert.position[0] += pos.x as f32;
                vert.position[1] += pos.y as f32;
                vert.position[2] += pos.z as f32;
                verts.push(vert);
            }
        }

        verts
    }
}
