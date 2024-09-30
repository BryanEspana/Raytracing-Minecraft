use ggez::{Context, GameResult};
use ggez::graphics::{self, Image}; 
use nalgebra as na;
use ggez::mint; 

pub struct Block {
    pub position: na::Vector3<f32>,
    pub texture: String,
}

impl Block {
    pub fn draw(&self, ctx: &mut Context, player_position: &na::Vector3<f32>, player_direction: f32) -> GameResult<()> {
        let texture = Image::new(ctx, "/grass.png")?;
        
        let relative_x = self.position.x - player_position.x;
        let relative_y = self.position.y - player_position.y;

        let rotated_x = relative_x * player_direction.cos() - relative_y * player_direction.sin();
        let rotated_y = relative_x * player_direction.sin() + relative_y * player_direction.cos();

        let iso_x = (rotated_x - rotated_y) * (32.0 / 2.0);
        let iso_y = (rotated_x + rotated_y) * (32.0 / 4.0);

        let offset_x = 400.0;
        let offset_y = 300.0;

        let params = graphics::DrawParam::new()
            .dest(mint::Point2 { x: iso_x + offset_x, y: iso_y + offset_y })
            .scale([2.0, 2.0]); 
        
        graphics::draw(ctx, &texture, params)
    }
}

