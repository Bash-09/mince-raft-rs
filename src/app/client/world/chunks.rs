use quartz_nbt::NbtTag;
use resources::blocks::BLOCKS;

use crate::network::{packets::{ChunkData, PacketDecoder}, types::VarInt};


pub struct ChunkSection {
    y: i32,

    blocks: [u16; 4096],
}


pub struct Chunk {
    x: i32,
    z: i32,

    heightmap: [u16; 256],
}

const MAX_BITS_PER_BLOCK: u64 = 15;

impl Chunk {
    pub fn new(data: &ChunkData) -> Chunk {

        println!("\n\nBEGIN CHUNK: {}, {}", data.x.0, data.z.0);

        println!("Bit mask length: {}", data.bit_mask_len.0);

        let mut chunk_sections_present = [false; 16];
        for i in 0..16 as usize {
            if data.bit_mask[0].0 & 0b1 << i != 0 {
                chunk_sections_present[i] = true;
            }
        }

        let mut pd = PacketDecoder::new(&data.data, 0);
        for i in 0..16 as usize {
            if !chunk_sections_present[i] {continue}

            // pd.print_remaining_bytes();

            println!("\n\n BEGIN CHUNK SECTION: {}", i);

            let block_count = pd.next_short().0;
            // GETTING WEIRD VALUES FOR THE BLOCK COUNT
            // Just gonna ignore it for now since it doesn't seem like a problem for now

            println!("Block count: {}", block_count);

            let mut bits_per_block = pd.next_ubyte().0 as u64;

            println!("Bits per block: {}", bits_per_block);

            if bits_per_block <= 4 {bits_per_block = 4;}
            if bits_per_block >= 9 {bits_per_block = MAX_BITS_PER_BLOCK;}

            let palette: Option<Vec<i32>>;

            if bits_per_block < 9 {
                let palette_len = pd.next_varint();
                let mut palette_vec: Vec<i32> = Vec::new();

                println!("Palette length: {}", palette_len.0);

                for p in 0..palette_len.0 as usize {
                    palette_vec.push(pd.next_varint().0);
                }
                palette = Some(palette_vec);
            } else {
                palette = None;
            }


            // if data.x.0 == -12 && data.z.0 == 5 && i == 9 {
                println!("\n\nPalette: ");

                match &palette {
                    Some(pal) => {

                        for p in pal.iter() {
                            print!("{}:", p);
                            if *p < 0 || *p >= BLOCKS.len() as i32 {
                                println!("Invalid block!");
                                continue;
                            }

                            println!("\t{}", BLOCKS[*p as usize].name);

                        }
                    },
                    None => {
                        println!("No palette!");
                    }
                }
            // }



            let array_len = pd.next_varint();
            println!("Array length: {}", array_len.0);
            let mut array = Vec::new();

            for _ in 0..array_len.0 as usize {
                array.push(pd.next_long().0 as u64);
            }

            let mut mask = 0;
            for j in 0..bits_per_block {
                mask |= 1 << j;
            }

            let blocks_per_long = 64 / bits_per_block;

            let mut blocks = [0u16; 4096];

            for j in 0..4096 as u64 {
                let long = j / blocks_per_long;
                let start = (j % blocks_per_long) * bits_per_block;

                let block = (array[long as usize] >> start) & mask;

                match &palette {
                    Some(pal) => {
                        blocks[j as usize] = pal[block as usize] as u16;
                    },
                    None => {
                        blocks[j as usize] = block as u16;
                    }
                }

            }


        }

        if data.x.0 == -12 && data.z.0 == 5 {
            // panic!("Stopped in chunk");
        }


        Chunk {
            x: data.x.0,
            z: data.z.0,
    
            heightmap: process_heightmap(data),
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
}


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
