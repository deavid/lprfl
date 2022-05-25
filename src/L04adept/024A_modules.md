# Modules - splitting files

Having all the program into a single file stops being convenient as the program
gets bigger and bigger. One hundred lines are fine, 1000 maybe, but 5000 is 
definitely too big to handle.

The problem is as the file grows we easily lose track of things, in which parts 
they were and so on.

The whole folder that `cargo new` created for us is called a project. So if we
recall when we created the initial project:

```console
$ cargo new learnrust
     Created binary (application) `learnrust` package
```

But so far we have been adding different binaries, and with that we have been 
creating different files.

```console
src
├── bin
│   ├── enums2.rs
│   ├── enums.rs
│   ├── hashmap.rs
│   ├── print.rs
│   ├── sample1.rs
│   ├── structs.rs
│   ├── turtle_loops.rs
│   ├── turtle_vars.rs
│   └── variables.rs
└── main.rs
```

However, these are not modules! Those are binaries.

Modules can be reused between different files while binaries can only be 
compiled as full programs. We cannot get a function from a binary into a 
different one.

To share functionality we need to move it into modules.

For example, let's say we need something to be reused. Maybe we want a file that
contains units of measure.

So we'll create a new file called `src/units.rs`.

And we add a few units of length. Or all of them now that we're on this:

```rust
pub enum LengthUnit {
    Kilometer,
    Meter,
    Millimeter,
    Inch,
    Angstrom,
    Mile,
    Furlong,
    Chain,
    Rod,
    Fathom,
    Yard,
    Foot,
    Parsec,
    LightYear,
    AstronomicalUnit,
}

```

Yay! We got a new module. Or do we?

So in order to be a full module we should be importing this from another file.

For `src/main.rs` we can add the following line at the top:

```rust
mod units; // <-- this line!

fn main() {
    println!("Hello, world!");
}
```

This will declare the module. Now we can use `units::LengthUnit`:

```rust
mod units;

fn main() {
    println!("Hello, world!");

    let m = units::LengthUnit::LightYear;
    dbg!(m);
}
```

This is difficult to type down, so we can add a `use` line to bring `LengthUnit`
into scope:

```rust
use units::LengthUnit;
```

Then we can just do:

```rust
let m = LengthUnit::LightYear;
```

And that's it! We're sharing code across files.

We just did a simple enum, but it can be constants, structs, functions, whatever.

## Adapting for binaries in src/bin

For the programs that we have inside the `src/bin/` folder this does not work
as Rust is expecting the modules to also live in `src/bin/`.

Unless you want to have a messy code where all the files are on this folder, we
need a way to tell Rust to read in the parent folder instead.

To fix this, we need to create a file named `src/lib.rs` (in the parent folder).

In this file we will have only the following lines:

```rust
pub mod units;
pub use units::LengthUnit;
```

This will enable us to use the modules in the whole package.

Let's create a new binary for testing this. We will call this
`src/bin/conversor.rs`.

Remember to update `Cargo.toml` to add this binary.

As usual, we need a `fn main()` to start a new binary: 

```rust
fn main() {
    // ...
}
```

Notice how `conversor.rs` does need a main, but `units.rs` does not.

Now we can bring to scope what we want. Feel free to choose the line you prefer:

```rust
use learnrust::units;
use learnrust::LengthUnit;
```

In my case, I think I'll settle with `use learnrust::LengthUnit;` as it's more
convenient.

```rust
use learnrust::LengthUnit;

fn main() {
    let furlong = LengthUnit::Furlong;
    dbg!(furlong);
}
```

Let's try `cargo run --bin conversor`:

```console
$ cargo run --bin conversor
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/conversor`
[src/bin/conversor.rs:5] furlong = Furlong
```

Yay! This works!




