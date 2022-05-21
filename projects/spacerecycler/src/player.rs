use std::time::{Duration, Instant};

use crate::vector::{Position, Vector};
use crate::HEIGHT;
use crate::MARGIN_W;
use crate::WIDTH;
use ggez::event::KeyCode;
use ggez::event::KeyMods;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::input::keyboard;
use ggez::Context;
use ggez::GameResult;
pub struct Ship {
    pub pos: Position,
    pub speed: Vector,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            pos: Position {
                x: 100.0,
                y: HEIGHT / 2.0,
            },
            speed: Default::default(),
        }
    }
}

impl Ship {
    const SIZE_X: f32 = 32.0;
    const SIZE_Y: f32 = 14.0;
    const X_SPEED: f32 = 1000.0;
    const Y_SPEED: f32 = 1000.0;
    const FRICTION: f32 = 1000.0;
    const FRICTION_TURBO: f32 = 2.0;

    pub fn update(&mut self, ctx: &mut Context, delta: Duration) -> GameResult<()> {
        let delta = delta.as_secs_f32();
        // Increase or decrease `position_x` by 0.5, or by 5.0 if Shift is held.
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.speed.dx += Self::X_SPEED * delta;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.speed.dx -= Self::X_SPEED * delta;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.speed.dy -= Self::Y_SPEED * delta;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.speed.dy += Self::Y_SPEED * delta;
        }
        let friction = match keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
            true => Self::FRICTION_TURBO,
            false => Self::FRICTION,
        };
        self.speed.dx /= friction.powf(delta);
        self.speed.dy /= friction.powf(delta);

        self.pos.x += self.speed.dx * delta;
        self.pos.y += self.speed.dy * delta;

        if self.pos.x > WIDTH - MARGIN_W {
            self.pos.x = WIDTH - MARGIN_W;
            if self.speed.dx > 0.0 {
                self.speed.dx = 0.0;
            }
        }
        if self.pos.x < MARGIN_W {
            self.pos.x = MARGIN_W;
            if self.speed.dx < 0.0 {
                self.speed.dx = 0.0;
            }
        }
        if self.pos.y > HEIGHT - MARGIN_W {
            self.pos.y = HEIGHT - MARGIN_W;
            if self.speed.dy > 0.0 {
                self.speed.dy = 0.0;
            }
        }
        if self.pos.y < MARGIN_W {
            self.pos.y = MARGIN_W;
            if self.speed.dy < 0.0 {
                self.speed.dy = 0.0;
            }
        }
        Ok(())
    }
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(
            self.pos.x - Self::SIZE_X / 2.0,
            self.pos.y - Self::SIZE_Y / 2.0,
            Self::SIZE_X,
            Self::SIZE_Y,
        );
        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::BLUE)?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        Ok(())
    }
}
