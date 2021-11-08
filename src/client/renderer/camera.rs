use std::ops::{Mul, MulAssign};

use cgmath::{Deg, Matrix4, PerspectiveFov, Rad, SquareMatrix, Vector3, prelude::*};
use glium::Display;



pub struct Camera {
    pos: Vector3<f32>,
    rot: Vector3<f32>,

    fov: f32,
    aspect: f32,

    pmat: Matrix4<f32>,
    vmat: Matrix4<f32>,
    pvmat: Matrix4<f32>,
}

impl Camera {

    /// Creates a new camera with default values and a 
    /// 
    /// # Default Values:
    /// 
    /// * Position: 0,0,0
    /// * Rotation: 0,0,0
    /// * Fov: 90 Degrees
    /// * Asepct Ratio: 1.333
    /// 
    pub fn new() -> Camera {
        let pmat = Matrix4::identity();
        let vmat = Matrix4::identity();
        let pvmat = Matrix4::identity();

        Camera {
            pos: Vector3::new(0.0, 0.0, 0.0),
            rot: Vector3::new(0.0, 0.0, 0.0),

            fov: 90.0,
            aspect: 1.333,

            pmat,
            vmat,
            pvmat,
        }
    }

    /// Create a new camera with the given values
    /// 
    /// # Arguments
    /// 
    /// * `dims: (u32, u32)` - The x/y dimensions of the display, used to calculate the perspective matrix according to the aspect ratio
    /// * `pos: Vector3<f32>` - The position in 3D space for this camera to be located at
    /// * `rot: Vector3<f32>` - The X/Y/Z rotations for the camera
    /// * `fov: f32` - The fov of the camera along the y-axis
    pub fn new_with_values(dims: (u32, u32), pos: Vector3<f32>, rot: Vector3<f32>, fov: f32) -> Camera {

        let pmat = Matrix4::identity();
        let vmat = Matrix4::identity();
        let pvmat = Matrix4::identity();

        let mut cam = Camera {
            pos,
            rot,

            fov,
            aspect: dims.0 as f32 / dims.1 as f32,

            pmat,
            vmat,
            pvmat,
        };
        cam.update();
        cam
    }


    /// Set the fov in the y-direction of this camera
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.update_pmat();
        self.update_pvmat();
    }

    /// Recalculate the Perspective matrix for this camera according to the aspect ratio of the given dimensions
    pub fn set_window_size(&mut self, dims: (u32, u32)) {
        self.aspect = dims.0 as f32 / dims.1 as f32;
        self.update_pmat();
        self.update_pvmat();
    }

    /// Recalculate the Perspective matrix for this camera according to a new aspect ratio
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.update_pmat();
        self.update_pvmat();
    }

    /// Set the position of this camera
    pub fn set_pos(&mut self, pos: Vector3<f32>) {
        self.pos = pos;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Set the rotation of this camera
    pub fn set_rot(&mut self, rot: Vector3<f32>) {
        self.rot = rot;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Set the position and rotation of this camera
    pub fn set_transform(&mut self, pos: Vector3<f32>, rot: Vector3<f32>) {
        self.pos = pos;
        self.rot = rot;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Translate this camera by the given amount
    pub fn translate(&mut self, pos: Vector3<f32>) {
        self.pos += pos;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Rotate this camera by the given amount
    pub fn rotate(&mut self, rot: Vector3<f32>) {
        self.rot += rot;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Translate and Rotate this camera by the given amounts
    /// 
    /// # Arguments
    /// 
    /// * `pos: Vector3<f32>` - How much to translate this camera by
    /// 
    /// * `rote: Vector3<f32>` - How much to rotate this camera by
    pub fn transform(&mut self, pos: Vector3<f32>, rot: Vector3<f32>) {
        self.pos += pos;
        self.rot += rot;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Returns the current position of the camera
    pub fn get_pos(&self) -> &Vector3<f32> {
        &self.pos
    }
    /// Returns the current X/Y/Z rotations of the camera
    pub fn get_rot(&self) -> &Vector3<f32> {
        &self.rot
    }
    /// Returns the current y-fov of the camera
    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    /// Returns the perspectiva matrix for this camera
    pub fn get_pmat(&self) -> &Matrix4<f32> {
        &self.pmat
    }
    /// Returns the view matrix for this camera
    pub fn get_vmat(&self) -> &Matrix4<f32> {
        &self.vmat
    }
    /// Returns the combine Perspective/View matrix for this camera
    pub fn get_pvmat(&self) -> &Matrix4<f32> {
        &self.pvmat
    }


    fn update_vmat(&mut self) {
        let mut vmat: Matrix4<f32> = Matrix4::identity();

        // vmat.z.z = -1.0; // Reflect across x/y plane

        vmat = vmat * Matrix4::from_angle_x(Deg(-self.rot.y));
        vmat = vmat * Matrix4::from_angle_y(Deg(-self.rot.x + 180.0));
        vmat = vmat * Matrix4::from_angle_z(Deg(-self.rot.z));
        vmat = vmat * Matrix4::from_translation(self.pos * -1.0);
        self.vmat = vmat;
    }

    fn update_pmat(&mut self) {
        self.pmat = Matrix4::from(PerspectiveFov{
            fovy: Rad::from(Deg(self.fov)),
            aspect: self.aspect,
            near: 0.001,
            far: 10_000.0,
        });
    }

    fn update_pvmat(&mut self) {
        self.pvmat = Matrix4::identity();
        self.pvmat = self.pvmat * self.pmat;
        self.pvmat = self.pvmat * self.vmat;
    }

    fn update(&mut self) {
        self.update_pmat();
        self.update_vmat();
        self.update_pvmat();
    }

}