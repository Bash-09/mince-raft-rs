use glam::{IVec2, IVec3, Vec2};
use glium::{Display, VertexBuffer};
use log::debug;
use resources::blocks::{BlockState, BLOCKS};

use crate::{
    network::packets::{ChunkData, PacketDecoder},
    renderer::Vertex,
};

#[derive(Debug)]
pub struct ChunkSection {
    pub y: i32,
    pub blocks: [u16; 4096],

    vbo: VertexBuffer<Vertex>,
}

impl ChunkSection {
    pub fn get_vbo(&self) -> &VertexBuffer<Vertex> {
        &self.vbo
    }
}

pub struct Chunk {
    pos: IVec2,

    heightmap: [u16; 256],

    sections: [Option<ChunkSection>; 16],
}

// Base 2 Log of number of state ids in the game
const MAX_BITS_PER_BLOCK: u64 = 15;

impl Chunk {
    pub fn new(dis: &Display, data: &ChunkData) -> Chunk {
        debug!("Processing chunk data");

        Chunk {
            pos: IVec2::new(data.x.0, data.z.0),

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
            return &BLOCKS[0];
        }
        return match &self.sections[y / 16] {
            Some(cs) => &BLOCKS[cs.blocks[((y % 16) * 16 * 16 + z * 16 + x) as usize] as usize],
            None => &BLOCKS[0],
        };
    }

    pub fn get_sections(&self) -> &[Option<ChunkSection>; 16] {
        &self.sections
    }
}

/// Extracts the heightmap from chunk data
fn process_heightmap(data: &ChunkData) -> [u16; 256] {
    let mut map = [0u16; 256];

    match &data.heightmaps.0.get::<_, &Vec<i64>>("MOTION_BLOCKING") {
        Ok(list) => {
            let vals_per_long: usize = 7;
            for i in 0..256 as usize {
                let long = i / vals_per_long;
                let offset = (i % vals_per_long) * 9;

                map[i] = ((list[long] >> offset) & 0x1ff) as u16;
            }
        }
        Err(e) => {
            panic!("Invalid chunk data: {}", e);
        }
    }

    map
}

