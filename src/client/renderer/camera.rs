use std::ops::{Mul, MulAssign};

use cgmath::{Deg, Matrix4, PerspectiveFov, Rad, SquareMatrix, Vector1, Vector3, prelude::*};
use glium::{Display, buffer::Content};


const NEAR_PLANE: f32 = 0.001;
const FAR_PLANE: f32 = 10_000.0;

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
            near: NEAR_PLANE,
            far: FAR_PLANE,
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


    pub fn get_look_vector(&self) -> Vector3<f32> {
        let mut dir: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

        let mut vmat: Matrix4<f32> = Matrix4::identity();

        vmat = vmat * Matrix4::from_angle_x(Deg(-self.rot.y));
        vmat = vmat * Matrix4::from_angle_y(Deg(-self.rot.x + 180.0));
        vmat = vmat * Matrix4::from_angle_z(Deg(-self.rot.z));

        match vmat.inverse_transform_vector(dir) {
            Some(d) => d,
            None => Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn generate_view_frustum(&self) -> ViewFrustum {
        let dir = self.get_look_vector();

        let near_pos = self.get_pos() + (dir * NEAR_PLANE);
        let far_pos = self.get_pos() + (dir * FAR_PLANE);

        let d_near = dir;
        let d_far = dir * -1.0;

        let mut vmat: Matrix4<f32> = Matrix4::identity();

        vmat = vmat * Matrix4::from_angle_x(Deg(-self.rot.y));
        vmat = vmat * Matrix4::from_angle_y(Deg(-self.rot.x + 180.0));
        vmat = vmat * Matrix4::from_angle_z(Deg(-self.rot.z));

        let fov_x = (self.aspect).atan();

        let mut d_left: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
        d_left = Matrix4::from_angle_y(Rad(fov_x)).transform_vector(d_left);
        d_left = d_left.normalize();
        d_left = vmat.inverse_transform_vector(d_left).expect("Couldn't transform vector");

        let mut d_right: Vector3<f32> = Vector3::new(-1.0, 0.0, 0.0);
        d_right = Matrix4::from_angle_y(Rad(-fov_x)).transform_vector(d_right);
        d_right = d_right.normalize();
        d_right = vmat.inverse_transform_vector(d_right).expect("Couldn't transform vector");

        let mut d_bottom: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
        d_bottom = Matrix4::from_angle_x(Deg(-self.fov/2.0)).transform_vector(d_bottom);
        d_bottom = d_bottom.normalize();
        d_bottom = vmat.inverse_transform_vector(d_bottom).expect("Couldn't transform vector");

        let mut d_top: Vector3<f32> = Vector3::new(0.0, -1.0, 0.0);
        d_top = Matrix4::from_angle_x(Deg(self.fov/2.0)).transform_vector(d_top);
        d_top = d_top.normalize();
        d_top = vmat.inverse_transform_vector(d_top).expect("Couldn't transform vector");


        ViewFrustum {
            near_pos,
            far_pos,

            d_near,
            d_left,
            d_right,
            d_bottom,
            d_top,
            d_far
        }
    }

}


pub struct ViewFrustum {
    near_pos: Vector3<f32>,
    far_pos: Vector3<f32>,

    d_near: Vector3<f32>,
    d_left: Vector3<f32>,
    d_right: Vector3<f32>,
    d_bottom: Vector3<f32>,
    d_top: Vector3<f32>,
    d_far: Vector3<f32>
}

impl ViewFrustum {

    pub fn accept_point(&self, point: &Vector3<f32>) -> bool {
        if !ViewFrustum::check_plane(&self.near_pos, &self.d_near, point) {return false}
        if !ViewFrustum::check_plane(&self.near_pos, &self.d_left, point) {return false}
        if !ViewFrustum::check_plane(&self.near_pos, &self.d_right, point) {return false}
        if !ViewFrustum::check_plane(&self.near_pos, &self.d_bottom, point) {return false}
        if !ViewFrustum::check_plane(&self.near_pos, &self.d_top, point) {return false}
        if !ViewFrustum::check_plane(&self.far_pos, &self.d_far, point) {return false}

        true
    }


    pub fn accept_points(&self, points: &Vec<Vector3<f32>>) -> bool {

        let mut accepted = false;
        for p in points {
            if ViewFrustum::check_plane(&self.near_pos,&self.d_near, p) {accepted = true}
        }
        if !accepted {return false}

        accepted = true;
        for p in points {
            if ViewFrustum::check_plane(&self.near_pos,&self.d_left, p) {accepted = true}
        }
        if !accepted {return false}

        accepted = true;
        for p in points {
            if ViewFrustum::check_plane(&self.near_pos,&self.d_right, p) {accepted = true}
        }
        if !accepted {return false}

        accepted = true;
        for p in points {
            if ViewFrustum::check_plane(&self.near_pos,&self.d_bottom, p) {accepted = true}
        }
        if !accepted {return false}

        accepted = true;
        for p in points {
            if ViewFrustum::check_plane(&self.near_pos,&self.d_top, p) {accepted = true}
        }
        if !accepted {return false}

        accepted = true;
        for p in points {
            if ViewFrustum::check_plane(&self.far_pos,&self.d_far, p) {accepted = true}
        }
        if !accepted {return false}

        true
    }

    fn check_plane(plane_pos: &Vector3<f32>, plane_norm: &Vector3<f32>, point: &Vector3<f32>) -> bool {
        let v = point - plane_pos;
        plane_norm.dot(v) > 0.0
    }

}