# dbg!() and format!()

Printing in console is nice. But sometimes we want just to quickly see what
are the contents of a variable or operation.

For this we can use the `dbg!()` macro[^1]:

```rust
let value = 123.0;
dbg!(value);

dbg!(1.5 * 3.2);
```

This prints:
```
??? TBD ??
```

This is quite useful! Avoids writing a lot and does what we need to understand
what the code is doing at different points.

## format!()

Here's another case. Sometimes we want the ability that `println!()` gives us
regarding on composing a string, but we don't want to print it. We want to 
store it.

For this, we have the `format!()` macro, that works exactly as `println!()` but
instead of printing it returns the result as a string:

```rust

let text = format!("your lucky number is {:05}. Try another day!", 7);

dbg!(text);
```

Nice!


--------
[^1] Looks like a function, you can think of it as a function, but to be correct,
things that end with `!` are called Macros. We won't see how to make these until
very late in (if at all), as you don't need to make macros to code 99.999% of 
programs.