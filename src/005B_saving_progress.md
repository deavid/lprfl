# Saving our progress

When reading this book, you'll find lots of small recipes to try out. You can
put them in your `main.rs` file and execute `cargo run`, but soon you'll find
that you need to delete your old code to put the new one.

And you might not want to keep removing the old code. That's understandable!

I would prefer to have something for you to build incrementally, but sadly at
this point it's not possible. We need to learn the basics for a while before I 
can give you any sort of tasks. So for the next chapters, we'll be using small
programs that are easy to understand. A few lines only each time.

But you, the reader, might want to keep those samples to play around later. And
avoid deleting them when trying out something new.

I have a solution for you, but you'll have to trust me here.

## Multiple binaries with Cargo

First, create the folder `learnrust/src/bin/`, and add a file named `sample1.rs`
in it.

In this file `learnrust/src/bin/sample1.rs`, add the following contents:

```rust
fn main() {
    println!("sample program 1");
}
```

Now open `Cargo.toml` file, and add the following lines:

```toml
[[bin]]
name = "sample1"
```

Your file now should look like this:

```toml
[package]
name = "learnrust"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at ...

[[bin]]
name = "sample1"

[dependencies]
```

With this done, we now have two programs in one project folder `learnrust/`.

## Executing the new program

The new program we created is called "sample1", and since we have two programs, 
we now need to specify to `cargo run` which program to run.

If we try to execute `cargo run` as usual, it will fail with this:

```console
$ cargo run 
error: `cargo run` could not determine which binary to run. 
Use the `--bin` option to specify a binary, or the `default-run` manifest key.
available binaries: learnrust, sample1
```
> Please read the error messages. Carefully. 99.9% of the time we get stuck
> because we don't pay attention to what the error is telling us.

To fix this, we run instead `cargo run --bin sample1`:

```console
$ cargo run --bin sample1
   Compiling learnrust v0.1.0 (/home/deavid/git/rust/lprfl/learnrust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/sample1`
sample program 1
```

Now we executed the new program.

Each time we want to add a new program, we just create another file in the 
`bin/` folder, add it to `Cargo.toml` as we just did, and voilà! We can have as 
many programs as we want.

## Executing the old program

Now, as you noticed, the old program in `main.rs` can no longer be executed by
running `cargo run`.

Instead, we will need to run `cargo run --bin learnrust`:

```console
$ cargo run --bin learnrust
   Compiling learnrust v0.1.0 (/home/deavid/git/rust/lprfl/learnrust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/learnrust`
Hello, world!
```
