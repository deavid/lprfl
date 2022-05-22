use std::time::{Duration, Instant};

use crate::asteroid::AsteroidField;
use crate::player::Ship;
use crate::vector::{Position, Vector};
use crate::HEIGHT;
use crate::MARGIN_W;
use crate::WIDTH;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::Context;
use ggez::GameResult;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Bullet {
    pub pos: Position,
    pub speed: Vector,
    pub life: f32,
    pub bounced: bool,
}

impl Bullet {
    pub const SIZE: f32 = 5.0;
    pub const LIFE: f32 = 500.0;
    const X_SPEED: f32 = 1800.0;
    const Y_SPEED: f32 = 150.0;
    const TRAIL_DIST_SECS: f32 = 1.0 / 20.0;
    const LIFE_CONSUMPTION_RATE: f32 = 300.0;
    const BULLET_COLOR: Color = Color {
        r: 1.00,
        g: 0.65,
        b: 0.00,
        a: 1.00,
    };
    const TRAIL_COLOR: Color = Color {
        r: 1.00,
        g: 0.85,
        b: 0.00,
        a: 0.05,
    };
    const BOUNCED_COLOR: Color = Color {
        r: 1.00,
        g: 0.20,
        b: 0.00,
        a: 1.00,
    };
    const BOUNCED_TRAIL_COLOR: Color = Color {
        r: 1.00,
        g: 0.30,
        b: 0.00,
        a: 0.08,
    };
    pub fn new(player: &Ship, spread_factor: f32) -> Self {
        let mut rng = rand::thread_rng();
        let pos = Position {
            x: player.pos.x + Ship::SIZE_X / 2.0 + rng.gen_range(0.0..Self::X_SPEED / 30.0),
            y: player.pos.y + rng.gen_range(-Ship::SIZE_Y / 2.0..Ship::SIZE_Y / 2.0),
        };
        let spread = spread_factor.powi(3) * Self::Y_SPEED;
        let speed = Vector {
            dx: Self::X_SPEED * (1.0 - spread_factor / 3.0) + player.speed.dx,
            dy: rng.gen_range(-spread..spread) + player.speed.dy,
        };
        Self {
            pos,
            speed,
            life: Self::LIFE * (1.0 - spread_factor / 2.0),
            bounced: false,
        }
    }
    pub fn update(&mut self, _ctx: &mut Context, delta: Duration) -> GameResult<()> {
        let delta = delta.as_secs_f32();

        self.life -= delta * Self::LIFE_CONSUMPTION_RATE;
        self.pos.x += self.speed.dx * delta;
        self.pos.y += self.speed.dy * delta;

        Ok(())
    }
    pub fn pending_destroy(&self) -> bool {
        if self.life <= 0.0 {
            return true;
        }
        if self.pos.x < -Self::SIZE {
            return true;
        }
        if self.pos.x > WIDTH + Self::SIZE {
            return true;
        }
        if self.pos.y > HEIGHT + Self::SIZE {
            return true;
        }
        if self.pos.y < -Self::SIZE {
            return true;
        }
        false
    }
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let color = match self.bounced {
            true => Self::BOUNCED_COLOR,
            false => Self::BULLET_COLOR,
        };
        let trail_color = match self.bounced {
            true => Self::BOUNCED_TRAIL_COLOR,
            false => Self::TRAIL_COLOR,
        };
        let p = ggez::mint::Point2 {
            x: self.pos.x,
            y: self.pos.y,
        };
        let trail_dist = Self::TRAIL_DIST_SECS * (self.life / Self::LIFE).sqrt().sqrt();
        let trail_len = self.speed.distance() * trail_dist;
        let trail_points = (trail_len / 5.0).round() as usize;
        for f in 1..=trail_points {
            let f = f as f32 / trail_points as f32;
            let last_pos = ggez::mint::Point2 {
                x: self.pos.x - self.speed.dx * trail_dist * f,
                y: self.pos.y - self.speed.dy * trail_dist * f,
            };
            let l = graphics::Mesh::new_line(
                ctx,
                &[last_pos, p],
                Self::SIZE * (1.0 - f / 2.0),
                trail_color,
            )?;
            graphics::draw(ctx, &l, DrawParam::default())?;
        }

        let size = Self::SIZE * (self.life / Self::LIFE).sqrt().sqrt().sqrt();

        let tolerance = 10.0 / size; // Max error for precise circles.

