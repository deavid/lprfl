# Need to write back: &mut and *

We saw functions, and to get the data back we were returning values:

```rust
fn test() -> f64 {
```

Sometimes, we want more than one value to be returned. We can use tuples:

```rust
fn test() -> (f64, f64, f64) {
```

This way multiple values can be returned, as we saw before.

But every time we're creating the values. Sometimes, we want to write on the
original variable.

As you're starting to learn how to code, I would recommend creating new 
variables each time, and maybe use shadowing.

But if this is definitely a bad idea for your use case, we can instead write into
a variable that was passed.

Consider the following code:

```rust
fn add_one(mut n: i64) {
    n += 1;
}

fn main() {
    let x = 5;
    add_one(x);
    println!("{}", x);
}
```

This **does not** add one to `x`. In fact, `x` value will be copied into `n`
inside the function, then it will add one there, and `n` will be discarded.

`x` is still 5 and untouched.

It doesn't matter if `x` is marked as `mut` or not. Is the value that is copied, 
and it is the copy that gets modified, not the original.

Functions provide isolation. They cannot touch the outer variables easily.
This is a feature, not a bug.

But what if we want to modify `x` inside the function? Is it possible?

Yes, but the trick resides in, instead of copying the value, copy the address 
where the value resides.

Let's see how this looks:

```rust
fn add_one(n: &mut i64) {
    *n += 1;
}

fn main() {
    let mut x = 5;
    add_one(&mut x);
    println!("{}", x);
}
```

Here, the clue is in the `&mut` that appears both in `add_one` declaration, and
in the call to that function.

The `&` operator in Rust borrows the variable. Usually for read-only, but as this
is a `&mut`, it will borrow it for write.

> In C++, the `&` operator retrieves the memory address. It is very similar, if
> not identical to what Rust does. But Rust semantics are borrowing, while in C++
> means literally to get the address.

This changes the type, from `i64` to `&mut i64`. This is important, as they are
very different types, even if they look similar. `i64` is a number, where 
`&mut i64` is a mutable borrow of a number.

The variable `n` no longer contains a number, but contains which variable we
borrowed.

```rust
x = 5;
n = &mut x;
// n contains a pointer to x: ->x
```

This is a bit confusing, and for now we will cover only the minimum amount to
get a basic understanding.

For example, this is wrong:
```rust
fn add_one(n: &mut i64) {
    n += 1;
}
```

Wrong, because `n += 1` is not possible here. The variable `n` is not a number.
It points to another variable. Trying to add \\(1\\) means that we want to 
change the variable that is pointing at. And that doesn't make any sense.

To make this work, we need to convert `n` from the type `&mut i64` back into
`i64`.

To do this, we use the `*` (asterisk) operator. Also known as dereference 
operator. This undoes the `&` operator.

```rust
   *n += 1;
```

At this stage I would say it is too early to start using this, but it's good
that you know that this exists and more or less how it's done.

In Rust, the main reason to do this instead of returning the value is when we 
have complex objects like Structs, and it can be quite a deal to return a full
copy. Other times it is just for performance reasons, as for long strings or 
buffers it might be better by using a supplied buffer to write instead of 
creating it each time.

Finally, in some specialized codebases, like programs that must run at real time,
we need to ensure that everything runs without waits. And allocating or freeing
memory has the potential to induce a wait on the operating system.

As said. This is quite special. I don't recommend this. 


