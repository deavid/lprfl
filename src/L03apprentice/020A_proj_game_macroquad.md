# Project: A simple game with Macroquad

Macroquad is a simple game engine aimed for beginners that only requires minimal
knowledge of Rust:

<https://crates.io/crates/macroquad>

We will use this game engine only for this game, then we will shift to something
more powerful. This means that in the process you'll have to learn the API of
this to never use it again, but in fairness, it is pretty simple.

The alternative would be to not do this project now and keep reading theory.
But that's not fun, and we should put the current knowledge into use, so it 
becomes the foundation for the next things that will come.


# Survival pong

A pong game where the player just makes the ball bounce against the wall. It 
scores for each wall hit (even top-down walls).

Every time the ball hits the paddle, it speeds up a bit.

Logically, the tactic has to be to make each shot hitting as many walls as 
possible before returning to the paddle.
