#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn dot(a: Vector3, b: Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
        Vector3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn normalize(self) -> Vector3 {
        let mag = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector3::new(self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
        v - n * 2.0 * Vector3::dot(v, n)
    }

    pub fn refract(v: Vector3, n: Vector3, eta: f32) -> Option<Vector3> {
        let cosi = -Vector3::dot(v, n).max(-1.0).min(1.0);
        let etai = 1.0;
        let etat = eta;
        let mut n = n;
        let mut eta = etai / etat;
        if cosi < 0.0 {
            cosi = -cosi;
        } else {
            n = -n;
            eta = etat / etai;
        }
        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        if k < 0.0 {
            None
        } else {
            Some(v * eta + n * (eta * cosi - k.sqrt()))
        }
    }
}

// Implementa operadores aritméticos para Vector3
use std::ops;

impl ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, scalar: f32) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, scalar: f32) -> Vector3 {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}
