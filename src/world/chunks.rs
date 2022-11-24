use std::io::{Cursor, Read};

use glam::{IVec2, IVec3};
use glium::{Display, VertexBuffer};
use log::debug;
use mcproto_rs::v1_16_3::ChunkData;

use crate::{
    renderer::Vertex,
    resources::{BlockState, BLOCKS}, network::read_varint,
};

pub type ChunkArray = [u16; 4096];

#[derive(Debug)]
pub struct ChunkSection {
    pub y: i32,
    pub blocks: ChunkArray,

    vbo: Option<VertexBuffer<Vertex>>,
}

impl ChunkSection {
    pub fn new(y: i32, blocks: ChunkArray) -> ChunkSection {
        ChunkSection {
            y,
            blocks,
            vbo: None,
        }
    }

    pub fn get_vbo(&self) -> &Option<VertexBuffer<Vertex>> {
        &self.vbo
    }

    pub fn load_mesh(&mut self, dis: &Display, verts: Vec<Vertex>) {
        self.vbo = Some(glium::VertexBuffer::new(dis, &verts).unwrap());
    }
}

pub struct Chunk {
    pos: IVec2,

    heightmap: [u16; 256],

    pub sections: [Option<ChunkSection>; 16],
}

// Base 2 Log of number of state ids in the game
const MAX_BITS_PER_BLOCK: u64 = 15;

impl Chunk {
    pub fn new(dis: &Display, data: &ChunkData) -> Chunk {
        debug!("Processing chunk data");

        Chunk {
            pos: IVec2::new(data.position.x, data.position.z),

            heightmap: process_heightmap(data),
            sections: process_sections(dis, data),
        }
    }

    pub fn get_coords(&self) -> &IVec2 {
        &self.pos
    }

    /// Returns the y value of the highest block at the x/z position provided in this chunk
    pub fn get_highest_block(&self, coords: IVec2) -> i32 {
        self.heightmap[coords.y as usize * 16 + coords.x as usize] as i32
    }

    /// Returns the block in this chunk at the position provided
    pub fn block_at(&self, pos: IVec3) -> &BlockState {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let z = pos.z as usize;
        if y >= 16 * 16 || x >= 16 || z >= 16 {
            return BLOCKS.get(&0).unwrap();
        }
        return if let Some(cs) = &self.sections[y / 16] {
            BLOCKS
            .get(&(cs.blocks[((y % 16) * 16 * 16 + z * 16 + x) as usize] as u32))
            .unwrap()
        } else {
            BLOCKS.get(&0).unwrap()
        };
    }
}

/// Extracts the heightmap from chunk data
fn process_heightmap(data: &ChunkData) -> [u16; 256] {
    let mut map = [0u16; 256];

    // match &data.heightmaps.root.get::<_, &Vec<i64>>("MOTION_BLOCKING") {
    //     Ok(list) => {
    //         let vals_per_long: usize = 7;
    //         for i in 0..256 as usize {
    //             let long = i / vals_per_long;
    //             let offset = (i % vals_per_long) * 9;
    //
    //             map[i] = ((list[long] >> offset) & 0x1ff) as u16;
    //         }
    //     }
    //     Err(e) => {
    //         panic!("Invalid chunk data: {}", e);
    //     }
    // }

    map
}

const INIT: Option<ChunkSection> = None;
/// Builds a list of chunk sections from chunk data
fn process_sections(dis: &Display, data: &ChunkData) -> [Option<ChunkSection>; 16] {
    // Check bit mask for which chunk sections are present
    let mut chunk_sections_present = [false; 16];
    for i in 0..16 {
        if data.primary_bit_mask.0 & 0b1 << i != 0 {
            chunk_sections_present[i] = true;
        }
    }

    let mut sections = [INIT; 16];
    // let mut sections: [Option<ChunkSection>; 16] = Default::default();

    // Decode data array
    let mut cur = Cursor::new(&*data.data);
    for i in 0..16 {
        if !chunk_sections_present[i] {
            continue;
        }

        let mut buf = [0u8; 2];
        cur.read_exact(&mut buf).unwrap();
        let block_count = i16::from_ne_bytes(buf);

        let mut buf = [0u8; 1];
        cur.read_exact(&mut buf).unwrap();
        let mut bits_per_block = buf[0] as u64;

        if bits_per_block <= 4 {
            bits_per_block = 4;
        }
        if bits_per_block >= 9 {
            bits_per_block = MAX_BITS_PER_BLOCK;
        }

        let palette: Option<Vec<i32>>;

        // Construct palette or no palette
        if bits_per_block < 9 {
            let palette_len = read_varint(&mut cur).unwrap();
            log::debug!("Got chunk with pallete of {} elements.", palette_len);
            let mut palette_vec: Vec<i32> = Vec::new();

            for _ in 0..palette_len as usize {
                palette_vec.push(read_varint(&mut cur).unwrap());
            }
            palette = Some(palette_vec);
        } else {
            palette = None;
        }

        // Get long array of blocks
        let array_len = read_varint(&mut cur).unwrap();
        let mut array = Vec::new();

        for _ in 0..array_len as usize {
            let mut buf = [0u8; 8];
            cur.read_exact(&mut buf).unwrap();
            array.push(i64::from_be_bytes(buf));
        }

        // Bit mask depending on bits per block
        let mut mask = 0;
        for j in 0..bits_per_block {
            mask |= 1 << j;
        }
        let mask = mask;

        let blocks_per_long = 64 / bits_per_block;

        let mut blocks = [0u16; 4096];

        // Extract blocks
        for j in 0..4096 {
            let long = j / blocks_per_long;
            let start = (j % blocks_per_long) * bits_per_block;

            // Get block id / palette index from long
            let block = (array[long as usize] >> start) & mask;

            // Get block from palette
            match &palette {
                Some(pal) => {
                    blocks[j as usize] = pal[block as usize] as u16;
                }
                None => {
                    blocks[j as usize] = block as u16;
                }
            }
        }

        sections[i] = Some(ChunkSection {
            y: i as i32,
            blocks,
            vbo: None,
        });
    }
    sections
}

/// Converts a block position to an index within a chunk section array
pub fn vec_to_index(pos: &IVec3) -> usize {
    ((pos.y % 16) * 16 * 16 + pos.z * 16 + pos.x) as usize
}

/// Converts an index within a chunk section array to a 3d block pos
pub fn index_to_vec(i: usize) -> IVec3 {
    let x = i % 16;
    let y = i / (16 * 16);
    let z = (i / 16) % 16;

    IVec3::new(x as i32, y as i32, z as i32)
}
