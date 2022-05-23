use crate::asteroid::AsteroidField;
use crate::asteroid::AsteroidKind;
use crate::bullet::Bullet;
use crate::bullet::MachineGun;
use crate::player;
use crate::player::Ship;
use crate::player::ShipAction;
use crate::sfx::Sfx;
use crate::HEIGHT;
use crate::MARGIN_W;
use crate::WIDTH;
use ggez::event;
use ggez::event::EventHandler;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::input::keyboard;
use ggez::Context;
use ggez::GameResult;
use std::cmp::Ordering;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Menu,
    Playing,
}

pub struct SpaceRecyclerGame {
    // Your state here...
    pub player_ship: player::Ship,
    pub asteroids: AsteroidField,
    pub bullets: MachineGun,
    pub last_update: Instant,
    pub score: f32,
    pub prev_score: f32,
    pub sfx: Sfx,
    pub status: GameStatus,
}

impl SpaceRecyclerGame {
    const COLOR_BG: Color = Color {
        r: 0.10,
        g: 0.15,
        b: 0.20,
        a: 1.0,
    };
    pub fn new(_ctx: &mut Context) -> GameResult<SpaceRecyclerGame> {
        // Load/create resources such as images here.
        Ok(SpaceRecyclerGame {
            player_ship: player::Ship::default(),
            asteroids: AsteroidField::default(),
            bullets: MachineGun::default(),
            last_update: Instant::now(),
            score: 0.0,
            prev_score: 0.0,
            sfx: Sfx::new()?,
            status: GameStatus::Menu,
            // ...
        })
    }
}

impl SpaceRecyclerGame {
    fn start_game(&mut self) {
        self.player_ship = player::Ship::default();
        self.asteroids = AsteroidField::default();
        self.bullets = MachineGun::default();
        self.last_update = Instant::now();
        self.score = 0.0;
        self.prev_score = 0.0;
        self.player_ship.lifes = 10;
        self.player_ship.old_lifes = 10;
    }
}

