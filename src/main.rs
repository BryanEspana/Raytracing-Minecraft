use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

#[derive(Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn normalize(&self) -> Vec3 {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3 { x: self.x / len, y: self.y / len, z: self.z / len }
    }
}

struct Camera {
    position: Vec3,
    fov: f32,
}

impl Camera {
    fn new(position: Vec3, fov: f32) -> Self {
        Camera { position, fov }
    }

    fn move_forward(&mut self, distance: f32) {
        self.position.z -= distance;
    }

    fn move_backward(&mut self, distance: f32) {
        self.position.z += distance;
    }

    fn get_ray_direction(&self, x: f32, y: f32, width: f32, height: f32) -> Vec3 {
        let aspect_ratio = width / height;
        let fov_adjustment = (self.fov / 2.0).to_radians().tan();

        let pixel_x = ((x + 0.5) / width * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let pixel_y = (1.0 - (y + 0.5) / height * 2.0) * fov_adjustment;

        Vec3::new(pixel_x, pixel_y, -1.0).normalize()
    }
}

struct Plane {
    position: Vec3,
    normal: Vec3,
    color: Vec3,
}

impl Plane {
    fn new(position: Vec3, normal: Vec3, color: Vec3) -> Self {
        Plane { position, normal, color }
    }

    fn intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<f32> {
        let denom = self.normal.dot(ray_direction);
        if denom.abs() > 1e-6 {
            let p0l0 = Vec3::new(self.position.x - ray_origin.x, self.position.y - ray_origin.y, self.position.z - ray_origin.z);
            let t = p0l0.dot(&self.normal) / denom;
            if t >= 0.0 {
                return Some(t);
            }
        }
        None
    }
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(800.0, 600.0);
        WindowBuilder::new()
            .with_title("Raytracing Minecraft Diorama")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 1.0), 90.0);

    // Setup the Pixels surface
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(800, 600, surface_texture)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    match keycode {
                        VirtualKeyCode::W => {
                            camera.move_forward(0.1);
                        }
                        VirtualKeyCode::S => {
                            camera.move_backward(0.1);
                        }
                        _ => {}
                    }
                }
            }

            Event::MainEventsCleared => {
                // Redibujar la escena
                render_scene(&mut pixels, &camera);
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                // Actualiza la ventana con el contenido de la imagen
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => (),
        }
    });
}

fn render_scene(pixels: &mut Pixels, camera: &Camera) {
    let frame = pixels.get_frame();

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % 800) as f32;
        let y = (i / 800) as f32;

        let ray_dir = camera.get_ray_direction(x, y, 800.0, 600.0);

        let color = if ray_dir.z > 0.0 {
            Vec3::new(0.7, 0.7, 0.7) // Fondo gris
        } else {
            Vec3::new(0.0, 0.0, 0.0) // Fondo negro
        };

        let rgba = [
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
            255,
        ];
        pixel.copy_from_slice(&rgba);
    }
}
