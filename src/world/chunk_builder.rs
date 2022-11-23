use std::convert::TryInto;

use glam::IVec3;

use crate::renderer::Vertex;

use super::chunks::ChunkSection;

pub struct ChunkBuilder {}

impl ChunkBuilder {
    pub fn generate_mesh(
        section: &ChunkSection,
        above: &Option<ChunkSection>,
        below: &Option<ChunkSection>,
        north: &Option<ChunkSection>,
        east: &Option<ChunkSection>,
        south: &Option<ChunkSection>,
        west: &Option<ChunkSection>,
    ) -> Vec<Vertex> {
        let mut verts: Vec<Vertex> = Vec::new();

        for (i, b) in section.blocks.iter().enumerate() {
            if *b == 0 {
                continue;
            }

            let y = (i / (16 * 16)) as i32;
            let z = ((i / 16) % 16) as i32;
            let x = (i % 16) as i32;

            // Top Face
            let nx = x;
            let ny = (y + 1).rem_euclid(16);
            let nz = z;
            let ni = (ny % 16) * 16 * 16 + nz * 16 + nx;
            let b_above = if y == 15 {
                if let Some(above) = above {
                    above.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            // Bottom Face
            let nx = x;
            let ny = (y - 1).rem_euclid(16);
            let nz = z;
            let ni = (ny % 16) * 16 * 16 + nz * 16 + nx;
            let b_below = if y == 0 {
                if let Some(below) = below {
                    below.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            // North Face
            let nx = x;
            let ny = y;
            let nz = (z - 1).rem_euclid(16);
            let ni = (ny % 16) * 16 * 16 + nz * 16 + nx;
            let b_north = if z == 0 {
                if let Some(north) = north {
                    north.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            // South Face
            let nx = x;
            let ny = y;
            let nz = (z + 1).rem_euclid(16);
            let ni = (ny % 16) * 16 * 16 + nz * 16 + nx;
            let b_south = if z == 15 {
                if let Some(south) = south {
                    south.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            // East Face
            let nx = (x + 1).rem_euclid(16);
            let ny = y;
            let nz = z;
            let ni = (ny % 16) * 16 * 16 + nz * 16 + nx;
            let b_east = if x == 15 {
                if let Some(east) = east {
                    east.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            // West Face
            let nx = (x - 1).rem_euclid(16);
            let ny = y;
            let nz = z;
            let ni = (ny % 16) * 16 * 16 + nz * 16 + nx;
            let b_west = if x == 0 {
                if let Some(west) = west {
                    west.blocks[ni as usize]
                } else {
                    0
                }
            } else {
                section.blocks[ni as usize]
            };

            verts.append(&mut ChunkBuilder::generate_block_mesh(
                IVec3::new(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    z.try_into().unwrap(),
                ),
                *b,
                b_above,
                b_below,
                b_north,
                b_east,
                b_south,
                b_west,
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
