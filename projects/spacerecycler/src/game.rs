use crate::asteroid::AsteroidField;
use crate::bullet::MachineGun;
use crate::player;
use crate::player::Ship;
use crate::HEIGHT;
use crate::MARGIN_W;
use crate::WIDTH;
use ggez::event::EventHandler;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::input::keyboard;
use ggez::Context;
use ggez::GameResult;
use std::time::Instant;

pub struct SpaceRecyclerGame {
    // Your state here...
    // lives: i64,
    pub player_ship: player::Ship,
    pub asteroids: AsteroidField,
    pub bullets: MachineGun,
    pub last_update: Instant,
}

impl SpaceRecyclerGame {
    const COLOR_BG: Color = Color {
        r: 0.10,
        g: 0.15,
        b: 0.20,
        a: 1.0,
    };
    pub fn new(_ctx: &mut Context) -> SpaceRecyclerGame {
        // Load/create resources such as images here.
        SpaceRecyclerGame {
            player_ship: player::Ship::default(),
            asteroids: AsteroidField::default(),
            bullets: MachineGun::default(),
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

        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.bullets.shoot(&self.player_ship);
        }
        self.bullets.check_asteroids(&mut self.asteroids);
        if self
            .asteroids
            .check_collision(self.player_ship.pos, Ship::SIZE_Y)
            .is_some()
        {
            self.player_ship.consume_life();
        }

        if self
            .bullets
            .check_collision_bounced(self.player_ship.pos, Ship::SIZE_Y)
            .is_some()
        {
            self.player_ship.consume_life();
        }
        // Maybe increase quality of simulation by doing smaller delta steps.
        self.asteroids.update(ctx, delta)?;
        self.player_ship.update(ctx, delta)?;
        self.bullets.update(ctx, delta)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Self::COLOR_BG);
        // Draw code here...

        self.asteroids.draw(ctx)?;
        self.bullets.draw(ctx)?;
        self.player_ship.draw(ctx)?;

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

        // Now display:
        graphics::present(ctx)
    }
}
