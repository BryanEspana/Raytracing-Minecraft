mod vector3;
mod ray;
mod camera;
mod materials;
mod objects;
mod lights;
mod texture;
mod scene;

use camera::Camera;
use scene::Scene;
use vector3::Vector3;
use image::{ImageBuffer, RgbImage};

fn main() {
    let width = 800;
    let height = 600;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let mut camera = Camera::new(
        Vector3::new(0.0, 1.0, -5.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
        width as f32 / height as f32,
    );

    let mut scene = Scene::new();
    scene.setup_scene();

    // Ciclo de renderizado
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f32 / (width as f32 - 1.0);
        let v = y as f32 / (height as f32 - 1.0);

        let ray = camera.get_ray(u, v);
        let color = scene.trace_ray(&ray, 0);

        *pixel = image::Rgb([
            (color.x.min(1.0).max(0.0) * 255.0) as u8,
            (color.y.min(1.0).max(0.0) * 255.0) as u8,
            (color.z.min(1.0).max(0.0) * 255.0) as u8,
        ]);
    }

    img.save("render.png").unwrap();
    println!("Imagen renderizada guardada como render.png");
}
