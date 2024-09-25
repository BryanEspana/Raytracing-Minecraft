use crate::vector3::Vector3;
use crate::texture::Texture;

pub struct Material {
    pub albedo: Vector3,
    pub specular: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub ref_idx: f32,
    pub texture: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub emissive_color: Option<Vector3>,
}

impl Material {
    pub fn new(
        albedo: Vector3,
        specular: f32,
        transparency: f32,
        reflectivity: f32,
        ref_idx: f32,
    ) -> Self {
        Material {
            albedo,
            specular,
            transparency,
            reflectivity,
            ref_idx,
            texture: None,
            normal_map: None,
            emissive_color: None,
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Vector3 {
        if let Some(texture) = &self.texture {
            texture.get_color(u, v)
        } else {
            self.albedo
        }
    }

    pub fn get_normal(&self, u: f32, v: f32, normal: Vector3) -> Vector3 {
        if let Some(normal_map) = &self.normal_map {
            let normal_color = normal_map.get_color(u, v);
            let perturbed_normal = Vector3::new(
                normal_color.x * 2.0 - 1.0,
                normal_color.y * 2.0 - 1.0,
                normal_color.z * 2.0 - 1.0,
            );
            (normal + perturbed_normal).normalize()
        } else {
            normal
        }
    }
}