const INIT: Option<ChunkSection> = None;
/// Builds a list of chunk sections from chunk data
fn process_sections(dis: &Display, data: &ChunkData) -> [Option<ChunkSection>; 16] {
    // Check bit mask for which chunk sections are present
    let mut chunk_sections_present = [false; 16];
    for i in 0..16 as usize {
        if data.bit_mask[0].0 & 0b1 << i != 0 {
            chunk_sections_present[i] = true;
        }
    }

    let mut sections = [INIT; 16];
    // let mut sections: [Option<ChunkSection>; 16] = Default::default();

    // Decode data array
    let mut pd = PacketDecoder::new(&data.data, 0);
    for i in 0..16 as usize {
        if !chunk_sections_present[i] {
            continue;
        }

        let block_count = pd.next_short().0;

        let mut bits_per_block = pd.next_ubyte().0 as u64;

        if bits_per_block <= 4 {
            bits_per_block = 4;
        }
        if bits_per_block >= 9 {
            bits_per_block = MAX_BITS_PER_BLOCK;
        }

        let palette: Option<Vec<i32>>;

        // Construct palette or no palette
        if bits_per_block < 9 {
            let palette_len = pd.next_varint();
            let mut palette_vec: Vec<i32> = Vec::new();

            for p in 0..palette_len.0 as usize {
                palette_vec.push(pd.next_varint().0);
            }
            palette = Some(palette_vec);
        } else {
            palette = None;
        }

        // Get long array of blocks
        let array_len = pd.next_varint();
        let mut array = Vec::new();

        for _ in 0..array_len.0 as usize {
            array.push(pd.next_long().0 as u64);
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
        for j in 0..4096 as u64 {
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

        let vbo = generate_mesh(dis, &blocks);

        sections[i] = Some(ChunkSection {
            y: i as i32,
            blocks,
            vbo,
        });
    }
    sections
}

fn generate_mesh(dis: &Display, blocks: &[u16; 4096]) -> VertexBuffer<Vertex> {
    let mut shapes: Vec<Vertex> = Vec::new();

    for (i, b) in blocks.iter().enumerate() {
        if *b == 0 {
            continue;
        }

        let y = (i / (16 * 16)) as f32;
        let z = ((i / 16) % 16) as f32;
        let x = (i % 16) as f32;

        // Top Face
        let nx = x as i32;
        let ny = y as i32 + 1;
        let nz = z as i32;
        let ni = ((ny % 16) * 16 * 16 + nz as i32 * 16 + nx as i32) as i32;
        let mut extra = false;
        if ni > 0 && ni < 4096 {
            extra = blocks[ni as usize] == 0;
        }
        if nx < 0 || nx > 15 || ny < 0 || ny > 15 || nz < 0 || nz > 15 || extra {
            shapes.push(Vertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y + 1.0, z],
            });
            shapes.push(Vertex {
                position: [x, y + 1.0, z],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x, y + 1.0, z],
            });
            shapes.push(Vertex {
                position: [x, y + 1.0, z + 1.0],
            });
        }

        // Bottom Face
        let nx = x as i32;
        let ny = y as i32 - 1;
        let nz = z as i32;
        let ni = ((ny % 16) * 16 * 16 + nz as i32 * 16 + nx as i32) as i32;
        let mut extra = false;
        if ni > 0 && ni < 4096 {
            extra = blocks[ni as usize] == 0;
        }
        if nx < 0 || nx > 15 || ny < 0 || ny > 15 || nz < 0 || nz > 15 || extra {
            shapes.push(Vertex {
                position: [x + 1.0, y, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x, y, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x, y, z],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x, y, z],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y, z],
            });
        }

        // North Face
        let nx = x as i32;
        let ny = y as i32;
        let nz = z as i32 - 1;
        let ni = ((ny % 16) * 16 * 16 + nz as i32 * 16 + nx as i32) as i32;
        let mut extra = false;
        if ni > 0 && ni < 4096 {
            extra = blocks[ni as usize] == 0;
        }
        if nx < 0 || nx > 15 || ny < 0 || ny > 15 || nz < 0 || nz > 15 || extra {
            shapes.push(Vertex {
                position: [x + 1.0, y + 1.0, z],
            });
            shapes.push(Vertex {
                position: [x, y, z],
            });
            shapes.push(Vertex {
                position: [x, y + 1.0, z],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y + 1.0, z],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y, z],
            });
            shapes.push(Vertex {
                position: [x, y, z],
            });
        }

        // South Face
        let nx = x as i32;
        let ny = y as i32;
        let nz = z as i32 + 1;
        let ni = ((ny % 16) * 16 * 16 + nz as i32 * 16 + nx as i32) as i32;
        let mut extra = false;
        if ni > 0 && ni < 4096 {
            extra = blocks[ni as usize] == 0;
        }
        if nx < 0 || nx > 15 || ny < 0 || ny > 15 || nz < 0 || nz > 15 || extra {
            shapes.push(Vertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x, y + 1.0, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x, y, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x, y, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y, z + 1.0],
            });
        }

        // East Face
        let nx = x as i32 + 1;
        let ny = y as i32;
        let nz = z as i32;
        let ni = ((ny % 16) * 16 * 16 + nz as i32 * 16 + nx as i32) as i32;
        let mut extra = false;
        if ni > 0 && ni < 4096 {
            extra = blocks[ni as usize] == 0;
        }
        if nx < 0 || nx > 15 || ny < 0 || ny > 15 || nz < 0 || nz > 15 || extra {
            shapes.push(Vertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y, z],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y + 1.0, z],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y + 1.0, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x + 1.0, y, z],
            });
        }

        // West Face
        let nx = x as i32 - 1;
        let ny = y as i32;
        let nz = z as i32;
        let ni = ((ny % 16) * 16 * 16 + nz as i32 * 16 + nx as i32) as i32;
        let mut extra = false;
        if ni > 0 && ni < 4096 {
            extra = blocks[ni as usize] == 0;
        }
        if nx < 0 || nx > 15 || ny < 0 || ny > 15 || nz < 0 || nz > 15 || extra {
            shapes.push(Vertex {
                position: [x, y + 1.0, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x, y + 1.0, z],
            });
            shapes.push(Vertex {
                position: [x, y, z],
            });
            shapes.push(Vertex {
                position: [x, y + 1.0, z + 1.0],
            });
            shapes.push(Vertex {
                position: [x, y, z],
            });
            shapes.push(Vertex {
                position: [x, y, z + 1.0],
            });
        }
    }

    glium::VertexBuffer::new(dis, &shapes).unwrap()
}
