use crate::vector3::Vector3;
use image::{DynamicImage, GenericImageView};

pub struct Texture {
    pub image: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let image = image::open(path).expect("No se pudo abrir la textura");
        let (width, height) = image.dimensions();
        Texture {
            image,
            width,
            height,
        }
    }

    pub fn get_color(&self, u: f32, v: f32, time: f32) -> Vector3 {
        let animated_u = (u + time * 0.1) % 1.0;
        let i = ((animated_u * self.width as f32) % self.width as f32) as u32;
        let j = (((1.0 - v) * self.height as f32) % self.height as f32) as u32;
    
        let pixel = self.image.get_pixel(i, j);
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;
    
        Vector3::new(r, g, b)
    }
    
}
