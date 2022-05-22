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

#[derive(Debug, Clone)]
pub struct Ship {
    pub pos: Position,
    pub speed: Vector,
    pub lifes: u64,
    pub old_lifes: u64,
    pub last_life: Instant,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            pos: Position {
                x: 100.0,
                y: HEIGHT / 2.0,
            },
            speed: Default::default(),
            lifes: 8,
            old_lifes: 8,
            last_life: Instant::now(),
        }
    }
}

impl Ship {
    pub const SIZE_X: f32 = 32.0;
    pub const SIZE_Y: f32 = 14.0;
    const LIFE_SIZE: f32 = 8.0;
    const MAX_LIFES: u64 = 10;
    const IMMUNITY_SECS: f32 = 5.0;
    const BLINK_RATE_SECS: f32 = 0.2;
    const X_SPEED: f32 = 1000.0;
    const Y_SPEED: f32 = 1000.0;
    const FRICTION: f32 = 1000.0;
    const FRICTION_TURBO: f32 = 2.0;
    const LIFE_COLOR: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    const USED_LIFE_COLOR: Color = Color {
        r: 0.25,
        g: 0.00,
        b: 0.25,
        a: 0.5,
    };

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
    pub fn consume_life(&mut self) {
        if self.last_life.elapsed().as_secs_f32() < Self::IMMUNITY_SECS {
            return;
        }
        self.last_life = Instant::now();
        self.old_lifes = self.lifes;
        if self.lifes >= 1 {
            self.lifes -= 1;
        } else {
            println!("WARN: Player was already dead. Attempted to remove even more lifes.");
        }
    }
    pub fn blinking(&self, rev: bool) -> bool {
        if self.last_life.elapsed().as_secs_f32() < Self::IMMUNITY_SECS {
            let b = (self.last_life.elapsed().as_secs_f32() / Self::BLINK_RATE_SECS).round() as i64;
            if rev {
                return b % 2 == 1;
            }
            return b % 2 == 0;
        }
        false
    }
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let color = match self.blinking(false) {
            true => Color::CYAN,
            false => Color::BLUE,
        };
        let rect = graphics::Rect::new(
            self.pos.x - Self::SIZE_X / 2.0,
            self.pos.y - Self::SIZE_Y / 2.0,
            Self::SIZE_X,
            Self::SIZE_Y,
        );
        let r1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(5.0),
            rect,
            Color::BLACK,
        )?;
        graphics::draw(ctx, &r1, DrawParam::default())?;
        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        // Drawing amount of lifes left
        for n in 0..Self::MAX_LIFES {
            let nf = n as f32;
            let tolerance = 5.0 / Self::LIFE_SIZE; // Max error for precise circles.
            let p = ggez::mint::Point2 {
                x: MARGIN_W * 2.0 + Self::LIFE_SIZE / 2.0 + Self::LIFE_SIZE * nf * 2.0,
                y: MARGIN_W * 3.0 + Self::LIFE_SIZE / 2.0 + 2.0,
            };
            let lifes = match self.blinking(true) {
                true => self.old_lifes,
                false => self.lifes,
            };
            let color = match (n + 1) <= lifes {
                true => Self::LIFE_COLOR,
                false => Self::USED_LIFE_COLOR,
            };
            let r1 = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                p,
                Self::LIFE_SIZE,
                tolerance,
                color,
            )?;
            graphics::draw(ctx, &r1, DrawParam::default())?;
            let r1 = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::stroke(2.0),
                p,
                Self::LIFE_SIZE,
                tolerance,
                Color::BLACK,
            )?;
            graphics::draw(ctx, &r1, DrawParam::default())?;
        }
        Ok(())
    }
}
