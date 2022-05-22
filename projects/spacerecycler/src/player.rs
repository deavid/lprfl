use std::time::{Duration, Instant};

use crate::asteroid::{AsteroidField, AsteroidKind};
use crate::sfx::Sfx;
use crate::vector::{Position, Vector};
use crate::HEIGHT;
use crate::MARGIN_W;
use crate::WIDTH;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::input::keyboard;
use ggez::Context;
use ggez::GameResult;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShipAction {
    None,
    GunFire,
    Collector(AsteroidKind),
    Turbo,
    Overheat,
}

impl std::fmt::Display for ShipAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipAction::None => write!(f, "CRUISE"),
            ShipAction::GunFire => write!(f, "FIRING MAIN CANNON"),
            ShipAction::Collector(k) => write!(f, "COLLECTING {}", k),
            ShipAction::Turbo => write!(f, "TURBO THRUST"),
            ShipAction::Overheat => write!(f, "SYSTEM OVERLOAD"),
        }
    }
}

impl Default for ShipAction {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone)]
pub struct Ship {
    pub pos: Position,
    pub speed: Vector,
    pub lifes: u64,
    pub old_lifes: u64,
    pub last_life: Instant,
    pub action: ShipAction,
    pub collector: Collector,
    pub play_life: Option<()>,
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
            action: Default::default(),
            collector: Default::default(),
            play_life: None,
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
    const X_SPEED: f32 = 1500.0;
    const Y_SPEED: f32 = 1000.0;
    const FRICTION: f32 = 1000.0;
    const FRICTION_TURBO: f32 = 200.0;
    const SPEED_FACTOR_TURBO: f32 = 2.0;
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
        self.collector.update(ctx, delta, self.action)?;

        let delta = delta.as_secs_f32();

        let friction = match self.action {
            ShipAction::Turbo => Self::FRICTION_TURBO,
            _ => Self::FRICTION,
        };
        let turbo = match self.action {
            ShipAction::Turbo => Self::SPEED_FACTOR_TURBO,
            _ => 1.0,
        };
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.speed.dx += Self::X_SPEED * delta * turbo;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.speed.dx -= Self::X_SPEED * delta * turbo;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.speed.dy -= Self::Y_SPEED * delta * turbo;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.speed.dy += Self::Y_SPEED * delta * turbo;
        }

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
        self.play_life = Some(());
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
            false => match &self.action {
                ShipAction::Turbo => Color::RED,
                ShipAction::GunFire => Color::YELLOW,
                ShipAction::Collector(k) => k.color(),
                _ => Color::BLUE,
            },
        };

        self.collector.draw(ctx, self.pos)?;

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
    pub fn play(&mut self, ctx: &mut Context, sfx: &mut Sfx) -> GameResult<()> {
        if self.play_life.take().is_some() {
            sfx.life_lost(ctx)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Collector {
    pub mode: Option<AsteroidKind>,
    pub radius: f32,
}
impl Default for Collector {
    fn default() -> Self {
        Self {
            mode: None,
            radius: 0.0,
        }
    }
}

impl Collector {
    pub const COLLECTOR_SIZE: f32 = 64.0;
    const COLLECTOR_SPEED: f32 = 80.0;
    pub fn update(
        &mut self,
        _ctx: &mut Context,
        delta: Duration,
        action: ShipAction,
    ) -> GameResult<()> {
        let delta = delta.as_secs_f32();

        let mut collector_on = false;
        if let ShipAction::Collector(k) = action {
            if self.radius < Ship::SIZE_Y {
                self.mode = Some(k);
            }
            if let Some(mode) = self.mode {
                if mode == k {
                    collector_on = true;
                }
            }
        }
        if collector_on {
            if self.radius < Self::COLLECTOR_SIZE {
                self.radius += Self::COLLECTOR_SPEED * delta;
            }
        } else if self.radius > Ship::SIZE_Y / 2.0 {
            self.radius -= Self::COLLECTOR_SPEED * delta;
            if self.radius < Ship::SIZE_Y {
                self.mode = None;
            }
        }

        Ok(())
    }
    pub fn draw(&mut self, ctx: &mut Context, pos: Position) -> GameResult<()> {
        if let Some(k) = self.mode {
            let tolerance = 10.0 / self.radius; // Max error for precise circles.
            let p = ggez::mint::Point2 { x: pos.x, y: pos.y };
            let mut rng = rand::thread_rng();

            let mut color = k.color();
            color.a /= 5.0;
            let r1 = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                p,
                self.radius * rng.gen_range(0.98..1.0),
                tolerance,
                color,
            )?;
            graphics::draw(ctx, &r1, DrawParam::default())?;
            let color = Color {
                r: 0.00,
                g: 0.00,
                b: 0.00,
                a: 0.20,
            };
            let r1 = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                p,
                self.radius * 0.8 * rng.gen_range(0.95..1.0),
                tolerance / 0.8,
                color,
            )?;
            graphics::draw(ctx, &r1, DrawParam::default())?;
            let r1 = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                p,
                self.radius * 0.6 * rng.gen_range(0.90..1.0),
                tolerance / 0.6,
                color,
            )?;
            graphics::draw(ctx, &r1, DrawParam::default())?;
        }
        Ok(())
    }
    pub fn check_asteroids(
        &mut self,
        pos: Position,
        asteroids: &mut AsteroidField,
        delta: Duration,
    ) -> Option<f32> {
        let delta = delta.as_secs_f32();
        let mode = self.mode?;
        let mut score = 0.0;
        let mut collected = false;
        let collisions: Vec<_> = asteroids.check_collision_many(pos, self.radius);
        for (col_vec, n) in collisions {
            let force = (col_vec.distance() - self.radius / 2.0).max(0.01);
            let asteroid = asteroids.asteroids.get_mut(n).unwrap();
            let p = 1.0 - (asteroid.life.min(force / 2.0) / asteroid.life);
            let col_vec = col_vec.unit();
            asteroid.speed.dx -= col_vec.dx * delta * 170.0;
            asteroid.speed.dy -= col_vec.dy * delta * 170.0;
            asteroid.speed.dx /= 1.5_f32.powf(delta);
            asteroid.speed.dy /= 1.5_f32.powf(delta);
            if asteroid.kind == AsteroidKind::Rock {
                continue;
            } else {
                collected = true;
                if force > 1.0 {
                    asteroid.size *= p.sqrt().sqrt();
                    let impact = asteroid.life.min(force);
                    asteroid.life -= impact;
                    if asteroid.kind == mode {
                        score += impact * asteroid.kind.money_factor();
                    } else {
                        score += -impact * asteroid.kind.money_factor();
                    }
                }
            }
        }
        if collected {
            Some(score)
        } else {
            None
        }
    }
}
