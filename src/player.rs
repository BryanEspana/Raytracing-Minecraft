
use ggez::{Context, GameResult};
use ggez::input::keyboard::KeyCode;
use ggez::graphics::{self, Color, Mesh};
use nalgebra as na; 


pub struct Player {
    pub position: na::Vector3<f32>, 
    velocity: na::Vector3<f32>,
    speed: f32, 
    pub direction: f32, 
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: na::Vector3::new(5.0, 5.0, 1.0),
            velocity: na::Vector3::new(0.0, 0.0, 0.0),
            speed: 0.1,
            direction: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        
        self.velocity *= 0.9;
    }

    pub fn handle_input(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::W => {
                self.velocity.x += self.direction.cos() * self.speed;
                self.velocity.y += self.direction.sin() * self.speed;
            }
            KeyCode::S => {
                self.velocity.x -= self.direction.cos() * self.speed;
                self.velocity.y -= self.direction.sin() * self.speed;
            }
            KeyCode::A => {
                self.direction -= 0.1;
            }
            KeyCode::D => {
                self.direction += 0.1;
            }
            _ => {}
        }
    }
}
