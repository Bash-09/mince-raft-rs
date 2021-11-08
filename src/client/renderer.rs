use cgmath::{Matrix4, SquareMatrix, Vector3};
use glium::backend::Facade;
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
                Vector3::new(0.0, 0.0, 2.0),
                Vector3::new(0.0, 0.0, 0.0),
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
            .. Default::default()
        };

        let pvmat = mat_to_array(self.cam.get_pvmat());

        for (pos, chunk) in serv.world.get_chunks() {
            // if pos.x != 0 || pos.y != 0 {continue;}
            for (y, section) in chunk.get_sections().iter().enumerate() {
                match section {
                    None => continue,
                    Some(cs) => {

                        let tmat: Matrix4<f32> = Matrix4::from_translation(Vector3::new(
                            (pos.x * 16) as f32,
                            (y     * 16) as f32,
                            (pos.y * 16) as f32,
                        ));

                        let uniforms = uniform! {
                            pvmat: pvmat,
                            tmat: mat_to_array(&tmat),
                        };

                        target.draw(cs.get_vbo(), &self.inds, &self.prog, &uniforms, &params).unwrap();

                    }
                }

            }
        }

    }

}

pub fn mat_to_array(m: &Matrix4<f32>) -> [[f32; 4]; 4] {
    [
        [m.x.x, m.x.y, m.x.z, m.x.w],
        [m.y.x, m.y.y, m.y.z, m.y.w],
        [m.z.x, m.z.y, m.z.z, m.z.w],
        [m.w.x, m.w.y, m.w.z, m.w.w]
    ]
}