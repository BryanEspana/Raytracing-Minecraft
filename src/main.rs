mod player;
mod block;

use ggez::{Context, ContextBuilder, GameResult, GameError};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use nalgebra as na;

use crate::player::Player;
use crate::block::Block;

struct MiniMinecraft {
    blocks: Vec<Block>,
    player: Player,
}

impl MiniMinecraft {
    pub fn new(_ctx: &mut Context) -> MiniMinecraft {
        let mut blocks = Vec::new();
        for x in 0..30 {
            for y in 0..30 {
                blocks.push(Block {
                    position: na::Vector3::new(x as f32, y as f32, 0.0),
                    texture: "grass.png".to_string(),
                });
            }
        }

        MiniMinecraft {
            blocks,
            player: Player::new(),
        }
    }
}


impl EventHandler<ggez::GameError> for MiniMinecraft {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.player.update();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(135, 206, 235)); 

        for block in &self.blocks {
            block.draw(ctx, &self.player.position, self.player.direction)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.player.handle_input(keycode);
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("mini_minecraft", "author")
        .add_resource_path("./assets/textures"); 
    
    let (mut ctx, event_loop) = cb.build().expect("Could not create ggez context!");

    let state = MiniMinecraft::new(&mut ctx);
    event::run(ctx, event_loop, state)
}
