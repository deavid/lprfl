# Project: A simple game with ggez

Now that we know a bit about structs, enums and similar stuff, we're ready for
our next game.

As expected, we will change the game engine to something more appropriate: ggez

<https://crates.io/crates/ggez>

In comparison with Macroquad that we have seen before, this one uses structs. 
Because of this we can organize our code much better and make a game that can 
grow in features without getting too convoluted.

# Space-recycler

A horizontal shooter game (ship) where the goal is to avoid asteroids while
collecting as much trash as possible.

For extra difficulty, we add later different recycling bins and the player
has to put each item in the right bin.
