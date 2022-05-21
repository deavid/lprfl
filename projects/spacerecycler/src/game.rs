use std::time::Instant;

use crate::player;
use crate::vector::Position;
use crate::vector::Vector;
use crate::HEIGHT;
use crate::MARGIN_W;
use crate::WIDTH;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::GameResult;

pub struct SpaceRecyclerGame {
    // Your state here...
    // lives: i64,
    pub player_ship: player::Ship,
    pub last_update: Instant,
}

impl SpaceRecyclerGame {
    pub fn new(_ctx: &mut Context) -> SpaceRecyclerGame {
        // Load/create resources such as images here.
        SpaceRecyclerGame {
            player_ship: player::Ship::default(),
            last_update: Instant::now(),
            // ...
        }
    }
}

impl EventHandler for SpaceRecyclerGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        let delta = self.last_update.elapsed();
        self.last_update = Instant::now();

        // Maybe increase quality of simulation by doing smaller delta steps.
        self.player_ship.update(ctx, delta)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        // Draw code here...

        let rect = graphics::Rect::new(
            MARGIN_W,
            MARGIN_W,
            WIDTH - MARGIN_W * 2.0,
            HEIGHT - MARGIN_W * 2.0,
        );
        let r1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(3.0),
            rect,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        self.player_ship.draw(ctx)?;
        // Now display:
        graphics::present(ctx)
    }
}