impl EventHandler for SpaceRecyclerGame {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            if self.status == GameStatus::Playing {
                self.status = GameStatus::Menu;
            } else {
                event::quit(ctx);
            }
        }
        if self.status == GameStatus::Menu && keycode == KeyCode::Space {
            self.status = GameStatus::Playing;
            self.bullets.last_shot = Instant::now() + Duration::from_secs_f32(1.0);
            if self.player_ship.lifes == 0 {
                self.start_game();
            }
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        let delta = self.last_update.elapsed();
        let delta32 = delta.as_secs_f32();
        self.last_update = Instant::now();

        if self.status != GameStatus::Playing {
            if self.player_ship.lifes == 0 {
                self.asteroids.update(ctx, delta)?;
                self.asteroids.update(ctx, delta)?;
            }

            return Ok(());
        }
        if self.player_ship.lifes == 0 {
            self.status = GameStatus::Menu;
        }
        self.prev_score = (self.score * delta32 * 4.0 + self.prev_score) / (1.0 + delta32 * 4.0);
        if (self.prev_score - self.score).abs() < 0.5 {
            self.prev_score = self.score;
        }

        let old_action = self.player_ship.action;
        self.player_ship.action = ShipAction::None;
        if !self.bullets.overheated() {
            if keyboard::is_key_pressed(ctx, KeyCode::V) {
                self.player_ship.action = ShipAction::Turbo;
                if old_action != ShipAction::Turbo {
                    self.sfx.turbo(ctx)?;
                }
                self.bullets.temp += 60.0 * delta32;
            }
            if keyboard::is_key_pressed(ctx, KeyCode::Z)
                || keyboard::is_key_pressed(ctx, KeyCode::Y)
            {
                self.player_ship.action = ShipAction::Collector(AsteroidKind::Aluminium);
                self.bullets.temp += 30.0 * delta32;
            }
            if keyboard::is_key_pressed(ctx, KeyCode::X) {
                self.player_ship.action = ShipAction::Collector(AsteroidKind::Plastic);
                self.bullets.temp += 30.0 * delta32;
            }
            if keyboard::is_key_pressed(ctx, KeyCode::C) {
                self.player_ship.action = ShipAction::Collector(AsteroidKind::Cardboard);
                self.bullets.temp += 35.0 * delta32;
            }
            if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                self.player_ship.action = ShipAction::GunFire;
                self.bullets.shoot(&self.player_ship);
            }
        }
        if self.bullets.overheated() {
            self.player_ship.action = ShipAction::Overheat;
        }

        let collector_score = self.player_ship.collector.check_asteroids(
            self.player_ship.pos,
            &mut self.asteroids,
            delta,
        );

        self.score -= self.bullets.check_asteroids(&mut self.asteroids) / 10.0;
        if self.score < 0.0 {
            self.score = 0.0;
        }

        match collector_score {
            Some(score) => {
                if score > 0.0 {
                    self.score += score;
                    self.sfx.collectorw1(ctx)?;
                } else {
                    self.score += score / 10.0;
                    self.sfx.collectorw2(ctx)?;
                }
            }
            None => {
                if let Some((c_vec, num)) = self
                    .asteroids
                    .check_collision(self.player_ship.pos, Ship::SIZE_Y)
                {
                    let ast = self.asteroids.asteroids.get_mut(num).unwrap();
                    ast.speed.dx += c_vec.dx;
                    ast.speed.dy += c_vec.dy;
                    self.player_ship.speed.dx -= c_vec.dx;
                    self.player_ship.speed.dy -= c_vec.dy;
                    self.player_ship.consume_life();
                }
            }
        }

        if let Some((c_vec, num)) = self
            .bullets
            .check_collision_bounced(self.player_ship.pos, Ship::SIZE_Y)
        {
            let bullet = self.bullets.bullets.get_mut(num).unwrap();
            if bullet.life > Bullet::LIFE / 4.0 {
                self.player_ship.consume_life();
            }
            let old_speed = bullet.speed.distance();
            let new_speed = c_vec.unit().scale(old_speed / 2.0);
            bullet.speed = new_speed;
            bullet.life /= 2.0;
        }

        if let ShipAction::Collector(_) = self.player_ship.action {
            self.sfx.collector(ctx, true)?;
        } else {
            self.sfx.collector(ctx, false)?;
        }
        // Maybe increase quality of simulation by doing smaller delta steps.
        self.asteroids.update(ctx, delta)?;
        self.player_ship.update(ctx, delta)?;
        self.bullets.update(ctx, delta)?;

        self.bullets.play(ctx, &mut self.sfx)?;
        self.player_ship.play(ctx, &mut self.sfx)?;
        self.asteroids.play(ctx, &mut self.sfx)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Self::COLOR_BG);
        // Draw code here...
        self.asteroids.draw(ctx)?;
        if self.player_ship.lifes > 0 {
            self.bullets.draw(ctx)?;
            self.player_ship.draw(ctx)?;
        }

        let rect = graphics::Rect::new(
            MARGIN_W,
            MARGIN_W,
            WIDTH - MARGIN_W * 2.0,
            HEIGHT - MARGIN_W * 2.0,
        );
        let r1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(6.0),
            rect,
            Self::COLOR_BG,
        )?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        let r1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(3.0),
            rect,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        // KEYS INFO
        let t = graphics::Text::new(" COLLECTOR        MOVEMENT          FIRE");
        let b = t.dimensions(ctx).bottom();

        let dest = ggez::mint::Vector2 {
            x: MARGIN_W * 2.0,
            y: HEIGHT - MARGIN_W * 3.0 - b,
        };
        graphics::draw(ctx, &t, DrawParam::default().dest(dest))?;

        let t = graphics::Text::new("Z/Y   X   C       ↑ ↓ ← → V         SPACE");
        let b = t.dimensions(ctx).bottom();

        let dest = ggez::mint::Vector2 {
            x: MARGIN_W * 2.0 + 1.0,
            y: HEIGHT - MARGIN_W * 2.0 - b + 1.0,
        };
        graphics::draw(ctx, &t, DrawParam::default().dest(dest).color(Color::BLACK))?;

        let t = graphics::Text::new("                  ↑ ↓ ← → V         SPACE");
        let b = t.dimensions(ctx).bottom();

        let dest = ggez::mint::Vector2 {
            x: MARGIN_W * 2.0,
            y: HEIGHT - MARGIN_W * 2.0 - b,
        };
        graphics::draw(ctx, &t, DrawParam::default().dest(dest))?;

        let t = graphics::Text::new("Z/Y");
        let b = t.dimensions(ctx).bottom();

        let dest = ggez::mint::Vector2 {
            x: MARGIN_W * 2.0,
            y: HEIGHT - MARGIN_W * 2.0 - b,
        };
        graphics::draw(
            ctx,
            &t,
            DrawParam::default()
                .dest(dest)
                .color(AsteroidKind::Aluminium.color()),
        )?;
        let t = graphics::Text::new("      X");
        let b = t.dimensions(ctx).bottom();

        let dest = ggez::mint::Vector2 {
            x: MARGIN_W * 2.0,
            y: HEIGHT - MARGIN_W * 2.0 - b,
        };
        graphics::draw(
            ctx,
            &t,
            DrawParam::default()
                .dest(dest)
                .color(AsteroidKind::Plastic.color()),
        )?;
        let t = graphics::Text::new("          C");
        let b = t.dimensions(ctx).bottom();

        let dest = ggez::mint::Vector2 {
            x: MARGIN_W * 2.0,
            y: HEIGHT - MARGIN_W * 2.0 - b,
        };
        graphics::draw(
            ctx,
            &t,
            DrawParam::default()
                .dest(dest)
                .color(AsteroidKind::Cardboard.color()),
        )?;

        if self.status == GameStatus::Menu {
            let t = if self.player_ship.lifes == 0 {
                graphics::Text::new("--- PRESS SPACE TO START ---")
            } else {
                graphics::Text::new("--- PRESS SPACE TO RESUME ---")
            };
            let c = t.dimensions(ctx).center();
            let dest = ggez::mint::Vector2 {
                x: WIDTH / 2.0 - c.x,
                y: HEIGHT / 2.0 - c.y,
            };

            graphics::draw(ctx, &t, DrawParam::default().dest(dest))?;
            if self.score > 0.0 {
                let t = graphics::Text::new(format!("SCORE: {:08.0}", self.score));
                let c = t.dimensions(ctx).center();
                let dest = ggez::mint::Vector2 {
                    x: WIDTH / 2.0 - c.x,
                    y: HEIGHT / 2.0 - c.y + MARGIN_W * 2.0,
                };

                graphics::draw(ctx, &t, DrawParam::default().dest(dest))?;
            }
        }

        if self.status == GameStatus::Playing {
            let t = graphics::Text::new(format!("SCORE: {:08.0}", self.prev_score));
            let dest = ggez::mint::Vector2 { x: 600.0, y: 30.0 };
            let baseparam = DrawParam::default().dest(dest);
            let drawparam = match self
                .score
                .partial_cmp(&self.prev_score)
                .unwrap_or(Ordering::Equal)
            {
                Ordering::Less => baseparam.color(Color::RED),
                Ordering::Equal => baseparam.dest(dest),
                Ordering::Greater => baseparam.dest(dest).color(Color::GREEN),
            };
            if self.score != self.prev_score {
                self.sfx.money(ctx, self.score - self.prev_score)?;
            }

            graphics::draw(ctx, &t, drawparam)?;

            let t = graphics::Text::new(self.player_ship.action.to_string());
            let dest = ggez::mint::Vector2 { x: 600.0, y: 50.0 };
            graphics::draw(ctx, &t, DrawParam::default().dest(dest))?;
        }
        // Now display:
        graphics::present(ctx)
    }
}
