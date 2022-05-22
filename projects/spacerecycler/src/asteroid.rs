use std::time::{Duration, Instant};

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
pub struct Asteroid {
    pub pos: Position,
    pub speed: Vector,
    pub size: f32,
    pub life: f32,
}

impl Default for Asteroid {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(Self::MIN_SIZE..Self::MAX_SIZE);
        Self {
            pos: Position {
                x: WIDTH + MARGIN_W * 5.0,
                y: rng.gen_range(MARGIN_W..HEIGHT - MARGIN_W),
            },
            speed: Vector {
                dx: Self::X_SPEED + rng.gen_range(-Self::Y_SPEED..Self::Y_SPEED),
                dy: rng.gen_range(-Self::Y_SPEED..Self::Y_SPEED),
            },
            life: Self::LIFE_POINTS_X_SIZE * size * size,
            size,
        }
    }
}

impl Asteroid {
    const LIFE_POINTS_X_SIZE: f32 = 2.0;
    const MIN_SIZE: f32 = 16.0;
    const MAX_SIZE: f32 = 32.0;
    const X_SPEED: f32 = -100.0;
    const Y_SPEED: f32 = 10.0;
    const COLOR: Color = Color {
        r: 0.90,
        g: 0.85,
        b: 0.80,
        a: 1.00,
    };
    pub fn update(&mut self, _ctx: &mut Context, delta: Duration) -> GameResult<()> {
        let delta = delta.as_secs_f32();

        self.pos.x += self.speed.dx * delta;
        self.pos.y += self.speed.dy * delta;

        Ok(())
    }
    pub fn pending_destroy(&self) -> bool {
        if self.life <= 0.0 {
            return true;
        }
        if self.pos.x < -self.size {
            return true;
        }
        if self.pos.y > HEIGHT + self.size {
            return true;
        }
        if self.pos.y < -self.size {
            return true;
        }
        false
    }
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let p = ggez::mint::Point2 {
            x: self.pos.x,
            y: self.pos.y,
        };
        let tolerance = 10.0 / self.size; // Max error for precise circles.

        let r1 = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            p,
            self.size,
            tolerance,
            Self::COLOR,
        )?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        let r1 = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(3.0),
            p,
            self.size,
            tolerance,
            Color::BLACK,
        )?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        Ok(())
    }
    pub fn check_collision(&self, pos: Position, size: f32) -> Option<Vector> {
        let v = self.pos.vector_to(&pos);
        let d2 = v.distance_2();
        if d2 >= (self.size + size).powi(2) {
            return None;
        }
        let d = d2.sqrt();
        let col_d = (self.size + size) - d;
        let col_v = v.scale(col_d / d);
        Some(col_v)
    }
}

pub struct AsteroidField {
    pub asteroids: Vec<Asteroid>,
    pub last_asteroid_created: Instant,
}

impl Default for AsteroidField {
    fn default() -> Self {
        Self {
            asteroids: vec![Asteroid::default()],
            last_asteroid_created: Instant::now(),
        }
    }
}

impl AsteroidField {
    const MAX_ASTEROIDS: usize = 20;
    const ASTEROID_CREATION_MIN_TIME: Duration = Duration::from_millis(300);

    pub fn update(&mut self, ctx: &mut Context, delta: Duration) -> GameResult<()> {
        let mut to_remove = vec![];

        for (n, asteroid) in self.asteroids.iter_mut().enumerate() {
            asteroid.update(ctx, delta)?;
            if asteroid.pending_destroy() {
                to_remove.push(n);
            }
        }

        for n in to_remove.into_iter().rev() {
            self.asteroids.remove(n);
            if self.asteroids.len() < Self::MAX_ASTEROIDS {
                self.asteroids.push(Asteroid::default());
            }
        }

        // Collisions?
        for _ in 0..10 {
            let mut collided = false;
            for i in 0..self.asteroids.len() - 1 {
                for j in i + 1..self.asteroids.len() {
                    let a = &self.asteroids[i];
                    let b = &self.asteroids[j];
                    if let Some(col_v) = a.check_collision(b.pos, b.size) {
                        let col_v = col_v.scale(1.0 / 10.0);
                        let size_f = a.size.powi(2) / b.size.powi(2);
                        let a = self.asteroids.get_mut(i).unwrap();
                        a.pos.x += col_v.dx / size_f;
                        a.pos.y += col_v.dy / size_f;
                        a.speed.dx += col_v.dx / size_f;
                        a.speed.dy += col_v.dy / size_f;
                        let b = self.asteroids.get_mut(j).unwrap();
                        b.pos.x -= col_v.dx * size_f;
                        b.pos.y -= col_v.dy * size_f;
                        b.speed.dx -= col_v.dx * size_f;
                        b.speed.dy -= col_v.dy * size_f;
                        collided = true;
                    }
                }
            }
            if !collided {
                break;
            }
        }

        if self.asteroids.len() < Self::MAX_ASTEROIDS
            && self.last_asteroid_created.elapsed() > Self::ASTEROID_CREATION_MIN_TIME
        {
            self.last_asteroid_created = Instant::now();
            self.asteroids.push(Asteroid::default());
        }
        Ok(())
    }

    pub fn check_collision(&self, pos: Position, size: f32) -> Option<(Vector, usize)> {
        for (n, asteroid) in self.asteroids.iter().enumerate() {
            if let Some(col_v) = asteroid.check_collision(pos, size) {
                return Some((col_v, n));
            }
        }
        None
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for asteroid in self.asteroids.iter_mut() {
            asteroid.draw(ctx)?;
        }
        Ok(())
    }
}
