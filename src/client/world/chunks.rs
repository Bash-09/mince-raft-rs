use log::debug;
use quartz_nbt::NbtTag;
use resources::blocks::{BLOCKS, BlockState};

use crate::client::network::{packets::{ChunkData, PacketDecoder}, types::VarInt};


#[derive(Debug, Clone, Copy)]
pub struct ChunkSection {
    pub y: i32,
    pub blocks: [u16; 4096],
}


pub struct Chunk {
    x: i32,
    z: i32,

    heightmap: [u16; 256],

    sections: [Option<ChunkSection>; 16],
}

// Base 2 Log of number of state ids in the game
const MAX_BITS_PER_BLOCK: u64 = 15;

impl Chunk {
    pub fn new(data: &ChunkData) -> Chunk {

        debug!("Processing chunk data");

        Chunk {
            x: data.x.0,
            z: data.z.0,
    
            heightmap: process_heightmap(data),
            sections: process_sections(data),
        }    
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_z(&self) -> i32 {
        self.z
    }

    pub fn get_coords(&self) -> (i32, i32) {
        (self.x, self.z)
    }

    pub fn get_highest_block(&self, coords: (i32, i32)) -> i32 {
        self.heightmap[coords.1 as usize * 16 + coords.0 as usize] as i32
    }

    pub fn block_at(&self, pos: (i32, i32, i32)) -> &BlockState {
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        let z = pos.2 as usize;
        if y >= 16*16 || x >= 16 || z >= 16 {return &BLOCKS[0]}
        return match &self.sections[y/16] {
            Some(cs) => {
                &BLOCKS[cs.blocks[((y%16)*16*16 + z*16 + x) as usize] as usize]
            },
            None => &BLOCKS[0],
        }
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


/// Builds a list of chunk sections from chunk data
fn process_sections(data: &ChunkData) -> [Option<ChunkSection>; 16] {

    // Check bit mask for which chunk sections are present
    let mut chunk_sections_present = [false; 16];
    for i in 0..16 as usize {
        if data.bit_mask[0].0 & 0b1 << i != 0 {
            chunk_sections_present[i] = true;
        }
    }

    let mut sections = [None; 16];

    // Decode data array
    let mut pd = PacketDecoder::new(&data.data, 0);
    for i in 0..16 as usize {
        if !chunk_sections_present[i] {continue}

        let block_count = pd.next_short().0;

        let mut bits_per_block = pd.next_ubyte().0 as u64;

        if bits_per_block <= 4 {bits_per_block = 4;}
        if bits_per_block >= 9 {bits_per_block = MAX_BITS_PER_BLOCK;}

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
                },
                None => {
                    blocks[j as usize] = block as u16;
                }
            }

        }
        sections[i] = Some(ChunkSection{
            y: i as i32,
            blocks,
        });
    }
    sections
}
