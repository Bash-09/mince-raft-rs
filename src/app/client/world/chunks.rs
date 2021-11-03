use quartz_nbt::NbtTag;

use crate::network::packets::ChunkData;

pub struct Chunk {
    x: i32,
    z: i32,

    heightmap: [u16; 256],
}

impl Chunk {
    pub fn new(data: &ChunkData) -> Chunk {
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
