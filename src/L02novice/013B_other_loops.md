# Other types of loops

So far we have seen `for` loops. Rust has other ways of doing loops that 
sometimes are better than `for`.

## Loop 

There is the infinite loop that is written by `loop`. But a program that has
an infinite loop never ends and might become stuck forever. Therefore,
when using these types of loops we need to make sure we break that loop at
some point.

For example:

```rust
let mut counter = 1;

loop {
    if counter < 100 {
        counter *= 2;
    }
    if counter < 130 {
        counter += 1;
    } else {
        // important to include a `break` to stop the loop at some point.
        break;
    }
    println!("counter = {}", counter);
}
println!("ended with counter = {}", counter);
```

## While

The other type of loop is `while` and does the same as `loop` but has a 
condition that must stay true to continue looping.

```rust
let mut counter = 1;

while counter < 20 {
    counter += 1;
    println!("counter = {}", counter);
}

println!("ended with counter = {}", counter);
```

It also allows to retrieve a value in-place on the comparison, like this:

```rust
let test = "Hello World!";
let mut chars = test.chars();

while let Some(c) = chars.next() {
    println!("c = {:?}", c);
}
```

`text.chars()` returns an iterator over the characters of `test`. Every time we
call `chars.next()` it returns an `Option<char>` that can be either `Some(char)`
or `None`.

The `while` loop will keep iterating until this function returns `None`. The 
result of the `Some` is stored in `c`, and we can print each 
character individually.

But I should insist on using `for .. in` where possible, because it tends to be
more legible:

```rust
let test = "Hello World!";

for c in test.chars() {
    println!("c = {:?}", c);
}
```
