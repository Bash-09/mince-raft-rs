

#[derive(Debug)]
pub struct Position {

    x: f64,
    y: f64,
    z: f64,

}

impl Position {

    pub fn new() -> Position {
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new_with_values(x: f64, y: f64, z: f64) -> Position {
        Position { x, y, z}
    }

    pub fn set(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }

}



#[derive(Debug)]
pub struct Velocity {

    x: f64,
    y: f64,
    z: f64,

}

impl Velocity {

    pub fn new() -> Velocity {
        Velocity {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new_with_values(x: f64, y: f64, z: f64) -> Velocity {
        Velocity { x, y, z}
    }

    pub fn set(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude2().sqrt()
    }

    pub fn magnitude2(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }
}



#[derive(Debug)]
pub struct Orientation {

    yaw: f64,
    pitch: f64,
    head_pitch: f64,

}

impl Orientation {

    pub fn new() -> Orientation {
        Orientation {
            yaw: 0.0,
            pitch: 0.0,
            head_pitch: 0.0,
        }
    }

    pub fn new_with_values(yaw: f64, pitch: f64, head_pitch: f64) -> Orientation {
        Orientation {yaw, pitch, head_pitch}
    }

    pub fn set(&mut self, yaw: f64, pitch: f64, head_pitch: f64) {
        self.yaw = yaw;
        self.pitch = pitch;
        self.head_pitch = head_pitch;
    }

    pub fn set_look_by_vector(&mut self, x: f64, y: f64, z: f64) {
        let r = (x*x + y*y + z*z).sqrt();
        let mut yaw = -x.atan2(z).to_degrees();
        if yaw < 0.0 {yaw += 360.0;}
        let pitch = -(y/r).asin().to_degrees();
        self.yaw = yaw;
        self.pitch = pitch;
    }

    pub fn rotate(&mut self, yaw: f64, pitch: f64, head_pitch: f64) {
        self.yaw += yaw;
        self.yaw %= 360.0;

        self.pitch += pitch;
        self.pitch %= 360.0;

        self.head_pitch += head_pitch;
        if self.head_pitch < -90.0 {self.head_pitch = -90.0;}
        if self.head_pitch > 90.0 {self.head_pitch = 90.0;}

    }


    pub fn get_yaw(&self) -> f64 {
        self.yaw
    }

    pub fn get_pitch(&self) -> f64 {
        self.pitch
    }

    pub fn get_head_pitch(&self) -> f64 {
        self.head_pitch
    }

    pub fn get_look_vector(&self) -> (f64, f64, f64) {
        let x = -self.head_pitch.to_radians().cos() * self.yaw.to_radians().sin();
        let y = -self.head_pitch.to_radians().sin();
        let z =  self.head_pitch.to_radians().cos() * self.yaw.to_radians().cos();
        (x, y, z)
    }

}