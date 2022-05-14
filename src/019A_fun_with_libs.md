# Having fun with libraries

Doing everything by ourselves might be rewarding, but it is also tiring. It takes a lot of effort and knowledge to code a proper program that is really useful. But I like a saying that goes: "Don't reinvent the wheel"

If you need something, it is highly probable that someone else did it already and shared it for free. Seriously. Most of the time it is just a problem of not googling enough, or using the right keywords.

People share their functions and modules as libraries called "crates" in a site called crates.io.

As a beginner, it is important to use as many crates as you can[^note]. They will allow you to create interesting programs easily, giving you a good sense of progress.
[^note] Of course, don't go crazy and import the whole site. It's not a race to see who imports more crates. Also, as you gain more experience you should be able to keep the number of imported libraries low.


The first library I want you to try is "rand": https://crates.io/crates/rand

This crate as the name suggests creates random numbers. We need to install it in our project, and for that, notice that on the right side of the website there are Install instructions and Documentation:
  

So we will follow the install instructions. We need to open our "Cargo.toml" file first. This file was created when we did "cargo new".

```toml
 [package]
 name = "learnrust"
 version = "0.1.0"
 edition = "2021"
 
 # See more keys and their definitions at ...
 
 [dependencies]
 rand = "0.8.4"   # Add dependencies here
```

We add the new crate to this file by inserting the line `rand = "0.8.4"` as the instructions say just below the `[dependencies]`.

If we execute now "cargo run" we get something different already:

```
 $ cargo run
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.15
   Compiling libc v0.2.107
   Compiling getrandom v0.2.3
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.4
   Compiling learnrust v0.1.0 (/home/deavid/git/rust/learnrust)
    Finished dev [unoptimized + debuginfo] target(s) in 2.54s
     Running `target/debug/learnrust`
 Hello world
```

Cargo noticed that we have added a new dependency, so it downloaded the required libraries and compiled them too! A lot of work that didn't require almost any manual action from our side.

The program however still does the same thing. Just installing a library is not going to make our program generate random numbers.

The simplest way to start with it is to use rand::random():

```rust
 fn main() {
   let randnum: i64 = rand::random();
   println!("Hello world: {}", randnum);
 }
```

We need to define the type of randnum this time. The reason is that this library detects where we want to save it and creates a random number as big as possible that does fit.

The result is:
 
``` 
 deavid@debian:~/git/rust/learnrust$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/learnrust`
 Hello world: 1976599895379426978
 deavid@debian:~/git/rust/learnrust$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/learnrust`
 Hello world: -2920297650750248329
 deavid@debian:~/git/rust/learnrust$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/learnrust`
 Hello world: 8971769657686972258
 deavid@debian:~/git/rust/learnrust$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/learnrust`
 Hello world: -8688429354790802443
```

As we can see it creates very big numbers, both positive and negative. A different random number is generated in each run.

We could make the number positive and smaller by choosing "u8" instead of "i64":

```rust
   let randnum: u8 = rand::random();
```

This creates the following results:

```
 Hello world: 140
 Hello world: 215
 Hello world: 99
 Hello world: 221
```

Because u8 is unsigned and can hold numbers between 0 and 255 (2⁸-1), this is the range that we get.

We can instead ask this library for a specific range:


```rust
 use rand::Rng;


 fn main() {
   let mut rng = rand::thread_rng();
   let randnum: i64 = rng.gen_range(1..=100);
   println!("Hello world: {}", randnum);
 }
```

In this case we need to create a random number generator object (rng). This will be holding the internal state of the random generator. It needs to be mutable because the state changes as each number is generated, so the numbers aren't repeated every time.

The gen_range(1..=100) specifies which range of numbers to retrieve, like in a for loop, but in this case we only get a single number, randomly.

The "use rand::Rng" on the top is needed to access the gen_range method. This is what Rust calls a Trait. We will go over these later on.

