use std::sync::RwLockReadGuard;

use glam::IVec3;

use crate::renderer::Vertex;

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
    ) -> Vec<Vertex> {
        let mut verts: Vec<Vertex> = Vec::new();

        let value = section.blocks;
        for (i, b) in value.iter().enumerate() {
            if *b == 0 {
                continue;
            }

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

            verts.append(&mut ChunkBuilder::generate_block_mesh(
                pos, *b, b_above, b_below, b_north, b_east, b_south, b_west,
            ));
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
    ) -> Vec<Vertex> {
        let mut verts: Vec<Vertex> = Vec::new();
        let pos = pos.as_vec3();

        // Above
        if above == 0 {
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y + 1.0, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y + 1.0, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y + 1.0, pos.z + 1.0],
            });
        }
        // Below
        if below == 0 {
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y, pos.z],
            });
        }
        // North
        if north == 0 {
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y + 1.0, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y, pos.z],
            });
        }
        // East
        if east == 0 {
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y, pos.z],
            });
        }
        // South
        if south == 0 {
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y + 1.0, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y + 1.0, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x + 1.0, pos.y, pos.z + 1.0],
            });
        }
        // West
        if west == 0 {
            verts.push(Vertex {
                position: [pos.x, pos.y + 1.0, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y + 1.0, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y + 1.0, pos.z + 1.0],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y, pos.z],
            });
            verts.push(Vertex {
                position: [pos.x, pos.y, pos.z + 1.0],
            });
        }

        verts
    }
}
