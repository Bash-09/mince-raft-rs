use std::sync::RwLockReadGuard;

use glam::IVec3;

use crate::{
    renderer::BlockVertex,
    resources::{block_models::BlockModel, BLOCKS, BLOCK_MODELS_PARSED, BLOCK_TEXTURES},
};

use super::chunks::{block_index_to_pos, block_pos_to_index, ChunkSection};

pub struct ChunkBuilder {}

impl ChunkBuilder {
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
            // verts.append(&mut ChunkBuilder::generate_block_mesh(
            //     pos, *b, b_above, b_below, b_north, b_east, b_south, b_west,
            // ));
        }

        verts
    }

    fn generate_block_mesh(
        pos: IVec3,
        block: u16,
        above: u16,
        below: u16,
        north: u16,
        east: u16,
        south: u16,
        west: u16,
    ) -> Vec<BlockVertex> {
        let mut verts: Vec<BlockVertex> = Vec::new();
        let pos = pos.as_vec3();

        // Above
        if above == 0 {
            let tex: f32 = BLOCK_TEXTURES.get("oak_log_top").unwrap().index as f32;

            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z],
                tex_coords: [1.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y + 1.0, pos.z],
                tex_coords: [0.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y + 1.0, pos.z],
                tex_coords: [0.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y + 1.0, pos.z + 1.0],
                tex_coords: [0.0, 1.0, tex],
            });
        }
        // Below
        if below == 0 {
            let tex: f32 = BLOCK_TEXTURES.get("oak_log_top").unwrap().index as f32;
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y, pos.z + 1.0],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y, pos.z + 1.0],
                tex_coords: [0.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y, pos.z],
                tex_coords: [0.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y, pos.z + 1.0],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y, pos.z],
                tex_coords: [0.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y, pos.z],
                tex_coords: [1.0, 0.0, tex],
            });
        }
        // North
        if north == 0 {
            let tex: f32 = BLOCK_TEXTURES.get("grass_block_side").unwrap().index as f32;
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y, pos.z],
                tex_coords: [0.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y + 1.0, pos.z],
                tex_coords: [0.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y, pos.z],
                tex_coords: [1.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y, pos.z],
                tex_coords: [0.0, 0.0, tex],
            });
        }
        // East
        if east == 0 {
            let tex: f32 = BLOCK_TEXTURES.get("grass_block_side").unwrap().index as f32;
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y, pos.z],
                tex_coords: [0.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z],
                tex_coords: [0.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y, pos.z + 1.0],
                tex_coords: [1.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y, pos.z],
                tex_coords: [0.0, 0.0, tex],
            });
        }
        // South
        if south == 0 {
            let tex: f32 = BLOCK_TEXTURES.get("grass_block_side").unwrap().index as f32;
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y + 1.0, pos.z + 1.0],
                tex_coords: [0.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y, pos.z + 1.0],
                tex_coords: [0.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y, pos.z + 1.0],
                tex_coords: [0.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x + 1.0, pos.y, pos.z + 1.0],
                tex_coords: [1.0, 0.0, tex],
            });
        }
        // West
        if west == 0 {
            let tex: f32 = BLOCK_TEXTURES.get("grass_block_side").unwrap().index as f32;
            verts.push(BlockVertex {
                position: [pos.x, pos.y + 1.0, pos.z + 1.0],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y + 1.0, pos.z],
                tex_coords: [0.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y, pos.z],
                tex_coords: [0.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y + 1.0, pos.z + 1.0],
                tex_coords: [1.0, 1.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y, pos.z],
                tex_coords: [0.0, 0.0, tex],
            });
            verts.push(BlockVertex {
                position: [pos.x, pos.y, pos.z + 1.0],
                tex_coords: [1.0, 0.0, tex],
            });
        }

        verts
    }
}
