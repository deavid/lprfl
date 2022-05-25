# Project: A simple game with ggez

Now that we know a bit about structs, enums and similar stuff, we're ready for
our next game.

As expected, we will change the game engine to something more appropriate: ggez

<https://crates.io/crates/ggez>

In comparison with Macroquad that we have seen before, this one uses structs. 
Because of this we can organize our code much better and make a game that can 
grow in features without getting too convoluted.

## Space-recycler

A horizontal shooter game (ship) where the goal is to avoid asteroids while
collecting as much trash as possible.

For extra difficulty, we'll have different recycling bins and the player
has to put each item in the right bin. Because this is a game about recycling!

This is a complex project, and it will take quite a bit of time to complete.
It took me around 20 hours to figure out everything, so I guess for someone
that is still learning it could take 1-2 weeks.

But no worries, I will be assisting, so hopefully that cuts back as much 
time as possible.

## Creating the new project

As this is a complete game, we will have it in its own crate (or folder). 
We'll begin by creating it:

```console
$ cargo new spacerecycler
```

We can add already the dependencies we will need in `Cargo.toml`:

```toml
[package]
name = "spacerecycler"
version = "0.1.0"
edition = "2021"

[dependencies]
ggez = "0.7.0"
rand = "0.8"

[profile.dev.package."*"]
opt-level = 2
```

That would add `ggez` at version 0.7 and `rand` at 0.8.

The part for `[profile.dev.package."*"]` is important. That tells Rust how
to compile the dependencies when in debug mode. `opt-level = 2` enables the
optimizer at the same level as for release.

Effectively this makes a debug build to have the libraries compiled as optimized
as possible. And it's important because `ggez` and all the dependencies that it
pulls are related to graphics. And graphics are intensive operations. While
developing, we will need those to be as fast as possible.

However, our code will be still compiled fully in debug mode. This means that
compilation times will be fast, and it will detect most of the errors at runtime
properly.

For now, proceed to build it. This will take a long time so make a cup of tea:

```console
$ cargo build
```

This will download a lot of libraries and compile them. Because of the amount
of stuff that needs to be built, expect 10 minutes of compilation time.

Remember, these dependencies will be built in almost release mode. Building in
release takes more time.

Once it is built, we can go ahead and start some coding.

## The first window

The setup phase is a bit large, so please trust me for a minute.

On `main.rs` copy the following contents:

```rust
mod game;

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
```

We need to create another file `game.rs` with the following:

```rust
struct SpaceRecyclerGame {
    // Your state here...
}

impl SpaceRecyclerGame {
    pub fn new(_ctx: &mut Context) -> SpaceRecyclerGame {
        // Load/create resources such as images here.
        MyGame {
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
        // Draw code here...
        graphics::present(ctx)
    }
}
```

<!-- TODO: test this and rename for a system that routes for menu + game -->

If we execute `cargo run` now, that should present us with a black screen.

The compilation time now it should be quite fast.

There's a lot here. Let's dissect step by step.






