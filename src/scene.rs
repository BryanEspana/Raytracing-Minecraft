use crate::objects::{Cube, Hittable, HitRecord};
use crate::ray::Ray;
use crate::vector3::Vector3;
use crate::materials::Material;
use crate::lights::Light;
use crate::texture::Texture;
use rand::Rng;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Light>,
    pub skybox: Option<Texture>,
    pub time: f32,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            skybox: None,
            time: 0.0,
        }
    }

    pub fn setup_scene(&mut self) {
        // Agrega objetos a la escena
        let ground_material = Material::new(
            Vector3::new(0.5, 0.5, 0.5),
            0.1,
            0.0,
            0.0,
            1.0,
        );

        let cube_material = Material::new(
            Vector3::new(0.2, 0.7, 0.3),
            0.5,
            0.0,
            0.5,
            1.0,
        );

        // Agrega texturas si es necesario
        // ground_material.texture = Some(Texture::from_file("ground.png"));
        // cube_material.texture = Some(Texture::from_file("cube.png"));

        let ground = Cube::new(
            Vector3::new(-5.0, -0.1, -5.0),
            Vector3::new(5.0, 0.0, 5.0),
            ground_material,
        );

        let cube = Cube::new(
            Vector3::new(-1.0, 0.0, -1.0),
            Vector3::new(1.0, 2.0, 1.0),
            cube_material,
        );

        self.objects.push(Box::new(ground));
        self.objects.push(Box::new(cube));

        // Agrega luces
        let light1 = Light::new(
            Vector3::new(5.0, 10.0, -5.0),
            Vector3::new(1.0, 1.0, 1.0),
            1.0,
        );

        let light2 = Light::new(
            Vector3::new(-5.0, 10.0, 5.0),
            Vector3::new(1.0, 0.0, 0.0),
            0.5,
        );

        self.lights.push(light1);
        self.lights.push(light2);

        // Agrega skybox si es necesario
        // self.skybox = Some(Texture::from_file("skybox.png"));
    }

    pub fn trace_ray(&self, ray: &Ray, depth: u32) -> Vector3 {
        if depth > 5 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = self.hit_objects(ray, 0.001, f32::MAX) {
            let emitted = if let Some(emissive) = hit.material.emissive_color {
                emissive
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            };

            let mut rng = rand::thread_rng();

            // Efecto Fresnel
            let cosine = -Vector3::dot(ray.direction, hit.normal).min(1.0).max(-1.0);
            let fresnel_effect = self.fresnel(cosine, hit.material.ref_idx);

            let reflect_dir = Vector3::reflect(ray.direction, hit.normal).normalize();
            let reflect_ray = Ray::new(hit.point, reflect_dir);
            let reflect_color = self.trace_ray(&reflect_ray, depth + 1);

            let refract_color = if hit.material.transparency > 0.0 {
                if let Some(refract_dir) = Vector3::refract(
                    ray.direction,
                    hit.normal,
                    hit.material.ref_idx,
                ) {
                    let refract_ray = Ray::new(hit.point, refract_dir.normalize());
                    self.trace_ray(&refract_ray, depth + 1)
                } else {
                    Vector3::new(0.0, 0.0, 0.0)
                }
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            };

            let mut color = emitted;

            // Iluminación directa
            for light in &self.lights {
                let light_dir = (light.position - hit.point).normalize();
                let light_distance = (light.position - hit.point).dot(light_dir);

                // Sombreado de sombras
                let shadow_ray = Ray::new(hit.point + hit.normal * 0.001, light_dir);
                if self.hit_objects(&shadow_ray, 0.001, light_distance).is_none() {
                    let diffuse = hit.material.get_color(hit.u, hit.v)
                        * light.color
                        * light.intensity
                        * Vector3::dot(hit.normal, light_dir).max(0.0);

                    color = color + diffuse;
                }
            }

            // Reflexión y refracción
            color = color
                + reflect_color * hit.material.reflectivity * fresnel_effect
                + refract_color * hit.material.transparency * (1.0 - fresnel_effect);

            color
        } else {
            // Color de fondo (skybox)
            if let Some(skybox) = &self.skybox {
                let direction = ray.direction;
                let u = 0.5 + (direction.z.atan2(direction.x)) / (2.0 * std::f32::consts::PI);
                let v = 0.5 - direction.y.asin() / std::f32::consts::PI;
                skybox.get_color(u, v)
            } else {
                Vector3::new(0.5, 0.7, 1.0) // Color de fondo predeterminado
            }
        }
    }

    pub fn hit_objects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut hit_record: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(temp_hit) = object.hit(ray, t_min, closest) {
                closest = temp_hit.t;
                hit_record = Some(temp_hit);
            }
        }

        hit_record
    }

    pub fn fresnel(&self, cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
