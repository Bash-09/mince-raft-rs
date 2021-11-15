use glam::{Mat4, Vec3};
use glium::index::NoIndices;
use glium::{Display, Surface};
use glium::*;
use log::info;

use crate::client::renderer::camera::Camera;

use super::{Client, server::Server};

mod shader;
mod camera;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);

pub struct Renderer {

    pub cam: Camera,

    vbo: VertexBuffer<Vertex>,
    inds: NoIndices,
    prog: Program,

}


impl Renderer {

    pub fn new(dis: &Display) -> Renderer {

        let v1 = Vertex { position: [-0.5, -0.5, 0.0]};
        let v2 = Vertex { position: [0.0, 0.5, 0.0]};
        let v3 = Vertex { position: [0.5, -0.25, 0.0]};
        let shape = vec![v1, v2, v3];
        let vbo = glium::VertexBuffer::new(dis, &shape).unwrap();
        let inds = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let prog = shader::read_shader(dis, "shaders/test/v.glsl", "shaders/test/f.glsl").expect("Failed to compile shaders");

        info!("Setup renderer!");

        Renderer {

            cam: Camera::new_with_values(
                dis.get_framebuffer_dimensions(),
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(0.0, 0.0, 0.0),
                90.0
            ),

            vbo,
            inds,
            prog,
        }
    }

    pub fn render_server(&mut self, target: &mut Frame, serv: &Server) {

        target.clear_color_and_depth((0.5, 0.7, 0.8, 1.0), 1.0);

        let params = DrawParameters {
            depth: Depth {
                test: draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };


        let vf = self.cam.generate_view_frustum();
        let pvmat = self.cam.get_pvmat().to_cols_array_2d();
        let mut points = vec![Vec3::new(0.0, 0.0, 0.0); 8];

        for (pos, chunk) in serv.world.get_chunks() {
            for (y, section) in chunk.get_sections().iter().enumerate() {
                match section {
                    None => continue,
                    Some(cs) => {

                        let cx = (pos.x * 16) as f32;
                        let cy = (y     * 16) as f32;
                        let cz = (pos.y * 16) as f32;

                        // Get points for corners of chunk section
                        points[0].x = cx;        points[0].y = cy;               points[0].z =  cz;
                        points[1].x = cx + 16.0; points[1].y = cy;               points[1].z =  cz;
                        points[2].x = cx;        points[2].y = cy + 16.0;        points[2].z =  cz;
                        points[3].x = cx;        points[3].y = cy;               points[3].z =  cz + 16.0;
                        points[4].x = cx + 16.0; points[4].y = cy + 16.0;        points[4].z =  cz;
                        points[5].x = cx + 16.0; points[5].y = cy;               points[5].z =  cz + 16.0;
                        points[6].x = cx;        points[6].y = cy + 16.0;        points[6].z =  cz + 16.0;
                        points[7].x = cx + 16.0; points[7].y = cy + 16.0;        points[7].z =  cz + 16.0;

                        // Frustum cull this chunk section
                        if !vf.accept_points(&points) {continue}

                        let tmat: Mat4 = Mat4::from_translation(Vec3::new(
                            cx, cy, cz,
                        ));

                        let uniforms = uniform! {
                            pvmat: pvmat,
                            tmat: tmat.to_cols_array_2d(),
                        };

                        target.draw(cs.get_vbo(), &self.inds, &self.prog, &uniforms, &params).unwrap();

                    }
                }

            }
        }

    }

}