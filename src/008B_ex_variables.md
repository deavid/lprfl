# Exercise: Variables

In the last exercise we saw lots of ways to print, but to be fair, using
placeholders doesn't make much sense if we have to write what goes inside, right?

So in this one we'll explore how to combine the previous things we learned with 
variables.

First things first, we'll create a new program again. This time we will call it
`variables.rs`. 

So go ahead and create it in the folder `learnrust/src/bin/`.

As usual, we will start with `fn main() {}`:

```rust
fn main() {

}
```

Now go to `Cargo.toml` and add:

```toml
[[bin]]
name = "variables"
```

Test that it works by running:

```console
$ cargo run --bin variables
```

# Hello, Waldo!

Let's try a variation of the mythical "Hello, World!" program:

```rust
{{#include ../learnrust/src/bin/variables.rs:waldo}}
```

And we can try a few variables with numbers and math:

```rust
{{#include ../learnrust/src/bin/variables.rs:math}}
```

Let's try to mutate one variable over and over:
```rust
{{#include ../learnrust/src/bin/variables.rs:mut}}
```

We can also do operations with variables:
```rust
{{#include ../learnrust/src/bin/variables.rs:sum}}
```

> HINT: Remember you have the play button/icon on each code block to execute 
> these samples in your browser.

## Done!

Here's the full program:
```rust
{{#include ../learnrust/src/bin/variables.rs:all}}
```

The output is:
```
Hello, Waldo!
The result of 3 * 7 is 21
4
6
4
4 + 3 = 7
```