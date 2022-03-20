use std::collections::HashMap;

use glam::{Mat4, Vec3};
use glium::index::{NoIndices, PrimitiveType::{LineStrip, TrianglesList}};
use glium::*;
use glium::{Display, Surface};
use log::info;

use crate::{renderer::camera::Camera, entities::{self, Entity}};

use super::{server::Server};

mod camera;
mod shader;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);

pub struct Renderer {
    pub cam: Camera,

    chunk_prog: Program,


    hitbox_prog: Program,
    hitbox_model: VertexBuffer<Vertex>,

}

impl Renderer {
    pub fn new(dis: &Display) -> Renderer {

        let hitbox_model = glium::VertexBuffer::new(dis, &entities::hitbox_model()).unwrap();

        let prog = shader::compile_shaders(dis, include_bytes!("../shaders/test/v.glsl"), include_bytes!("../shaders/test/f.glsl"))
            .expect("Failed to compile shaders");

        let hitbox_prog = shader::compile_shaders(dis, include_bytes!("../shaders/hitboxes/v.glsl"), include_bytes!("../shaders/hitboxes/f.glsl"))
            .expect("Failed to compile shaders");

        log::debug!("Setup renderer!");

        Renderer {
            cam: Camera::new_with_values(
                dis.get_framebuffer_dimensions(),
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(0.0, 0.0, 0.0),
                90.0,
            ),

            hitbox_model,
            chunk_prog: prog,
            hitbox_prog,
        }
    }

    pub fn render_hitboxes(&mut self, target: &mut Frame, ents: &HashMap<i32, Entity>) {

        let params = DrawParameters {
            depth: Depth {
                test: draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullingDisabled,
            polygon_mode: PolygonMode::Line,
            line_width: Some(2.0),
            ..Default::default()
        };

        let inds = NoIndices(glium::index::PrimitiveType::LinesList);
        let pvmat = self.cam.get_pvmat().to_cols_array_2d();

        for ent in ents.values() {
            let e = ent.get_type();

            let mut tmat = Mat4::IDENTITY;
            tmat *= Mat4::from_translation(ent.pos);
            tmat *= Mat4::from_scale(Vec3::new(e.width, e.height, e.width));
    
            let uniforms = uniform! {
                pvmat: pvmat,
                tmat: tmat.to_cols_array_2d(),
            };

            target.draw(&self.hitbox_model, inds, &self.hitbox_prog, &uniforms, &params)
                .expect("Error rendering hitbox");

        }

    }

    pub fn render_server(&mut self, target: &mut Frame, serv: &Server) {
        target.clear_color_and_depth((0.5, 0.7, 0.8, 1.0), 1.0);

        let params = DrawParameters {
            depth: Depth {
                test: draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let inds = NoIndices(TrianglesList);

        let vf = self.cam.generate_view_frustum();
        let pvmat = self.cam.get_pvmat().to_cols_array_2d();
        let mut points = vec![Vec3::new(0.0, 0.0, 0.0); 8];

        for (pos, chunk) in serv.get_world().get_chunks() {

            // Try to frustum cull this whole chunk column
            let cx = (pos.x * 16) as f32;
            let cz = (pos.y * 16) as f32;

            points[0] = Vec3::new(cx, 0.0, cz);
            points[1] = Vec3::new(cx + 16.0, 0.0, cz);
            points[2] = Vec3::new(cx + 16.0, 0.0, cz + 16.0);
            points[3] = Vec3::new(cx, 0.0, cz + 16.0);
            points[4] = Vec3::new(cx, 256.0, cz);
            points[5] = Vec3::new(cx + 16.0, 256.0, cz);
            points[6] = Vec3::new(cx + 16.0, 256.0, cz + 16.0);
            points[7] = Vec3::new(cx, 256.0, cz + 16.0);

            if !vf.accept_points(&points) {
                continue;
            }

            for (y, section) in chunk.get_sections().iter().enumerate() {
                match section {
                    None => continue,
                    Some(cs) => {
                        let cy = (y * 16) as f32;

                        // Get points for corners of chunk section
                        points[0].x = cx;
                        points[0].y = cy;
                        points[0].z = cz;
                        points[1].x = cx + 16.0;
                        points[1].y = cy;
                        points[1].z = cz;
                        points[2].x = cx;
                        points[2].y = cy + 16.0;
                        points[2].z = cz;
                        points[3].x = cx;
                        points[3].y = cy;
                        points[3].z = cz + 16.0;
                        points[4].x = cx + 16.0;
                        points[4].y = cy + 16.0;
                        points[4].z = cz;
                        points[5].x = cx + 16.0;
                        points[5].y = cy;
                        points[5].z = cz + 16.0;
                        points[6].x = cx;
                        points[6].y = cy + 16.0;
                        points[6].z = cz + 16.0;
                        points[7].x = cx + 16.0;
                        points[7].y = cy + 16.0;
                        points[7].z = cz + 16.0;

                        // Frustum cull this chunk section
                        if !vf.accept_points(&points) {
                            continue;
                        }

                        let tmat: Mat4 = Mat4::from_translation(Vec3::new(cx, cy, cz));

                        let uniforms = uniform! {
                            pvmat: pvmat,
                            tmat: tmat.to_cols_array_2d(),
                        };

                        target
                            .draw(cs.get_vbo(), inds, &self.chunk_prog, &uniforms, &params)
                            .unwrap();
                    }
                }
            }
        }

        self.render_hitboxes(target, serv.get_entities());

    }
}
