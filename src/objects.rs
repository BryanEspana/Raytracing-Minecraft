use crate::ray::Ray;
use crate::vector3::Vector3;
use crate::materials::Material;

pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f32,
    pub material: Material,
    pub u: f32,
    pub v: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

// Implementación de un cubo
pub struct Cube {
    pub min: Vector3,
    pub max: Vector3,
    pub material: Material,
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return None;
            }
        }

        let point = ray.at(t_min);
        let normal = self.get_normal(point);
        let u = 0.0; // Calcula las coordenadas de textura si es necesario
        let v = 0.0;

        Some(HitRecord {
            point,
            normal,
            t: t_min,
            material: self.material.clone(),
            u,
            v,
        })
    }
}

impl Cube {
    pub fn new(min: Vector3, max: Vector3, material: Material) -> Self {
        Cube { min, max, material }
    }

    pub fn get_normal(&self, point: Vector3) -> Vector3 {
        let epsilon = 1e-4;
        if (point.x - self.min.x).abs() < epsilon {
            Vector3::new(-1.0, 0.0, 0.0)
        } else if (point.x - self.max.x).abs() < epsilon {
            Vector3::new(1.0, 0.0, 0.0)
        } else if (point.y - self.min.y).abs() < epsilon {
            Vector3::new(0.0, -1.0, 0.0)
        } else if (point.y - self.max.y).abs() < epsilon {
            Vector3::new(0.0, 1.0, 0.0)
        } else if (point.z - self.min.z).abs() < epsilon {
            Vector3::new(0.0, 0.0, -1.0)
        } else {
            Vector3::new(0.0, 0.0, 1.0)
        }
    }
}
