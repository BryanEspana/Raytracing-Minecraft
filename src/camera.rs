use crate::ray::Ray;
use crate::vector3::Vector3;

pub struct Camera {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(
        position: Vector3,
        target: Vector3,
        up: Vector3,
        fov: f32,
        aspect_ratio: f32,
    ) -> Self {
        Camera {
            position,
            target,
            up,
            fov,
            aspect_ratio,
            zoom: 1.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let theta = (self.fov * std::f32::consts::PI / 180.0) / self.zoom;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        let w = (self.position - self.target).normalize();
        let u_cam = Vector3::cross(self.up, w).normalize();
        let v_cam = Vector3::cross(w, u_cam);

        let horizontal = u_cam * viewport_width;
        let vertical = v_cam * viewport_height;
        let lower_left_corner =
            self.position - horizontal / 2.0 - vertical / 2.0 - w;

        let direction =
            lower_left_corner + horizontal * u + vertical * v - self.position;

        Ray::new(self.position, direction)
    }

    pub fn zoom(&mut self, amount: f32) {
        self.zoom *= amount;
    }
}
