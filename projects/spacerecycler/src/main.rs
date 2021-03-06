mod asteroid;
mod bullet;
mod game;
mod player;
mod sfx;
mod vector;

use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::event;
use ggez::ContextBuilder;

use game::SpaceRecyclerGame;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 600.0;
const MARGIN_W: f32 = 15.0;

fn main() {
    let window_cfg = WindowSetup {
        title: "Space Recycler".to_owned(),
        vsync: false,
        samples: NumSamples::Four,
        ..Default::default()
    };
    let window_mode = WindowMode {
        width: WIDTH,
        height: HEIGHT,
        maximized: false,
        resizable: false,
        visible: true,
        resize_on_scale_factor_change: false,
        ..Default::default()
    };

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("space_recycler", "Cool Game Author")
        .window_setup(window_cfg)
        .window_mode(window_mode)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let space_recycler = SpaceRecyclerGame::new(&mut ctx).expect("Error intializing game");

    // Run!
    event::run(ctx, event_loop, space_recycler);
}
