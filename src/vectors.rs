use std::ops;

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z}
    }
}

impl Vec3 {

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x -self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        Vec3 {x, y, z}
    }

    pub fn norm(self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let l = self.norm();
        let x = self.x / l;
        let y = self.y / l;
        let z = self.z / l;
        Vec3 {x, y, z}
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

#[derive(Copy, Clone)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4 { x, y, z, w}
    }

    pub fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Vec4 {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;
        Vec4 {x, y, z, w}
    }
}

#[derive(Copy, Clone)]
pub struct Mat4 {
    pub m00: f64,
    pub m01: f64,
    pub m02: f64,
    pub m03: f64,
    pub m10: f64,
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
    pub m20: f64,
    pub m21: f64,
    pub m22: f64,
    pub m23: f64,
    pub m30: f64,
    pub m31: f64,
    pub m32: f64,
    pub m33: f64,
}

impl Mat4 {
    pub fn new(m00: f64, m01: f64, m02: f64, m03: f64, m10: f64, m11: f64, m12: f64, m13: f64,
               m20: f64, m21: f64, m22: f64, m23: f64, m30: f64, m31: f64, m32: f64, m33: f64) -> Mat4 {
        Mat4 {m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33}
    }
}


impl ops::Mul<Vec4> for Mat4 {

    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        let x = self.m00 * rhs.x + self.m01 * rhs.y + self.m02 * rhs.z + self.m03 * rhs.w;
        let y = self.m10 * rhs.x + self.m11 * rhs.y + self.m12 * rhs.z + self.m13 * rhs.w;
        let z = self.m20 * rhs.x + self.m21 * rhs.y + self.m22 * rhs.z + self.m23 * rhs.w;
        let w = self.m30 * rhs.x + self.m31 * rhs.y + self.m32 * rhs.z + self.m33 * rhs.w;
        Vec4 {x, y, z, w}
    }
}