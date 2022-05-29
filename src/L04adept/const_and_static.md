# Const and Static

So far we saw `let` and `let mut`. If it's not `mut`, it cannot be changed, right?

Well, we have other two ways of creating variables: `const` and `static`.

Both have something in common: They can be used to declare names that can be
used across all your program, meaning, that they don't need to be tied to a 
particular function or scope.

Programming languages typically had global variables, those that you can use
across all functions. But Rust almost doesn't have any global variable.

One of the biggest pain points for people that already know how to code in 
another language, and they come to Rust is the expectation to create global 
variables, and they find it near impossible to do.

## Const 

First, let's explain what `const` does. It stands for "constant", and this
means that the value they contain cannot be changed.

We can do something like:

```rust
const NUM_THREADS:u32 = 6;

fn main() {
    for n in 0..NUM_THREADS {
        println!("Creating thread {}...", n + 1);
    }
}
```

Upon compiling, Rust will basically replace all instances where you used a 
constant with the actual value:

```rust
// const NUM_THREADS:u32 = 6;

fn main() {
    for n in 0..6 {
        println!("Creating thread {}...", n + 1);
    }
}
```

Also, `const` must have a type. You can't do:

```rust
const NUM_THREADS = 6;
```

We must do:

```rust
const NUM_THREADS:u32 = 6;
```

If you have a complex struct, it will recreate the struct in every place.
For these uses, `static` will be better suited.

```rust
const ADMIN: User = User {
    id: 1,
    active: true,
    posts: 0,
    // ... 50 more fields ...
}
```

But constants must be values that can be obtained at compile time. Values that
require running code at runtime will not work as constants:

```rust
fn calc_pi() -> f64 {
    // ...
    return pi;
}

const PI: f64 = calc_pi(); // <- this doesn't work!
```

Some values surprisingly need to be computed at runtime. For example, vectors.

```rust
const LUCKY_NUMBERS: Vec<i64> = vec![1,6,32];
```

That's because it really equates to:

```rust
const LUCKY_NUMBERS: Vec<i64> = {
    let mut v: Vec<i64> = Vec::new();
    v.push(1);
    v.push(6);
    v.push(32);
};
```

You can do arrays:

```rust
const LUCKY_NUMBERS: [i64; 3] = [1,6,32];
```

But again, this will create the array each time you use the name.

It might be a bit puzzling, but you can create a constant text if it's a &str,
but not if it is a String. 

Or maybe it's easy to understand that this is a constant value:

```rust
"Hello world"
```

But this is not:

```rust
"Hello world".to_owned()
```

It needs a function call, therefore it can't be executed in compile time.

## Const functions

For the sake of understanding I want to just mention that there exists something
called constant functions.

Instead of:

```rust
fn function_name() {
```

They have the `const` keyword:

```rust
const fn function_name() {
```

These functions can be executed at compile time, and will produce *const* values.

The Rust standard library has a few of those and can be used to create new
constant values.

However, they are very limited. Almost every function can't be const because 
they do things that Rust can't execute at compile time.

I won't teach in this book how to create these functions, but we can use already
existing ones to create constant values.

## Static

While `const` could be created inside a function or globally in the file, 
`static` can also be created inside a function or globally.

Static variables are full-blown variables like `let`. They live in your computer
RAM, and they have a memory address.

However, the values they can be initialized to are mostly the same as for `const`.

The value must be able to be obtained at compile time, for example, as a result
of a const function.

Global variables can be modified at runtime in other languages. But not in Rust.

Confusingly, Rust does allow you to have a `static mut`, but it doesn't allow
you to change it.

```rust
statuc mut PAGE_HITS: i32 = 0;

fn main() {
    PAGE_HITS += 1; // error!
}
```

This is because changing a static variable is unsafe. It can be done by using
an unsafe block:

```rust
statuc mut PAGE_HITS: i32 = 0;

fn main() {
    unsafe {
        PAGE_HITS += 1;
    }
}
```

While this does work, this is very bad practice. If you use `unsafe`, you must 
know what you're doing. And turns out that even experienced programmers don't
really know what problems might arise from doing these things.

So the best thing to do is to avoid `unsafe` altogether and don't write into
`static`.

Global variables are a symptom of a bad design. If you find yourself trying to
do a mutable static, think how to change your program to convert it into regular
local variables.

There are other, safe ways, of having mutable statics. We will look onto them
later on.