        let r1 =
            graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), p, size, tolerance, color)?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        Ok(())
    }
    pub fn check_collision(&self, pos: Position, size: f32) -> Option<Vector> {
        let v = self.pos.vector_to(&pos);
        let d2 = v.distance_2();
        if d2 >= (Self::SIZE + size).powi(2) {
            return None;
        }
        let d = d2.sqrt();
        let col_d = (Self::SIZE + size) - d;
        let col_v = v.scale(col_d / d);
        Some(col_v)
    }
}

#[derive(Debug, Clone)]
pub struct MachineGun {
    pub bullets: Vec<Bullet>,
    pub last_shot: Instant,
    pub temp: f32,
}

impl Default for MachineGun {
    fn default() -> Self {
        Self {
            bullets: Default::default(),
            last_shot: Instant::now(),
            temp: 60.0,
        }
    }
}

impl MachineGun {
    const SHOT_DURATION: Duration = Duration::from_millis(50);
    const MAX_TEMP: f32 = 200.0;
    const TEMP_SHOT: f32 = 1.4;
    const COOLING_RATE: f32 = 1.1;
    const BLINK_RATE_SECS: f32 = 0.2;

    pub fn shoot(&mut self, player: &Ship) {
        if self.last_shot > Instant::now() {
            // Rust might panic if we do self.last_shot.elapsed() and returns negative.
            return;
        }
        if self.last_shot.elapsed() < Self::SHOT_DURATION {
            // Gun loading next bullet.
            return;
        }
        if self.temp + Self::TEMP_SHOT >= Self::MAX_TEMP {
            // Gun too hot. Try later.
            self.last_shot = Instant::now() + Duration::from_secs(3);
            return;
        }
        self.last_shot = Instant::now();
        self.bullets
            .push(Bullet::new(player, self.temp / Self::MAX_TEMP));
        self.temp += Self::TEMP_SHOT;
    }

    pub fn blinking(&self, rev: bool) -> bool {
        let now = Instant::now();
        if self.last_shot > now {
            let remaining = self.last_shot - now;
            let b = (remaining.as_secs_f32() / Self::BLINK_RATE_SECS).round() as i64;
            if rev {
                return b % 2 == 1;
            }
            return b % 2 == 0;
        }
        false
    }
    pub fn update(&mut self, ctx: &mut Context, delta: Duration) -> GameResult<()> {
        let mut to_remove = vec![];
        let delta32 = delta.as_secs_f32();
        self.temp /= Self::COOLING_RATE.powf(delta32);

        for (n, bullet) in self.bullets.iter_mut().enumerate() {
            bullet.update(ctx, delta)?;
            if bullet.pending_destroy() {
                to_remove.push(n);
            }
        }

        for n in to_remove.into_iter().rev() {
            self.bullets.remove(n);
        }

        Ok(())
    }
    pub fn check_asteroids(&mut self, asteroids: &mut AsteroidField) -> f32 {
        let mut points = 0.0;
        for bullet in self.bullets.iter_mut() {
            if let Some((col_vec, n)) = asteroids.check_collision(bullet.pos, Bullet::SIZE) {
                let asteroid = asteroids.asteroids.get_mut(n).unwrap();
                let p = 1.0 - (asteroid.life.min(bullet.life / 2.0) / asteroid.life);
                asteroid.size *= p.sqrt().sqrt();
                let impact = asteroid.life.min(bullet.life);
                asteroid.life -= impact;
                points += impact * asteroid.kind.money_factor();

                //                bullet.life -= asteroid.life;
                bullet.pos.x -= col_vec.dx;
                bullet.pos.y -= col_vec.dy;
                bullet.speed = col_vec.unit().scale(-Bullet::X_SPEED / 2.0);
                bullet.bounced = true;
            }
        }
        points
    }

    pub fn check_collision_bounced(&self, pos: Position, size: f32) -> Option<(Vector, usize)> {
        for (n, bullet) in self.bullets.iter().enumerate() {
            if !bullet.bounced {
                continue;
            }
            if let Some(col_v) = bullet.check_collision(pos, size) {
                return Some((col_v, n));
            }
        }
        None
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for bullet in self.bullets.iter_mut() {
            bullet.draw(ctx)?;
        }

        let rect = graphics::Rect::new(MARGIN_W * 2.0, MARGIN_W * 2.0, Self::MAX_TEMP, 10.0);
        let r1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            rect,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &r1, DrawParam::default())?;
        let color = match self.blinking(false) {
            false => Color::RED,
            true => Color::YELLOW,
        };
        let rect = graphics::Rect::new(MARGIN_W * 2.0, MARGIN_W * 2.0, self.temp, 10.0);
        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        Ok(())
    }
}
