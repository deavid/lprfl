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
}

impl Bullet {
    const X_SPEED: f32 = 1500.0;
    const Y_SPEED: f32 = 20.0;
    const TRAIL_DIST_SECS: f32 = 1.0 / 20.0;
    const LIFE_CONSUMPTION_RATE: f32 = 300.0;
    const SIZE: f32 = 5.0;
    const LIFE: f32 = 300.0;
    const COLOR: Color = Color {
        r: 1.00,
        g: 0.85,
        b: 0.00,
        a: 1.00,
    };
    const TRAIL_COLOR: Color = Color {
        r: 1.00,
        g: 0.85,
        b: 0.00,
        a: 0.05,
    };
    pub fn new(player: &Ship) -> Self {
        let mut rng = rand::thread_rng();
        let pos = Position {
            x: player.pos.x + Ship::SIZE_X / 2.0 + rng.gen_range(0.0..Self::X_SPEED / 30.0),
            y: player.pos.y,
        };
        let speed = Vector {
            dx: Self::X_SPEED + player.speed.dx,
            dy: rng.gen_range(-Self::Y_SPEED..Self::Y_SPEED) + player.speed.dy,
        };
        Self {
            pos,
            speed,
            life: Self::LIFE,
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
        let p = ggez::mint::Point2 {
            x: self.pos.x,
            y: self.pos.y,
        };
        if self.speed.distance_2() > 1000.0 {
            for f in 1..10 {
                let f = f as f32 / 10.0;
                let last_pos = ggez::mint::Point2 {
                    x: self.pos.x - self.speed.dx * Self::TRAIL_DIST_SECS * f,
                    y: self.pos.y - self.speed.dy * Self::TRAIL_DIST_SECS * f,
                };
                let l = graphics::Mesh::new_line(
                    ctx,
                    &[last_pos, p],
                    Self::SIZE * (1.0 - f / 2.0),
                    Self::TRAIL_COLOR,
                )?;
                graphics::draw(ctx, &l, DrawParam::default())?;
            }
        }

        let tolerance = 10.0 / Self::SIZE; // Max error for precise circles.

        let r1 = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            p,
            Self::SIZE,
            tolerance,
            Self::COLOR,
        )?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        Ok(())
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
    const MAX_TEMP: f32 = 100.0;
    const TEMP_SHOT: f32 = 0.4;
    const COOLING_RATE: f32 = 1.03;
    pub fn shoot(&mut self, player: &Ship) {
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
        self.bullets.push(Bullet::new(player));
        self.temp += Self::TEMP_SHOT;
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
    pub fn check_asteroids(&mut self, asteroids: &mut AsteroidField) {
        for bullet in self.bullets.iter_mut() {
            if let Some((col_vec, n)) = asteroids.check_collision(bullet.pos, Bullet::SIZE) {
                let asteroid = asteroids.asteroids.get_mut(n).unwrap();
                asteroid.life -= bullet.life;
                //                bullet.life -= asteroid.life;
                bullet.pos.x -= col_vec.dx;
                bullet.pos.y -= col_vec.dy;
                bullet.speed = col_vec.unit().scale(-Bullet::X_SPEED / 2.0);
            }
        }
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

        let rect = graphics::Rect::new(MARGIN_W * 2.0, MARGIN_W * 2.0, self.temp, 10.0);
        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::RED)?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        Ok(())
    }
}
