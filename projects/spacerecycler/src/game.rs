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
}

impl SpaceRecyclerGame {
    pub fn new(_ctx: &mut Context) -> SpaceRecyclerGame {
        // Load/create resources such as images here.
        SpaceRecyclerGame {
            // ...
        }
    }
}

impl EventHandler for SpaceRecyclerGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

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

        // Draw code here...
        graphics::present(ctx)
    }
}
