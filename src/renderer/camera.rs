use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use glium::{buffer::Content, Display};

const NEAR_PLANE: f32 = 0.001;
const FAR_PLANE: f32 = 10_000.0;

pub struct Camera {
    pos: Vec3,
    rot: Vec3,

    fov: f32,
    aspect: f32,

    pmat: Mat4,
    vmat: Mat4,
    pvmat: Mat4,
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
        let pmat = Mat4::IDENTITY;
        let vmat = Mat4::IDENTITY;
        let pvmat = Mat4::IDENTITY;

        Camera {
            pos: Vec3::new(0.0, 0.0, 0.0),
            rot: Vec3::new(0.0, 0.0, 0.0),

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
    /// * `pos: Vec3` - The position in 3D space for this camera to be located at
    /// * `rot: Vec3` - The X/Y/Z rotations for the camera
    /// * `fov: f32` - The fov of the camera along the y-axis
    pub fn new_with_values(dims: (u32, u32), pos: Vec3, rot: Vec3, fov: f32) -> Camera {
        let pmat = Mat4::IDENTITY;
        let vmat = Mat4::IDENTITY;
        let pvmat = Mat4::IDENTITY;

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
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Set the rotation of this camera
    pub fn set_rot(&mut self, rot: Vec3) {
        self.rot = rot;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Set the position and rotation of this camera
    pub fn set_transform(&mut self, pos: Vec3, rot: Vec3) {
        self.pos = pos;
        self.rot = rot;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Translate this camera by the given amount
    pub fn translate(&mut self, pos: Vec3) {
        self.pos += pos;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Rotate this camera by the given amount
    pub fn rotate(&mut self, rot: Vec3) {
        self.rot += rot;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Translate and Rotate this camera by the given amounts
    ///
    /// # Arguments
    ///
    /// * `pos: Vec3` - How much to translate this camera by
    ///
    /// * `rote: Vec3` - How much to rotate this camera by
    pub fn transform(&mut self, pos: Vec3, rot: Vec3) {
        self.pos += pos;
        self.rot += rot;
        self.update_vmat();
        self.update_pvmat();
    }

    /// Returns the current position of the camera
    pub fn get_pos(&self) -> &Vec3 {
        &self.pos
    }
    /// Returns the current X/Y/Z rotations of the camera
    pub fn get_rot(&self) -> &Vec3 {
        &self.rot
    }
    /// Returns the current y-fov of the camera
    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    /// Returns the perspectiva matrix for this camera
    pub fn get_pmat(&self) -> &Mat4 {
        &self.pmat
    }
    /// Returns the view matrix for this camera
    pub fn get_vmat(&self) -> &Mat4 {
        &self.vmat
    }
    /// Returns the combine Perspective/View matrix for this camera
    pub fn get_pvmat(&self) -> &Mat4 {
        &self.pvmat
    }

    fn update_vmat(&mut self) {
        let mut vmat: Mat4 = Mat4::IDENTITY;

        vmat *= Mat4::from_rotation_x((-self.rot.y).to_radians());
        vmat *= Mat4::from_rotation_y((-self.rot.x + 180.0).to_radians());
        vmat *= Mat4::from_rotation_z((-self.rot.z).to_radians());

        vmat *= Mat4::from_translation(self.pos * -1.0);

        self.vmat = vmat;
    }

    fn update_pmat(&mut self) {
        self.pmat = Mat4::perspective_rh(self.fov.to_radians(), self.aspect, NEAR_PLANE, FAR_PLANE);
    }

    fn update_pvmat(&mut self) {
        self.pvmat = Mat4::IDENTITY;
        self.pvmat = self.pvmat * self.pmat;
        self.pvmat = self.pvmat * self.vmat;
    }

    fn update(&mut self) {
        self.update_pmat();
        self.update_vmat();
        self.update_pvmat();
    }

    pub fn get_look_vector(&self) -> Vec3 {
        let mut dir: Vec4 = Vec4::new(0.0, 0.0, -1.0, 1.0);

        let mut vmat: Mat4 = Mat4::IDENTITY;

        vmat *= Mat4::from_rotation_x((-self.rot.y).to_radians());
        vmat *= Mat4::from_rotation_y((-self.rot.x + 180.0).to_radians());
        vmat *= Mat4::from_rotation_z((-self.rot.z).to_radians());

        (vmat.inverse() * dir).xyz()
    }

    pub fn generate_view_frustum(&self) -> ViewFrustum {
        let dir = self.get_look_vector();

        let near_pos = *self.get_pos() + (dir * NEAR_PLANE);
        let far_pos = *self.get_pos() + (dir * FAR_PLANE);

        let d_near = dir;
        let d_far = dir * -1.0;

        let mut vmat: Mat4 = Mat4::IDENTITY;

        vmat = vmat * Mat4::from_rotation_x((-self.rot.y).to_radians());
        vmat = vmat * Mat4::from_rotation_y((-self.rot.x + 180.0).to_radians());
        vmat = vmat * Mat4::from_rotation_z((-self.rot.z).to_radians());

        let inv_vmat = vmat.inverse();

        let fov_x = (self.aspect).atan();

        let mut d_left: Vec4 = Vec4::new(1.0, 0.0, 0.0, 1.0);
        d_left = Mat4::from_rotation_y(fov_x) * d_left;
        d_left = d_left.normalize();
        d_left = inv_vmat * d_left;

        let mut d_right: Vec4 = Vec4::new(-1.0, 0.0, 0.0, 1.0);
        d_right = Mat4::from_rotation_y(-fov_x) * d_right;
        d_right = d_right.normalize();
        d_right = inv_vmat * d_right;

        let mut d_bottom: Vec4 = Vec4::new(0.0, 1.0, 0.0, 1.0);
        d_bottom = Mat4::from_rotation_x((-self.fov / 2.0).to_radians()) * d_bottom;
        d_bottom = d_bottom.normalize();
        d_bottom = inv_vmat * d_bottom;

        let mut d_top: Vec4 = Vec4::new(0.0, -1.0, 0.0, 1.0);
        d_top = Mat4::from_rotation_x((self.fov / 2.0).to_radians()) * d_top;
        d_top = d_top.normalize();
        d_top = inv_vmat * d_top;

        ViewFrustum {
            near_pos,
            far_pos,

            d_near,
            d_left: d_left.xyz(),
            d_right: d_right.xyz(),
            d_bottom: d_bottom.xyz(),
            d_top: d_top.xyz(),
            d_far,
        }
    }
}

pub struct ViewFrustum {
    near_pos: Vec3,
    far_pos: Vec3,

    d_near: Vec3,
    d_left: Vec3,
    d_right: Vec3,
    d_bottom: Vec3,
    d_top: Vec3,
    d_far: Vec3,
}

impl ViewFrustum {
    pub fn accept_point(&self, point: &Vec3) -> bool {
        if !ViewFrustum::check_plane(&self.near_pos, &self.d_near, point) {
            return false;
        }
        if !ViewFrustum::check_plane(&self.near_pos, &self.d_left, point) {
            return false;
        }
        if !ViewFrustum::check_plane(&self.near_pos, &self.d_right, point) {
            return false;
        }
        if !ViewFrustum::check_plane(&self.near_pos, &self.d_bottom, point) {
            return false;
        }
        if !ViewFrustum::check_plane(&self.near_pos, &self.d_top, point) {
            return false;
        }
        if !ViewFrustum::check_plane(&self.far_pos, &self.d_far, point) {
            return false;
        }

        true
    }

    pub fn accept_points(&self, points: &Vec<Vec3>) -> bool {
        let mut accepted = false;
        for p in points {
            if ViewFrustum::check_plane(&self.near_pos, &self.d_near, p) {
                accepted = true
            }
        }
        if !accepted {
            return false;
        }

        accepted = true;
        for p in points {
            if ViewFrustum::check_plane(&self.near_pos, &self.d_left, p) {
                accepted = true
            }
        }
        if !accepted {
            return false;
        }

        accepted = true;
        for p in points {
            if ViewFrustum::check_plane(&self.near_pos, &self.d_right, p) {
                accepted = true
            }
        }
        if !accepted {
            return false;
        }

        accepted = true;
        for p in points {
            if ViewFrustum::check_plane(&self.near_pos, &self.d_bottom, p) {
                accepted = true
            }
        }
        if !accepted {
            return false;
        }

        accepted = true;
        for p in points {
            if ViewFrustum::check_plane(&self.near_pos, &self.d_top, p) {
                accepted = true
            }
        }
        if !accepted {
            return false;
        }

        accepted = true;
        for p in points {
            if ViewFrustum::check_plane(&self.far_pos, &self.d_far, p) {
                accepted = true
            }
        }
        if !accepted {
            return false;
        }

        true
    }

    fn check_plane(plane_pos: &Vec3, plane_norm: &Vec3, point: &Vec3) -> bool {
        let v = *point - *plane_pos;
        plane_norm.dot(v) > 0.0
    }
}
